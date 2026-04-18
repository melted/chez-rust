use std::env::{self, current_dir, set_current_dir};
use std::fs::copy;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let system_lib = env::var("CARGO_FEATURE_SYSTEM").is_ok();
    if !system_lib {
        build()?;
    } else {
        // Link to system lib
        // TODO: It's named differently everywhere, what to do?
    }
    println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}

fn build() -> Result<()> {
    check_chez_dir()?;
    let platform = get_platform()?;
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let debug = env::var("DEBUG").is_ok();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let config = BuildConfig::new_config(platform, debug, Path::new(&out_dir).to_path_buf());
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
    let out = Command::new("./build.bat").args([&config.platform, "/MT"]).output().expect("failed to run build.bat");
    let strout = String::from_utf8(out.stdout).unwrap_or_default();
    let strerr = String::from_utf8(out.stderr).unwrap_or_default();
    println!("{}\n", strerr);
    println!("{}", strout);
    let chez_lib_dir = current_dir()?.join(&config.platform).join("boot").join(&config.platform);
    println!("chezpath= {chez_lib_dir:?}");
    for file in chez_lib_dir.read_dir()? {
        if let Ok(entry) = file {
            println!("entry = {entry:?}");
            let name_field = entry.file_name();
            let name = name_field.to_str().unwrap();
            if name.ends_with("mt.lib") & name.starts_with("csv") {
                let target = config.target_path.join(name);
                copy(entry.path(), target)?;
                println!("cargo::rustc-link-search=native={}", config.target_path.to_string_lossy());
                println!("cargo::rustc-link-lib=static={}", entry.path().file_stem().unwrap().to_string_lossy());
                println!("cargo::rerun-if-changed={}", entry.path().to_string_lossy());
            }
            if entry.path().extension().unwrap_or_default().to_str() == Some("boot") {
                let target = config.target_path.join(entry.file_name());
                copy(entry.path(), target)?;
                println!("copied boot file {entry:?}");
            }
        }
    }
    let zlib_path = current_dir()?.join(&config.platform).join("zlibmts").join("zlib.lib");
    copy(&zlib_path, config.target_path.join(&zlib_path.file_name().unwrap()))?;
    println!("cargo::rustc-link-lib=static={}", zlib_path.file_stem().unwrap().to_string_lossy());
    let lz4_path = current_dir()?.join(&config.platform).join("lz4mts/lib").join("liblz4.lib");
    copy(&lz4_path, config.target_path.join(&lz4_path.file_name().unwrap()))?;
    println!("cargo::rustc-link-lib=static=liblz4");
    println!("cargo::rustc-link-lib=advapi32");
    println!("cargo::rustc-link-lib=user32");
    println!("cargo::rustc-link-lib=ole32");
    println!("cargo::rustc-link-lib=rpcrt4");
    Ok(())
}

fn build_unix(config : &BuildConfig) -> Result<()> {
    let out = Command::new("./configure").args(["-m",&config.platform]).output().expect("failed to run configure");
    let strout = String::from_utf8(out.stdout).unwrap_or_default();
    let strerr = String::from_utf8(out.stderr).unwrap_or_default();
    println!("{}\n", strerr);
    println!("{}", strout);
    let out = Command::new("make").output().expect("failed to run make");
    let strout = String::from_utf8(out.stdout).unwrap_or_default();
    let strerr = String::from_utf8(out.stderr).unwrap_or_default();
    println!("{}\n", strerr);
    println!("{}", strout);
    let build_dir = current_dir()?.join(&config.platform);
    let chez_lib_dir = build_dir.join("boot").join(&config.platform);
    println!("chezpath= {chez_lib_dir:?}");
    println!("cargo::rustc-link-search=native={}", config.target_path.to_string_lossy());
    let lib_path = chez_lib_dir.join("libkernel.a");
    copy(lib_path, config.target_path.join("libkernel.a"))?;
    println!("cargo::rustc-link-lib=kernel");
    let lib_path = build_dir.join("lz4/lib/liblz4.a");
    copy(lib_path, config.target_path.join("liblz4.a"))?;
    println!("cargo::rustc-link-lib=lz4");
    let lib_path = build_dir.join("zlib/libz.a");
    copy(lib_path, config.target_path.join("libz.a"))?;
    println!("cargo::rustc-link-lib=z");
    println!("cargo::rustc-link-lib=ncurses");
    copy(chez_lib_dir.join("petite.boot"), config.target_path.join("petite.boot"))?;
    copy(chez_lib_dir.join("scheme.boot"), config.target_path.join("scheme.boot"))?;
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
    pub debug:bool,
    pub target_path:PathBuf
}

impl BuildConfig {
    pub fn new_config(platform : String, debug:bool, target_path: PathBuf) -> Self {
        BuildConfig { platform, debug, target_path }
    }
}