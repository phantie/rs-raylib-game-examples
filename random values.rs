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

    let target_fps = 60;
    let everyN = 2;

    rl.set_target_fps(target_fps);

    let mut random_value = rv(-7, 7);
    let mut frames_counter = 0;

    while !rl.window_should_close() {
        
        if frames_counter / (target_fps * everyN) == 1 {
            random_value = rv(-7, 7);
            frames_counter = 0;
        }

        frames_counter += 1;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        d.draw_text("Every 2 seconds a new random value is generated:", 130, 100, 20, Color::MAROON);

        d.draw_text(format!("{}", random_value).as_str(), 360, 180, 80, Color::LIGHTGRAY);
    }
}
