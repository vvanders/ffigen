#ifndef _ffigen_test_scaffold_H_
#define _ffigen_test_scaffold_H_

extern "C" {
	typedef char* RustString;
	extern "C" unsigned char test_u8(unsigned char p);
	extern "C" unsigned short test_u16(unsigned short p);
	extern "C" unsigned int test_u32(unsigned int p);
	extern "C" char test_i8(char p);
	extern "C" short test_i16(short p);
	extern "C" int test_i32(int p);
	extern "C" float test_f32(float p);
	extern "C" double test_f64(double p);
	extern "C" bool test_bool(bool p);
	extern "C" RustString test_string(const char* p);
	extern "C" RustString test_string_ref(const char* p);
	extern "C" RustString test_str_ref(const char* p);
	extern "C" unsigned int mod_value(unsigned int p);
	extern "C" unsigned int sub_mod_value(unsigned int p);
	extern "C" unsigned int test_multi_param_unsigned(unsigned char p1, unsigned short p2, unsigned int p3);
	extern "C" int test_multi_param_signed(char p1, short p2, int p3);
	extern "C" RustString test_multi_str(const char* p1, const char* p2, const char* p3);
	
	extern "C" void release_rust_string(RustString str);
}

#endif    //_ffigen_test_scaffold_H_