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
    Skip,
    Jump(usize),
}

impl PointerAction {
    fn skip_or_next(condition: bool, addr: usize) -> PointerAction {
        if condition {
            PointerAction::Skip
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
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3;

        let pc_action = match (nibbles) {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(), // CLS
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(), // RET
            (0x01, _, _, _) => self.op_1nnn(nnn), // JP
            (0x02, _, _, _) => self.op_2nnn(nnn), // CALL
            (0x03, _, _, _) => self.op_3xkk(x, kk), // SE Vx, byte
            (0x04, _, _, _) => self.op_4xkk(x, kk), //SNE Vx, byte
            (0x05, _, _, 0x00) => self.op_5xy0(x, y), //SE Vx, Vy
            (0x06, _, _, _) => self.op_6xkk(x, kk), // LD Vx, byte
            (0x07, _, _, _) => self.op_7xkk(x, kk), // ADD Vx, byte
            (0x08, _, _, 0x00) => self.op_8xy0(x, y), // LD Vx, Vy
            (0x08, _, _, 0x01) => self.op_8xy1(x, y), // OR Vx, Vy
            (0x08, _, _, 0x02) => self.op_8xy2(x, y), // AND Vx, Vy
            (0x08, _, _, 0x03) => self.op_8xy3(x, y), // XOR Vx, Vy
            (0x08, _, _, 0x04) => self.op_8xy4(x, y), // ADD Vx, Vy
            (0x08, _, _, 0x05) => self.op_8xy5(x, y), // SUB Vx, Vy
            (0x08, _, _, 0x06) => self.op_8xy6(x), // SHR Vx {, Vy}
            (0x08, _, _, 0x07) => self.op_8xy7(x, y), // SHR Vx {, Vy}
            _ => PointerAction::Next
        };

        match pc_action {
            PointerAction::Next => self.pc += 2,
            PointerAction::Skip => self.pc += 4,
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
    fn op_3xkk(&mut self, x: usize, kk: u8) -> PointerAction {
        PointerAction::skip_or_next((self.v[x] == kk), self.pc)
    }

    ///SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_4xkk(&mut self, x: usize, kk: u8) -> PointerAction {
        PointerAction::skip_or_next((self.v[x] != kk), self.pc)
    }

    /// SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_5xy0(&mut self, x: usize, y: usize) -> PointerAction {
        PointerAction::skip_or_next((self.v[x] == self.v[y]), self.pc)
    }

    /// LD Vx, byte
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn op_6xkk(&mut self, x: usize, kk: u8) -> PointerAction {
        self.v[x] = kk;
        PointerAction::Next
    }

    /// ADD Vx, byte
    ///  Set Vx = Vx + kk.
    ///  Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_7xkk(&mut self, x: usize, kk: u8) -> PointerAction {
        self.v[x] = self.v[x] + kk;
        PointerAction::Next
    }

    /// LD Vx, Vy
    /// Set Vx = Vy.
    ///  Stores the value of register Vy in register Vx.
    fn op_8xy0(&mut self, x: usize, y: usize) -> PointerAction {
        self.v[x] = self.v[y];
        PointerAction::Next
    }

    /// OR Vx, Vy
    ///  Set Vx = Vx OR Vy.
    ///  Performs a bitwise OR on the values of Vx and Vy,
    /// then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy1(&mut self, x: usize, y: usize) -> PointerAction {
        let value = self.v[x] | self.v[y];
        self.v[x] = value;
        PointerAction::Next
    }

    /// AND Vx, Vy
    ///  Set Vx = Vx AND Vy.
    ///  Performs a bitwise AND on the values of Vx and Vy,
    /// then stores the result in Vx. A bitwise AND compares the
    /// corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn op_8xy2(&mut self, x: usize, y: usize) -> PointerAction {
        let value = self.v[x] & self.v[y];
        self.v[x] = value;
        PointerAction::Next
    }

    /// XOR Vx, Vy
    ///  Set Vx = Vx XOR Vy.
    ///  Performs a bitwise exclusive OR on the values of Vx and Vy,
    /// then stores the result in Vx. An exclusive OR compares the
    /// corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn op_8xy3(&mut self, x: usize, y: usize) -> PointerAction {
        let value = self.v[x] ^ self.v[y];
        self.v[x] = value;
        PointerAction::Next
    }

   /// ADD Vx, Vy
   ///  Set Vx = Vx + Vy, set VF = carry.
   ///  The values of Vx and Vy are added together. If the result is greater
   /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_8xy4(&mut self, x: usize, y: usize) -> PointerAction {
        let value = self.v[x] as u16 + self.v[y] as u16;
        if value > 0xFF {
            self.v[0x0F] = 1;
        } else {
            self.v[0x0F] = 0;
        }
        self.v[x] = value as u8;
        PointerAction::Next
    }

    /// SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) -> PointerAction {
        self.v[0x0F] = if self.v[x] > self.v[y] {1} else {0};
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        PointerAction::Next
    }

    /// SHR Vx {, Vy}
    ///  Set Vx = Vx SHR 1.
    ///  If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8xy6(&mut self, x: usize) -> PointerAction {
        self.v[0x0F] = if self.v[x] & 1 == 1 {1} else {0};
        self.v[x] = self.v[x] / 2;
        PointerAction::Next
    }

    /// SUBN Vx, Vy
    ///  Set Vx = Vy - Vx, set VF = NOT borrow.
    ///  If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) -> PointerAction {
        self.v[0x0F] = if self.v[x] < self.v[y] {1} else {0};
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        PointerAction::Next
    }
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;