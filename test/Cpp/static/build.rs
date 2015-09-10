extern crate ffigen;
extern crate msbuild_util;

use std::process::Command;
use std::fs;
use std::env;

fn main() {
    let mut context = ffigen::Context::new();

    context.set_root("../../scaffold".to_string());
    context.add_lang(ffigen::Lang::Cpp, &[ffigen::Config::Output(".".to_string()), ffigen::Config::StaticOnly]);

    ffigen::gen(&context);

    //Build C# parts
    let config = match env::var("PROFILE").unwrap_or_else(|e| panic!("Count not get confing from env {}", e)).as_ref() {
        "debug" => "debug",
        "release" => "release",
        cfg => panic!("Unknown config {}", cfg)
    };
    
    build(&config.to_string());

    //Run cargo
    let mut cargo = Command::new("cargo");

    cargo.current_dir("../../scaffold")
        .arg("build");

    if config == "release" {
        cargo.arg("--release");
    }

    let cargo_result = cargo.output()
        .unwrap_or_else(|e| panic!("Unable to run cargo {}", e));

    if !cargo_result.status.success() {
        panic!("Unable to run cargo {} {}", String::from_utf8_lossy(&cargo_result.stderr), String::from_utf8_lossy(&cargo_result.stdout));
    }

    let target = format!("../../scaffold/target/{}/libffi_test_scaffold.rlib", config);
    let dest = format!("x64/{}/libffi_test_scaffold.rlib", config);

    fs::copy(&target, &dest)
        .unwrap_or_else(|e| panic!("Unable to copy file from {} to {}, {}", target, dest, e));

    //Make sure we know what version we should run
    println!("cargo:rustc-cfg=config_{}", config);
}

#[cfg(target_os = "windows")]
fn build(config: &String) {
    let msbuild = msbuild_util::MSBuild::new()
        .project("CppStatic.vcxproj")
        .platform(msbuild_util::Platform::X64)
        .config(match config.as_ref() {
            "debug" => msbuild_util::Config::Debug,
            "release" => msbuild_util::Config::Release,
            cfg => panic!("Unknown config {}", cfg)
        })
        .build();

    if let Err(e) = msbuild {
        match e {
            msbuild_util::InvokeError::MSBuildNotFound => panic!("Failed to find MSBuild"),
            msbuild_util::InvokeError::BuildFailure(s) => panic!("Build Failed {}", s)
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn build(config: &String) {
}