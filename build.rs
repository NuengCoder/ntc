// build.rs
#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/ntc_image.ico");
    if let Err(e) = res.compile() {
        eprintln!("Warning: Failed to compile Windows resource: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {
    // For non-Windows platforms (Android, Linux, macOS)
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=not_windows");
}