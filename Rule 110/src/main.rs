extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const BG_COLOR: Color = Color::RGB(0x18, 0x18, 0x18);
const LINE_COLOR: Color = Color::RGB(0xff, 0xff, 0xff);
const WINDOW_SIZE: u32 = 1000;

fn main() {
    let size: usize = std::env::args()
        .nth(1)
        .expect("Please insert size as command-line flag")
        .parse()
        .unwrap();
    let rect_size: u32 = WINDOW_SIZE / size as u32;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rule 110", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut rects: Vec<Rect> = vec![Rect::new(
        (size as i32 - 1) * rect_size as i32,
        0,
        rect_size,
        rect_size,
    )];
    let mut line: Vec<bool> = vec![false; size + 2];
    line[size] = true;
    let mut newline: Vec<bool> = vec![false; size + 2];
    for q in 0..size - 1 {
        for w in 0..size {
            newline[w + 1] = match line.get(w..w + 3).unwrap() {
                [true, true, true] | [true, false, false] | [false, false, false] => false,
                [true, true, false]
                | [true, false, true]
                | [false, true, true]
                | [false, true, false]
                | [false, false, true] => {
                    rects.push(Rect::new(
                        w as i32 * rect_size as i32,
                        (q as i32 + 1) * rect_size as i32,
                        rect_size,
                        rect_size,
                    ));
                    true
                }
                _ => unreachable!(),
            };
        }
        line = newline.clone();
    }

    canvas.set_draw_color(BG_COLOR);
    canvas.clear();
    canvas.set_draw_color(LINE_COLOR);
    _ = canvas.fill_rects(&rects);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { .. } => break 'running,
                _ => {}
            }
        }
    }
}
