use std::process::Command;

pub struct MSBuild {
    project: Option<String>,
    platform: Option<Platform>,
    config: Option<Config>
}

pub enum Config {
    Debug,
    Release
}

pub enum Platform {
    X86,
    X64
}

pub enum InvokeError {
    MSBuildNotFound,
    BuildFailure(String)
}

impl MSBuild {
    pub fn new() -> MSBuild {
        MSBuild {
            project: None,
            platform: None,
            config: None
        }
    }

    pub fn platform(&mut self, platform: Platform) -> &mut MSBuild {
        self.platform = Some(platform);

        self
    }

    pub fn project<S: ToString>(&mut self, project: S) -> &mut MSBuild {
        self.project = Some(project.to_string());
        
        self
    }

    pub fn config(&mut self, config: Config) -> &mut MSBuild {
        self.config = Some(config);

        self
    }

    pub fn build(&self) -> Result<String, InvokeError> {
        let platform_msbuild = try!(get_platform_msbuild());

        let mut msbuild = Command::new(platform_msbuild);
        
        if let Some(ref project) = self.project {
            msbuild.arg(project.clone());
        }

        if let Some(ref platform) = self.platform {
            let platform_str = match *platform {
                Platform::X64 => "x64",
                Platform::X86 => "x86"
            };

            msbuild.arg(format!("/p:Platform={}", platform_str));
        }

        if let Some(ref config) = self.config {
            let config_str = match *config {
                Config::Debug => "debug",
                Config::Release => "release"
            };

            msbuild.arg(format!("/p:Configuration={}", config_str));
        }

        let msbuild_output = match msbuild.output() {
            Ok(s) => s,
            Err(_) => return Err(InvokeError::MSBuildNotFound)
        };

        let output = String::from_utf8_lossy(&msbuild_output.stdout).into_owned();

        if !msbuild_output.status.success() {
            return Err(InvokeError::BuildFailure(output));
        }

        Ok(output)
    }
}

#[cfg(not(target_os = "windows"))]
fn get_platform_msbuild() -> Result<String, InvokeError> {
    Ok("xbuild".to_string())
}

#[cfg(target_os = "windows")]
fn get_platform_msbuild() -> Result<String, InvokeError> {
    let msbuild_loc = match Command::new("reg.exe")
        .args(&["query", r#""HKLM\SOFTWARE\Microsoft\MSBuild\ToolsVersions\4.0"#, "/v", "MSBuildToolsPath"])
        .output() {
            Ok(s) => s,
            Err(_) => return Err(InvokeError::MSBuildNotFound)
    };

    if !msbuild_loc.status.success() {
        return Err(InvokeError::MSBuildNotFound);
    }

    let msbuild_stdout: String = String::from_utf8_lossy(&msbuild_loc.stdout).into_owned();
    let msbuild_path =  match msbuild_stdout
        .split("REG_SZ")
        .nth(1)
        .and_then(|path| Some(path.trim())) {
            Some(s) => s,
            None => return Err(InvokeError::MSBuildNotFound)
    };

    Ok(msbuild_path.to_string() + "\\MSBuild.exe")
}
