struct Cpu {
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
    keys: [usize; 16],
}

impl Cpu {

    pub fn new() -> Self {

    }
}