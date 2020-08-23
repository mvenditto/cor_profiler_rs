using System;
using System.Linq;

namespace SampleProgram
{
    class Program
    {
        static int TestMethod(String value)
        {
            var args = value.Split('+').Select(int.Parse).ToList();
            var (a, b) = (args[0], args[1]);
            var x = new SampleLibrary.Class1();
            return x.Add(a, b);
        }
    }
}
