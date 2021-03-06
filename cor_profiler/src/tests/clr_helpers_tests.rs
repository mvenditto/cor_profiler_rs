use crate::cor_helpers::{
    create_clr_metahost,
    get_latest_installed_runtime,
    get_clr_runtime_version_string,
    get_metadata_dispenser
};

use crate::types::{
    ofReadWriteMask,
};

use crate::interfaces::{
    IMetaDataDispenser
};

use std::{
    ptr,
    ffi::c_void
};

use crate::guids::IID_IMetaDataImport2;
use crate::utils::to_widestring;
use com::sys::S_OK;
use crate::tests::common::*;

extern crate env_logger;

#[test]
fn test_create_clr_metahost() {
    unwrap_or_fail(create_clr_metahost(), "Couldn't create CLRMetaHost");
}

#[test]
fn test_clr_get_latest_installed_runtime() {
    let metahost = unwrap_or_fail(
        create_clr_metahost(), 
        "Couldn't create CLRMetaHost"
    );

    let _ = unwrap_or_fail(
        get_latest_installed_runtime(&metahost), 
        "Couldn't get latest installed runtime"
    );
}

#[test]
fn test_clr_runtime_get_metadata_dispenser() {
    let metahost = unwrap_or_fail(
        create_clr_metahost(), 
        "Couldn't create CLRMetaHost"
    );

    let runtime = unwrap_or_fail(
        get_latest_installed_runtime(&metahost), 
        "Couldn't get latest installed runtime"
    );

    unwrap_or_fail(
        get_metadata_dispenser(&runtime),
        "Couldn't get metadata dispenser"
    );
}

#[test]
fn test_clr_metadata_dispenser_open_scope() {
    let metahost = unwrap_or_fail(
        create_clr_metahost(), 
        "Couldn't create CLRMetaHost"
    );

    let runtime = unwrap_or_fail(
        get_latest_installed_runtime(&metahost), 
        "Couldn't get latest installed runtime"
    );

    let runtime_version = unwrap_or_fail(
        get_clr_runtime_version_string(&runtime), 
        "Couldn't get runtime version string"
    );

    let scope = r#"C:\Windows\Microsoft.NET\Framework\"#.to_string() 
            + &runtime_version 
            + r#"\System.Net.Http.dll"#;
   
    let metadata_dispenser = unwrap_or_fail(
        get_metadata_dispenser(&runtime),
        "Couldn't get metadata dispenser"
    );

    let scope_name = to_widestring(&scope);

    unsafe {
        let mut unkn: *mut c_void = ptr::null_mut();
        let hr = metadata_dispenser.open_scope(
            scope_name.as_ptr(), 
            ofReadWriteMask, 
            &IID_IMetaDataImport2, 
            &mut unkn);
        assert_ok!(hr, format!("failed to open {} metadata", scope));
    }
}