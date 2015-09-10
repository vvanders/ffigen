#include <stdio.h>
#include <string.h>
#include "ffigen_test_scaffold.h"

#define ASSERT_EQ(expected, actual) if(expected != actual) {\
	printf("Mismatched value on line %d",__LINE__); \
	return 1;\
}

int main()
{
	ASSERT_EQ(8, ffigen_test_scaffold::test_u8(8));
	ASSERT_EQ(16, ffigen_test_scaffold::test_u8(16));
	ASSERT_EQ(32, ffigen_test_scaffold::test_u8(32));
	ASSERT_EQ(-8, ffigen_test_scaffold::test_i8(-8));
	ASSERT_EQ(-16, ffigen_test_scaffold::test_i8(-16));
	ASSERT_EQ(-32, ffigen_test_scaffold::test_i8(-32));

	ASSERT_EQ(true, ffigen_test_scaffold::test_bool(true));
	ASSERT_EQ(false, ffigen_test_scaffold::test_bool(false));

	ASSERT_EQ(32.f, ffigen_test_scaffold::test_f32(32.f));
	ASSERT_EQ(64.0, ffigen_test_scaffold::test_f64(64.0));

	ffigen_test_scaffold::RustString str = ffigen_test_scaffold::test_string("foo");
	ASSERT_EQ(0, strcmp(str, "foo"));
	ffigen_test_scaffold::release_rust_string(str);

	str = ffigen_test_scaffold::test_string_ref("bar");
	ASSERT_EQ(0, strcmp(str, "bar"));
	ffigen_test_scaffold::release_rust_string(str);

	str = ffigen_test_scaffold::test_str_ref("baz");
	ASSERT_EQ(0, strcmp(str, "baz"));
	ffigen_test_scaffold::release_rust_string(str);

	ASSERT_EQ(32, ffigen_test_scaffold::mod_value(32));
	ASSERT_EQ(32, ffigen_test_scaffold::sub_mod_value(32));

	ASSERT_EQ(1 + 2 + 3, ffigen_test_scaffold::test_multi_param_unsigned(1, 2, 3));
	ASSERT_EQ(-1 - 2 - 3, ffigen_test_scaffold::test_multi_param_signed(-1, -2, -3));

	str = ffigen_test_scaffold::test_multi_str("foo", "bar", "baz");
	ASSERT_EQ(0, strcmp("foobarbaz", str));
	ffigen_test_scaffold::release_rust_string(str);

    return 0;
}