use std::fs::File;
use std::io::Read;

pub struct RomReader {
    pub data: [u8; 3584],
    size: usize,
}

impl RomReader {
    pub fn new(rom_file: &str) -> Self {
        let mut f = File::open(rom_file).expect("file not found");

        // Max buffer size = memory size - 512
        let mut buffer = [0u8; 3584];

        let bytes = if let Ok(bytes) = f.read(&mut buffer) {
            bytes
        } else {
            0
        };
        RomReader {
            data: buffer,
            size: bytes
        }
    }
}