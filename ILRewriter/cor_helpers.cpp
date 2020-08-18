#include "cor_helpers.h"
#include <iostream>
#include <vector>
#include <corhlpr.h>
#include <corprof.h>
#include <metahost.h>
#pragma comment(lib, "mscoree.lib")


C_ICLRMetaHost __cdecl clr_create_meta_host(HRESULT* hr)
{

    ICLRMetaHost* metahost = nullptr;
    *hr = CLRCreateInstance(CLSID_CLRMetaHost, IID_ICLRMetaHost,(void**)&metahost);
    return metahost;
}

C_ICLRRuntimeInfo __cdecl clr_get_latest_installed_runtime(
    C_ICLRMetaHost metahost, HRESULT* hr
) {
    try {
        auto _metahost = reinterpret_cast<ICLRMetaHost*>(metahost);
        IEnumUnknown* runtimes = nullptr;
        *hr = _metahost->EnumerateInstalledRuntimes(&runtimes);
        if (hr < 0) return NULL;
        ICLRRuntimeInfo* runtime = nullptr;
        ICLRRuntimeInfo* latest_runtime = nullptr;
        ULONG fetched = 0;
        while ((*hr = runtimes->Next(1, (IUnknown**)&runtime, &fetched)) == S_OK && fetched > 0) {
            latest_runtime = runtime;
        }
        if (hr < 0) return NULL;
        return latest_runtime;
    } catch (...){
        *hr = E_FAIL;
    }
    return NULL;
}

const wchar_t* __cdecl clr_runtime_info_get_version_string(C_ICLRRuntimeInfo runtime_info, HRESULT* hr)
{
    auto _runtime_info = reinterpret_cast<ICLRRuntimeInfo*>(runtime_info);
    LPWSTR buff = new WCHAR[2048];
    DWORD bytes = 2048;
    *hr = _runtime_info->GetVersionString(buff, &bytes);
    return buff;
}

IMetaDataDispenser* clr_runtime_get_metadata_dispenser(C_ICLRRuntimeInfo runtime_info, HRESULT* hr)
{
    auto _runtime_info = reinterpret_cast<ICLRRuntimeInfo*>(runtime_info);
    IMetaDataDispenser* metadata_dispenser = nullptr;
    *hr = _runtime_info->GetInterface(CLSID_CorMetaDataDispenser, IID_IMetaDataDispenser, (void**)&metadata_dispenser);
    return metadata_dispenser;
}

UINT __cdecl cor_sig_compress_token(mdToken token, void* out_buffer)
{
    return CorSigCompressToken(token, out_buffer);
}

COR_SIGNATURE* __cdecl cor_sig_compress_token_2(mdToken token, ULONG* compressed_tk_length)
{
    auto buff = new COR_SIGNATURE[4];
    *compressed_tk_length = CorSigCompressToken(token, (void*)(&buff[0]));
    std::cout << "cor_sig_compress_token_2 length=" << compressed_tk_length << "\n";
    return &buff[0];
}

mdToken cor_sig_uncompress_token_2(COR_SIGNATURE* sig, ULONG* uncompressed_tk_length)
{
    mdToken token;
    *uncompressed_tk_length = CorSigUncompressToken(sig, &token);
    return token;
}
