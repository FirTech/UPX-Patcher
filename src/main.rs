use chrono::Local;
use colored::{control, Colorize};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

mod pe;
mod patcher;

use patcher::Patcher;

const UPX_SECTION_0_NAME: &str = ".rsrc\0";
const UPX_SECTION_1_NAME: &str = ".reloc\0";
const UPX_SECTION_2_NAME: &str = ".debug\0";
const DOS_STUB_MESSAGE: &str = "This application requires a modern operating system.";

fn main() {
    if launched_from_explorer() {
        println!("This is a command line tool.\n\nYou need to open cmd.exe and run it from there.");
        sleep(Duration::from_secs(5));
        std::process::exit(0);
    }

    control::set_virtual_terminal(true).ok();

    println!("{}", "\n UPX-Patcher\n".yellow());
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(0);
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("{}", "Error: File does not exist.".red());
        std::process::exit(1);
    }

    let mut patcher = Patcher::new();
    let mut rng = rand::rng();

    match run_patcher(&mut patcher, file_path, &mut rng) {
        Ok(_) => {
            log_message("Successfully patched!");
        }
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
}

fn run_patcher(patcher: &mut Patcher, file_path: &str, rng: &mut impl Rng) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut mz_header = [0; 2];
    file.read_exact(&mut mz_header)?;
    if mz_header != [0x4D, 0x5A] {
        return Err("It doesn't look like a valid Windows executable.".into());
    }
    drop(file);

    let upx_pattern = b"UPX0";
    if !patcher.is_pattern_present(file_path, upx_pattern)? {
        return if patcher.is_pattern_present(file_path, DOS_STUB_MESSAGE.as_ref())? {
            Err("This file already patched.".into())
        } else {
            Err("This file is not packed with UPX.".into())
        };
    }

    log_message("Sections confusing...");

    patcher.patch_bytes(file_path, b"UPX0\0", UPX_SECTION_0_NAME.as_ref())?;
    patcher.patch_bytes(file_path, b"UPX1\0", UPX_SECTION_1_NAME.as_ref())?;
    patcher.patch_bytes(file_path, b"UPX2\0", UPX_SECTION_2_NAME.as_ref())?;

    log_message("Version block confusing...");

    let offset = patcher.find_string_offset(file_path, "UPX!")?;
    if offset == -1 {
        return Err("Can't get UPX version block offset.".into());
    }

    let bytes_to_replace = 15;
    let mut random_version = vec![0u8; bytes_to_replace];
    rng.fill(&mut random_version[..]);

    patcher.patch_bytes_by_offset(file_path, offset - (bytes_to_replace as i64) + 4, &random_version)?;

    log_message("Replacing standart DOS Stub message...");
    patcher.patch_bytes(file_path, b"This program cannot be run in DOS mode.", DOS_STUB_MESSAGE.as_ref())?;

    log_message("WinAPI changing...");
    patcher.patch_bytes(file_path, b"ExitProcess", b"CopyContext")?;

    log_message("EntryPoint patching...");
    let is_build64 = pe::is_64(file_path)?;

    if is_build64 {
        patcher.patch_bytes(file_path, &[0x0, 0x53, 0x56], &[0x0, 0x55, 0x56])?;
    } else {
        patcher.patch_bytes(file_path, &[0x0, 0x60, 0xBE], &[0x0, 0x55, 0xBE])?;
    }

    Ok(())
}

fn log_message(message: &str) {
    let now = Local::now();
    print!("{}", format!("{} -> ", now.format("%H:%M:%S")).dimmed());
    println!("{}", message);
}

fn launched_from_explorer() -> bool {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let my_pid = sysinfo::get_current_pid().unwrap();
    if let Some(proc) = sys.process(my_pid) {
        if let Some(parent_pid) = proc.parent() {
            if let Some(parent_proc) = sys.process(parent_pid) {
                return parent_proc.name().eq_ignore_ascii_case("explorer.exe");
            }
        }
    }
    false
}
