use crate::font_set::FONT_SET;
use crate::utils::RomReader;
use std::process::exit;
use sdl2::hint::set;

pub struct CycleState<'a> {
    pub vram_changed: bool,
    pub vram: &'a [[u8; 64]; 32],
    pub sound: bool,
}

enum PointerAction {
    Next,
    Jump(usize),
}

impl PointerAction {
    fn skip_or_next(condition: bool, addr: usize) -> PointerAction {
        if condition {
            PointerAction::Jump(addr)
        } else {
            PointerAction::Next
        }
    }
}

pub struct Cpu {
    /// Cpu
    /// Used these articles as reference:
    /// http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
    /// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
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
            memory,
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
            sound: self.sound_timer > 0,
        };
        if self.sound_timer == 1 {
            self.sound_timer = 0;
        }

        cycle_state
    }

    /// Fetch and decode opcodes
    /// Since chip8 opcodes are two bytes long we are combining
    /// Two bytes from memory at pc and pc+1
    fn fetch_and_decode_opcode(&mut self) -> u16 {
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

        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        println!("opcode: {:b}", opcode);
        println!("nnn: {:b}", nnn);
        println!("kk: {:b}", kk);
        println!("x: {:b}", x);
        println!("y: {:b}", y);
        println!("n: {:b}", n);

        let pc_action = match (nibbles) {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(), // CLS
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(), // RET
            (0x01, _, _, _) => self.op_1nnn(nnn), // JP
            (0x02, _, _, _) => self.op_2nnn(nnn), // CALL
            _ => PointerAction::Next
        };

        match pc_action {
            PointerAction::Next => self.pc += 2,
            PointerAction::Jump(address) => self.pc = address,
        }
    }

    /// CLS
    /// Clear the display
    fn op_00e0(&mut self) -> PointerAction {
        self.vram = [[0; 64]; 32];
        self.vram_changed = true;
        PointerAction::Next
    }

    /// RET - Return from a subroutine
    /// The interpreter sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    fn op_00ee(&mut self) -> PointerAction {
        self.sp -= 1;
        PointerAction::Jump(self.stack[self.sp])
    }

    /// JP addr
    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn op_1nnn(&mut self, nnn: usize) -> PointerAction {
        PointerAction::Jump(nnn)
    }

    /// CALL addr
    /// The interpreter increments the stack pointer,
    /// then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_2nnn(&mut self, nnn: usize) -> PointerAction {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        PointerAction::Jump(nnn)
    }

    /// 3xkk - SE Vx, byte
    ///  Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn op_3xkk(&mut self, x: u16, kk: u8) -> PointerAction {
        PointerAction::skip_or_next((self.v[x] == kk), self.pc+2)
    }
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;