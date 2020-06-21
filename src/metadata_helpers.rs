use std::fmt;

use crate::types::*;

use crate::interfaces::{
    ICorProfilerInfo,
    ICorProfilerInfo2,
    ICorProfilerInfo10,
    IMetaDataImport,
    IMetaDataEmit
};

use com::{
    ComPtr,
    sys::{HRESULT, S_OK}
};

use std::{
    ffi::c_void,
    ptr
};

extern crate widestring;
use widestring::{
    U16String,
    U16CString
};

use crate::guids::{
    IID_IMetaDataImport,
    IID_IMetaDataEmit
};

macro_rules! is_fail {
    ($x:expr) => {
        $x < 0;
    }
}

pub struct FunctionInfo {
    pub module_id: ModuleID,
    pub class_id: ClassID,
    pub metadata_token: mdToken,
    pub function_name: String
}

pub struct FunctionFullNameInfo {
    pub module_name: String,
    pub class_name: String,
    pub function_name: String
}

impl fmt::Display for FunctionFullNameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{}.{}.{}"#, self.module_name, self.class_name, self.function_name)
    }
}

pub unsafe fn il_test(info: & ComPtr<dyn ICorProfilerInfo10>, module_id: ModuleID, method_token: mdMethodDef) {
    info!("il_test");

    let mut unkn: *mut c_void = ptr::null_mut();

    // get the MetaData for this function's module
    let mut hr = info.get_module_meta_data(
        module_id,
        CorOpenFlags::ofRead as u32,
        &IID_IMetaDataImport,
        (&mut unkn) as *mut _ as *mut *mut c_void // https://github.com/rust-lang/rust/issues/17417
    );

    if is_fail!(hr) {
        error!("get_module_meta_data failed with hr=0x{:x}", hr);
    }

    // let md_import = ComPtr::<dyn IMetaDataImport>::new(unkn as *mut _).upgrade();

    let mut code_size: ULONG = 0;
    let mut code_buff: LPCBYTE = ptr::null_mut();
    
    hr = info.get_il_function_body(module_id, method_token, &mut code_buff, &mut code_size);
    
    if is_fail!(hr) {
        error!("get_il_function_body failed with hr=0x{:x}", hr);
    }

    info!("code size: {}", code_size);

    let instructions: &[BYTE] = 
        std::slice::from_raw_parts(code_buff, code_size as usize);
    
    info!("instructions {:x?}", instructions);    

}

pub fn new_user_string(info: & ComPtr<dyn ICorProfilerInfo2>, module_id: ModuleID, string: String) -> Result<mdString, HRESULT> {
    let mut token: mdString = 0;
    let size = string.len();
    let user_str_value = U16String::from(string).as_ptr() as LPCWSTR;

    unsafe {
        let mut unkn: *mut c_void = ptr::null_mut();
        
        let mut hr = info.get_module_meta_data(
            module_id,
            CorOpenFlags::ofRead as u32,
            &IID_IMetaDataEmit,
            (&mut unkn) as *mut _ as *mut *mut c_void
        );

        if is_fail!(hr) {
            error!("get_module_meta_data failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        let md_emit = ComPtr::<dyn IMetaDataEmit>::new(unkn as *mut _).upgrade();

        hr = md_emit.define_user_string(
            user_str_value,
            size as ULONG,
            &mut token
        );

        if is_fail!(hr) {
            error!("define_user_string failed with hr=0x{:x}", hr);
            return Err(hr);
        }
    }

    Ok(token)
} 

pub fn get_function_info(info: & ComPtr<dyn ICorProfilerInfo2>, function_id: FunctionID) -> Result<FunctionInfo, HRESULT> {
    if function_id == 0 {
        warn!("Cannot retrieve name of a native function.");
        return Err(E_FAIL);
    }

    let mut class_id: ClassID = 0;
    let mut module_id: ModuleID = 0;
    let mut token: mdToken = 0;
    let mut n_type_args: ULONG32 = 0;
    let mut type_args: [ClassID; 32] = [0; 32];   
    let frame_info: COR_PRF_FRAME_INFO = 0;
    let mut func_name = String::new();

    unsafe {
        let mut hr;
        
        hr = info.get_function_info2(
            function_id,
            frame_info,
            &mut class_id,
            &mut module_id,
            &mut token,
            32,
            &mut n_type_args,
            type_args.as_mut_ptr()
        );

        if is_fail!(hr) {
            error!("get_function_info2 failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        let mut unkn: *mut c_void = ptr::null_mut();

        // get the MetaData for this function's module
        hr = info.get_module_meta_data(
            module_id,
            CorOpenFlags::ofRead as u32,
            &IID_IMetaDataImport,
            (&mut unkn) as *mut _ as *mut *mut c_void // https://github.com/rust-lang/rust/issues/17417
        );

        if is_fail!(hr) {
            error!("get_module_meta_data failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        let md_import = ComPtr::<dyn IMetaDataImport>::new(unkn as *mut _).upgrade();

        let mut buffer: [WCHAR; 256] = [0; 256];
        let wstr = buffer.as_mut_ptr() as LPWSTR;
        let mut func_name_len: ULONG = 0;
        let func_name_internal: U16String;
    
        hr = md_import.get_method_props(
            token,
            ptr::null_mut() as *mut mdTypeDef,
            wstr,
            256,
            &mut func_name_len,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut()
        );

        if is_fail!(hr) {
            error!("get_method_props failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        func_name_internal = U16String::from_ptr(wstr, (func_name_len-1) as usize);
        func_name += &func_name_internal.to_string_lossy();

        // we got a generic function
        if n_type_args > 0 {
            func_name += "<";
    
            for _type_arg in 0..n_type_args {
                match get_class_name(info, class_id) {
                    Ok(class_name) => func_name += &class_name,
                    Err(hr) => return Err(hr)
                }
            }

            func_name += ">";
        }
    }

    return Ok(FunctionInfo {
        class_id,
        module_id,
        metadata_token: token,
        function_name: String::from(func_name)
    });
}

pub fn get_class_name(info: & ComPtr<dyn ICorProfilerInfo2>, class_id: ClassID) -> Result<String,HRESULT> {

    if class_id == 0 {
        warn!("class_id was null");
        return Err(E_FAIL);
    }

    let mut module_id: ModuleID = 0;
    let mut class_token: mdTypeDef = 0;
    let mut parent_class_id: ClassID = 0;
    let mut n_type_args: ULONG32 = 0;
    let mut type_args: [ClassID; 32] = [0; 32];  
    let mut hr: HRESULT = S_OK;
    let mut class_name = "".to_owned();

    unsafe {
        hr = info.get_class_idinfo2(
            class_id,
            &mut module_id,
            &mut class_token,
            &mut parent_class_id,
            32,
            &mut n_type_args,
            type_args.as_mut_ptr()
        );

        match hr {
            CORPROF_E_CLASSID_IS_ARRAY => 
                return Ok("ArrayClass".to_string()),
            CORPROF_E_CLASSID_IS_COMPOSITE => 
                return Ok("CompositeClass".to_string()),
            CORPROF_E_DATAINCOMPLETE => 
                return Ok("DataIncomplete".to_string()),
            _ if is_fail!(hr) => {
                error!("get_class_idinfo2 failed with hr=0x{:x}", hr);
                return Err(E_FAIL);
            },
            _ => ()
        }

        let mut unkn: *mut c_void = ptr::null_mut();

        // get the MetaData for this function's module
        hr = info.get_module_meta_data(
            module_id,
            CorOpenFlags::ofRead as u32,
            &IID_IMetaDataImport,
            (&mut unkn) as *mut _ as *mut *mut c_void // https://github.com/rust-lang/rust/issues/17417
        );

        if is_fail!(hr) {
            error!("get_module_meta_data failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        let md_import = ComPtr::<dyn IMetaDataImport>::new(unkn as *mut _).upgrade();

        let mut buffer: [WCHAR; 1024] = [0; 1024];
        let wstr = buffer.as_mut_ptr() as LPWSTR;
        let mut class_name_len: ULONG = 0;
        let class_name_internal: U16CString;

        hr = md_import.get_type_def_props(
            class_token,
            wstr,
            1024,
            &mut class_name_len,
            ptr::null_mut(),
            ptr::null_mut()
        );

        if is_fail!(hr) {
            error!("get_type_def_props failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        class_name_internal = U16CString::from_ptr_str(wstr);
        class_name += &class_name_internal.to_string_lossy();

        // we got a generic function
        if n_type_args > 0 {

            class_name += "<";

            for i in 0..n_type_args {

                match get_class_name(info, type_args[i as usize]) {
                    Ok(type_name) => class_name += &type_name,
                    Err(hr) => return Err(hr)
                }

                if i + 1  != n_type_args {
                    class_name += ", ";
                }

            }

            class_name += ">";
        }

    }

    return Ok(class_name);
}

pub fn get_function_name(info: & ComPtr<dyn ICorProfilerInfo2>, function_id: FunctionID) -> Result<String, HRESULT> {

    return get_function_info(info, function_id).map(|info| info.function_name);

}

pub fn get_module_name(info: & ComPtr<dyn ICorProfilerInfo2>, module_id: ModuleID) -> Result<String, HRESULT> {

    let mut buffer: [WCHAR; 256] = [0; 256];
    let wstr = buffer.as_mut_ptr() as LPWSTR;
    let mut mod_name_len: ULONG = 0;
    let mod_name_internal: U16String;
    let mut assembly_id: AssemblyID = 0;

    unsafe {
        let hr = info.get_module_info(
            module_id,
            ptr::null_mut(),
            256,
            &mut mod_name_len,
            wstr,
            &mut assembly_id
        );

        if is_fail!(hr) {
            error!("get_module_info failed with hr=0x{:x}", hr);
            return Err(hr)
        }

        mod_name_internal = U16String::from_ptr(wstr, (mod_name_len-1) as usize);
    }

    return Ok(mod_name_internal.to_string_lossy());
}

pub fn get_function_fully_qualified_name(info: & ComPtr<dyn ICorProfilerInfo2>, function_id: FunctionID) -> Result<String, HRESULT> {
    let func_info = match get_function_info(info, function_id) {
        Err(hr) => return Err(hr),
        Ok(func_info) => func_info
    };

    let class_name = match get_class_name(info, func_info.class_id) {
        Err(_) => "<Unknown>".to_string(),
        Ok(class_name) => class_name
    };

    return Ok(class_name + "." + &func_info.function_name);
}

pub fn get_function_name_info(info: & ComPtr<dyn ICorProfilerInfo2>, function_id: FunctionID) -> Result<FunctionFullNameInfo, HRESULT> {

    let func_info = match get_function_info(info, function_id) {
        Err(hr) => return Err(hr),
        Ok(func_info) => func_info
    };

    let class_name = match get_class_name(info, func_info.class_id) {
        Err(hr) => return Err(hr),
        Ok(class_name) => class_name
    };

    let module_name = match get_module_name(info, func_info.module_id) {
        Err(hr) => return Err(hr),
        Ok(module_name) => module_name
    };

    return Ok(FunctionFullNameInfo {
        function_name: func_info.function_name,
        module_name,
        class_name
    });
}