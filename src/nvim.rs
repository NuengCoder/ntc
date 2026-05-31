// src/nvim.rs - Neovim/Android helpers

use anyhow::Result;

/// Send content to Neovim's clipboard register
#[cfg(target_os = "android")]
pub fn copy_to_nvim_register(content: &str, register: char) -> Result<bool> {
    use std::process::Command;
    
    // Try to use nvim --headless to set register
    let lua_cmd = format!(
        r#"vim.fn.setreg('{}', {})"#,
        register,
        serde_json::to_string(content)?
    );
    
    let status = Command::new("nvim")
        .args(["--headless", "-c", &lua_cmd, "-c", "qa"])
        .status();
    
    Ok(status.is_ok() && status.unwrap().success())
}

/// Display content in a Neovim split
pub fn send_to_neovim(content: &str, filename: &str) -> Result<()> {
    use std::fs;
    use std::process::Command;
    
    let temp_file = std::env::temp_dir().join(format!("ntc_nvim_{}.txt", filename));
    fs::write(&temp_file, content)?;
    
    // Try to open in existing Neovim instance via --remote
    let status = Command::new("nvim")
        .args(["--remote", "--servername", "NTC", temp_file.to_str().unwrap()])
        .status();
    
    if status.is_err() {
        // Open new instance
        Command::new("nvim")
            .arg(temp_file.to_str().unwrap())
            .spawn()?;
    }
    
    Ok(())
}