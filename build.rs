use std::path::Path;

fn build_frontend() {
    let dist_dir = Path::new("ntc_web").join("dist");
    if !dist_dir.join("index.html").exists() {
        panic!("ntc_web/dist/index.html not found. Run 'cd ntc_web && npm run build' first.");
    }
    println!("cargo:rerun-if-changed=ntc_web/src");
    println!("cargo:rerun-if-changed=ntc_web/public");
    println!("cargo:rerun-if-changed=ntc_web/index.html");
    println!("cargo:rerun-if-changed=ntc_web/package.json");
    println!("cargo:rerun-if-changed=ntc_web/vite.config.ts");
}

#[cfg(windows)]
fn main() {
    build_frontend();
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/ntc_image.ico");
    if let Err(e) = res.compile() {
        eprintln!("Warning: Failed to compile Windows resource: {}", e);
    }
}

#[cfg(not(windows))]
fn main() {
    build_frontend();
}
