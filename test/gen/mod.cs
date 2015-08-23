namespace rust {
	[DllImport("ffi_sample.dll")]
	uint foo(uint p);
	[DllImport("ffi_sample.dll")]
	uint bar();
}