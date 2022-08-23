#[cfg(not(feature = "profile-with-puffin"))]
#[macro_export]
macro_rules! puffin_scope {
    ($($_:tt)*) => {};
}

#[cfg(not(feature = "profile-with-puffin"))]
#[macro_export]
macro_rules! puffin_finish_frame {
    ($($_:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "profile-with-puffin")]
macro_rules! puffin_scope {
    ($name:expr) => {
        $crate::puffin::profile_scope!($name);
    };
    ($name:expr, $data:expr) => {
        $crate::puffin::profile_scope!($name, $data);
    };
}

#[macro_export]
macro_rules! puffin_register_thread {
    () => {};
    ($name:expr) => {
        // puffin uses the thread name
    };
}

/// Finishes the frame. This isn't strictly necessary for some kinds of applications but a pretty
/// normal thing to track in games.
#[macro_export]
#[cfg(feature = "profile-with-puffin")]
macro_rules! puffin_finish_frame {
    () => {
        $crate::puffin::GlobalProfiler::lock().new_frame();
    };
}
