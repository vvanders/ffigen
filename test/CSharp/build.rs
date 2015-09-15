extern crate ffigen;
extern crate msbuild_util;

use std::process::Command;
use std::fs;
use std::env;

fn main() {
    let mut context = ffigen::Context::new();

    context.set_root("../scaffold".to_string());
    context.add_lang(ffigen::Lang::CSharp, &[ffigen::Config::Output(".".to_string())]);

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

    cargo.current_dir("../scaffold")
        .arg("build");

    if config == "release" {
        cargo.arg("--release");
    }

    let cargo_result = cargo.output()
        .unwrap_or_else(|e| panic!("Unable to run cargo {}", e));

    if !cargo_result.status.success() {
        panic!("Unable to run cargo {} {}", String::from_utf8_lossy(&cargo_result.stderr), String::from_utf8_lossy(&cargo_result.stdout));
    }

    let target = format!("../scaffold/target/{}/{}", config, get_output_lib(&"ffi_test_scaffold"));
    let dest = format!("bin/x64/{}/{}", capatalize(&config.to_string()), get_output_lib(&"ffi_test_scaffold"));

    fs::copy(&target, &dest)
        .unwrap_or_else(|e| panic!("Unable to copy file from {} to {}, {}", target, dest, e));

    //Make sure we know what version we should run
    println!("cargo:rustc-cfg=config_{}", config);
}

fn capatalize(content: &String) -> String {
    if content.len() == 0 {
        return "".to_string();
    }

    content[..1].chars()
        .flat_map(|c| c.to_uppercase())
        .chain(content[1..].chars())
        .collect::<String>()
}

#[cfg(target_os = "windows")]
fn get_output_lib(name: &str) -> String {
	format!("{}.dll", name)
}

#[cfg(not(target_os = "windows"))]
fn get_output_lib(name: &str) -> String {
	format!("lib{}.so", name)
}

fn build(config: &String) {
    let msbuild = msbuild_util::MSBuild::new()
        .project("CSharp.csproj")
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
