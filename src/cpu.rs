use crate::font_set::FONT_SET;
use crate::utils::RomReader;
use std::process::exit;

pub struct CycleState<'a> {
    pub vram_changed: bool,
    pub vram: &'a [[u8; 64]; 32],
    pub sound: bool,
}

pub struct Cpu {
    /// Cpu
    /// Used this article as a reference:
    /// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
    opcode: u16,
    // Opcode
    memory: [u8; 4096],
    // Memory TODO: Remove pub
    v: [u8; 16],
    // CPU registers
    pc: usize,
    // Index register
    i: usize,
    // Progmemory counter,
    pub vram: [[u8; 64]; 32],
    pub vram_changed: bool,
    pub delay_timer: u8,
    pub sound_timer: u8,
    stack: [usize; 16],
    sp: usize,
    keys: [bool; 16],
    keys_updated: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut memory = [0u8; 4096];
        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }

        Cpu {
            opcode: 0,
            memory: memory,
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
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn cycle(&mut self) -> CycleState {
        self.vram_changed = false;
        let opcode = self.fetch_and_decode_opcode(); // Decode opcode and set to self.opcode
        self.run_opcode(opcode);

        let cycle_state = CycleState {
            vram_changed: self.vram_changed,
            vram: &self.vram,
            sound: self.sound_timer > 0
        };
        if self.sound_timer == 1 {
            self.sound_timer = 0;
        }

        cycle_state
    }


    fn fetch_and_decode_opcode(&mut self) -> u16 {
        /// Fetch and decode opcodes
        /// Since chip8 opcodes are two bytes long we are combining
        /// Two bytes from memory at pc and pc+1
        let byte1 = (self.memory[self.pc] as u16) << 8;
        let byte2 = self.memory[self.pc + 1] as u16;
        self.opcode = byte1 | byte2;

        println!("{:b}", byte1);
        println!("0000000{:b}", byte2);
        println!("{:b}", self.opcode);
        self.opcode
    }

    fn run_opcode(&mut self, opcode: u16) {

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        println!("opcode: {:b}", self.opcode);
        println!("nnn: {:b}", nnn);
        println!("kk: {:b}", kk);
        println!("x: {:b}", x);
        println!("y: {:b}", y);
        println!("n: {:b}", n);

        match (nibbles) {
            (0, 0, 0xe, 0) => self.op_00e0(), // CLS
            (_, _, _, _) => ()
        }
    }

    fn op_00e0(&mut self){
        /// CLS
        /// Clear the display
        self.vram = [[0; 64]; 32];
        self.vram_changed = true;
    }

}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;