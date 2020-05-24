use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use rand::Rng;
use std::thread;
use std::time;

const SCALE_FACTOR: u32 = 20;
const W_HEIGHT: u32 = 32 as u32;
const W_WIDTH: u32 = 64 as u32;

pub struct Display {
    canvas: WindowCanvas
}

impl Display {
    pub fn new(sdl2_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl2_context.video().unwrap();
        let window = video_subsystem.window(
            "rChip8",
            W_WIDTH * SCALE_FACTOR,
            W_HEIGHT * SCALE_FACTOR,
        ).position_centered()
            .opengl().build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap_or_else(|e| panic!("Error: {}", e));

        canvas.set_draw_color(color(0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; W_WIDTH as usize]; W_HEIGHT as usize]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;
                self.canvas.set_draw_color(color(col));
                self.canvas.fill_rect(
                    Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR)
                ).unwrap_or_else(|e| panic!("Error: {}", e));
            }
        }
        self.canvas.present();
    }

    // Just for testing display
    pub fn display_tester(&mut self) {
        let mut vram: [[u8; 64]; 32] = [[1; 64 as usize]; 32 as usize];
        let mut rng = rand::thread_rng();

        loop {
            for _i in 0..100 {
                let vram2 = vram.clone();
                for (y, _row) in vram2.iter().enumerate() {
                    for (x, _row) in vram2[y].iter().enumerate() {
                        vram[y][x] = rng.gen_range(0, 2) as u8;
                    }
                }
                self.draw(&vram);
                thread::sleep(time::Duration::from_millis(11))
            }
        }
    }
}

pub fn color(input: u8) -> pixels::Color {
    match input {
        0 => return pixels::Color::RGB(0, 0, 0),
        1 => return pixels::Color::RGB(255, 255, 255),
        _ => panic!("Invalid value")
    }
}
