#include "sigparse_wrapper.h"
#include <winerror.h>

HRESULT __cdecl parse_signature(SigParserCallbacks callbacks, sig_byte* sig, ULONG sig_size)
{
	SigParserWrapper sig_parser;
	return sig_parser.Parse(sig, sig_size)
		? S_OK
		: E_FAIL;
}