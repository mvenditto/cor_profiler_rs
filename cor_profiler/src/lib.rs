#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub(crate) mod guids;
#[macro_use] pub(crate) mod cor_helpers;
pub(crate) mod il_rewriter;
pub(crate) mod opcodes;
pub(crate) mod cor_profiler;
pub(crate) mod types;
pub(crate) mod interfaces;
pub(crate) mod metadata_helpers;
pub(crate) mod data_container;
pub(crate) mod sig_parser;
pub(crate) mod hooks_loader;
pub(crate) mod utils;
pub(crate) mod safe_array;
mod tests;

use cor_profiler::{
    CLSID_COR_PROFILER,
    CorProfiler
};

use std::ffi::c_void;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate com;
use com::runtime::init_runtime;


#[no_mangle]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    if reason == 1 {
        init_runtime().expect("Failed to initialize COM Library");
        env_logger::init();
        println!("COM Initialized.");
    }
}

com::inproc_dll_module![(CLSID_COR_PROFILER, CorProfiler),];