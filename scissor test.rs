#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::cmp::{max, min};
use structopt::StructOpt;

mod options;

trait Drawable {
    fn draw(&self, d: &mut impl RaylibDraw3D);
}

trait Random {
    fn rand() -> Self;
}

const rv: &dyn Fn(i32, i32) -> i32 = &get_random_value;


fn main() {
    let opt = options::Opt::from_args();
    let (ww, wh) = (opt.width, opt.height);

    let (mut rl, thread) = raylib::init().size(ww, wh).title("ray").vsync().build();

    let mut scissor_area = rrect(0, 0, 300, 300);

    let mut scissor_mode = true;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_S) {
            scissor_mode = !scissor_mode;
        }

        scissor_area.x = rl.get_mouse_x() as f32 - scissor_area.width / 2.0;
        scissor_area.y = rl.get_mouse_y() as f32 - scissor_area.height / 2.0;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        if scissor_mode {
            let mut d = d.begin_scissor_mode(
                scissor_area.x as i32,
                scissor_area.y as i32,
                scissor_area.width as i32,
                scissor_area.height as i32
            );

            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::RED,
            );

            d.draw_text(
                "Move the mouse around to reveal this text!",
                190,
                200,
                20,
                Color::LIGHTGRAY,
            );
        }

        d.draw_rectangle_lines_ex(scissor_area, 1, Color::BLACK);
        d.draw_text("Press S to toggle scissor test", 10, 10, 20, Color::BLACK);
    }
}
