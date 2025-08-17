use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};

pub struct Patcher;

impl Patcher {
    pub fn new() -> Self {
        Patcher
    }

    pub fn patch_bytes(&mut self, file_path: &str, pattern: &[u8], replacement: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;
        let mut buffer = vec![0; 4096];
        let mut position = 0;

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            for i in 0..bytes_read {
                if i + pattern.len() > bytes_read {
                    // Need to handle patterns that cross buffer boundaries
                    // For simplicity, we assume the pattern is smaller than the buffer
                    break;
                }
                if buffer[i..i + pattern.len()] == *pattern {
                    file.seek(SeekFrom::Start(position + i as u64))?;
                    file.write_all(replacement)?;
                    return Ok(());
                }
            }
            position += bytes_read as u64;
        }

        Ok(())
    }

    pub fn is_pattern_present(&self, file_path: &str, pattern: &[u8]) -> io::Result<bool> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(buffer.windows(pattern.len()).any(|window| window == pattern))
    }

    pub fn find_string_offset(&self, file_path: &str, search_string: &str) -> io::Result<i64> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let search_bytes = search_string.as_bytes();
        if let Some(pos) = buffer.windows(search_bytes.len()).position(|window| window == search_bytes) {
            Ok(pos as i64)
        } else {
            Ok(-1)
        }
    }

    pub fn patch_bytes_by_offset(&mut self, file_path: &str, offset: i64, replacement_bytes: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(file_path)?;
        file.seek(SeekFrom::Start(offset as u64))?;
        file.write_all(replacement_bytes)?;
        Ok(())
    }
}
