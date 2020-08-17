#pragma once
#include <corhlpr.h>
#include <metahost.h>


extern "C" {

    typedef void* C_ICLRMetaHost;
    typedef void* C_ICLRRuntimeInfo;

    __declspec(dllexport) HRESULT clr_create_meta_host(C_ICLRMetaHost* out_metahost);
    /*
    __declspec(dllexport) HRESULT  clr_get_installed_runtimes(
        C_ICLRMetaHost metahost, C_ICLRRuntimeInfo* installed_runtimes_out, ULONG* installed_runtimes_length);

    __declspec(dllexport) HRESULT clr_runtime_get_metadata_dispenser(
        C_ICLRRuntimeInfo* runtime, IMetaDataDispenser* metadata_dispenser_out);*/

	__declspec(dllexport) UINT cor_sig_compress_token(
        mdToken token, void* out_buffer);

	__declspec(dllexport) COR_SIGNATURE* cor_sig_compress_token_2(
        mdToken token, ULONG* compressed_tk_length);

	__declspec(dllexport) mdToken cor_sig_uncompress_token_2(
        COR_SIGNATURE* sig, ULONG* uncompressed_tk_length);

}