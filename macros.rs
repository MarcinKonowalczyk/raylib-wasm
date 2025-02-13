#[macro_export]
macro_rules! cstr {
    ($($arg: tt)*) => { std::fmt::format("{}\0", format_args!($($arg)*)).as_ptr() as *const i8 }
}
