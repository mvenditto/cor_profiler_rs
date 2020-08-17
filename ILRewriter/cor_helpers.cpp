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

HRESULT __cdecl clr_get_installed_runtimes(
    C_ICLRMetaHost metahost, 
    C_ICLRRuntimeInfo* installed_runtimes_out,
    ULONG* installed_runtimes_length
) {
    try {
        auto _metahost = reinterpret_cast<ICLRMetaHost*>(metahost);
        IEnumUnknown* runtimes = nullptr;
        auto hr = _metahost->EnumerateInstalledRuntimes(&runtimes);
        if (hr < 0) return hr;
        ICLRRuntimeInfo* curr_runtime = nullptr;
        ULONG fetched = 0;
        std::vector<ICLRRuntimeInfo*>* vect = new std::vector<ICLRRuntimeInfo*>;
        while ((hr = runtimes->Next(1, (IUnknown**)&curr_runtime, &fetched)) == S_OK && fetched > 0) {
            ICLRRuntimeInfo* runtime = curr_runtime;
            vect->push_back(runtime);
        }
        if (hr < 0) return hr;
        installed_runtimes_out = (C_ICLRRuntimeInfo*)(&vect->at(0));
        *installed_runtimes_length = (ULONG)vect->size();
        return S_OK;
    } catch (...){
        return E_FAIL;
    }
}
/*
HRESULT __cdecl clr_runtime_get_metadata_dispenser(C_ICLRRuntimeInfo* runtime, IMetaDataDispenser* metadata_dispenser_out) {
    auto _runtime = reinterpret_cast<ICLRRuntimeInfo*>(runtime);
    auto hr = _runtime->GetInterface(
            CLSID_CorMetaDataDispenser, 
            IID_IMetaDataDispenser,
            (void**)&metadata_dispenser_out);
    return hr;
}*/

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
