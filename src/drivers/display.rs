use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

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
        let mut canvas = window.into_canvas().build().unwrap_or_else(|_e| panic!("ERROR"));

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
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }

}

pub fn color(input: u8) -> pixels::Color {
    match input {
        0 => return pixels::Color::RGB(0,0,0),
        1 => return pixels::Color::RGB(255,255,255),
        _ => panic!("Invalid value")
    }
}