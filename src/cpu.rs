use crate::font_set::FONT_SET;
use crate::utils::RomReader;
use std::process::exit;

pub struct Cpu {
    opcode: u16,
    // Opcode
    ram: [u8; 4096],
    // Memory TODO: Remove pub
    v: [u8; 16],
    // CPU registers
    pc: usize,
    // Index register
    i: usize,
    // Program counter,
    pub vram: [[u8; 64]; 32],
    pub vram_changed: bool,
    delay_timer: u8,
    sound_timer: u8,
    stack: [usize; 16],
    sp: usize,
    keys: [bool; 16],
    keys_updated: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut ram = [0u8; 4096];
        for i in 0..FONT_SET.len() {
            ram[i] = FONT_SET[i];
        }

        Cpu {
            opcode: 0,
            ram: ram,
            v: [0; 16],
            pc: 0x200,
            i: 0,
            vram: [[0; 64]; 32],
            vram_changed: false,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [false; 16],
            keys_updated: false,
        }
    }

    pub fn read_data_to_memory(&mut self, input: &[u8]) {
        for (i, &byte) in input.iter().enumerate() {
            let address = 0x200 + 1;
            if address < 4096 {
                self.ram[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn cycle(&mut self) {
        self.fetch_and_decode_opcode(); // Decode opcode and set to self.opcode
    }

    fn fetch_and_decode_opcode(&mut self) {
        /// Fetch and decode opcodes
        /// Since chip8 opcodes are two bytes long we are combining
        /// Two bytes from ram at pc and pc+1
        let byte1 = (self.ram[self.pc] as u16) << 8;
        let byte2 = self.ram[self.pc+1] as u16;
        self.opcode = byte1 | byte2;

        println!("{:b}", byte1);
        println!("0000000{:b}", byte2 );
        println!("{:b}", self.opcode);
    }
}