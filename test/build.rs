use std::process::Command;
use std::env;

fn main() {
    cargo_dep("CSharp");
    cargo_dep("Cpp/dynamic");
}

fn cargo_dep(path: &str) {
    let config = match env::var("PROFILE").unwrap_or_else(|e| panic!("Count not get confing from env {}", e)).as_ref() {
        "debug" => "debug",
        "release" => "release",
        cfg => panic!("Unknown config {}", cfg)
    };

    //Run cargo
    let mut cargo = Command::new("cargo");

    cargo.current_dir(path)
        .arg("build");

    if config == "release" {
        cargo.arg("--release");
    }

    cargo.output()
        .unwrap_or_else(|e| panic!("Unable to run cargo {}", e));
}
