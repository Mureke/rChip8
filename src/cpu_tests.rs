use super::*;

const PC: usize = 0x200;

fn new_cpu_with_inital_data() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = PC;
    cpu.v = [0, 0, 1, 1, 3, 4, 5, 6, 7, 8, 4, 4, 2, 1, 3, 4];
    cpu
}


fn set_register_values_and_run(mut cpu: Cpu, x: u8, y: u8, opcode: u16) -> Cpu {
    cpu.v[0] = x;
    cpu.v[1] = y;
    cpu.run_opcode(opcode);
    cpu
}

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

#[test]
fn test_delay_and_sound_timer() {
    let mut cpu = Cpu::new();
    cpu.sound_timer = 10;
    cpu.delay_timer = 10;
    cpu.cycle();
    assert_eq!(cpu.sound_timer, 9);
    assert_eq!(cpu.delay_timer, 9);
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
    assert_eq!(cpu.stack[0], 0x02668);
    assert_eq!(cpu.pc, 0x0267)
}

// SE Vx, byte
#[test]
fn test_op3xkk() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x3003);
    assert_eq!(cpu.pc, PC + 2);
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x3000);
    assert_eq!(cpu.pc, PC + 4);
}

// SNE Vx, byte
#[test]
fn test_op4xkk() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x4003);
    assert_eq!(cpu.pc, PC + 4);
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x4000);
    assert_eq!(cpu.pc, PC + 2);
}

// SE Vx, Vy
#[test]
fn test_op5xy0() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x5020);
    assert_eq!(cpu.pc, PC + 2);
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x5010);
    assert_eq!(cpu.pc, PC + 4);
}

//  LD Vx, byte
#[test]
fn test_op6xkk() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x6415);
    assert_eq!(cpu.v[4], 0x0015)
}

// ADD Vx, byte
#[test]
fn test_op7xkk() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x7315); // x = 3, kk = 21
    assert_eq!(cpu.v[3], 0x0016) // v[3] = 22
}

//  LD Vx, Vy
#[test]
fn test_op8xy0() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x8350);
    assert_eq!(cpu.v[3], 4)
}

//  OR Vx, Vy
#[test]
fn test_op8xy1() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x8351);
    assert_eq!(cpu.v[3], 0x05)
}

//  AND Vx, Vy
#[test]
fn test_op8xy2() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x8562);
    assert_eq!(cpu.v[5], 0x04)
}

//  XOR Vx, Vy
#[test]
fn test_op8xy3() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.run_opcode(0x8563);
    assert_eq!(cpu.v[5], 0x01)
}

//  ADD Vx, Vy
#[test]
fn test_op8xy4() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 255, 0x8014);
    assert_eq!(cpu.v[0x0F], 1);
    assert_eq!(cpu.v[0x00], 0);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 2, 0x8014);
    assert_eq!(cpu.v[0x0F], 0);
    assert_eq!(cpu.v[0x00], 0x03);
}

//  SUB Vx, Vy
#[test]
fn test_op8xy5() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 6, 2, 0x8015);
    assert_eq!(cpu.v[0x0F], 1);
    assert_eq!(cpu.v[0x00], 0x04);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 3, 3, 0x8015);
    assert_eq!(cpu.v[0x0F], 0);
    assert_eq!(cpu.v[0x00], 0x00);
}

// SHR Vx {, Vy}
#[test]
fn test_op8xy6() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 2, 0x8016);
    assert_eq!(cpu.v[0x0F], 1);
    assert_eq!(cpu.v[0x00], 0x00);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 2, 3, 0x8016);
    assert_eq!(cpu.v[0x0F], 0);
    assert_eq!(cpu.v[0x00], 0x01);
}

// SHR Vx {, Vy}
#[test]
fn test_op8xy7() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 2, 0x8017);
    assert_eq!(cpu.v[0x0F], 1);
    assert_eq!(cpu.v[0x00], 0x01);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 255, 15, 0x8017);
    assert_eq!(cpu.v[0x0F], 0);
    assert_eq!(cpu.v[0x00], 0x10);
}

// SHL Vx {, Vy}
#[test]
fn test_op8xye() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 128, 2, 0x801E);
    assert_eq!(cpu.v[0x0F], 1);
    assert_eq!(cpu.v[0x00], 0);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 40, 15, 0x801E);
    assert_eq!(cpu.v[0x0F], 0);
    assert_eq!(cpu.v[0x00], 80);
}

// SNE Vx, Vy
#[test]
fn test_op9xy0() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 1, 0x9010);
    assert_eq!(cpu.pc, PC + 2);
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 2, 0x9010);
    assert_eq!(cpu.pc, PC + 4);
}

// LD I, addr
#[test]
fn test_annn() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 1, 0xAFFF);
    assert_eq!(cpu.i, 0xFFF);
}

// JP V0, addr
#[test]
fn test_bnnn() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 4, 1, 0xB001);
    assert_eq!(cpu.pc, 0x05);
}

// RND Vx, byte
// Not sure how to test this since it generates random numbers but will test that pc is corect
#[test]
fn test_cxkk() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 4, 1, 0xC001);
    assert_eq!(cpu.pc, PC + 0x002);
}

#[test]
fn test_dxyn() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.i = 0;
    cpu.memory[0] = 0b11111111;
    cpu.memory[1] = 0b00000000;
    cpu.vram[0][0] = 1;
    cpu.vram[0][1] = 0;
    cpu.vram[1][0] = 1;
    cpu.vram[1][1] = 0;
    cpu.v[0] = 0;
    cpu = set_register_values_and_run(cpu, 0, 1, 0xd002);

    assert_eq!(cpu.vram[0][0], 0);
    assert_eq!(cpu.vram[0][1], 1);
    assert_eq!(cpu.vram[1][0], 1);
    assert_eq!(cpu.vram[1][1], 0);
    assert_eq!(cpu.v[0x0f], 1);
    assert!(cpu.vram_changed);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_ex9e() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.keys[4] = true;
    cpu.v[2] = 4;
    cpu = set_register_values_and_run(cpu, 2, 0, 0xe29e);
    assert_eq!(cpu.pc, PC + 4);

    let mut cpu = new_cpu_with_inital_data();
    cpu.keys[5] = false;
    cpu.v[2] = 4;
    cpu = set_register_values_and_run(cpu, 2, 0, 0xe29e);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_exa1() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.keys[4] = true;
    cpu.v[2] = 4;
    cpu = set_register_values_and_run(cpu, 2, 0, 0xe2A1);
    assert_eq!(cpu.pc, PC + 2);

    let mut cpu = new_cpu_with_inital_data();
    cpu.keys[5] = false;
    cpu.v[2] = 4;
    cpu = set_register_values_and_run(cpu, 2, 0, 0xe2A1);
    assert_eq!(cpu.pc, PC + 4);
}

#[test]
fn test_fx07() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.delay_timer = 6;
    cpu = set_register_values_and_run(cpu, 1, 0, 0xf107);
    assert_eq!(cpu.v[1], 6);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx0a() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 6, 0xf10A);

    assert_eq!(cpu.wait_for_input, true);
    assert_eq!(cpu.input_address, 1);
    assert_eq!(cpu.pc, PC + 2);

    cpu.cycle();
    assert_eq!(cpu.wait_for_input, true);
    assert_eq!(cpu.input_address, 1);

    cpu.keys[0] = true;

    cpu.cycle();
    assert_eq!(cpu.wait_for_input, false);
    assert_eq!(cpu.v[0x01], 0);
}

#[test]
fn test_fx15() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 6, 0xf115);
    assert_eq!(cpu.delay_timer, 6);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx18() {
    let mut cpu = new_cpu_with_inital_data();
    cpu = set_register_values_and_run(cpu, 1, 6, 0xf118);
    assert_eq!(cpu.sound_timer, 6);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx1e() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.i = 10;
    cpu = set_register_values_and_run(cpu, 1, 6, 0xf11e);
    assert_eq!(cpu.i, 16);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx29() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.i = 10;
    cpu = set_register_values_and_run(cpu, 5, 0, 0xf029);
    assert_eq!(cpu.i, 25);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx33() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.v[5] = 187;
    cpu.i = 300;
    cpu = set_register_values_and_run(cpu, 5, 2, 0xf533);
    assert_eq!(cpu.memory[300], 1);
    assert_eq!(cpu.memory[301], 8);
    assert_eq!(cpu.memory[302], 7);
    assert_eq!(cpu.pc, PC + 2);
}

#[test]
fn test_fx55() {

    let mut cpu = new_cpu_with_inital_data();
    cpu.i = 300;
    cpu.run_opcode(0xf555);
    for i in 0..5 {
        assert_eq!(cpu.memory[300+i], cpu.v[i])
    }
    assert_eq!(cpu.pc, PC + 2);

}

#[test]
fn test_fx65() {
    let mut cpu = new_cpu_with_inital_data();
    cpu.i = 300;
    for i in 0..7 {
        cpu.memory[cpu.i + i] = i as u8;
    }
    cpu.run_opcode(0xf765);

    for i in 0..7 {
        assert_eq!(cpu.v[i], cpu.memory[cpu.i + i])

    }
    assert_eq!(cpu.pc, PC + 2);

}

// TODO: Write tests for opcodes and write opcode
