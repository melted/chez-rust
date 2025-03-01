use std::env;


fn main() {
    let platform = get_platform();
    if cfg!(windows) {
        build_windows();
    } else {
        build_unix();
    }
}

fn build_windows() {

}

fn build_unix() {
    
}

fn get_platform() -> String {
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
        _ => ""   
    };
    format!("t{}{}",cpu, pl)
}