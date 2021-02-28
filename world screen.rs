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
    fn draw(&self, d: &mut impl RaylibDraw3D);
}

const rv: &dyn Fn(i32, i32) -> i32 = &get_random_value;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("ray");
    let (ww, wh) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(rvec3(10, 10, 10), rvec3(0, 0, 0), rvec3(0, 1, 0), 45.0);

    let cube_position = rvec3(0, 0, 0);
    let mut cube_screen_position: Vector2;
    let cube_size = rvec3(2, 2, 2);

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        cube_screen_position = rl.get_world_to_screen(
            rvec3(cube_position.x, cube_position.y + 2.5, cube_position.z),
            camera,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(camera);

            d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
            d.draw_grid(10, 1.0);
        }

        d.draw_text(
            "Enemy: 100 / 100",
            cube_screen_position.x as i32 - measure_text("Enemy: 100/100", 20) / 2,
            cube_screen_position.y as i32,
            20,
            Color::BLACK,
        );
        d.draw_text(
            "Text is always on top of the cube",
            (ww - measure_text("Text is always on top of the cube", 20)) / 2,
            25,
            20,
            Color::GRAY,
        );
    }
}
