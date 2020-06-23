use super::*;

#[test]
fn test_cpu_intialization() {
    let cpu = Cpu::new();
    assert_eq!(cpu.opcode, 0);
    assert_eq!(cpu.pc, 0x200); // Test pc location
    assert_eq!(cpu.memory[0x00], 0xF0); // Test font set
    assert_eq!(cpu.memory[0x01], 0x90); // Test font set
    assert_eq!(cpu.memory[0x4F], 0x80); // Test font set
}

#[test]
fn test_load_data() {
    let mut cpu = Cpu::new();
    let data = [0, 0, 33, 43];
    cpu.read_data_to_memory(&data);

    let mempointer = 0x200;
    assert_eq!(cpu.memory[mempointer], 0);
    assert_eq!(cpu.memory[mempointer + 1], 0);
    assert_eq!(cpu.memory[mempointer + 2], 33);
    assert_eq!(cpu.memory[mempointer + 3], 43);
}

// CLS = Clear the display
#[test]
fn test_op00e0() {
    let mut cpu = Cpu::new();
    cpu.vram = [[128; 64]; 32];
    cpu.run_opcode(0x00e0);
    assert_eq!(cpu.vram_changed, true);

    for (y, _row) in cpu.vram.iter().enumerate() {
        for (x, _row) in cpu.vram[y].iter().enumerate() {
            assert_eq!(cpu.vram[y][x], 0)
        }
    }
}

// RET
#[test]
fn test_op00ee() {
    let mut cpu = Cpu::new();
    cpu.sp = 4;
    cpu.stack[3] = 0x664;
    cpu.run_opcode(0x00ee);
    assert_eq!(cpu.pc, 0x664);
    assert_eq!(cpu.sp, 3);
}

// JP addr
#[test]
fn test_op1nnn() {
    let mut cpu = Cpu::new();
    cpu.run_opcode(0x1267);
    assert_eq!(cpu.pc, 0x0267);
}

// CALL addr
#[test]
fn test_op2nnn() {
    let mut cpu = Cpu::new();
    cpu.sp = 0;
    cpu.pc = 0x02666;
    cpu.run_opcode(0x2267);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], 0x02666);
    assert_eq!(cpu.pc, 0x0267)
}

// SE Vx, byte
#[test]
fn test_op3xkk() {
    let mut cpu = Cpu::new();
    cpu.sp = 0;
    // TODO: Finish this
}

// TODO: Write tests for opcodes and write opcode
//
// based on tests