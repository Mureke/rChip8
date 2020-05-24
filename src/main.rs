mod cpu;
mod utils;
use utils::Display;

use std::process::exit;
use std::time;
use std::thread;

fn main() {
    let sdl2_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl2_context);

    display.display_tester();

    exit(0)
}