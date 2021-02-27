#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut box_posY = (wh/2 - 20) as f32;
    let scroll_speed = 4.0;




    rl.set_target_fps(60);

    // let deltaX = 2.0;
    // let deltaY = 2.0;

    while !rl.window_should_close() {
        box_posY -= rl.get_mouse_wheel_move() * scroll_speed;



        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_rectangle(ww/2 - 40, box_posY as i32, 80, 80, Color::MAROON);
        d.draw_text("Use mouse wheel to move the cube up and down!", 10, 10, 20, Color::GRAY);
        d.draw_text(format!("Box position Y: {}", box_posY).as_str(), 10, 40, 20, Color::LIGHTGRAY);

    }
}
