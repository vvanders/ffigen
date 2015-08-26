using System.Runtime.InteropServices;
namespace rust {
	class ffigen_test {
		[DllImport("ffi_sample.dll")]
		static extern uint foo(uint p);
		[DllImport("ffi_sample.dll")]
		static extern uint bar();
		[DllImport("ffi_sample.dll")]
		static extern uint baz(uint p);
		[DllImport("ffi_sample.dll")]
		static extern uint baz2(uint p);
	}
}