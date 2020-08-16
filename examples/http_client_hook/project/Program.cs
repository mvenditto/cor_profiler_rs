using System;
using System.Net.Http;
using System.Threading.Tasks;

namespace test
{
    class Program
    {
        static readonly HttpClient client = new HttpClient();
			
        static async Task Main(string[] args)
        {
			var request = new HttpRequestMessage(HttpMethod.Get, @"http:\\www.google.com");
			var res = await client.SendAsync(request);
            Console.WriteLine($"req={request.Method} status={res.StatusCode}");
        }
    }
}
