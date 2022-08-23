//
// To use this library, enable one of the feature flags. Each backend implementation provides the
// exact same interface. Only one may be active at a time.
//

/// Proc macro for creating a scope around the function, using the name of the function for the
/// scope's name
///
/// This must be done as a proc macro because tracing requires a const string
///
/// ```
/// #[profiling::function]
/// fn my_function() {
///
/// }
/// ```
#[cfg(feature = "procmacros")]
pub use profiling_procmacros::function;

#[cfg(feature = "profile-with-puffin")]
pub use puffin;
#[cfg(feature = "profile-with-puffin")]
mod puffin_impl;

#[cfg(feature = "profile-with-optick")]
pub use optick;
#[cfg(feature = "profile-with-optick")]
mod optick_impl;

#[cfg(feature = "profile-with-superluminal")]
pub use superluminal_perf;
#[cfg(feature = "profile-with-superluminal")]
mod superluminal_impl;
#[cfg(feature = "profile-with-superluminal")]
pub use superluminal_impl::*;

#[cfg(feature = "profile-with-tracing")]
pub use tracing;
#[cfg(feature = "profile-with-tracing")]
mod tracing_impl;

#[cfg(feature = "profile-with-tracy")]
pub use tracy_client;
#[cfg(feature = "profile-with-tracy")]
mod tracy_impl;

#[macro_export]
macro_rules! scope {
    ($($tt:expr),*) => {
        #[cfg(feature = "profile-with-puffin")]
        $crate::puffin_scope!($($tt),*);

        #[cfg(feature = "profile-with-optick")]
        $crate::optick_scope!($($tt),*);

        #[cfg(feature = "profile-with-superluminal")]
        $crate::superluminal_scope!($($tt),*);

        #[cfg(feature = "profile-with-tracing")]
        $crate::tracing_scope!($($tt),*);

        #[cfg(feature = "profile-with-tracy")]
        $crate::tracy_scope!($($tt),*);
    };
}

#[macro_export]
macro_rules! register_thread {
    ($tt:tt) => {
        #[cfg(feature = "profile-with-puffin")]
        $crate::puffin_register_thread!($tt);

        #[cfg(feature = "profile-with-optick")]
        $crate::optick_register_thread!($tt);

        #[cfg(feature = "profile-with-superluminal")]
        $crate::superluminal_register_thread!($tt);

        #[cfg(feature = "profile-with-tracing")]
        $crate::tracing_register_thread!($tt);

        #[cfg(feature = "profile-with-tracy")]
        $crate::tracy_register_thread!($tt);
    };
}

#[macro_export]
macro_rules! finish_frame {
    () => {
        #[cfg(feature = "profile-with-puffin")]
        $crate::puffin_finish_frame!();

        #[cfg(feature = "profile-with-optick")]
        $crate::optick_finish_frame!();

        #[cfg(feature = "profile-with-superluminal")]
        $crate::superluminal_finish_frame!();

        #[cfg(feature = "profile-with-tracing")]
        $crate::tracing_finish_frame!();

        #[cfg(feature = "profile-with-tracy")]
        $crate::tracy_finish_frame!();
    };
}
