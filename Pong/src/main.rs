extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const BG_COLOR: Color = Color::RGB(0x18, 0x18, 0x18);
const FG_COLOR: Color = Color::RGB(0xff, 0xff, 0xff);
// const window_width: u32 = 1280;
// const window_height: u32 = 720;
const BALL_SIZE: u32 = 20;
const SPEED: i32 = 7;
const PADDLE_THICKNESS: u32 = 15;
const PADDLE_SIZE: u32 = 100;
const PADDLE_PADDING: u32 = 20;
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = sdl_context
        .video()
        .unwrap()
        .window("Rule 110", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap()
        .into_canvas()
        .build()
        .unwrap();

    let mut rects: [Rect; 3] = [
        Rect::new(
            (WINDOW_WIDTH / 2 - BALL_SIZE / 2) as i32,
            (WINDOW_HEIGHT / 2 - BALL_SIZE / 2) as i32,
            BALL_SIZE,
            BALL_SIZE,
        ),
        Rect::new(
            (WINDOW_WIDTH - PADDLE_PADDING - PADDLE_THICKNESS) as i32,
            (WINDOW_HEIGHT / 2 - PADDLE_SIZE / 2) as i32,
            PADDLE_THICKNESS,
            PADDLE_SIZE,
        ),
        Rect::new(
            PADDLE_PADDING as i32,
            (WINDOW_HEIGHT / 2 - PADDLE_SIZE / 2) as i32,
            PADDLE_THICKNESS,
            PADDLE_SIZE,
        ),
    ];

    let mut paddler = 0;
    let mut paddlel = 0;
    let mut balldx = 1;
    let mut balldy = 1;
    let mut paused = true;
    let mut ball: Rect = rects[0];

    'running: loop {
        if !paused {
            if rects[1].y + paddler * SPEED < (WINDOW_HEIGHT - PADDLE_SIZE) as i32
                && rects[1].y + paddler * SPEED >= 0 as i32
            {
                rects[1].set_y(rects[1].y + paddler * SPEED);
            }
            if rects[2].y + paddlel * SPEED < (WINDOW_HEIGHT - PADDLE_SIZE) as i32
                && rects[2].y + paddlel * SPEED >= 0 as i32
            {
                rects[2].set_y(rects[2].y + paddlel * SPEED);
            }

            ball.set_x(rects[0].x + balldx * SPEED);
            if ball.has_intersection(rects[1]) || ball.has_intersection(rects[2]) {
                balldx *= -1;
            }
            if ball.x < 0 || ball.x >= WINDOW_WIDTH as i32 {
                balldx *= -1;
                rects[0].x = (WINDOW_WIDTH / 2 - BALL_SIZE / 2) as i32;
                rects[0].y = (WINDOW_HEIGHT / 2 - BALL_SIZE / 2) as i32;
            }
            ball.set_x(rects[0].x);
            ball.set_y(rects[0].y + balldy * SPEED);
            if ball.has_intersection(rects[1])
                || ball.has_intersection(rects[2])
                || ball.y < 0
                || ball.y + BALL_SIZE as i32 >= WINDOW_HEIGHT as i32
            {
                balldy *= -1
            }

            rects[0].set_x(rects[0].x + balldx * SPEED);
            rects[0].set_y(ball.y);
        }

        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        canvas.set_draw_color(FG_COLOR);
        _ = canvas.fill_rects(&rects);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => paddler = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => paddler = -1,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => paddler = 0,

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => paddlel = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => paddlel = -1,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => paddlel = 0,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => paused = !paused,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
