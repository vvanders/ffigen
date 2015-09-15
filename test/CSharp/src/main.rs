use std::process::Command;

fn main() {
	run();
}

#[test]
fn test() {
	run();
}

#[cfg(target_os = "windows")]
fn run() {
    let exec = Command::new(format!("bin/x64/{}/CSharp.exe", get_config()))
		.output()
		.unwrap_or_else(|e| panic!("Unable to run {}", e));

	if !exec.status.success() {
		panic!("Command failed {} {}", String::from_utf8_lossy(&exec.stderr), String::from_utf8_lossy(&exec.stdout));
	}
}

#[cfg(not(target_os = "windows"))]
fn run() {
	let exec = Command::new("mono")
        .arg(format!("bin/x64/{}/CSharp.exe", get_config()))
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
