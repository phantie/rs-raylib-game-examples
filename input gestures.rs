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

const MAX_GESTURE_STRINGS: usize = 20;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut touch_pos;
    let touch_area = Rectangle::new(220.0, 10.0, (ww - 230) as f32, (wh - 20) as f32);

    let mut gesture_strings: Vec<String> = vec![];

    let mut current_gesture: GestureType = GestureType::GESTURE_NONE;
    let mut last_gesture: GestureType;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        last_gesture = current_gesture;
        current_gesture = rl.get_gesture_detected();
        touch_pos = rl.get_touch_position(0);

        if touch_area.check_collision_point_rec(touch_pos)
            && current_gesture != GestureType::GESTURE_NONE
        {
            if current_gesture != last_gesture {
                gesture_strings.push(format!("{:?}", current_gesture));

                if gesture_strings.len() >= MAX_GESTURE_STRINGS {
                    gesture_strings.clear()
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_rectangle_rec(touch_area, Color::GRAY);
        d.draw_rectangle(225, 15, ww - 240, wh - 30, Color::RAYWHITE);
        d.draw_text(
            "GESTURES TEST AREA",
            ww - 270,
            wh - 40,
            20,
            Color::GRAY.fade(0.5),
        );

        for i in 0..gesture_strings.len() {
            d.draw_rectangle(
                10,
                30 + 20 * i as i32,
                200,
                20,
                Color::LIGHTGRAY.fade(if i % 2 == 0 { 0.5 } else { 0.3 }),
            );

            d.draw_text(
                gesture_strings[i].as_str(),
                35,
                36 + 20 * i as i32,
                10,
                if i < gesture_strings.len() - 1 {
                    Color::DARKGRAY
                } else {
                    Color::MAROON
                },
            )
        }

        d.draw_rectangle_lines(10, 29, 200, wh - 40, Color::GRAY);
        d.draw_text("DECTED_GESTURES", 50, 15, 10, Color::GRAY);

        if current_gesture != GestureType::GESTURE_NONE {
            d.draw_circle_v(touch_pos, 30.0, Color::MAROON)
        }
    }
}
