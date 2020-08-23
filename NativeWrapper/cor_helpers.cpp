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

UINT __cdecl cor_sig_compress_token(mdToken token, void* out_buffer)
{
    return CorSigCompressToken(token, out_buffer);
}

COR_SIGNATURE* __cdecl cor_sig_compress_token_2(mdToken token, ULONG* compressed_tk_length)
{
    auto buff = new COR_SIGNATURE[4];
    *compressed_tk_length = CorSigCompressToken(token, (void*)(&buff[0]));
    return &buff[0];
}

mdToken cor_sig_uncompress_token_2(COR_SIGNATURE* sig, ULONG* uncompressed_tk_length)
{
    mdToken token;
    *uncompressed_tk_length = CorSigUncompressToken(sig, &token);
    return token;
}
