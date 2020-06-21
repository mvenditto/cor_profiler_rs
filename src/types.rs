#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::c_void;

extern crate winapi;
use winapi::shared::{
    basetsd,
    ntdef,
    minwindef,
    guiddef
};

extern crate com;
use com::sys::HRESULT;


pub type mdToken = basetsd::LONG32;
pub type mdModule = mdToken;
pub type mdTypeDef = mdToken;
pub type mdMethodDef = mdToken;
pub type mdFieldDef = mdToken;
pub type mdTypeRef = mdToken;
pub type mdInterfaceImpl = mdToken;
pub type mdMemberRef = mdToken;
pub type mdPermission = mdToken;
pub type mdSignature = mdToken;
pub type mdTypeSpec = mdToken;
pub type mdModuleRef = mdToken;
pub type mdEvent = mdToken;
pub type mdString = mdToken;
pub type mdCustomAttribute = mdToken;
pub type mdParamDef = mdToken;
pub type mdProperty = mdToken;
pub type mdGenericParam = mdToken;
pub type mdMethodSpec = mdToken;
pub type mdGenericParamConstraint = mdToken;
pub type mdAssemblyRef = mdToken;
pub type mdAssembly = mdToken;
pub type mdFile = mdToken;
pub type mdExportedType = mdToken;
pub type mdManifestResource = mdToken;
pub type HCORENUM = *mut c_void;
pub type CorElementType = ntdef::ULONG;
pub type LPCBYTE = *const minwindef::BYTE;
pub type LPBYTE = *mut minwindef::BYTE;
pub type COR_SIGNATURE = minwindef::BYTE;
pub type PCOR_SIGNATURE = *mut COR_SIGNATURE;
pub type PCCOR_SIGNATURE = *const COR_SIGNATURE;
pub type ProcessID = basetsd::UINT_PTR;
pub type AssemblyID = basetsd::UINT_PTR;
pub type AppDomainID = basetsd::UINT_PTR;
pub type ModuleID = basetsd::UINT_PTR;
pub type ClassID = basetsd::UINT_PTR;
pub type ThreadID = basetsd::UINT_PTR;
pub type ContextID = basetsd::UINT_PTR;
pub type FunctionID = basetsd::UINT_PTR;
pub type GCHandleID = basetsd::UINT_PTR;
pub type ObjectID = basetsd::UINT_PTR;
pub type COR_PRF_ELT_INFO = basetsd::UINT_PTR;
pub type ReJITID = basetsd::UINT_PTR;
pub type COR_PRF_FRAME_INFO = basetsd::UINT_PTR;
pub type DWORD = minwindef::DWORD;
pub type LPWSTR = ntdef::LPWSTR;
pub type LPCWSTR = ntdef::LPCWSTR;
pub type ULONG = ntdef::ULONG;
pub type UINT = minwindef::UINT;
pub type USHORT = ntdef::USHORT;
pub type WCHAR = ntdef::WCHAR;
pub type SIZE_T = basetsd::SIZE_T;
pub type UINT_PTR = basetsd::UINT_PTR;
pub type ULONG32 = basetsd::ULONG32;
pub type BOOL = ntdef::INT;
pub type BYTE = minwindef::BYTE;
pub type INT = ntdef::INT;
pub type GUID = guiddef::GUID;
pub type REFGUID = guiddef::REFGUID;
pub type HANDLE = ntdef::HANDLE;
pub type REFIID = guiddef::REFIID;
pub type PVOID = ntdef::PVOID;
pub type MDUTF8CSTR = *const ntdef::CHAR;
pub type UVCP_CONSTANT = *const ntdef::CHAR;
pub type INT8 = basetsd::INT8;
pub type INT16 = basetsd::INT16;
pub type INT32 = basetsd::INT32;
pub type INT64 = basetsd::INT64;

pub const E_FAIL: HRESULT = 0x80004005i64 as HRESULT;
pub const CORPROF_E_CLASSID_IS_ARRAY: HRESULT = 0x80131365i64 as HRESULT;
pub const CORPROF_E_CLASSID_IS_COMPOSITE: HRESULT = 0x80131366i64 as HRESULT;
pub const CORPROF_E_DATAINCOMPLETE: HRESULT = 0x80131351i64 as HRESULT;

#[repr(C)]
pub struct OSINFO {
    dwOSPlatformId: DWORD,
    dwOSMajorVersion: DWORD,
    dwOSMinorVersion: DWORD
}

#[repr(C)]
pub struct ASSEMBLYMETADATA {
    usMajorVersion: USHORT,
    usMinorVersion: USHORT,
    usBuildNumber: USHORT,
    usRevisionNumber: USHORT,
    szLocale: LPWSTR,
    cbLocale: ULONG,
    rProcessor: *mut DWORD,
    ulProcessor: ULONG,
    rOS: *mut OSINFO,
    ulOS: ULONG,
}

#[repr(C)]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pbPublicKeyOrToken: *mut c_void,
    cbPublicKeyOrToken: ULONG,
    szName: LPCWSTR,
    pMetaData: *mut ASSEMBLYMETADATA,
    pbHashValue: *mut c_void,
    cbHashValue: ULONG,
    dwAssemblyRefFlags: DWORD
}

#[repr(C)]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    startAddress: UINT_PTR,
    length: ULONG
}

#[repr(C)]
pub struct COR_PRF_CODE_INFO {
    startAddress: UINT_PTR,
    size: SIZE_T
}

#[repr(C)]
pub enum COR_PRF_GC_ROOT_KIND {
    COR_PRF_GC_ROOT_OTHER = 0, 
    COR_PRF_GC_ROOT_STACK = 1,
    COR_PRF_GC_ROOT_FINALIZER = 2, 
    COR_PRF_GC_ROOT_HANDLE = 3
}
  
#[repr(C)]
pub enum COR_PRF_GC_ROOT_FLAGS {
    COR_PRF_GC_ROOT_PINNING = 0x00000001, 
    COR_PRF_GC_ROOT_WEAKREF = 0x00000002,
    COR_PRF_GC_ROOT_INTERIOR = 0x00000004, 
    COR_PRF_GC_ROOT_REFCOUNTED = 0x00000008
}

#[repr(C)]
pub enum COR_PRF_FINALIZER_FLAGS {
    COR_PRF_FINALIZER_CRITICAL = 0x00000001
}

#[repr(C)]
pub enum COR_PRF_GC_GENERATION {
    COR_PRF_GC_GEN_0 = 0, 
    COR_PRF_GC_GEN_1 = 1, 
    COR_PRF_GC_GEN_2 = 2,
    COR_PRF_GC_LARGE_OBJECT_HEAP = 3
}
  
#[repr(C)]
pub enum COR_PRF_CLAUSE_TYPE {
    COR_PRF_CLAUSE_NONE = 0, 
    COR_PRF_CLAUSE_FILTER = 1, 
    COR_PRF_CLAUSE_CATCH = 2,
    COR_PRF_CLAUSE_FINALLY = 3
}

#[repr(C)]
pub enum COR_PRF_GC_REASON {
    COR_PRF_GC_OTHER = 0, 
    COR_PRF_GC_INDUCED = 1
}


pub type COR_PRF_REJIT_FLAGS = u32;
pub const COR_PRF_REJIT_BLOCK_INLINING: COR_PRF_REJIT_FLAGS = 0x1;
pub const COR_PRF_REJIT_INLINING_CALLBACKS: COR_PRF_REJIT_FLAGS = 0x2;

pub type COR_PRF_MONITOR = u32;
pub const COR_PRF_MONITOR_NONE: COR_PRF_MONITOR = 0;
pub const COR_PRF_MONITOR_FUNCTION_UNLOADS: COR_PRF_MONITOR = 1;
pub const COR_PRF_MONITOR_CLASS_LOADS: COR_PRF_MONITOR = 2;
pub const COR_PRF_MONITOR_MODULE_LOADS: COR_PRF_MONITOR = 4;
pub const COR_PRF_MONITOR_ASSEMBLY_LOADS: COR_PRF_MONITOR = 8;
pub const COR_PRF_MONITOR_APPDOMAIN_LOADS: COR_PRF_MONITOR = 16;
pub const COR_PRF_MONITOR_JIT_COMPILATION: COR_PRF_MONITOR = 32;
pub const COR_PRF_MONITOR_EXCEPTIONS: COR_PRF_MONITOR = 64;
pub const COR_PRF_MONITOR_GC: COR_PRF_MONITOR = 128;
pub const COR_PRF_MONITOR_OBJECT_ALLOCATED: COR_PRF_MONITOR = 256;
pub const COR_PRF_MONITOR_THREADS: COR_PRF_MONITOR = 512;
pub const COR_PRF_MONITOR_REMOTING: COR_PRF_MONITOR = 1024;
pub const COR_PRF_MONITOR_CODE_TRANSITIONS: COR_PRF_MONITOR = 2048;
pub const COR_PRF_MONITOR_ENTERLEAVE: COR_PRF_MONITOR = 4096;
pub const COR_PRF_MONITOR_CCW: COR_PRF_MONITOR = 8192;
pub const COR_PRF_MONITOR_REMOTING_COOKIE: COR_PRF_MONITOR = 17408;
pub const COR_PRF_MONITOR_REMOTING_ASYNC: COR_PRF_MONITOR = 33792;
pub const COR_PRF_MONITOR_SUSPENDS: COR_PRF_MONITOR = 65536;
pub const COR_PRF_MONITOR_CACHE_SEARCHES: COR_PRF_MONITOR = 131072;
pub const COR_PRF_ENABLE_REJIT: COR_PRF_MONITOR = 262144;
pub const COR_PRF_ENABLE_INPROC_DEBUGGING: COR_PRF_MONITOR = 524288;
pub const COR_PRF_ENABLE_JIT_MAPS: COR_PRF_MONITOR = 1048576;
pub const COR_PRF_DISABLE_INLINING: COR_PRF_MONITOR = 2097152;
pub const COR_PRF_DISABLE_OPTIMIZATIONS: COR_PRF_MONITOR = 4194304;
pub const COR_PRF_ENABLE_OBJECT_ALLOCATED: COR_PRF_MONITOR = 8388608;
pub const COR_PRF_MONITOR_CLR_EXCEPTIONS: COR_PRF_MONITOR = 16777216;
pub const COR_PRF_MONITOR_ALL: COR_PRF_MONITOR = 17301503;
pub const COR_PRF_ENABLE_FUNCTION_ARGS: COR_PRF_MONITOR = 33554432;
pub const COR_PRF_ENABLE_FUNCTION_RETVAL: COR_PRF_MONITOR = 67108864;
pub const COR_PRF_ENABLE_FRAME_INFO: COR_PRF_MONITOR = 134217728;
pub const COR_PRF_ENABLE_STACK_SNAPSHOT: COR_PRF_MONITOR = 268435456;
pub const COR_PRF_USE_PROFILE_IMAGES: COR_PRF_MONITOR = 536870912;
pub const COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST: COR_PRF_MONITOR = 1073741824;
pub const COR_PRF_DISABLE_ALL_NGEN_IMAGES: COR_PRF_MONITOR = 2147483648;
pub const COR_PRF_ALL: COR_PRF_MONITOR = 2415919103;
pub const COR_PRF_REQUIRE_PROFILE_IMAGE: COR_PRF_MONITOR = 536877056;
pub const COR_PRF_ALLOWABLE_AFTER_ATTACH: COR_PRF_MONITOR = 268501694;
pub const COR_PRF_MONITOR_IMMUTABLE: COR_PRF_MONITOR = 4009544704;

#[repr(C)]
pub enum COR_PRF_MISC {
    PROFILER_PARENT_UNKNOWN = 0xFFFFFFFD, 
    PROFILER_GLOBAL_CLASS = 0xFFFFFFFE,
    PROFILER_GLOBAL_MODULE = 0xFFFFFFFF
}

#[repr(C)]
pub enum COR_PRF_JIT_CACHE {
    COR_PRF_CACHED_FUNCTION_FOUND = 0,
    COR_PRF_CACHED_FUNCTION_NOT_FOUND = 1
}

#[repr(C)]
pub enum COR_PRF_TRANSITION_REASON {
    COR_PRF_TRANSITION_CALL = 0,
    COR_PRF_TRANSITION_RETURN = 1
}

#[repr(C)]
pub enum COR_PRF_SUSPEND_REASON {
    COR_PRF_SUSPEND_OTHER = 0, 
    COR_PRF_SUSPEND_FOR_GC = 1,
    COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN = 2,
    COR_PRF_SUSPEND_FOR_CODE_PITCHING = 3, 
    COR_PRF_SUSPEND_FOR_SHUTDOWN = 4,
    COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER = 6, 
    COR_PRF_SUSPEND_FOR_GC_PREP = 7
}

#[repr(C)]
pub enum COR_PRF_STATIC_TYPE {  
    COR_PRF_FIELD_NOT_A_STATIC = 0x0,  
    COR_PRF_FIELD_APP_DOMAIN_STATIC = 0x1,  
    COR_PRF_FIELD_THREAD_STATIC = 0x2,  
    COR_PRF_FIELD_CONTEXT_STATIC = 0x4,  
    COR_PRF_FIELD_RVA_STATIC = 0x8  
}

#[repr(C)]
pub enum COR_PRF_RUNTIME_TYPE   
{  
    COR_PRF_DESKTOP_CLR = 0x1,  
    COR_PRF_CORE_CLR    = 0x2,  
}

#[repr(C)]
pub enum CorOpenFlags
{
    ofRead              =   0x00000000,     
    ofWrite             =   0x00000001,     
    ofCopyMemory        =   0x00000002,    
    ofReadOnly          =   0x00000010,     
    ofTakeOwnership     =   0x00000020,    
	ofNoTypeLib         =   0x00000080, 
    ofNoTransform       =   0x00001000, 
    ofCheckIntegrity    =   0x00000800,
    ofReserved1         =   0x00000100,   
    ofReserved2         =   0x00000200,  
    ofReserved3         =   0x00000400,  
    ofReserved          =   0xffffe740 
}

pub const ofReadWriteMask: u32 = 0x00000001;

pub type FunctionEnter = extern "stdcall" fn (FunctionID);
pub type FunctionLeave = extern "stdcall" fn (FunctionID) ;
pub type FunctionTailcall = extern "stdcall" fn (FunctionID);
pub type FunctionIDMapper = extern "stdcall" fn (FunctionID, *mut BOOL) -> UINT_PTR;

pub type StackSnapshotCallback = extern "stdcall" fn(
    FunctionID, 
    UINT_PTR, 
    COR_PRF_FRAME_INFO, 
    ULONG32, 
    *mut BYTE, 
    *mut c_void
) -> HRESULT;

pub type FunctionEnter2 = extern "stdcall" fn(
    FunctionID, 
    UINT_PTR, 
    COR_PRF_FRAME_INFO,
    *mut COR_PRF_FUNCTION_ARGUMENT_INFO
);

pub type FunctionLeave2 = extern "stdcall" fn(
    FunctionID, 
    UINT_PTR, 
    COR_PRF_FRAME_INFO,
    *mut COR_PRF_FUNCTION_ARGUMENT_INFO
);


pub type FunctionTailcall2 = extern "stdcall" fn (
    FunctionID,
    UINT_PTR,
    COR_PRF_FRAME_INFO
);

pub type FunctionIDMapper2  = extern "stdcall" fn(
    FunctionID, 
    *mut c_void, 
    *mut BOOL
) -> UINT_PTR;

pub type FunctionEnter3 = extern "stdcall" fn (FunctionID);
pub type FunctionLeave3 = extern "stdcall" fn (FunctionID);
pub type FunctionEnter3WithInfo = extern "stdcall" fn (FunctionID, COR_PRF_ELT_INFO);
pub type FunctionLeave3WithInfo = extern "stdcall" fn (FunctionID, COR_PRF_ELT_INFO);
pub type FunctionTailcall3 = extern "stdcall" fn (FunctionID);
pub type FunctionTailcall3WithInfo = extern "stdcall" fn (FunctionID, COR_PRF_ELT_INFO);

pub type ObjectReferenceCallback = extern "stdcall" fn (
    ObjectID,
    *mut ObjectID,
    *mut c_void
);

#[repr(C)]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    num_ranges: ULONG,
    total_argument_size: ULONG,
    accurate: BOOL
}

#[repr(C)]
pub struct COR_IL_MAP {
    old_offset: ULONG32,
    new_offset: ULONG32,
    accurate: BOOL
}

#[repr(C)]
pub struct COR_PRF_GC_GENERATION_RANGE {
    generation: COR_PRF_GC_GENERATION,
    range_start: ObjectID,
    range_length: UINT_PTR,
    range_length_reserved: UINT_PTR
}

#[repr(C)]
pub struct COR_PRF_EX_CLAUSE_INFO  {
    clause_type: COR_PRF_CLAUSE_TYPE,
    program_counter: UINT_PTR,
    frame_pointer: UINT_PTR,
    shadow_stack_pointer: UINT_PTR
}

pub trait ICorProfilerObjectEnum { }
pub trait ICorProfilerFunctionEnum { }
pub trait ICorProfilerModuleEnum { }
pub trait ICorProfilerThreadEnum { }
pub trait ICorProfilerMethodEnum { }

#[repr(C)]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    il_offset: ULONG32,
    native_start_offset: ULONG32,
    native_end_offset: ULONG32
}

#[repr(C)]
pub struct COR_FIELD_OFFSET {  
    ridOfField: mdFieldDef,
    ulOffset: ULONG
}