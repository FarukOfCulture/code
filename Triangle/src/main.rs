extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const WINDOW_SIZE: u32 = 1000;
const BG_COLOR: Color = Color::RGB(0x18, 0x18, 0x18);
const LINE_COLOR: Color = Color::RGB(0xff, 0xff, 0xff);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = sdl_context
        .video()
        .unwrap()
        .window("Triangle", WINDOW_SIZE, WINDOW_SIZE)
        .build()
        .unwrap()
        .into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(BG_COLOR);
    canvas.clear();

    canvas.set_draw_color(LINE_COLOR);
    triangle(
        &mut canvas,
        WINDOW_SIZE as i32 / 2,
        0,
        0,
        WINDOW_SIZE as i32,
        WINDOW_SIZE as i32,
        WINDOW_SIZE as i32,
    );

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

fn interpolate(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    if y1 == y2 {
        return Vec::new();
    }
    let mut y1 = y1;
    let mut y2 = y2;
    let mut x1 = x1;
    let mut x2 = x2;
    if y1 > y2 {
        (y1, y2) = (y2, y1);
        (x1, x2) = (x2, x1);
    }
    let step = (x2 - x1) as f32 / (y2 - y1) as f32;

    let mut res: Vec<(i32, i32)> = Vec::new();

    let mut count = 0;
    for ty in y1..y2 {
        let tx = x1 + (step * count as f32) as i32;
        res.push((tx, ty));
        count += 1
    }
    res
}

fn triangle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
) {
    let mut points: Vec<(i32, i32)> = Vec::new();
    points.append(&mut interpolate(x1, y1, x2, y2));
    points.append(&mut interpolate(x2, y2, x3, y3));
    points.append(&mut interpolate(x3, y3, x1, y1));
    points.sort_by(|a, b| a.1.cmp(&b.1));

    for q in (0..points.len()).step_by(2) {
        for w in points[q].0.min(points[q + 1].0)..points[q + 1].0.max(points[q].0) {
            points.push((w, points[q].1))
        }
    }

    for q in points {
        _ = canvas.draw_point(Point::new(q.0, q.1));
    }
}
