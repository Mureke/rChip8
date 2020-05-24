mod cpu;
mod utils;
mod font_set;

use cpu::Cpu;
use utils::Display;

use std::process::exit;


fn main() {
    let sdl2_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl2_context);

    display.display_tester();

    let mut processor = Cpu::new();

    exit(0)
}