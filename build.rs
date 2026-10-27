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
    let chess_dir = Path::new("ntc_chess_web").join("dist");
    if !chess_dir.join("index.html").exists() {
        panic!("ntc_chess_web/dist/index.html not found. Run 'cd ntc_chess_web && npm run build' first.");
    }
    println!("cargo:rerun-if-changed=ntc_chess_web/src");
    println!("cargo:rerun-if-changed=ntc_chess_web/public");
    println!("cargo:rerun-if-changed=ntc_chess_web/index.html");
    println!("cargo:rerun-if-changed=ntc_chess_web/package.json");
    println!("cargo:rerun-if-changed=ntc_chess_web/vite.config.ts");
}

/// Stamp the git short hash into `BUILD_GIT_HASH` so `--version`
/// distinguishes a fresh repo build from a stale installed binary
/// (`"dev"` when git is unavailable). No `rerun-if` needed: the hash
/// only changes on new commits, which already rebuild everything.
fn set_build_hash() {
    let hash = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "dev".to_string());
    println!("cargo:rustc-env=BUILD_GIT_HASH={}", hash);
}

/// Embed `assets/ntc_image.ico` (plus version info) into the .exe.
///
/// Build-script stderr is only shown with `-vv`, so a failure here used to
/// leave the binary iconless with no trace in normal/CI output. The icon
/// path is absolute (a build script's CWD is not guaranteed) and the file
/// is watched, so changing the .ico re-embeds it instead of reusing a
/// stale binary.
#[cfg(windows)]
fn embed_windows_icon() {
    println!("cargo:rerun-if-changed=assets/ntc_image.ico");
    println!("cargo:rerun-if-changed=build.rs");
    // rc.exe/link.exe can only compile resources for Windows targets whose
    // target_env is gnu or msvc — cross-compiling (e.g. to Android) would
    // otherwise fail with "Can only compile resource file when target_env
    // is gnu or msvc". Non-Windows binaries carry no .ico anyway.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if target_os != "windows" || (target_env != "msvc" && target_env != "gnu") {
        return;
    }
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let icon = Path::new(&manifest).join("assets").join("ntc_image.ico");
    if !icon.exists() {
        println!(
            "cargo:warning=Windows icon not found at {} — the .exe will have no icon",
            icon.display()
        );
        return;
    }
    let mut res = winres::WindowsResource::new();
    // Explicit resource id 1: Explorer picks the first icon group and some
    // tools ignore groups that start higher.
    res.set_icon_with_id(&icon.to_string_lossy(), "1");
    res.set(
        "FileDescription",
        "ntc — terminal file manager, editor and shell",
    );
    res.set("ProductName", "ntc");
    res.set("OriginalFilename", "ntc.exe");
    match res.compile() {
        Ok(()) => eprintln!("embedded Windows icon from {}", icon.display()),
        Err(e) => println!(
            "cargo:warning=Failed to embed Windows icon ({}): {} — install the Windows SDK (rc.exe) or build with the MSVC toolchain",
            icon.display(),
            e
        ),
    }
}

#[cfg(windows)]
fn main() {
    build_frontend();
    set_build_hash();
    arm64_link_search();
    embed_windows_icon();
}

#[cfg(not(windows))]
fn main() {
    build_frontend();
    set_build_hash();
    arm64_link_search();
}

/// ARM64 Windows library search paths (portable, machine-local).
///
/// Active for any aarch64-pc-windows-msvc-like target, including the custom
/// aarch64-win7-windows-msvc triple (matched via CARGO_CFG_* so the JSON
/// file name never matters). lld-link does not read the LIB environment
/// and RUSTFLAGS cannot carry spaced paths (cargo splits on whitespace),
/// so the absolute dirs are emitted here via `cargo:rustc-link-search`,
/// computed at build time:
///
/// - packaging/arm64-libs (repo-local legacy_stdio_definitions.lib), from
///   CARGO_MANIFEST_DIR — always.
/// - MSVC/SDK ARM64 dirs from the vcvars environment (VCToolsInstallDir,
///   WindowsSdkDir + WindowsSDKVersion) — when present, i.e. when building
///   through packaging/build_win32.cmd or packaging/build_win7arm.cmd.
fn arm64_link_search() {
    let is_arm64_msvc = std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("aarch64")
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    if !is_arm64_msvc {
        return;
    }
    println!("cargo:rerun-if-env-changed=VCToolsInstallDir");
    println!("cargo:rerun-if-env-changed=WindowsSdkDir");
    println!("cargo:rerun-if-env-changed=WindowsSDKVersion");
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let lib = Path::new(&manifest).join("packaging").join("arm64-libs");
        println!("cargo:rustc-link-search=native={}", lib.display());
    }
    if let Ok(vc) = std::env::var("VCToolsInstallDir") {
        let lib = Path::new(&vc).join("lib").join("ARM64");
        println!("cargo:rustc-link-search=native={}", lib.display());
    }
    match (
        std::env::var("WindowsSdkDir"),
        std::env::var("WindowsSDKVersion"),
    ) {
        (Ok(dir), Ok(ver)) => {
            let base = Path::new(&dir).join("lib").join(&ver);
            println!(
                "cargo:rustc-link-search=native={}",
                base.join("ucrt").join("arm64").display()
            );
            println!(
                "cargo:rustc-link-search=native={}",
                base.join("um").join("arm64").display()
            );
        }
        _ => {}
    }
}
