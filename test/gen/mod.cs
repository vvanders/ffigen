namespace rust {
	[DllImport("ffi_sample.dll")]
	uint foo(uint p);
	[DllImport("ffi_sample.dll")]
	uint bar();
	[DllImport("ffi_sample.dll")]
	uint baz(uint p);
	[DllImport("ffi_sample.dll")]
	uint baz2(uint p);
}