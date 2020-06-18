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
    assert_eq!(cpu.memory[mempointer+1] , 0);
    assert_eq!(cpu.memory[mempointer+2], 33);
    assert_eq!(cpu.memory[mempointer+3], 43);
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
// TODO: Write tests for opcodes and write opcode
//
// based on tests