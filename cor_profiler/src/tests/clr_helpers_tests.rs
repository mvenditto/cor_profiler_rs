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

#[test]
fn test_clr_get_installed_runtimes() {
    let maybe_metahost = ICLRMetaHost::create();
    match maybe_metahost {
        Ok(metahost) => {
            let maybe_installed_runtimes = metahost.get_installed_runtimes();
            if maybe_installed_runtimes.is_err() {
                let hr = maybe_installed_runtimes.err().unwrap();
                let msg = format!("Received error hr=0x{:x} from get_installed_runtimes()", hr);
                assert!(false, msg);
            } else {
                let runtimes = maybe_installed_runtimes.unwrap();
                assert!(runtimes.len() > 0, "");
                println!("num installed runtimes: {}", runtimes.len());
            }
        },
        Err(hr) => {
            let msg = format!("Received error hr=0x{:x} while creating ICLRMetaHost", hr);
            assert!(false, msg);
        }
    }
}
