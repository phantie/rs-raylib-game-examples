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
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut camera = Camera3D::perspective(rvec3(10, 10, 10), rvec3(0, 0, 0), rvec3(0, 1, 0), 45.0);

    let cube_position = rvec3(0, 1, 1);
    let cube_size = rvec3(2, 2, 2);

    let mut ray = Ray::default();

    let mut collision = false;

    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            if !collision {
                ray = rl.get_mouse_ray(rl.get_mouse_position(), camera);
                collision = BoundingBox::new(
                    rvec3(
                        cube_position.x - cube_size.x / 2.0,
                        cube_position.y - cube_size.y / 2.0,
                        cube_position.z - cube_size.z / 2.0,
                    ),
                    rvec3(
                        cube_position.x + cube_size.x / 2.0,
                        cube_position.y + cube_size.y / 2.0,
                        cube_position.z + cube_size.z / 2.0,
                    ),
                )
                .check_collision_ray_box(ray);
            } else {
                collision = false;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d = d.begin_mode3D(camera);
            if collision {
                d.draw_cube(
                    cube_position,
                    cube_size.x,
                    cube_size.y,
                    cube_size.z,
                    Color::RED,
                );

                d.draw_cube_wires(
                    cube_position,
                    cube_size.x,
                    cube_size.y,
                    cube_size.z,
                    Color::MAROON,
                );

                d.draw_cube_wires(
                    cube_position,
                    cube_size.x + 0.2,
                    cube_size.y + 0.2,
                    cube_size.z + 0.2,
                    Color::GREEN,
                );
            } else {
                d.draw_cube(
                    cube_position,
                    cube_size.x,
                    cube_size.y,
                    cube_size.z,
                    Color::GRAY,
                );

                d.draw_cube_wires(
                    cube_position,
                    cube_size.x,
                    cube_size.y,
                    cube_size.z,
                    Color::DARKGRAY,
                );
            }

            d.draw_ray(ray, Color::MAROON);
            d.draw_grid(10, 1.0);
        }

        d.draw_text(
            "Try selecting the box with mouse!",
            240,
            10,
            20,
            Color::DARKGRAY,
        );

        if collision {
            d.draw_text(
                "BOX SELECTED",
                (ww - measure_text("BOX SELECTED", 30)) / 2,
                (wh as f32 * 0.1) as i32,
                30,
                Color::GREEN,
            );
        }

        d.draw_fps(10, 10);
    }
}
