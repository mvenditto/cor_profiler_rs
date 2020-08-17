use crate::cor_helpers::{
    ICLRMetaHost
};

#[test]
fn test_create_clr_metahost() {
    let metahost = ICLRMetaHost::create();
    if metahost.is_err() {
        let hr = metahost.err().unwrap();
        let msg = format!("Received error hr=0x{:x} while creating ICLRMetaHost", hr);
        assert!(false, msg);
    }
}
