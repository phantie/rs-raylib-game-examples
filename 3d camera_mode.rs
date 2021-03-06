#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

trait Drawable {
    fn draw(&self, d: &mut impl RaylibDraw);
}



fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");

    let camera = Camera3D::perspective(
        rvec3(0, 10, 10),
        rvec3(0, 0, 0),
        rvec3(0, 1, 0),
        45.0,
    );

    let cube_position = rvec3(0, 0, 0);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(camera);

            d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
        
            d.draw_grid(10, 1.0);
        }

        d.draw_fps(10, 10);
    }
}