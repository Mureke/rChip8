use crate::font_set::FONT_SET;

pub struct Cpu {
    opcode: u8, // Opcode
    ram: [u8; 4096], // Memory
    v: [u8; 16], // CPU registers
    pc: usize, // Index register
    i: usize, // Program counter,
    vram: [[u8; 64]; 32],
    vram_changed: bool,
    delay_timer: u8,
    sound_timer: u8,
    stack: [usize; 16],
    sp: usize,
    keys: [bool; 16],
    keys_updated: bool
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
            keys_updated: false
        }
    }
}