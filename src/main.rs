mod cpu;
mod drivers;
use drivers::Display;

use std::process::exit;
use std::time;
use std::thread;
use rand::Rng;


fn main() {
    let sdl2_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl2_context);

    let mut vram : [[u8; 64]; 32] = [[1; 64 as usize]; 32 as usize];
    let mut rng = rand::thread_rng();

    loop {
        for _i in 0..40 {
            let vram2 = vram.clone();
            for (y, _row) in vram2.iter().enumerate() {
                for (x, _row) in vram2[y].iter().enumerate(){
                    vram[y][x] = rng.gen_range(0, 2) as u8;
                    print!("{}", vram[y][x])
                }
            }
            display.draw(&vram);
            thread::sleep(time::Duration::from_secs(1))
        }
        exit(0)
    }
}

// fn draw_loop(canvas: &mut WindowCanvas, mode: bool) {
//     let mut color1: Color;
//     let mut color2: Color;
//     let color_black = Color::RGB(0, 0, 0);
//     let color_white = Color::RGB(255, 255, 255);
//
//     for i in 0..W_HEIGHT {
//         if i % 2 == 0 {
//             color1 = color_white;
//             color2 = color_black;
//         } else {
//             color1 = color_black;
//             color2 = color_white;
//         }
//         for j in 0..W_WIDTH {
//             let x = (j as u32) * SCALE_FACTOR;
//             let y = (i as u32) * SCALE_FACTOR;
//             if !mode {
//                 // Remove all white dots
//                 canvas.set_draw_color(color_black);
//             } else if j % 2 == 0 {
//                 canvas.set_draw_color(color1);
//             } else {
//                 canvas.set_draw_color(color2);
//             }
//             canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
//             canvas.present();
//             thread::sleep(time::Duration::from_millis(1))
//         }
//     }
// }