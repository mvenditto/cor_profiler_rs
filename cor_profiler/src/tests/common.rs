use com::sys::HRESULT;
use ctor;

#[macro_export]
macro_rules! assert_ok {
    ($hr:expr, $arg:expr) => {
        assert!($hr >= 0, format!("{} hr=0x{:x}", $arg, $hr))
    };
}

#[macro_export]
macro_rules! assert_err {
    ($hr:expr, $arg:expr) => {
        assert!($hr < 0, format!("{} hr=0x{:x}", $arg, $hr))
    };
}

#[cfg(test)]
#[ctor::ctor]
fn init_log_for_tests() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

#[cfg(test)]
pub(crate) fn unwrap_or_fail<T>(r: Result<T, HRESULT>, msg: &str) -> T {
    match r {
        Err(hr) => {
            let err_lbl = format!(" hr=0x{:x}", hr);
            let assert_msg = msg.to_string() + &err_lbl;
            assert!(false, assert_msg);
            panic!();
        }
        Ok(val) => val
    }
}

#[cfg(test)]
pub(crate) fn unwrap_or_fail_opt<T>(r: Option<T>, msg: &str) -> T {
    match r {
        None => {
            assert!(false, msg.to_string());
            panic!();
        }
        Some(val) => val
    }
}