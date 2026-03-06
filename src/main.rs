use anyhow::Result;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use rand::RngExt;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cli;
mod patcher;
mod pe;

use cli::Cli;
use patcher::Patcher;

const UPX_SECTION_0_NAME: &str = ".rsrc\0";
const UPX_SECTION_1_NAME: &str = ".reloc\0";
const UPX_SECTION_2_NAME: &str = ".debug\0";
const DOS_STUB_MESSAGE: &str = "This application requires a modern operating system.";

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !Path::new(&cli.file_path).exists() {
        eprintln!("{}", "Error: File does not exist.".red());
        std::process::exit(1);
    }

    let mut patcher = Patcher::new();
    let mut rng = rand::rng();

    // Validate file
    let mut file = fs::File::open(&cli.file_path)?;
    let mut mz_header = [0; 2];
    file.read_exact(&mut mz_header)?;
    if mz_header != [0x4D, 0x5A] {
        eprintln!(
            "{}",
            "Error: It doesn't look like a valid Windows executable.".red()
        );
        std::process::exit(1);
    }
    drop(file);

    // Check if file is UPX packed
    let upx_pattern = b"UPX0";
    if !patcher.is_pattern_present(&cli.file_path, upx_pattern)? {
        if patcher.is_pattern_present(&cli.file_path, DOS_STUB_MESSAGE.as_ref())? {
            eprintln!("{}", "Error: This file already patched.".red());
        } else {
            eprintln!("{}", "Error: This file is not packed with UPX.".red());
        }
        std::process::exit(1);
    }

    // Backup original file if requested
    if cli.backup {
        let backup_path = format!("{}.bak", cli.file_path);
        if let Err(e) = fs::copy(&cli.file_path, &backup_path) {
            eprintln!("{}", format!("Error: Failed to create backup: {}", e).red());
            std::process::exit(1);
        }
        log_message(&format!("Backup created: {}", backup_path));
    }

    // Section patching
    if !cli.no_section {
        log_message("Sections confusing...");
        patcher.patch_bytes(&cli.file_path, b"UPX0\0", UPX_SECTION_0_NAME.as_ref())?;
        patcher.patch_bytes(&cli.file_path, b"UPX1\0", UPX_SECTION_1_NAME.as_ref())?;
        patcher.patch_bytes(&cli.file_path, b"UPX2\0", UPX_SECTION_2_NAME.as_ref())?;
    }

    // Version block confusing
    if !cli.no_version {
        log_message("Version block confusing...");
        let offset = patcher.find_string_offset(&cli.file_path, "UPX!")?;
        if offset == -1 {
            eprintln!("{}", "Error: Can't get UPX version block offset.".red());
            std::process::exit(1);
        }

        let bytes_to_replace = 15;
        let mut random_version = vec![0u8; bytes_to_replace];
        rng.fill(&mut random_version[..]);

        patcher.patch_bytes_by_offset(
            &cli.file_path,
            offset - (bytes_to_replace as i64) + 4,
            &random_version,
        )?;
    }

    // DOS stub replacement
    if !cli.no_dos {
        log_message("Replacing standard DOS Stub message...");
        patcher.patch_bytes(
            &cli.file_path,
            b"This program cannot be run in DOS mode.",
            DOS_STUB_MESSAGE.as_ref(),
        )?;
    }

    // Entry point patching
    if !cli.no_entry {
        log_message("EntryPoint patching...");
        let is_build64 = pe::is_64(&cli.file_path)?;

        if is_build64 {
            patcher.patch_bytes(&cli.file_path, &[0x0, 0x53, 0x56], &[0x0, 0x55, 0x56])?;
        } else {
            patcher.patch_bytes(&cli.file_path, &[0x0, 0x60, 0xBE], &[0x0, 0x55, 0xBE])?;
        }
    }

    log_message("Successfully patched!");

    Ok(())
}

/// Log message with timestamp
///
/// # Arguments
/// * `message` - The message to log
fn log_message(message: &str) {
    let now = Local::now();
    print!("{}", format!("{} -> ", now.format("%H:%M:%S")).dimmed());
    println!("{}", message);
}
