# Fix `src/ntpkg/installers/imagemagick.rs` build errors for Android/unsupported platforms

## Problem

When compiling for Android (or any platform not matching `windows`/`linux`/`macos`), `imagemagick.rs` fails with:

- `warning: unused import: Context`
- `warning: unused import: PathBuf`
- `warning: unused imports: load_manifest and save_manifest`
- `warning: unused import: crate::t`
- `error[E0425]: cannot find function install_platform in this scope`

Root cause: `install_platform` is defined only inside `#[cfg(target_os = "...")]` blocks, but it is called unconditionally from `install_imagemagick`. On Android (`target_os = "android"`), none of those blocks match, so the symbol is missing and every import used exclusively inside those blocks becomes unused.

## Fix

Edit `src/ntpkg/installers/imagemagick.rs`:

### 1. Gate imports by platform

Replace the current import block with:

```rust
use anyhow::Result;
#[cfg(target_os = "windows")]
use anyhow::Context;
use std::fs;
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use std::process::Command;

use super::super::download::download_with_progress;
use super::super::manager::platform_key;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use super::super::manager::{load_manifest, save_manifest};
use super::super::types::PackageSource;
use crate::output::{print_info, print_success};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::t;
```

### 2. Add a fallback `install_platform` for unsupported platforms

Append this after the existing `#[cfg(target_os = "macos")]` block:

```rust
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn install_platform(
    name: &str,
    version: &str,
    _installer: &Path,
    _vdir: &Path,
) -> Result<()> {
    print_info(&format!(
        "ImageMagick installation is not supported on this platform"
    ));
    Ok(())
}
```

## Validation

```powershell
cargo build --target <android-target>   # should compile without the 4 warnings + 1 error
cargo build                             # should still compile for host (Windows/Linux/macOS)
```

## Risk

Low. The fallback simply returns `Ok(())` with an informational message, matching the macOS "not supported" behavior. No behavior changes on supported platforms.
