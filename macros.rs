#[macro_export]
macro_rules! cstr {
    ($($arg: tt)*) => {
        std::ffi::CString::new(format!($($arg)*)).unwrap()
    }
}
