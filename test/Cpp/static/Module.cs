using System;
using System.Runtime.InteropServices;

namespace rust {
    class ffiutils {
        [DllImport("ffi_test_scaffold.dll")]
        private static extern void release_cstr(IntPtr strptr);

        //Creates a string from an allocated c string, consuming(releasing) the backing allocation
        public static string consume_cstr(IntPtr strptr) {
            string str = Marshal.PtrToStringAnsi(strptr);
            release_cstr(strptr);

            return str;
        }
    }

    class ffigen_test_scaffold {
		[DllImport("ffi_test_scaffold.dll")]
		public static extern byte test_u8(byte p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern ushort test_u16(ushort p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern uint test_u32(uint p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern sbyte test_i8(sbyte p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern short test_i16(short p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern int test_i32(int p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern float test_f32(float p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern double test_f64(double p);

		[return: MarshalAs(UnmanagedType.I1)]
		[DllImport("ffi_test_scaffold.dll")]
		public static extern bool test_bool(bool p);

		[DllImport("ffi_test_scaffold.dll")]
		private static extern System.IntPtr test_string_marshal(string p);
		public static string test_string(string p) {
            return ffiutils.consume_cstr(test_string_marshal(p));
        }

		[DllImport("ffi_test_scaffold.dll")]
		private static extern System.IntPtr test_string_ref_marshal(string p);
		public static string test_string_ref(string p) {
            return ffiutils.consume_cstr(test_string_ref_marshal(p));
        }

		[DllImport("ffi_test_scaffold.dll")]
		private static extern System.IntPtr test_str_ref_marshal(string p);
		public static string test_str_ref(string p) {
            return ffiutils.consume_cstr(test_str_ref_marshal(p));
        }

		[DllImport("ffi_test_scaffold.dll")]
		public static extern uint mod_value(uint p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern uint sub_mod_value(uint p);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern uint test_multi_param_unsigned(byte p1, ushort p2, uint p3);

		[DllImport("ffi_test_scaffold.dll")]
		public static extern int test_multi_param_signed(sbyte p1, short p2, int p3);

		[DllImport("ffi_test_scaffold.dll")]
		private static extern System.IntPtr test_multi_str_marshal(string p1, string p2, string p3);
		public static string test_multi_str(string p1, string p2, string p3) {
            return ffiutils.consume_cstr(test_multi_str_marshal(p1, p2, p3));
        }

    }
}