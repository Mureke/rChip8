mod cpu;
mod utils;
mod font_set;

use cpu::Cpu;
use utils::Display;
use utils::RomReader;

use std::process::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_filename = &args[1];

    let sdl2_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl2_context);

    let rom = RomReader::new(rom_filename);
    let mut processor = Cpu::new();
    processor.read_data_to_memory(&rom.data);

    assert_eq!(processor.ram[0x200 + 1000], rom.data[1000]);
    exit(0)
}