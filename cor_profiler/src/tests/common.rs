use com::sys::HRESULT;
use ctor;

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
            panic!("");
        }
        Ok(val) => val
    }
}

#[cfg(test)]
pub(crate) fn unwrap_or_fail_opt<T>(r: Option<T>, msg: &str) -> T {
    match r {
        None => {
            let err_lbl = format!("Couldn't get value for {}", msg);
            let assert_msg = msg.to_string() + &err_lbl;
            assert!(false, assert_msg);
            panic!("");
        }
        Some(val) => val
    }
}