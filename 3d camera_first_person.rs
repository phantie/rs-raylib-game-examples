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

const MAX_COLUMNS: i32 = 20;

struct Column {
    height: f32,
    position: Vector3,
    color: Color,
}

type Columns = Vec<Column>;

const rv: &dyn Fn(i32, i32) -> i32 = &get_random_value;

impl Column {
    fn generate() -> Self {
        let height = rv(1, 12) as f32;
        Self {
            height: height,
            position: Vector3 {
                x: rv(-15, 15) as f32,
                y: height / 2.0,
                z: rv(-15, 15) as f32,
            },
            color: Color::new(rv(20, 255) as u8, rv(10, 55) as u8, 30, 255),
        }
    }
}

impl Drawable for Column {
    fn draw(&self, d: &mut impl RaylibDraw3D) {
        d.draw_cube(self.position, 2.0, self.height, 2.0, self.color);
        d.draw_cube_wires(self.position, 2.0, self.height, 2.0, Color::MAROON);
    }
}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(rvec3(4, 2, 4), rvec3(0, 1.8, 0), rvec3(0, 1, 0), 60.0);

    let columns: Columns = (0..MAX_COLUMNS).map(|_| Column::generate()).collect();

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(camera);

            d.draw_plane(rvec3(0, 0, 0), rvec2(32, 32), Color::LIGHTGRAY);

            d.draw_cube(rvec3(-16, 2.5, 0), 1.0, 5.0, 32.0, Color::BLUE);
            d.draw_cube(rvec3(16, 2.5, 0), 1.0, 5.0, 32.0, Color::LIME);
            d.draw_cube(rvec3(0, 2.5, 16), 32.0, 5.0, 1.0, Color::GOLD);

            columns.iter().for_each(|c| c.draw(&mut d));
        }

        d.draw_rectangle(10, 10, 220, 70, Color::SKYBLUE.fade(0.5));
        d.draw_rectangle_lines(10, 10, 220, 70, Color::BLUE);

        d.draw_text(
            "First person camera default controls:",
            20,
            20,
            10,
            Color::BLACK,
        );
        d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);

        d.draw_fps(ww - 90, 10);
    }
}
