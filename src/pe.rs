use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const PE_SIGNATURE_OFFSET: u64 = 0x3C;
const COFF_HEADER_OFFSET: u64 = 0x04;

pub fn get_offset_of_pe(file_path: &str) -> io::Result<u64> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(PE_SIGNATURE_OFFSET))?;
    let mut offset_bytes = [0; 4];
    file.read_exact(&mut offset_bytes)?;
    Ok(u32::from_le_bytes(offset_bytes) as u64)
}

pub fn is_64(file_path: &str) -> io::Result<bool> {
    let pe_offset = get_offset_of_pe(file_path)?;
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(pe_offset + COFF_HEADER_OFFSET))?;
    let mut machine_type_bytes = [0; 2];
    file.read_exact(&mut machine_type_bytes)?;
    let machine_type = u16::from_le_bytes(machine_type_bytes);

    Ok(machine_type == 0x8664)
}
