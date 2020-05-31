mod cpu;
mod utils;
mod font_set;

use cpu::Cpu;
use utils::Display;
use utils::RomReader;
use utils::Audio;

use std::process::exit;
use std::env;
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_filename = &args[1];

    let sdl2_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl2_context);
    let mut audio = Audio::new(&sdl2_context);

    for i in 0..10 {
        audio.start_audio();
        thread::sleep(time::Duration::from_millis(200));
        audio.stop_audio();
        thread::sleep(time::Duration::from_millis(300));
    }

    let rom = RomReader::new(rom_filename);
    let mut processor = Cpu::new();
    processor.read_data_to_memory(&rom.data);

    assert_eq!(processor.ram[0x200 + 1000], rom.data[1000]);
    exit(0)
}