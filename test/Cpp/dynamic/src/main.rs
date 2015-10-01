use std::process::Command;

fn main() {
}

#[test]
fn test() {
}

#[cfg(target_os = "windows")]
fn run() {
    let exec = Command::new(format!("bin/x64/{}/CppDynamic.exe", get_config()))
		.output()
		.unwrap_or_else(|e| panic!("Unable to run {}", e));

	if !exec.status.success() {
		panic!("Command failed {} {}", String::from_utf8_lossy(&exec.stderr), String::from_utf8_lossy(&exec.stdout));
	}
}

#[cfg(not(target_os = "windows"))]
fn run() {
	let exec = Command::new("cpp_test")
        .current_dir(format!("target/{}", get_config().to_lowercase()))
		.output()
		.unwrap_or_else(|e| panic!("Unable to run {}", e));

	if !exec.status.success() {
		panic!("Command failed {} {}", String::from_utf8_lossy(&exec.stderr), String::from_utf8_lossy(&exec.stdout));
	}
}

#[cfg(config_debug)]
fn get_config() -> &'static str {
	"Debug"
}

#[cfg(config_release)]
fn get_config() -> &'static str {
	"Release"
}
