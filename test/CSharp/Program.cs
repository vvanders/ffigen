using Microsoft.VisualStudio.TestTools.UnitTesting;
using rust;
using System.Runtime.InteropServices;

namespace CSharp
{
    class Program
    {

        [DllImport("ffi_sample.dll")]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool test_bool(bool p);

        static void Main(string[] args)
        {
            Assert.AreEqual((byte)8, ffigen_test.test_u8(8));
            Assert.AreEqual((ushort)16, ffigen_test.test_u16(16));
            Assert.AreEqual((uint)32, ffigen_test.test_u32(32));
            Assert.AreEqual((sbyte)-8, ffigen_test.test_i8(-8));
            Assert.AreEqual((short)-16, ffigen_test.test_i16(-16));
            Assert.AreEqual((int)-32, ffigen_test.test_i32(-32));

            Assert.AreEqual(false, ffigen_test.test_bool(false));
            Assert.AreEqual(true, ffigen_test.test_bool(true));

            Assert.AreEqual(32.0f, ffigen_test.test_f32(32.0f));
            Assert.AreEqual(64.0, ffigen_test.test_f64(64.0f));

            Assert.AreEqual("foo", ffigen_test.test_string("foo"));
            Assert.AreEqual("foo", ffigen_test.test_string_ref("foo"));
            Assert.AreEqual("foo", ffigen_test.test_str_ref("foo"));

            Assert.AreEqual((uint)32, ffigen_test.mod_value(32));
            Assert.AreEqual((uint)32, ffigen_test.sub_mod_value(32));
            Assert.AreEqual((uint)(1 + 2 + 3), ffigen_test.test_multi_param_unsigned(1, 2, 3));

            Assert.AreEqual(-1 - 2 - 3, ffigen_test.test_multi_param_signed(-1, -2, -3));
            Assert.AreEqual("foobarbaz", ffigen_test.test_multi_str("foo", "bar", "baz"));
        }
    }
}
