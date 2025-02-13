#[macro_export]
macro_rules! cstr {
    ($($arg: tt)*) => { std::fmt::format(format_args!($($arg)*)).as_ptr() as *const i8 }
}
