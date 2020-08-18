use crate::cor_helpers::{
    ICLRMetaHost
};

use crate::types::ofReadWriteMask;

use crate::interfaces::i_meta_data_dispenser::IMetaDataDispenser;

use std::{
    ptr,
    ffi::c_void
};

use crate::guids::IID_IMetaDataImport2;
use crate::utils::to_widestring;
use com::sys::{HRESULT, S_OK};

fn unwrap_or_fail<T>(r: Result<T, HRESULT>, msg: &str) -> T {
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

#[test]
fn test_create_clr_metahost() {
    unwrap_or_fail(ICLRMetaHost::create(), "Couldn't create ICLRMetaHost");
}

#[test]
fn test_clr_get_latest_installed_runtime() {
    let metahost = unwrap_or_fail(
        ICLRMetaHost::create(), 
        "Couldn't create ICLRMetaHost"
    );

    let runtime = unwrap_or_fail(
        metahost.get_latest_installed_runtime(), 
        "Couldn't get latest installed runtime"
    );

    match runtime.get_version_string() {
        Ok(version_string) => 
            println!("\tlatest runtime version: {}", version_string),
        _ => ()
    }
}

#[test]
fn test_clr_runtime_get_metadata_dispenser() {
    let metahost = unwrap_or_fail(
        ICLRMetaHost::create(), 
        "Couldn't create ICLRMetaHost"
    );

    let runtime = unwrap_or_fail(
        metahost.get_latest_installed_runtime(), 
        "Couldn't get latest installed runtime"
    );

    unwrap_or_fail(
        runtime.get_metadata_dispenser(),
        "Couldn't get metadata dispenser"
    );
}

#[test]
fn test_clr_metadata_dispenser_open_scope() {
    let metahost = unwrap_or_fail(
        ICLRMetaHost::create(), 
        "Couldn't create ICLRMetaHost"
    );

    let runtime = unwrap_or_fail(
        metahost.get_latest_installed_runtime(), 
        "Couldn't get latest installed runtime"
    );

    let runtime_version = unwrap_or_fail(
        runtime.get_version_string(), 
        "Couldn't get runtime version string"
    );

    let scope = r#"C:\Windows\Microsoft.NET\Framework\"#.to_string() 
            + &runtime_version 
            + r#"\mscorlib.dll"#;
    
    let metadata_dispenser = unwrap_or_fail(
        runtime.get_metadata_dispenser(),
        "Couldn't get metadata dispenser"
    );

    println!("\topen scope: {}", scope);
    let scope_name = to_widestring(&scope);
     
    unsafe {
        let mut unk: *mut c_void = ptr::null_mut();
        let hr = metadata_dispenser.open_scope(
            scope_name.as_ptr(), 
            ofReadWriteMask, 
            &IID_IMetaDataImport2, 
            &mut unk);
        assert!(hr == S_OK, format!("failed to open {} metadata hr=0x{:x}",scope, hr));
    }

}