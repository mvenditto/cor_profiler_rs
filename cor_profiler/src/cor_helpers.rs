#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::{slice, ffi::c_void, ptr};

use num_derive::{
    FromPrimitive,
    ToPrimitive
};

use crate::utils::to_widestring;

use crate::types::{
    mdToken,
    mdTypeDef,
    COR_SIGNATURE,
    ULONG,
    LPWSTR,
    DWORD,
    WCHAR,
    HMODULE
};

extern crate widestring;
use widestring::U16String;

use com::{
    ComPtr,
    ComRc,
    sys::HRESULT,
    interfaces::IUnknown
};

use crate::interfaces::{
    ICLRMetaHost,
    ICLRRuntimeInfo,
    IEnumUnknown,
    IMetaDataDispenser
};

use crate::guids::{
    CLSID_CorMetaDataDispenser,
    IID_IMetaDataDispenser
};


#[macro_export]
macro_rules! type_from_token {
    ($tk:expr) => {
        ($tk as ULONG32) & 0xff000000;
    }
}

#[macro_export]
macro_rules! is_td_nested {
    ($flags:expr) => {
        (($flags as $crate::types::DWORD) & $crate::cor_helpers::CorTypeAttr::tdVisibilityMask) 
            >= $crate::cor_helpers::CorTypeAttr::tdNestedPublic
    }
}

pub(crate) struct CorSignature {
    arguments: Vec<COR_SIGNATURE>
}

impl CorSignature {

    pub fn new() -> CorSignature {
        CorSignature {
            arguments: vec![
                CorCallingConvention::IMAGE_CEE_CS_CALLCONV_DEFAULT as COR_SIGNATURE,   // callconv
                0,  // arguments number
                CorElementType::ELEMENT_TYPE_VOID as COR_SIGNATURE // return type
            ]
        }
    }

    pub fn call_conv(mut self, call_conv: CorCallingConvention) -> CorSignature {
        self.arguments[0] = call_conv as COR_SIGNATURE;
        self
    }

    pub fn ret(mut self, ret: CorElementType) -> CorSignature {
        self.arguments[2] = ret as COR_SIGNATURE;
        self
    }

    pub fn arg(mut self, arg: CorElementType) -> CorSignature {
        self.arguments[1] += 1;
        self.arguments.push(arg as COR_SIGNATURE);
        self
    }

    fn add_complex_arg(&mut self, token: mdToken, arg_type: CorElementType) {
        self.arguments[1] += 1;
        self.arguments.push(arg_type as COR_SIGNATURE);
        unsafe {
            let mut tk_len: ULONG = 0;
            let x = cor_sig_compress_token_2(token, &mut tk_len);
            let ct: &[COR_SIGNATURE] = slice::from_raw_parts(x, 4);
            for i in 0..tk_len {
                info!("push byte {:x} to signature", ct[i as usize]);
                self.arguments.push(ct[i as usize]);
            }
            info!("compressed_token: {:?} - {}", &ct[0 .. tk_len as usize], tk_len);
        }
    }

    pub fn arg_class(mut self, token: mdTypeDef) -> CorSignature {
        self.add_complex_arg(token, CorElementType::ELEMENT_TYPE_CLASS);
        self
    }

    pub fn arg_value_type(mut self, token: mdTypeDef) -> CorSignature {
        self.add_complex_arg(token, CorElementType::ELEMENT_TYPE_VALUETYPE);
        self
    }

    pub fn pack<'a>(self) -> Vec<COR_SIGNATURE> {
        info!("produced cor_sig: {:?}", self.arguments);
        self.arguments
    }
}

pub mod CorTypeAttr {
    use crate::types::DWORD;

    pub const tdVisibilityMask: DWORD = 0x00000007;
    pub const tdNotPublic: DWORD = 0x00000000;
    pub const tdPublic: DWORD = 0x00000001;
    pub const tdNestedPublic: DWORD = 0x00000002;
    pub const tdNestedPrivate: DWORD = 0x00000003;
    pub const tdNestedFamily: DWORD = 0x00000004;
    pub const tdNestedAssembly: DWORD = 0x00000005;
    pub const tdNestedFamANDAssem: DWORD = 0x00000006;
    pub const tdNestedFamORAssem: DWORD = 0x00000007;
    pub const tdLayoutMask: DWORD = 0x00000018;
    pub const tdAutoLayout: DWORD = 0x00000000;
    pub const tdSequentialLayout: DWORD = 0x00000008;
    pub const tdExplicitLayout: DWORD = 0x00000010;
    pub const tdClassSemanticsMask: DWORD = 0x00000060;
    pub const tdClass: DWORD = 0x00000000;
    pub const tdInterface: DWORD = 0x00000020;
    pub const tdAbstract: DWORD = 0x00000080;
    pub const tdSealed: DWORD = 0x00000100;
    pub const tdSpecialName: DWORD = 0x00000400;
    pub const tdImport: DWORD = 0x00001000;
    pub const tdSerializable: DWORD = 0x00002000;
    pub const tdStringFormatMask: DWORD = 0x00030000;
    pub const tdAnsiClass: DWORD = 0x00000000;
    pub const tdUnicodeClass: DWORD = 0x00010000;
    pub const tdAutoClass: DWORD = 0x00020000;
    pub const tdCustomFormatClass: DWORD = 0x00030000;
    pub const tdCustomFormatMask: DWORD = 0x00C00000;
    pub const tdBeforeFieldInit: DWORD = 0x00100000;
    pub const tdForwarder: DWORD = 0x00200000;
    pub const tdReservedMask: DWORD = 0x00040800;
    pub const tdRTSpecialName: DWORD = 0x00000800;
    pub const tdHasSecurity: DWORD = 0x00040000;
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u8)]
pub(crate) enum CorCallingConvention {
    IMAGE_CEE_CS_CALLCONV_DEFAULT       = 0x0,  
    IMAGE_CEE_CS_CALLCONV_VARARG        = 0x5,  
    IMAGE_CEE_CS_CALLCONV_FIELD         = 0x6,  
    IMAGE_CEE_CS_CALLCONV_LOCAL_SIG     = 0x7,  
    IMAGE_CEE_CS_CALLCONV_PROPERTY      = 0x8,  
    IMAGE_CEE_CS_CALLCONV_UNMGD         = 0x9,  
    IMAGE_CEE_CS_CALLCONV_GENERICINST   = 0xa,  
    IMAGE_CEE_CS_CALLCONV_NATIVEVARARG  = 0xb,  
    IMAGE_CEE_CS_CALLCONV_MAX           = 0xc,  
    IMAGE_CEE_CS_CALLCONV_MASK          = 0x0f,  
    IMAGE_CEE_CS_CALLCONV_HASTHIS       = 0x20,  
    IMAGE_CEE_CS_CALLCONV_EXPLICITTHIS  = 0x40,  
    IMAGE_CEE_CS_CALLCONV_GENERIC       = 0x10  
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u8)]
pub(crate) enum CorElementType {
    ELEMENT_TYPE_END            = 0x0,
    ELEMENT_TYPE_VOID           = 0x1,
    ELEMENT_TYPE_BOOLEAN        = 0x2,
    ELEMENT_TYPE_CHAR           = 0x3,
    ELEMENT_TYPE_I1             = 0x4,
    ELEMENT_TYPE_U1             = 0x5,
    ELEMENT_TYPE_I2             = 0x6,
    ELEMENT_TYPE_U2             = 0x7,
    ELEMENT_TYPE_I4             = 0x8,
    ELEMENT_TYPE_U4             = 0x9,
    ELEMENT_TYPE_I8             = 0xa,
    ELEMENT_TYPE_U8             = 0xb,
    ELEMENT_TYPE_R4             = 0xc,
    ELEMENT_TYPE_R8             = 0xd,
    ELEMENT_TYPE_STRING         = 0xe,

    ELEMENT_TYPE_PTR            = 0xf,
    ELEMENT_TYPE_BYREF          = 0x10,

    ELEMENT_TYPE_VALUETYPE      = 0x11,
    ELEMENT_TYPE_CLASS          = 0x12,
    ELEMENT_TYPE_VAR            = 0x13,
    ELEMENT_TYPE_ARRAY          = 0x14,
    ELEMENT_TYPE_GENERICINST    = 0x15,
    ELEMENT_TYPE_TYPEDBYREF     = 0x16,

    ELEMENT_TYPE_I              = 0x18,
    ELEMENT_TYPE_U              = 0x19,
    ELEMENT_TYPE_FNPTR          = 0x1B,
    ELEMENT_TYPE_OBJECT         = 0x1C,
    ELEMENT_TYPE_SZARRAY        = 0x1D,
    ELEMENT_TYPE_MVAR           = 0x1e,

    ELEMENT_TYPE_CMOD_REQD      = 0x1F,
    ELEMENT_TYPE_CMOD_OPT       = 0x20,

    ELEMENT_TYPE_INTERNAL       = 0x21,
    ELEMENT_TYPE_MAX            = 0x22,

    ELEMENT_TYPE_MODIFIER       = 0x40,
    ELEMENT_TYPE_SENTINEL       = 0x01 | 0x40,
    ELEMENT_TYPE_PINNED         = 0x05 | 0x40
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
#[repr(u32)]
pub(crate) enum CorTokenType
{
    mdtModule               = 0x00000000,       //
    mdtTypeRef              = 0x01000000,       //
    mdtTypeDef              = 0x02000000,       //
    mdtFieldDef             = 0x04000000,       //
    mdtMethodDef            = 0x06000000,       //
    mdtParamDef             = 0x08000000,       //
    mdtInterfaceImpl        = 0x09000000,       //
    mdtMemberRef            = 0x0a000000,       //
    mdtCustomAttribute      = 0x0c000000,       //
    mdtPermission           = 0x0e000000,       //
    mdtSignature            = 0x11000000,       //
    mdtEvent                = 0x14000000,       //
    mdtProperty             = 0x17000000,       //
    mdtMethodImpl           = 0x19000000,       //
    mdtModuleRef            = 0x1a000000,       //
    mdtTypeSpec             = 0x1b000000,       //
    mdtAssembly             = 0x20000000,       //
    mdtAssemblyRef          = 0x23000000,       //
    mdtFile                 = 0x26000000,       //
    mdtExportedType         = 0x27000000,       //
    mdtManifestResource     = 0x28000000,       //
    mdtGenericParam         = 0x2a000000,       //
    mdtMethodSpec           = 0x2b000000,       //
    mdtGenericParamConstraint = 0x2c000000,
    mdtString               = 0x70000000,       //
    mdtName                 = 0x71000000,       //
    mdtBaseType             = 0x72000000,       // Leave this on the high end value. This does not correspond to metadata table
}

type C_ICLRMetaHost = *mut c_void;
type C_ICLRRuntimeInfo = *mut c_void;

pub(crate) struct CLRMetaHost {
    native: C_ICLRMetaHost
}

//#[derive(Copy)]
pub(crate) struct CLRRuntimeInfo {
    native: C_ICLRRuntimeInfo
}

pub fn create_clr_metahost() -> Result<ComRc<dyn ICLRMetaHost>, HRESULT> {
    unsafe {
        let mut hr: HRESULT = 0;
        let meta_host_ptr = clr_create_meta_host(&mut hr);
        if hr < 0 {
            return Err(hr);
        }
        Ok(ComPtr::<dyn ICLRMetaHost>::new(meta_host_ptr as *mut _).upgrade())
    }
}

pub fn get_latest_installed_runtime(metahost: &ComRc<dyn ICLRMetaHost>) -> Result<ComRc<dyn ICLRRuntimeInfo>,HRESULT> {
    unsafe {
        let mut unk_enum: *mut c_void = ptr::null_mut();
        let mut hr = metahost.enumerate_installed_runtimes((&mut unk_enum) as *mut _ as *mut *mut dyn IEnumUnknown);
        if hr < 0 { return Err(hr); }
        let mut unk: *mut c_void = ptr::null_mut();
        let mut runtime: *mut c_void = ptr::null_mut();
        let ienum = ComPtr::<dyn IEnumUnknown>::new(unk_enum as *mut _).upgrade();
        let mut fetched: ULONG = 0;
        loop {
            hr = ienum.next(1, (&mut unk) as *mut _ as *mut *mut dyn IUnknown, &mut fetched);
            if hr < 0 || fetched <= 0 { break }
            runtime = unk;
        }
        if runtime.is_null() { return Err(hr); }
        Ok(ComPtr::<dyn ICLRRuntimeInfo>::new(runtime as *mut _).upgrade())
    }
}


pub fn get_metadata_dispenser(runtime: &ComRc<dyn ICLRRuntimeInfo>) -> Result<ComRc<dyn IMetaDataDispenser>,HRESULT> {
    unsafe {
        let mut unk: *mut c_void = ptr::null_mut();
        let hr = runtime.get_interface2(&CLSID_CorMetaDataDispenser, &IID_IMetaDataDispenser, &mut unk);
        if hr < 0 { return Err(hr); }
        Ok(ComPtr::<dyn IMetaDataDispenser>::new(unk as *mut _).upgrade())
    }
}

pub fn get_clr_runtime_version_string(runtime: &ComRc<dyn ICLRRuntimeInfo>) -> Result<String, HRESULT> {
    unsafe {
        let mut buffer: [WCHAR; 256] = [0; 256];
        let wstr = buffer.as_mut_ptr() as LPWSTR;
        let mut bytes: DWORD = 256;
        let hr = runtime.get_version_string(wstr, &mut bytes);
        if hr < 0 { return Err(hr); }
        let version_string  = 
            U16String::from_ptr(wstr, (bytes-1) as usize).to_string_lossy();
        return Ok(version_string);
    }
}

pub fn load_library(runtime: &ComRc<dyn ICLRRuntimeInfo>, library_name: &str) -> Result<HMODULE, HRESULT> {
    unsafe {
        let wstr = to_widestring(library_name);
        let mut hmodule: HMODULE = ptr::null_mut();
        let hr = runtime.load_library(wstr.as_ptr(), &mut hmodule);
        if hr < 0 { return Err(hr); }
        Ok(hmodule)
    }
}

#[link(name = "Native", kind="static")]
extern {
    pub fn clr_create_meta_host(hr: *mut HRESULT) -> C_ICLRMetaHost;

    pub fn cor_sig_compress_token(token: mdToken, out_buff: *mut c_void) -> ULONG;

    pub fn cor_sig_compress_token_2(token: mdToken, compressed_tk_len: *mut ULONG) -> *mut COR_SIGNATURE;

    pub fn cor_sig_uncompress_token_2(sig: *const COR_SIGNATURE, uncompressed_tk_len: *mut ULONG) -> mdToken;
}