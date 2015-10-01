#include "ffigen_test_scaffold.h"

#ifdef WIN32
    #define WIN32_LEAN_AND_MEAN
    #include <windows.h>
#else
    #include <dlfcn.h>
#endif  //WIN32

namespace ffigen_test_scaffold {
	void* GetAddr(const char* name) {
		#ifdef WIN32    
		    static HMODULE dllHandle = LoadLibrary(L"ffi_test_scaffold.dll");
		    return GetProcAddress(dllHandle, name);
		#else
		    static void* soHandle = dlopen("./libffi_test_scaffold.so", RTLD_LAZY);
		    return dlsym(soHandle, name);
		#endif  //WIN32
	}
	
	unsigned char test_u8(unsigned char p) {
		typedef unsigned char (*FuncSignature)(unsigned char);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_u8"));
		return funcPtr(p);
	}
	
	unsigned short test_u16(unsigned short p) {
		typedef unsigned short (*FuncSignature)(unsigned short);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_u16"));
		return funcPtr(p);
	}
	
	unsigned int test_u32(unsigned int p) {
		typedef unsigned int (*FuncSignature)(unsigned int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_u32"));
		return funcPtr(p);
	}
	
	char test_i8(char p) {
		typedef char (*FuncSignature)(char);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_i8"));
		return funcPtr(p);
	}
	
	short test_i16(short p) {
		typedef short (*FuncSignature)(short);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_i16"));
		return funcPtr(p);
	}
	
	int test_i32(int p) {
		typedef int (*FuncSignature)(int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_i32"));
		return funcPtr(p);
	}
	
	float test_f32(float p) {
		typedef float (*FuncSignature)(float);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_f32"));
		return funcPtr(p);
	}
	
	double test_f64(double p) {
		typedef double (*FuncSignature)(double);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_f64"));
		return funcPtr(p);
	}
	
	bool test_bool(bool p) {
		typedef bool (*FuncSignature)(bool);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_bool"));
		return funcPtr(p);
	}
	
	RustString test_string(const char* p) {
		typedef RustString (*FuncSignature)(const char*);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_string_marshal"));
		return funcPtr(p);
	}
	
	RustString test_string_ref(const char* p) {
		typedef RustString (*FuncSignature)(const char*);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_string_ref_marshal"));
		return funcPtr(p);
	}
	
	RustString test_str_ref(const char* p) {
		typedef RustString (*FuncSignature)(const char*);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_str_ref_marshal"));
		return funcPtr(p);
	}
	
	unsigned int mod_value(unsigned int p) {
		typedef unsigned int (*FuncSignature)(unsigned int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("mod_value"));
		return funcPtr(p);
	}
	
	unsigned int sub_mod_value(unsigned int p) {
		typedef unsigned int (*FuncSignature)(unsigned int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("sub_mod_value"));
		return funcPtr(p);
	}
	
	unsigned int test_multi_param_unsigned(unsigned char p1, unsigned short p2, unsigned int p3) {
		typedef unsigned int (*FuncSignature)(unsigned char, unsigned short, unsigned int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_multi_param_unsigned"));
		return funcPtr(p1, p2, p3);
	}
	
	int test_multi_param_signed(char p1, short p2, int p3) {
		typedef int (*FuncSignature)(char, short, int);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_multi_param_signed"));
		return funcPtr(p1, p2, p3);
	}
	
	RustString test_multi_str(const char* p1, const char* p2, const char* p3) {
		typedef RustString (*FuncSignature)(const char*, const char*, const char*);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("test_multi_str_marshal"));
		return funcPtr(p1, p2, p3);
	}
	
	void release_rust_string(RustString str) {
		typedef unsigned char (*FuncSignature)(RustString);
		static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("release_cstr"));
		funcPtr(str);
	}
}

