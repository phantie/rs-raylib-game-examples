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

impl Random for Color {
    fn rand() -> Color {
        Self::new(
            rv(100, 250) as u8,
            rv(50, 150) as u8,
            rv(10, 100) as u8,
            254,
        )
    }
}

const rv: &dyn Fn(i32, i32) -> i32 = &get_random_value;

fn main() {
    let opt = options::Opt::from_args();
    let (ww, wh) = (opt.width, opt.height);

    let (mut rl, thread) = raylib::init()
        .size(ww, wh)
        .title("ray")
        .resizable()
        .vsync()
        .build();

    rl.set_window_min_size(320, 240);

    let game_screen_width = 640;
    let game_screen_height = 480;

    let mut target = rl
        .load_render_texture(&thread, game_screen_width as u32, game_screen_height as u32)
        .unwrap();
    target
        .texture()
        .set_texture_filter(&thread, TextureFilterMode::FILTER_BILINEAR);

    let mut colors: Vec<Color> = (0..10).map(|_| Color::rand()).collect();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let current_ww = rl.get_screen_width();
        let current_wh = rl.get_screen_height();

        let scale = (current_ww as f32 / game_screen_width as f32)
            .min(current_wh as f32 / game_screen_height as f32);

        if rl.is_key_pressed(KEY_SPACE) {
            colors = (0..10).map(|_| Color::rand()).collect();
        }

        let mouse = rl.get_mouse_position();
        let virtual_mouse = rvec2(
            (mouse.x - (current_ww as f32 - (game_screen_width as f32 * scale))),
            (mouse.y - (current_wh  as f32- (game_screen_height as f32 * scale))),
        );

        let virtual_mouse = clamp_value(
            virtual_mouse,
            Vector2::default(),
            rvec2(current_ww, current_wh),
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_texture_mode(&thread, &mut target);

            d.clear_background(Color::RAYWHITE);

            for (i, color) in colors.iter().enumerate() {
                d.draw_rectangle(
                    0,
                    i as i32 * game_screen_height / 10,
                    game_screen_width,
                    game_screen_height / 10,
                    color,
                );
            }

            d.draw_text("If executed inside a window,\nyou can resize the window,\nand see the screen scaling!", 10, 25, 20, Color::WHITE);

            d.draw_text(
                format!("Default Mouse: [{} , {}]", mouse.x, mouse.y).as_str(),
                350,
                25,
                20,
                Color::GREEN,
            );

            d.draw_text(
                format!(
                    "Virtual  Mouse: [{} , {}]",
                    virtual_mouse.x, virtual_mouse.y
                )
                .as_str(),
                350,
                55,
                20,
                Color::YELLOW,
            );
        }

        d.draw_texture_pro(
            target.texture(),
            Rectangle::new(
                0.0,
                0.0,
                target.texture().width as f32,
                -target.texture().height as f32,
            ),
            Rectangle::new(
                (current_ww as f32 - (game_screen_width as f32 * scale)) * 0.5,
                (current_wh as f32 - (game_screen_height as f32 * scale)) * 0.5,
                (current_ww as f32 * scale),
                (current_wh as f32 * scale),
            ),
            Vector2::default(),
            f32::default(),
            Color::WHITE,
        );
    }
}

fn clamp_value(v: Vector2, min: Vector2, max: Vector2) -> Vector2 {
    let x = if v.x < max.x { max.x } else { v.x };
    let y = if v.y < max.y { max.y } else { v.y };

    Vector2 {
        x: if x < min.x { min.x } else { x },
        y: if y < min.y { min.y } else { y },
    }
}
