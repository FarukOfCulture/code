extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;

const WINDOW_SIZE: u32 = 1000;
const BG_COLOR: Color = Color::RGB(0x18, 0x18, 0x18);
const FG_COLOR: Color = Color::RGB(0xff, 0xff, 0xff);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = sdl_context
        .video()
        .unwrap()
        .window("Circle", WINDOW_SIZE, WINDOW_SIZE)
        .build()
        .unwrap()
        .into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(BG_COLOR);
    canvas.clear();

    canvas.set_draw_color(FG_COLOR);
    circle(&mut canvas, 100, 100, 100);
    circle(&mut canvas, 200, 200, 99);

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

fn circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, r: i32) {
    let mut points: Vec<sdl2::rect::Point> = Vec::new();

    for ty in -r..r {
        for tx in -r..r {
            if tx.pow(2) + ty.pow(2) <= r.pow(2) {
                points.push(sdl2::rect::Point::new(x + tx, y + ty))
            }
        }
    }
    _ = canvas.draw_points(points.as_slice());
}

fn circle_has_intersection(x1: i32, y1: i32, r1: i32, x2: i32, y2: i32, r2: i32) -> bool {
    (((x1.max(x2) - x1.min(x2)).pow(2) + (y1.max(y2) - y1.min(y2)).pow(2)) as f32).sqrt()
        <= (r1 + r2) as f32
}
