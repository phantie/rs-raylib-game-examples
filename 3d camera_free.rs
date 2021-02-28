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

    let mut camera = Camera3D::perspective(
        rvec3(10, 10, 10),
        rvec3(0, 0, 0),
        rvec3(0, 1, 0),
        45.0,
    );

    let cube_position = rvec3(0, 0, 0);

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        if rl.is_key_down(KEY_Z) {
            camera.target = rvec3(0, 0, 0)
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(camera);

            d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
        
            d.draw_grid(10, 1.0);
        }

        d.draw_rectangle(10, 10, 320, 133, Color::SKYBLUE.fade(0.5));
        d.draw_rectangle_lines(10, 10, 320, 133, Color::BLUE);

        d.draw_text("Free camera default controls:", 20, 20, 10, Color::BLACK);
        d.draw_text("- Mouse Wheel to Zoom in-out", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Mouse Wheel Pressed to Pan", 40, 60, 10, Color::DARKGRAY);
        d.draw_text("- Alt + Mouse Wheel Pressed to Rotate", 40, 80, 10, Color::DARKGRAY);
        d.draw_text("- Alt + Ctrl + Mouse Wheel Pressed for Smooth Zoom", 40, 100, 10, Color::DARKGRAY);
        d.draw_text("- Z to zoom to (0, 0, 0)", 40, 120, 10, Color::DARKGRAY);
    }
}