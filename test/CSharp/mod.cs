using System.Runtime.InteropServices;

namespace rust {
    class ffigen_test {
		[DllImport("ffi_sample.dll")]
		public static extern uint foo(uint p);
		[DllImport("ffi_sample.dll", EntryPoint="foostr_marshal")]
		public static extern void foostr(string p, uint p2);
		[DllImport("ffi_sample.dll")]
		public static extern uint bar();
		[DllImport("ffi_sample.dll")]
		public static extern uint baz(uint p);
		[DllImport("ffi_sample.dll")]
		public static extern uint baz2(uint p);
    }
}