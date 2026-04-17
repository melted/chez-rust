use std::env::{self, current_dir, set_current_dir};
use std::io::{Error, Result};
use std::path::Path;

fn main() -> Result<()> {
    let system_lib = env::var("CARGO_FEATURE_SYSTEM").is_ok();
    if !system_lib {
        build()?;
    } else {
        // Link to system lib
    }
    Ok(())
}

fn build() -> Result<()> {
    check_chez_dir()?;
    let platform = get_platform()?;
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let debug = env::var("DEBUG").is_ok();
    let config = BuildConfig::new_config(platform, debug);
    if cfg!(windows) && target_env != "gnu" {
        build_windows(&config)?;
    } else {
        build_unix(&config)?;
    }
    Ok(())
}

fn check_chez_dir() -> Result<()> {
    let path = Path::new("ChezScheme");
    if !path.exists() {
        println!("cargo::error=No ChezScheme directory. Check out the git submodule.");
        return Err(Error::new(std::io::ErrorKind::NotFound, "ChezScheme not found"));
    }
    set_current_dir(path)?;
    Ok(())
}

fn build_windows(config : &BuildConfig) -> Result<()> {
    

    Ok(())
}

fn build_unix(config : &BuildConfig) -> Result<()> {
    Ok(())
}

fn get_platform() -> Result<String> {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let cpu = match arch.as_str() {
        "aarch64" => "arm64",
        "x86_64" => "a6",
        "i686" => "i3",
        "riscv64gc" => "rv64",
        _ => "pb64l"
    };
    let pl = match os.as_str() {
        "macos" => "osx",
        "linux" => "le",
        "windows" => "nt",
        "freebsd" => "fb",
        "netbsd" => "nb",
        "openbsd" => "ob",
        _ => return Err(Error::new(std::io::ErrorKind::Unsupported, "Unsupported OS"))
    };
    Ok(format!("t{}{}",cpu, pl))
}

pub struct BuildConfig {
    pub platform:String,
    pub debug:bool
}

impl BuildConfig {
    pub fn new_config(platform : String, debug:bool) -> Self {
        BuildConfig { platform, debug }
    }
}