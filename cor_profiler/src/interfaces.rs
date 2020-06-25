#![allow(unused_imports)]

pub(crate) mod i_cor_profiler_callback;
pub(crate) use i_cor_profiler_callback::{
    ICorProfilerCallback,
    ICorProfilerCallback2,
    ICorProfilerCallback3,
    ICorProfilerCallback4,
    ICorProfilerCallback5,
    ICorProfilerCallback6,
    ICorProfilerCallback7,
    ICorProfilerCallback8,
    ICorProfilerFunctionControl,
    ICorProfilerAssemblyReferenceProvider
};

pub(crate) mod i_cor_profiler_info;
pub(crate) use i_cor_profiler_info::{
    ICorProfilerInfo,
    ICorProfilerInfo2,
    ICorProfilerInfo3,
    ICorProfilerInfo4,
    ICorProfilerInfo5,
    ICorProfilerInfo6,
    ICorProfilerInfo7,
    ICorProfilerInfo8,
    ICorProfilerInfo9,
    ICorProfilerInfo10,
    IMethodMalloc
};

pub(crate) mod i_meta_data_import;
pub(crate) use i_meta_data_import::IMetaDataImport;

pub(crate) mod i_meta_data_emit;
pub(crate) use i_meta_data_emit::{
    IMetaDataEmit,
    IMetaDataEmit2
};

pub(crate) mod i_meta_data_assembly_emit;
pub(crate) use i_meta_data_assembly_emit::{
    IMetaDataAssemblyEmit
};

pub(crate) mod i_meta_data_assembly_import;
pub(crate) use i_meta_data_assembly_import::{
    IMetaDataAssemblyImport
};