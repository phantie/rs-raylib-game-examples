#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
// use raylib::consts::GestureType::*;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const MAX_BUILDINGS: usize = 100;

trait Drawable {
    fn draw(&self, d: &mut impl RaylibDraw);
}

#[derive(Debug)]
struct Building {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    color: Color,
}

impl Drawable for Building {
    fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_rectangle(
            self.x,
            self.y,
            self.width as i32,
            self.height as i32,
            self.color,
        );
    }
}

struct Player {
    shape: Rectangle,
    color: Color,
}


impl Drawable for Player {
    fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_rectangle_rec(self.shape, self.color)
    }
}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut player = Player {
        shape: Rectangle::new(400.0, 280.0, 40.0, 40.0),
        color: Color::RED,
    };

    let mut buildings: Vec<Building> = vec![];

    let mut spacing = 0;

    for _ in 0..MAX_BUILDINGS {
        let height = get_random_value::<i32>(50, 200) as u32;

        let building = Building {
            width: get_random_value::<i32>(50, 200) as u32,
            height: height,
            x: -6000 + spacing,
            y: wh - 130 - height as i32,
            color: Color {
                r: get_random_value::<i32>(200, 240) as u8,
                g: get_random_value::<i32>(200, 240) as u8,
                b: get_random_value::<i32>(200, 250) as u8,
                a: 255,
            },
        };

        spacing += building.width as i32;
        buildings.push(building);
    }

    let mut camera = Camera2D {
        target: Vector2 {
            x: player.shape.x + 20.0,
            y: player.shape.y + 20.0,
        },
        offset: Vector2 {
            x: ww as f32 / 2.0,
            y: wh as f32 / 2.0,
        },
        rotation: f32::default(),
        zoom: 1.0,
    };

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_down(KEY_RIGHT) {
            player.shape.x += 2.0
        } else if rl.is_key_down(KEY_LEFT) {
            player.shape.x -= 2.0
        }

        camera.target = Vector2 {
            x: player.shape.x + 20.0,
            y: player.shape.y + 20.0,
        };

        if rl.is_key_down(KEY_A) {
            camera.rotation -= 1.0
        } else if rl.is_key_down(KEY_D) {
            camera.rotation += 1.0
        }

        if camera.rotation > 40.0 {
            camera.rotation = 40.0
        } else if camera.rotation < -40.0 {
            camera.rotation = -40.0
        }

        camera.zoom += rl.get_mouse_wheel_move() * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0
        } else if camera.zoom < 0.1 {
            camera.zoom = 0.1
        }

        if rl.is_key_pressed(KEY_R) {
            camera.zoom = 1.0;
            camera.rotation = f32::default();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_mode2D(camera);

            d.draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);

            buildings.iter().for_each(|building| building.draw(&mut d));

            player.draw(&mut d);

            d.draw_line(
                camera.target.x as i32,
                -wh * 10,
                camera.target.x as i32,
                wh * 10,
                Color::GREEN,
            );

            d.draw_line(
                -ww * 10,
                camera.target.y as i32,
                ww * 10,
                camera.target.y as i32,
                Color::GREEN,
            );
        }

        d.draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

        d.draw_rectangle(0, 0, ww, 5, Color::RED);
        d.draw_rectangle(0, 5, 5, wh - 10, Color::RED);
        d.draw_rectangle(ww - 5, 5, 5, wh - 10, Color::RED);
        d.draw_rectangle(0, wh - 5, ww, 5, Color::RED);

        d.draw_rectangle(10, 10, 250, 113, Color::SKYBLUE.fade(0.5));
        d.draw_rectangle_lines(10, 10, 250, 113, Color::BLUE);

        d.draw_text("Free 2d camera controls:", 20, 20, 10, Color::BLACK);
        d.draw_text("- Right/Left to move Offset", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, Color::DARKGRAY);
        d.draw_text("- A / D to Rotate", 40, 80, 10, Color::DARKGRAY);
        d.draw_text(
            "- R to reset Zoom and Rotation",
            40,
            100,
            10,
            Color::DARKGRAY,
        );
    }
}
