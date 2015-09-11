# ffigen
[![Build Status](https://travis-ci.org/vvanders/ffigen.svg?branch=master)](https://travis-ci.org/vvanders/ffigen)
[![](http://meritbadge.herokuapp.com/ffigen)](https://crates.io/crates/ffigen)

Automatic generation of FFI bindings for calling Rust from other languages.

Note that this library is currently a work-in-progress and not yet ready for serious use.

# Overview
This library allows automatic code generation of the necessary stubs needed to call Rust via C FFI from other languages.
Using code generation and integration with build.rs you can be guaranteed that all function signatures are correct and that values are marshaled appropriately.

# Using
Any rust function that is marked:
```
#[no_mangle]  
pub extern ...
```

Will be exported as stubs to the appropriate language. Note that if you use String, &String or &str you will need to include "ffigen" module in your lib.rs/main.rs. This is for the appropriate string marshaling code.

Below is a template for build.rs that will auto-generate code when "cargo build" is invoked

```
extern crate ffigen;  
  
fn main() {  
    let mut context = ffigen::Context::new();  
  
    context.add_lang(ffigen::Lang::Cpp, &[ffigen::Config::Output("path_to_cpp_source".to_string())]);  
    ffigen::gen(&context);  
}
```

Adding the following line to your Cargo.toml will invoke the build script:
```
[package]
...
build="build.rs"
```

See lib.rs for all languages and config options.

More detailed getting started guide to follow.

# Supported Languages
Currently the following languages are supported:
* C\#
* C++ (dynamc only on win32)

with support for other languages at a future date.

# Unsupported values
Current ffi will not export the following parameter/return types:
* structs
* enums
* arrays
* pointers
