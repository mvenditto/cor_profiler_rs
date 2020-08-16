using System;
using System.IO;
using System.Net.Http;
using System.Reflection;
using System.Security;

[assembly: AssemblyKeyFileAttribute("sgKey.snk")]

namespace Helpers
{
    public static class Class1
    {
        public static void HttpClientSendAsyncHook(HttpRequestMessage httpRequest)
        {
            Console.WriteLine($"snooped HTTP Request: method={httpRequest.Method} url={httpRequest.RequestUri}");
        }
    }
}
