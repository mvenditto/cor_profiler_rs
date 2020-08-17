#pragma once
#include <corhlpr.h>
#include <metahost.h>


extern "C" {

    typedef void* C_ICLRMetaHost;
    typedef void* C_ICLRRuntimeInfo;

    __declspec(dllexport) C_ICLRMetaHost clr_create_meta_host(HRESULT* hr);
    
    __declspec(dllexport) C_ICLRRuntimeInfo  clr_get_latest_installed_runtime(C_ICLRMetaHost metahost, HRESULT* hr);

    __declspec(dllexport) const wchar_t* clr_runtime_info_get_version_string(C_ICLRRuntimeInfo runtime_info, HRESULT* hr);

	__declspec(dllexport) UINT cor_sig_compress_token(
        mdToken token, void* out_buffer);

	__declspec(dllexport) COR_SIGNATURE* cor_sig_compress_token_2(
        mdToken token, ULONG* compressed_tk_length);

	__declspec(dllexport) mdToken cor_sig_uncompress_token_2(
        COR_SIGNATURE* sig, ULONG* uncompressed_tk_length);

}