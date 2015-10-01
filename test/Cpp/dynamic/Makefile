all: cpp_test

cpp_test: main.cpp ffigen_test_scaffold.cpp
	g++ -g -o target/$(CONFIG)/cpp_test main.cpp ffigen_test_scaffold.cpp -ldl

