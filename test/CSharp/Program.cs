using System;
using System.Runtime.InteropServices;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace CSharp
{
    class Program
    {
        static void Main(string[] args)
        {
            string str = rust.ffigen_test.foostr("bar", 2);
            uint result = rust.ffigen_test.foo(1);
            Console.Write(result);
        }
    }
}
