#![allow(non_snake_case)]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
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

    let mut ball_pos: Vector2 = 
        Vector2::new(-100.0, -100.0);
    let mut ball_color = Color::DARKBLUE;

    rl.set_target_fps(60);

    // let deltaX = 2.0;
    // let deltaY = 2.0;

    while !rl.window_should_close() {
        ball_pos = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            ball_color = Color::MAROON
        }
        else if rl.is_mouse_button_pressed(MOUSE_MIDDLE_BUTTON) {
            ball_color = Color::LIME
        };
        if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
            ball_color = Color::DARKBLUE
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);

        d.draw_circle_v(ball_pos, 40 as f32, ball_color);
    }
}
