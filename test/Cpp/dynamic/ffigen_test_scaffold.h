#ifndef _ffigen_test_scaffold_H_
#define _ffigen_test_scaffold_H_

namespace ffigen_test_scaffold {
	typedef char* RustString;
	unsigned char test_u8(unsigned char p);
	unsigned short test_u16(unsigned short p);
	unsigned int test_u32(unsigned int p);
	char test_i8(char p);
	short test_i16(short p);
	int test_i32(int p);
	float test_f32(float p);
	double test_f64(double p);
	bool test_bool(bool p);
	RustString test_string(const char* p);
	RustString test_string_ref(const char* p);
	RustString test_str_ref(const char* p);
	unsigned int mod_value(unsigned int p);
	unsigned int sub_mod_value(unsigned int p);
	unsigned int test_multi_param_unsigned(unsigned char p1, unsigned short p2, unsigned int p3);
	int test_multi_param_signed(char p1, short p2, int p3);
	RustString test_multi_str(const char* p1, const char* p2, const char* p3);
	
	void release_rust_string(RustString str);
}

#endif    //_ffigen_test_scaffold_H_