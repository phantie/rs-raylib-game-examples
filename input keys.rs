#![allow(non_snake_case)]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn static_file(name: &str) -> String {
    use std::path::Path;

    let mbstr = Path::new("src")
        .join("static")
        .join(name)
        .into_os_string()
        .into_string();

    match mbstr {
        Ok(res) => res,
        Err(_) => panic!("use human names for files (not {}), thanks", name),
    }
}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut ball_pos: Vector2 = Vector2::new(ww as f32 / 2.0, wh as f32 / 2.0);
    rl.set_target_fps(60);

    let deltaX = 2.0;
    let deltaY = 2.0;

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_RIGHT) {
            ball_pos.x += deltaX
        };
        if rl.is_key_down(KEY_LEFT) {
            ball_pos.x -= deltaX
        };
        if rl.is_key_down(KEY_DOWN) {
            ball_pos.y += deltaY
        };
        if rl.is_key_down(KEY_UP) {
            ball_pos.y -= deltaY
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);

        d.draw_circle_v(ball_pos, 50 as f32, Color::MAROON);
    }
}
