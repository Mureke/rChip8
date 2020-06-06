use std::fs::File;
use std::io::Read;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;

pub struct EventHandler {
    events: sdl2::EventPump,
}

impl EventHandler {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        EventHandler {
            events: sdl_context.event_pump().unwrap()
        }
    }

    pub fn event_poller(&mut self) -> Result<[bool; 16], ()> {
        /// Method for collecting key events and watching quit events.
        /// Created mostly using this document: https://rust-sdl2.github.io/rust-sdl2/sdl2/keyboard/struct.KeyboardState.html
        ///
        /// Returns:
        /// Result<[bool; 16], ()>
        /// Where boolean values indicates different keys pressed
        ///
        /// Usage example:
        ///  let mut events = EventHandler::new(&sdl2_context);
        ///  while let Ok(keypad) = events.event_poller() {
        ///       // Do something
        ///  }

        // Return error which exists main loop if Quit event is found or if
        // esc is pressed
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(())
                },
                _ => {}
            }
        }

        // Get pressed keys from event pump as vector
        let pressed_keys: Vec<Keycode> = self.events.keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Initialize keystate with false
        let mut key_state = [false; 16];

        // Get all pressed keys
        for key in pressed_keys {
            let key_index = match key {
                Keycode::Num1 => Some(0x01),
                Keycode::Num2 => Some(0x02),
                Keycode::Num3 => Some(0x03),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x04),
                Keycode::W => Some(0x05),
                Keycode::E=> Some(0x06),
                Keycode::R => Some(0x0d),
                Keycode::A => Some(0x07),
                Keycode::S => Some(0x08),
                Keycode::D => Some(0x09),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X=> Some(0x00),
                Keycode::C => Some(0x0b),
                Keycode::V => Some(0x0f),
                _ => None
            };

            //Store keys to key state and return key state to be passed to cpu
            if let Some(i) = key_index {
                key_state[i] = true;
            }
        }
        Ok(key_state)
    }

}