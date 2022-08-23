extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn function(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let instrumented_function_name = function.sig.ident.to_string();

    let body = &function.block;
    let new_body = impl_block(body, &instrumented_function_name);

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}

fn impl_block(
    body: &syn::Block,
    _instrumented_function_name: &str,
) -> syn::Block {
    let mut stream = proc_macro2::TokenStream::new();

    #[cfg(feature = "profile-with-puffin")]
    stream.extend(quote!{ profiling::puffin::profile_function!(); });

    #[cfg(feature = "profile-with-optick")]
    stream.extend(quote!{ profiling::optick::event!(); });

    #[cfg(feature = "profile-with-superluminal")]
    stream.extend(quote!{ 
        let _superluminal_guard = profiling::superluminal::SuperluminalGuard::new(#_instrumented_function_name);
    });

    #[cfg(feature = "profile-with-tracing")]
    stream.extend(quote!{ 
        let _tracing_span = profiling::tracing::info_span!(#_instrumented_function_name);
    });

    #[cfg(feature = "profile-with-tracy")]
    stream.extend(quote!{ 
        let _tracy_span = profiling::tracy_client::span!(#_instrumented_function_name, 0);
    });

    parse_quote! {
        {

            #stream
            #body
        }
    }
}
