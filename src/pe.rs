use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const PE_SIGNATURE_OFFSET: u64 = 0x3C;
const COFF_HEADER_OFFSET: u64 = 0x04;

/// Get the offset of the PE signature in a file
///
/// # Arguments
/// * `file_path` - The path to the file to be checked.
///
/// # Returns
///
/// * `io::Result<u64>` - Returns the offset of the PE signature in the file if found, otherwise returns an `io::Error`.
pub fn get_offset_of_pe(file_path: &str) -> io::Result<u64> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(PE_SIGNATURE_OFFSET))?;
    let mut offset_bytes = [0; 4];
    file.read_exact(&mut offset_bytes)?;
    Ok(u32::from_le_bytes(offset_bytes) as u64)
}
/// Check if a PE file is 64-bit
///
/// # Arguments
/// * `file_path` - The path to the file to be checked.
///
/// # Returns
///
/// * `io::Result<bool>` - Returns `Ok(true)` if the file is a 64-bit PE file, otherwise returns `Ok(false)`.
pub fn is_64(file_path: &str) -> io::Result<bool> {
    let pe_offset = get_offset_of_pe(file_path)?;
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(pe_offset + COFF_HEADER_OFFSET))?;
    let mut machine_type_bytes = [0; 2];
    file.read_exact(&mut machine_type_bytes)?;
    let machine_type = u16::from_le_bytes(machine_type_bytes);

    Ok(machine_type == 0x8664)
}
