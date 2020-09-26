mod cpu;
mod utils;
mod font_set;

use cpu::Cpu;
use utils::Display;
use utils::RomReader;
use utils::Audio;
use utils::EventHandler;

use std::process::exit;
use std::env;
use std::thread;
use std::time;

fn main() {
    // Get rom file name from args
    let args: Vec<String> = env::args().collect();
    let rom_filename = &args[1];

    // Initialize sdl2
    let sdl2_context = sdl2::init().unwrap();

    // Initialize display driver
    let mut display = Display::new(&sdl2_context);

    // Initialize audio driver
    let mut audio = Audio::new(&sdl2_context);

    // Initialize keypad
    let mut event_handler = EventHandler::new(&sdl2_context);

    // Load game
    let rom = RomReader::new(rom_filename);

    // Initialize machine
    let mut processor = Cpu::new();

    // Load game to machine memory
    processor.read_data_to_memory(&rom.data);

    // Main loop.
    while let Ok(keys) = event_handler.event_poller() {
        let cycle_state = processor.cycle(keys);
        if cycle_state.vram_changed {
            display.draw(cycle_state.vram);
        }

        // Check delay timers and output timers
        if cycle_state.sound  {
            audio.start_audio()
        } else {
            audio.stop_audio()
        }

        thread::sleep(time::Duration::from_millis(2))
    }
    exit(0)

}