use std::fmt;

use crate::types::*;

use crate::interfaces::{
    ICorProfilerInfo,
    ICorProfilerInfo2,
    ICorProfilerInfo10,
    IMetaDataImport,
    IMetaDataImport2,
    IMetaDataEmit,
    IMetaDataAssemblyEmit,
    IMetaDataAssemblyImport
};

use com::{
    ComRc,
    ComPtr,
    ComInterface,
    sys::{HRESULT, S_OK}
};

use std::{
    ptr,
    ffi::{
        c_void,
        OsStr
    },
    os::windows::ffi::OsStrExt, slice
};

use crate::cor_helpers::{
    CorElementType,
    CorElementType::*,
    CorTokenType,
    CorTokenType::*,
    cor_sig_uncompress_token_2
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


use num_traits::FromPrimitive;

macro_rules! is_fail {
    ($x:expr) => {
        $x < 0;
    }
}

pub struct ModuleInfo<'a> {
    pub module_name: &'a str,
    pub assembly_name: &'a str,
    pub app_domain_id: AppDomainID,
    pub metadata_import: &'a ComRc<dyn IMetaDataImport>,
    pub metadata_emit: &'a ComRc<dyn IMetaDataEmit>
}

pub struct FunctionInfo<'a> {
    pub module_id: ModuleID,
    pub class_id: ClassID,
    pub parent_token: mdToken,
    pub type_info: TypeInfo,
    pub metadata_token: mdToken,
    pub function_name: String,
    pub signature: &'a[COR_SIGNATURE],
    pub type_arguments_num: u32
}

pub struct FunctionFullNameInfo {
    pub module_name: String,
    pub class_name: String,
    pub function_name: String
}

pub struct TypeInfo {
    pub type_name: String,
    pub metadata_token: mdToken
}

impl fmt::Display for FunctionFullNameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{}.{}.{}"#, self.module_name, self.class_name, self.function_name)
    }
}

fn str_to_lpcwstr_1(rust_str: &str) -> LPCWSTR {
    U16String::from(rust_str.to_owned()).as_ptr() as LPCWSTR
}

fn str_to_lpcwstr_2(rust_str: &str) -> LPCWSTR {
    OsStr::new(rust_str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_ptr() as LPCWSTR
}

fn str_to_lpwstr_1(rust_str: &str) -> LPWSTR {
    U16CString::from_str(rust_str.to_string()).unwrap().as_ptr() as LPWSTR
}

fn str_to_lpwstr_2(rust_str: &str) -> LPWSTR {
    OsStr::new(rust_str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_ptr() as LPWSTR
}

pub fn get_module_info<T: ComInterface + ICorProfilerInfo + ?Sized>(profiler_info: &ComRc<T>, module_id: ModuleID) -> Result<String, HRESULT> {

    let mut buffer: [WCHAR; 256] = [0; 256];
    let wstr = buffer.as_mut_ptr() as LPWSTR;
    let mut mod_name_len: ULONG = 0;
    let mod_name_internal: U16String;
    let mut assembly_id: AssemblyID = 0;

    unsafe {
        let hr = profiler_info.get_module_info(
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
        return Ok(mod_name_internal.to_string_lossy());
    }
}

pub fn get_meta_data_interface<T: ComInterface + ICorProfilerInfo + ?Sized, I: ComInterface + ?Sized>(
    info: &ComRc<T>, module_id: ModuleID) -> Result<ComRc<I>, HRESULT> {
    
    let mut unkn: *mut c_void = ptr::null_mut();

    let i_guid = &I::IID;
    let guid = GUID {
        Data1: i_guid.data1,
        Data2: i_guid.data2,
        Data3: i_guid.data3,
        Data4: i_guid.data4
    };

    unsafe {
        let hr = info.get_module_meta_data(
            module_id,
            CorOpenFlags::ofRead as u32,
            &guid,
            (&mut unkn) as *mut _ as *mut *mut c_void
        );

        if is_fail!(hr) {
            error!("get_module_meta_data failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        return Ok(ComPtr::<I>::new(unkn as *mut _).upgrade())
    }
}

pub fn define_assembly_reference(
    assembly_emit: & ComRc<dyn IMetaDataAssemblyEmit>,
    assembly_public_token: &[BYTE],
    assembly_name: &str,
    assembly_locale: &str,
    assembly_version: &str
) -> Result<mdAssemblyRef, HRESULT> {

    let v: Vec<u16> = assembly_version
        .split(".")
        .map(|x| x.parse().expect("cannot parse"))
        .collect();

    let (major, minor, build, revision) = (v[0], v[1], v[2], v[3]);

    let locale = OsStr::new(assembly_locale)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_ptr() as LPWSTR;

    let name = OsStr::new(assembly_name)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
        .as_ptr() as LPWSTR;

    let metadata = ASSEMBLYMETADATA {
        usMajorVersion: major,
        usMinorVersion: minor,
        usBuildNumber: build,
        usRevisionNumber: revision,
        szLocale: locale,
        cbLocale: (assembly_locale.len()-1) as ULONG,
        rOS: ptr::null_mut(),
        rProcessor: ptr::null_mut(),
        ulProcessor: 0,
        ulOS: 0
    };

    let mut assembly_ref: mdAssemblyRef = 0;

    unsafe {
        let hr = assembly_emit.define_assembly_ref(
            assembly_public_token.as_ptr() as *const c_void,
            assembly_public_token.len() as ULONG,
            name,
            &metadata,
            ptr::null_mut(),
            0,
            0,
            &mut assembly_ref
        );

        if is_fail!(hr) {
            Err(hr)
        } else {
            Ok(assembly_ref)
        }
    }
}

pub fn define_type_ref(
    metadata_emit: & ComRc<dyn IMetaDataEmit>,
    resolution_scope: mdAssemblyRef,
    type_name: &str
) -> Result<mdTypeRef, HRESULT> {

    let mut type_ref: mdToken = 0;
        
    let type_name_native = OsStr::new(type_name)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    unsafe {
        let hr = metadata_emit.define_type_ref_by_name(
            resolution_scope, 
            type_name_native.as_ptr() as LPCWSTR,
            &mut type_ref
        );

        if is_fail!(hr) {
            Err(hr)
        } else {
            Ok(type_ref)
        }
    }

}

pub fn enum_type_refs(
    metadata_import: &ComRc<dyn IMetaDataImport>,
    type_name: &str
) -> Result<Option<mdTypeRef>,HRESULT> {

    let mut type_refs_enum: HCORENUM = ptr::null_mut() as *mut c_void;
    let mut type_refs_buff: [mdTypeRef;1024] = [0; 1024];
    let mut num_tokens: ULONG = 0;
    let mut type_def_name_buffer: [WCHAR; 256] = [0; 256];
    // let mut mod_ref_name_buffer: [WCHAR; 256] = [0; 256];
    let mut num_chars: ULONG = 0;
    // let mut attr_flags: DWORD = 0;
    // let mut tk_extends: mdToken = 0;
    let mut resolution_scope: mdToken = 0;
    let mut hr: HRESULT = S_OK;
    // let  = buffer.as_mut_ptr() as LPWSTR;

    unsafe {
        loop {
            
            hr = metadata_import.enum_type_refs(
                &mut type_refs_enum,
                type_refs_buff.as_mut_ptr(),
                1024 as ULONG,
                &mut num_tokens
            );

            for i in 0..num_tokens {

                hr = metadata_import.get_type_ref_props(
                    type_refs_buff[i as usize],
                    &mut resolution_scope,
                    type_def_name_buffer.as_mut_ptr() as LPWSTR,
                    255 as ULONG,
                    &mut num_chars
                );
                
                let native = 
                    U16String::from_ptr(
                      type_def_name_buffer.as_mut_ptr() as LPWSTR, 
                    (num_chars - 1) as usize).to_string_lossy();
                info!("typeRef: {}", native);
                if native == type_name {
                    metadata_import.close_enum(type_refs_enum);
                    return Ok(Some(type_refs_buff[i as usize]))
                }
            }

            if hr < 0 { break; }
        }

        metadata_import.close_enum(type_refs_enum);

        if hr < 0 {
            return Err(hr);
        }
    
        return Ok(None);
    }

}

pub fn enum_assembly_refs(
    metadata_assembly_import: &ComRc<dyn IMetaDataAssemblyImport>,
    type_name: &str
) -> Result<Option<mdAssemblyRef>,HRESULT> {

    let mut assembly_refs_enum: HCORENUM = ptr::null_mut() as *mut c_void;
    let mut assembly_buff: [mdAssemblyRef;1024] = [0; 1024];
    let mut num_tokens: ULONG = 0;
    let mut assembly_name_buffer: [WCHAR; 256] = [0; 256];
    let mut num_chars: ULONG = 0;
    let mut hr: HRESULT = S_OK;

    unsafe {
        loop {
            
            hr = metadata_assembly_import.enum_assembly_refs(
                &mut assembly_refs_enum,
                assembly_buff.as_mut_ptr(),
                1024 as ULONG,
                &mut num_tokens
            );

            for i in 0..num_tokens {

                hr = metadata_assembly_import.get_assembly_ref_props(
                    assembly_buff[i as usize],
                    ptr::null_mut(),
                    ptr::null_mut(),
                    assembly_name_buffer.as_mut_ptr() as LPWSTR,
                    256,
                    &mut num_chars,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut()
                );
                
                let native = 
                    U16String::from_ptr(
                        assembly_name_buffer.as_mut_ptr() as LPWSTR, 
                    (num_chars - 1) as usize).to_string_lossy();
                info!("assemblyRef: {}", native);
                if native == type_name {
                    metadata_assembly_import.close_enum(assembly_refs_enum);
                    return Ok(Some(assembly_buff[i as usize]))
                }
            }

            if hr < 0 { break; }
        }

        metadata_assembly_import.close_enum(assembly_refs_enum);

        if hr < 0 {
            return Err(hr);
        }
    
        return Ok(None);
    }

}

pub fn define_member_ref(
    metadata_emit: & ComRc<dyn IMetaDataEmit>,
    import_token: mdTypeRef,
    member_name: &str,
    signature: &[COR_SIGNATURE]
) -> Result<mdTypeRef, HRESULT> {

    let mut member_ref: mdMemberRef = 0;

    let method_name_native = OsStr::new(member_name)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    unsafe {
        let hr = metadata_emit.define_member_ref(
            import_token,
            method_name_native.as_ptr(),
            signature.as_ptr(),
            signature.len() as ULONG,
            &mut member_ref
        );

        if is_fail!(hr) {
            Err(hr)
        } else {
            Ok(member_ref)
        }
    }

}

pub unsafe fn il_test(info: & ComRc<dyn ICorProfilerInfo10>, module_id: ModuleID, method_token: mdMethodDef) {
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

    // let md_import = ComRc::<dyn IMetaDataImport>::new(unkn as *mut _).upgrade();

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

pub fn new_user_string<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>, module_id: ModuleID, string: String) -> Result<mdString, HRESULT> {
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

// #define TypeFromToken(tk) ((ULONG32)((tk) & 0xff000000))
macro_rules! type_from_token {
    ($tk:expr) => {
        ($tk as ULONG32) & 0xff000000;
    }
}

pub fn get_type_info(metadata_import: &ComRc<dyn IMetaDataImport2>, token: mdToken) -> Result<TypeInfo,HRESULT> {

    let mut parent_token: mdToken = mdTokenNil;
    let mut buffer: [WCHAR; 256] = [0; 256];
    let wstr = buffer.as_mut_ptr() as LPWSTR;
    let mut type_name_len: ULONG = 0;
    let mut hr: HRESULT = E_FAIL;

    let token_type_part = type_from_token!(token);
    let token_type: CorTokenType = FromPrimitive::from_u32(token_type_part).unwrap();

    unsafe {
        match token_type {
            mdtTypeDef => {
                hr = metadata_import.get_type_def_props(
                    token, wstr, 256, &mut type_name_len, ptr::null_mut(), ptr::null_mut());
            },
            mdtTypeRef => {
                hr = metadata_import.get_type_ref_props(
                    token, &mut parent_token, wstr, 256, &mut type_name_len);
            },
            mdtTypeSpec => {
                let mut signature: PCCOR_SIGNATURE = ptr::null_mut();
                let mut signature_length: ULONG = 0;
                hr = metadata_import.get_type_spec_from_token(
                    token, &mut signature, &mut signature_length);
                if is_fail!(hr) || signature_length < 3 {
                    return Err(E_FAIL);
                }
                let sig: &[COR_SIGNATURE] = slice::from_raw_parts(
                    signature, signature_length as usize);

                let elem_type: CorElementType = FromPrimitive::from_u8(sig[0]).unwrap();
                match elem_type {
                    ELEMENT_TYPE_GENERICINST => {
                        let mut uncompressed_tk_len: ULONG = 0;
                        let type_token: mdToken = cor_sig_uncompress_token_2(
                            &sig[2], &mut uncompressed_tk_len);
                        return get_type_info(&metadata_import, type_token);
                    },
                    _ => ()
                };
            },
            mdtModuleRef => {
                hr = metadata_import.get_module_ref_props(
                    token, wstr, 256, &mut type_name_len);
            },
            mdtMemberRef => {
                unimplemented!("get_type_info -> mdtMemberRef");
            },
            mdtMethodDef => {
                unimplemented!("get_type_info -> mdtMethodDef");
            }
            _ => return Err(E_FAIL)
        };

        if is_fail!(hr) || type_name_len <= 0 {
            return Err(E_FAIL);
        }
    
        let type_name  = 
            U16String::from_ptr(wstr, (type_name_len-1) as usize).to_string_lossy();
    
        return Ok(TypeInfo{type_name, metadata_token: token});
    }
}

pub fn get_type_for_signature(
    metadata_import: &ComRc<dyn IMetaDataImport2>, 
    function_info: &FunctionInfo,
    current_index: usize, 
    token_length: &mut ULONG) -> Result<TypeInfo, HRESULT> {

    let type_token_start = &function_info.signature[current_index];
    unsafe {
        let type_token = 
            cor_sig_uncompress_token_2(type_token_start, token_length);
        let type_info = 
            get_type_info(&metadata_import, type_token)?;
        return Ok(type_info);
    }
}

pub fn get_function_signatures_types(metadata_import: &ComRc<dyn IMetaDataImport2>, function_info: &FunctionInfo) -> Result<Vec<String>, HRESULT> {

    let signature_size = function_info.signature.len();
    let generic_count = function_info.type_arguments_num;
    let param_count = function_info.signature[1];

    let mut curr_index: usize = 2;

    if generic_count > 0 {
        curr_index += 1;
    }

    let expected_number_of_params = param_count + 1;
    let mut curr_type_index: usize = 0;
    let mut type_names: Vec<String> = (0..expected_number_of_params).map(|_| String::new()).collect();
    //let mut generic_arg_stack: Vec<i32> = 
    //    Vec::with_capacity(generic_count as usize);
    let mut append_to_type = "".to_string();
    let mut current_type_name = "".to_string();

    while curr_index < signature_size {

        // let mut type_token: mdToken;
        let mut token_length: ULONG = 0;
        let param = function_info.signature[curr_index];
        let cor_element_type: CorElementType 
            = FromPrimitive::from_u8(param).unwrap();

        match cor_element_type {
            ELEMENT_TYPE_VOID => current_type_name.push_str("System.Void"),
            ELEMENT_TYPE_BOOLEAN => current_type_name.push_str("System.Boolean"),
            ELEMENT_TYPE_CHAR => current_type_name.push_str("System.Char16"),
            ELEMENT_TYPE_I1 => current_type_name.push_str("System.SByte"),
            ELEMENT_TYPE_U1 => current_type_name.push_str("System.Byte"),
            ELEMENT_TYPE_I2 => current_type_name.push_str("System.Int16"),
            ELEMENT_TYPE_U2 => current_type_name.push_str("System.UInt16"),
            ELEMENT_TYPE_I4 => current_type_name.push_str("System.Int32"),
            ELEMENT_TYPE_U4 => current_type_name.push_str("System.UInt32"),
            ELEMENT_TYPE_I8 => current_type_name.push_str("System.Int64"),
            ELEMENT_TYPE_U8 => current_type_name.push_str("System.UInt64"),
            ELEMENT_TYPE_R4 => current_type_name.push_str("System.Single"),
            ELEMENT_TYPE_R8 => current_type_name.push_str("System.Double"),
            ELEMENT_TYPE_STRING => current_type_name.push_str("System.String"),
            ELEMENT_TYPE_OBJECT => current_type_name.push_str("System.Object"),   
            ELEMENT_TYPE_CLASS | ELEMENT_TYPE_VALUETYPE => {
                curr_index += 1;
                let type_info = get_type_for_signature(
                    &metadata_import, &function_info, curr_index, &mut token_length
                ).unwrap();
                unsafe {
                    let mut examined_type_token = type_info.metadata_token;
                    let mut examined_type_name = type_info.type_name.to_owned();
                    let mut ongoing_type_name = examined_type_name.to_owned(); 

                    while examined_type_name.contains(".") {
                        
                        let mut potential_parent_token: mdToken = mdTokenNil;
                        metadata_import.get_nested_class_props(examined_type_token, &mut potential_parent_token);
                        if potential_parent_token == mdTokenNil {
                            break;
                        }
                        let nesting_type = 
                            get_type_info(&metadata_import, potential_parent_token).unwrap();
                        let nesting_type_name = nesting_type.type_name.to_owned();
                        examined_type_token = nesting_type.metadata_token;
                        examined_type_name.replace_range(.., &nesting_type_name);
                        ongoing_type_name = format!("{}+{}", examined_type_name, ongoing_type_name);
                    }
                    curr_index += (token_length as usize) - 1;
                    current_type_name.push_str(&ongoing_type_name);
                }
            },
            ELEMENT_TYPE_SZARRAY => {
                append_to_type.push_str("[]");
                while function_info.signature[curr_index + 1] == 0x1D {
                    append_to_type.push_str("[]");
                    curr_index += 1;
                }
                continue;
            },
            ELEMENT_TYPE_MVAR => {
                unimplemented!("ELEMENT_TYPE_MVAR")
            },
            ELEMENT_TYPE_GENERICINST => {
                unimplemented!("ELEMENT_TYPE_MVAR")
            },
            ELEMENT_TYPE_BYREF => {
                current_type_name.push_str("ref");
            },
            ELEMENT_TYPE_END => {
                continue;
            }
            _ => current_type_name.push_str(&format!("0x{:x}", param))

        };

        if !append_to_type.is_empty() {
            current_type_name.push_str(&append_to_type);
            append_to_type = "".to_string();
        }
        debug!("type[{}|0x{:x}] =>  {}", curr_type_index, param, current_type_name);
        type_names[curr_type_index] = current_type_name;
        current_type_name = "".to_string();
        curr_type_index += 1;
        curr_index += 1;

        // TODO generics
    }

        
    debug!("signature: {:?}", type_names);
    
    Ok(type_names)
}

pub fn get_function_info<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>, function_id: FunctionID) -> Result<FunctionInfo, HRESULT> {
    if function_id == 0 {
        warn!("Cannot retrieve name of a native function.");
        return Err(E_FAIL);
    }

    let mut class_id: ClassID = 0;
    let mut module_id: ModuleID = 0;
    let mut parent_token: mdToken = mdTokenNil;
    let mut token: mdToken = 0;
    let mut n_type_args: ULONG32 = 0;
    let mut type_args: [ClassID; 32] = [0; 32];   
    let frame_info: COR_PRF_FRAME_INFO = 0;
    let mut func_name = String::new();
    let signature: &[COR_SIGNATURE];
    let mut type_info: TypeInfo;

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

        let metadata_import= 
            get_meta_data_interface::<T, dyn IMetaDataImport2>(info, module_id).unwrap();

        let type_token = type_from_token!(token);
        let cor_token_type: CorTokenType = FromPrimitive::from_u32(type_token).unwrap();

        match cor_token_type {
            mdtMemberRef => hr = metadata_import.get_member_ref_props(
                token, &mut parent_token, ptr::null_mut(), 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()),
            mdtMethodDef => hr = metadata_import.get_member_props(
                token, &mut parent_token, ptr::null_mut(), 0, ptr::null_mut(), 
                ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 
                ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut()),
            mdtMethodSpec => hr = metadata_import.get_method_spec_props(
                token, &mut parent_token, ptr::null_mut(), ptr::null_mut()),
            _ => ()
        }

        type_info = get_type_info(&metadata_import, parent_token).unwrap();

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
        let mut cor_signature: PCCOR_SIGNATURE = ptr::null_mut();
        let mut cor_signature_size: ULONG = 0;
    
        hr = md_import.get_method_props(
            token,
            ptr::null_mut() as *mut mdTypeDef,
            wstr,
            256,
            &mut func_name_len,
            ptr::null_mut(),
            &mut cor_signature,
            &mut cor_signature_size,
            ptr::null_mut(),
            ptr::null_mut()
        );

        if is_fail!(hr) {
            error!("get_method_props failed with hr=0x{:x}", hr);
            return Err(hr);
        }

        signature = std::slice::from_raw_parts(
            cor_signature, 
            cor_signature_size as usize);

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
        type_info,
        parent_token,
        module_id,
        metadata_token: token,
        function_name: String::from(func_name),
        signature,
        type_arguments_num: n_type_args
    });
}

pub fn get_class_name<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>, class_id: ClassID) -> Result<String,HRESULT> {

    if class_id == 0 {
        warn!("get_class_name class_id was null");
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

pub fn get_function_name<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>, function_id: FunctionID) -> Result<String, HRESULT> {

    return get_function_info(info, function_id).map(|info| info.function_name);

}

pub fn get_module_name<T: ICorProfilerInfo + ComInterface + ?Sized>(info: &ComRc<T>, module_id: ModuleID) -> Result<String, HRESULT> {

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

pub fn get_function_fully_qualified_name<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>,  function_id: FunctionID) -> Result<String, HRESULT> {
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

pub fn get_function_name_info<T: ICorProfilerInfo2 + ComInterface + ?Sized>(info: & ComRc<T>,  function_id: FunctionID) -> Result<FunctionFullNameInfo, HRESULT> {

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