use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Download nowplaying-cli if not present
    download_nowplaying_cli();

    tauri_build::build()
}

fn download_nowplaying_cli() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let bin_dir = Path::new(&out_dir).join("bin");
    let nowplaying_path = bin_dir.join("nowplaying-cli");

    // Create bin directory if it doesn't exist
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    }

    // Skip download if binary already exists and is executable
    if nowplaying_path.exists() {
        println!("nowplaying-cli already exists, skipping download");
        return;
    }

    println!("Downloading nowplaying-cli...");

    // Download the latest release for macOS
    let download_url = "https://github.com/kirtan-shah/nowplaying-cli/releases/latest/download/nowplaying-cli";

    let output = Command::new("curl")
        .args(["-L", "-o", nowplaying_path.to_str().unwrap(), download_url])
        .output()
        .expect("Failed to download nowplaying-cli");

    if !output.status.success() {
        panic!("Failed to download nowplaying-cli: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Make the binary executable
    let chmod_output = Command::new("chmod")
        .args(["+x", nowplaying_path.to_str().unwrap()])
        .output()
        .expect("Failed to make nowplaying-cli executable");

    if !chmod_output.status.success() {
        panic!("Failed to make nowplaying-cli executable: {}", String::from_utf8_lossy(&chmod_output.stderr));
    }

    println!("nowplaying-cli downloaded successfully to: {}", nowplaying_path.display());
}
