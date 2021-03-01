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

    rl.set_target_fps(60);

    let mut dropped_files: Vec<String> = vec![];

    while !rl.window_should_close() {
        if rl.is_file_dropped() {
            dropped_files = rl.get_dropped_files();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        d.draw_text(
            if dropped_files.len() == 0 {
                "Drop your files to this window!"
            } else {
                "Dropped files:"
            },
            100,
            40,
            20,
            Color::DARKGRAY,
        );

        for (i, file) in dropped_files.iter().enumerate() {
            d.draw_rectangle(
                0,
                (85 + 40 * i) as i32,
                ww,
                40,
                Color::LIGHTGRAY.fade(if i % 2 == 0 { 0.5 } else { 0.3 }),
            );

            d.draw_text(file, 120, (100 + 40*i) as i32, 10, Color::GRAY);
        }
    }
}
