#[cfg(not(feature = "disable"))]
pub mod instructor;
#[cfg(not(feature = "disable"))]
pub mod timer;

#[cfg(not(feature = "disable"))]
pub enum TimingMode {
    Init,
    Run,
    Free,
}

#[cfg(not(feature = "disable"))]
pub use instructor::begin_session as begin_profiling;
#[cfg(feature = "disable")]
pub fn begin_profiling(_name: &str) {}

#[cfg(not(feature = "disable"))]
pub use instructor::end_session as end_profiling;
#[cfg(feature = "disable")]
pub fn end_profiling() {}

#[cfg(not(feature = "disable"))]
#[macro_export]
macro_rules! time_init {
    () => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            type_name_of(f)
        };

        let _timer = $crate::timer::Timer::new(&name[..name.len() - 3], $crate::TimingMode::Init);
    };
    ($name:expr) => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            let mut name = name[..name.len() - 3].to_string();
            name.push_str($name);
            name
        };

        let _timer = $crate::timer::Timer::new(&name, $crate::TimingMode::Init);
    };
}

#[cfg(not(feature = "disable"))]
#[macro_export]
macro_rules! time_run {
    () => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            type_name_of(f)
        };

        let _timer = $crate::timer::Timer::new(&name[..name.len() - 3], $crate::TimingMode::Run);
    };
    ($name:expr) => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let full_name = type_name_of(f);
            let mut full_name = full_name[..full_name.len() - 3].to_string();
            full_name.push_str($name);
            full_name
        };

        let _timer = $crate::timer::Timer::new(&name, $crate::TimingMode::Run);
    };
}

#[cfg(not(feature = "disable"))]
#[macro_export]
macro_rules! time_free {
    () => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            type_name_of(f)
        };

        let _timer = $crate::timer::Timer::new(&name[..name.len() - 3], $crate::TimingMode::Free);
    };
    ($name:expr) => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let full_name = type_name_of(f);
            let mut full_name = full_name[..full_name.len() - 3].to_string();
            full_name.push_str($name);
            full_name
        };

        let _timer = $crate::timer::Timer::new(&name, $crate::TimingMode::Free);
    };
}

#[cfg(feature = "disable")]
#[macro_export]
macro_rules! time_init {
    () => {};
    ($name:expr) => {};
}

#[cfg(feature = "disable")]
#[macro_export]
macro_rules! time_run {
    () => {};
    ($name:expr) => {};
}

#[cfg(feature = "disable")]
#[macro_export]
macro_rules! time_free {
    () => {};
    ($name:expr) => {};
}
