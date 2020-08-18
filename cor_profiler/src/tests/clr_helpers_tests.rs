use crate::cor_helpers::{
    ICLRMetaHost
};

use crate::types::{
    ofReadWriteMask,
    mdToken
};

use crate::interfaces::{
    IMetaDataDispenser,
    IMetaDataImport2
};

use crate::metadata_helpers::{
    get_type_info,
    TypeInfo
};

use std::{
    ptr,
    ffi::c_void
};

use crate::guids::IID_IMetaDataImport2;
use crate::utils::to_widestring;

use com::{
    ComPtr,
    ComRc,
    interfaces::IUnknown,
    sys::{HRESULT, S_OK}
};

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

fn unwrap_or_fail_opt<T>(r: Option<T>, msg: &str) -> T {
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
            + r#"\System.Net.Http.dll"#;
    
    let metadata_dispenser = unwrap_or_fail(
        runtime.get_metadata_dispenser(),
        "Couldn't get metadata dispenser"
    );

    println!("\topen scope: {}", scope);
    let scope_name = to_widestring(&scope);

    unsafe {
        let mut unkn: *mut c_void = ptr::null_mut();
        let hr = metadata_dispenser.open_scope(
            scope_name.as_ptr(), 
            ofReadWriteMask, 
            &IID_IMetaDataImport2, 
            &mut unkn);
        assert!(hr == S_OK, format!("failed to open {} metadata hr=0x{:x}",scope, hr));

        let iunk = 
            ComPtr::<dyn IUnknown>::new(unkn as *mut _).upgrade();

        let metadata_import =  unwrap_or_fail_opt(
            iunk.get_interface::<dyn IMetaDataImport2>(),
            "IMetaDataImport"
        );

        let http_client_token: mdToken = 0x02000012;

        let type_info = unwrap_or_fail(
            get_type_info(&metadata_import, http_client_token),
            &format!("Failed to retrieve TypeInfo for 0x{:x}", http_client_token),
        );

        assert_eq!("System.Net.Http.HttpClient", type_info.type_name);

    }

}