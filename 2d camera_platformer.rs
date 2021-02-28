#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate raylib;
use raylib::consts::KeyboardKey;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use structopt::StructOpt;

mod options;

const G: f32 = 400.0;
const PLAYER_JUMP_SPEED: f32 = 350.0;
const PLAYER_HOR_SPEED: f32 = 350.0;

trait Drawable {
    fn draw(&self, d: &mut impl RaylibDraw);
}

struct Player {
    position: Vector2,
    speed: f32,
    can_jump: bool,
}

impl Player {
    fn update(&mut self, rl: &mut RaylibHandle, env_items: &EnvItems, dt: f32) {
        if rl.is_key_down(KEY_LEFT) {
            self.position.x -= PLAYER_HOR_SPEED * dt;
        }
        if rl.is_key_down(KEY_RIGHT) {
            self.position.x += PLAYER_HOR_SPEED * dt;
        }
        if rl.is_key_down(KEY_SPACE) && self.can_jump {
            self.speed = -PLAYER_JUMP_SPEED;
            self.can_jump = false;
        }

        let mut hit_obstacle = false;

        let mut rect: &Rectangle;
        let mut pos: &Vector2;

        for ei in env_items.iter() {
            rect = &ei.rect;
            pos = &self.position;

            if ei.blocking
                && rect.x <= pos.x
                && rect.x + ei.rect.width >= pos.x
                && rect.y >= pos.y
                && rect.y < pos.y + self.speed * dt
            {
                hit_obstacle = true;
                self.speed = 0.0;
                self.position.y = rect.y;
            }
        }

        if hit_obstacle {
            self.can_jump = true
        } else {
            self.position.y += self.speed * dt;
            self.speed += G * dt;
            self.can_jump = false;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_rectangle_rec(
            Rectangle::new(self.position.x - 20.0, self.position.y - 40.0, 40.0, 40.0),
            Color::RED,
        );
    }
}

struct EnvItem {
    rect: Rectangle,
    blocking: bool,
    color: Color,
}

impl EnvItem {
    fn new(rect: Rectangle, blocking: bool, color: Color) -> Self {
        Self {
            rect,
            blocking,
            color,
        }
    }
}

impl Drawable for EnvItem {
    fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_rectangle_rec(self.rect, self.color)
    }
}

type EnvItems = Vec<EnvItem>;

fn Vector2Subtract(v1: &Vector2, v2: &Vector2) -> Vector2 {
    Vector2::new(v1.x - v2.x, v1.y - v2.y)
}

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Key shower");
    let (ww, wh) = (opt.width, opt.height);

    let mut player = Player {
        position: Vector2 { x: 400.0, y: 280.0 },
        speed: f32::default(),
        can_jump: false,
    };

    let mut env_items: EnvItems = vec![
        EnvItem::new(
            Rectangle::new(0.0, 0.0, 1000.0, 400.0),
            false,
            Color::LIGHTGRAY,
        ),
        EnvItem::new(Rectangle::new(0.0, 400.0, 1000.0, 200.0), true, Color::GRAY),
        EnvItem::new(Rectangle::new(300.0, 200.0, 400.0, 10.0), true, Color::GRAY),
        EnvItem::new(Rectangle::new(250.0, 300.0, 100.0, 10.0), true, Color::GRAY),
        EnvItem::new(Rectangle::new(650.0, 300.0, 100.0, 10.0), true, Color::GRAY),
    ];

    let mut camera = Camera2D {
        target: player.position,
        offset: Vector2::new(ww as f32 / 2.0, wh as f32 / 2.0),
        rotation: f32::default(),
        zoom: 1.0,
    };

    let camera_updaters: Vec<
        &dyn Fn(&mut RaylibHandle, &mut Camera2D, &mut Player, &EnvItems, f32, i32, i32),
    > = vec![
        &update_camera_center,
        &update_camera_center_inside_map,
        &update_camera_center_smooth_follow,
        &update_camera_even_out_on_landing,
        &update_camera_player_bounds_push,
    ];

    let camera_descriptions = vec![
        "Follow player center",
        "Follow player center, but clamp to map edges",
        "Follow player center; smoothed",
        "Follow player center horizontally; updateplayer center vertically after landing",
        "Player push camera on getting too close to screen edge",
    ];

    let mut camera_option = 0;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        player.update(&mut rl, &env_items, dt);

        camera.zoom += rl.get_mouse_wheel_move() * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0
        } else if camera.zoom < 0.25 {
            camera.zoom = 0.25
        }

        if rl.is_key_pressed(KEY_R) {
            camera.zoom = 1.0;
            player.position = Vector2::new(400.0, 280.0);
        }

        if rl.is_key_pressed(KEY_C) {
            camera_option = (camera_option + 1) % camera_updaters.len();
        }

        camera_updaters[camera_option](&mut rl, &mut camera, &mut player, &env_items, dt, ww, wh);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        {
            let mut d = d.begin_mode2D(camera);
            env_items.iter().for_each(|ei| ei.draw(&mut d));
            player.draw(&mut d);
        }

        d.draw_text("Controls:", 20, 20, 10, Color::BLACK);
        d.draw_text("- Right/Left to move", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Space to jump", 40, 60, 10, Color::DARKGRAY);
        d.draw_text(
            "- Mouse Wheel to Zoom in-out, R to reset zoom",
            40,
            80,
            10,
            Color::DARKGRAY,
        );
        d.draw_text("- C to change camera mode", 40, 100, 10, Color::DARKGRAY);
        d.draw_text("Current camera mode:", 20, 120, 10, Color::BLACK);
        d.draw_text(
            camera_descriptions[camera_option],
            40,
            140,
            10,
            Color::DARKGRAY,
        );
    }
}

fn update_camera_center(
    rl: &mut RaylibHandle,
    camera: &mut Camera2D,
    player: &mut Player,
    env_items: &EnvItems,
    dt: f32,
    width: i32,
    height: i32,
) {
    camera.offset = Vector2::new(width as f32 / 2.0, height as f32 / 2.0);
    camera.target = player.position;
}

fn fminf(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn fmaxf(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn update_camera_center_inside_map(
    rl: &mut RaylibHandle,
    camera: &mut Camera2D,
    player: &mut Player,
    env_items: &EnvItems,
    dt: f32,
    width: i32,
    height: i32,
) {
    let width = width as f32;
    let height = height as f32;

    camera.target = player.position;
    camera.offset = Vector2::new(width / 2.0, height as f32 / 2.0);
    let (mut minX, mut minY, mut maxX, mut maxY) = (1000.0, 1000.0, -1000.0, -1000.0);
    let mut r: &Rectangle;

    for ei in env_items.iter() {
        r = &ei.rect;

        minX = fminf(r.x, minX);
        maxX = fmaxf(r.x + r.width, maxX);
        minY = fminf(r.y, minY);
        maxY = fmaxf(r.y + r.height, maxY);
    }

    let max = rl.get_world_to_screen2D(Vector2::new(maxX, maxY), *camera);

    let min = rl.get_world_to_screen2D(Vector2::new(minX, minY), *camera);

    if max.x < width {
        camera.offset.x = width * (3.0 / 2.0) - max.x;
    }
    if max.y < height {
        camera.offset.y = height * (3.0 / 2.0) - max.y;
    }
    if min.x > 0.0 {
        camera.offset.x = width / 2.0 - min.x
    }
    if min.y > 0.0 {
        camera.offset.y = height / 2.0 - min.y
    }
}

fn update_camera_center_smooth_follow(
    rl: &mut RaylibHandle,
    camera: &mut Camera2D,
    player: &mut Player,
    env_items: &EnvItems,
    dt: f32,
    width: i32,
    height: i32,
) {
    let width = width as f32;
    let height = height as f32;

    let min_speed = 30.0;
    let min_effect_len = 10.0;
    let fraction_speed = 0.8;

    camera.offset = Vector2::new(width / 2.0, height / 2.0);
    let diff = Vector2Subtract(&player.position, &camera.target);
    let len = Vector2Length(&diff);

    if len > min_effect_len {
        let speed = fmaxf(fraction_speed * len, min_speed);
        camera.target = Vector2Add(&camera.target, &Vector2Scale(&diff, speed * dt / len))
    }
}

fn Vector2Length(v: &Vector2) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}

fn Vector2Add(v1: &Vector2, v2: &Vector2) -> Vector2 {
    Vector2::new(v1.x + v2.x, v1.y * v2.y)
}

fn Vector2Scale(v: &Vector2, scale: f32) -> Vector2 {
    Vector2::new(v.x * scale, v.y * scale)
}

fn update_camera_even_out_on_landing(
    rl: &mut RaylibHandle,
    camera: &mut Camera2D,
    player: &mut Player,
    env_items: &EnvItems,
    dt: f32,
    width: i32,
    height: i32,
) {
    let width = width as f32;
    let height = height as f32;

    let even_out_speed = 700.0;
    let mut evening_out = false;
    let mut even_out_target = f32::default();

    camera.offset = Vector2::new(width / 2.0, height / 2.0);
    camera.target.x = player.position.x;

    if evening_out {
        if even_out_target > camera.target.y {
            if camera.target.y > even_out_target {
                camera.target.y = even_out_target;
                evening_out = false;
            }
        } else {
            camera.target.y -= even_out_speed * dt;

            if camera.target.y < even_out_target {
                camera.target.y = even_out_target;
                evening_out = false;
            }
        }
    } else {
        if player.can_jump && player.speed == 0.0 && player.position.y != camera.target.y {
            evening_out = true;
            even_out_target = player.position.y;
        }
    }
}

fn update_camera_player_bounds_push(
    rl: &mut RaylibHandle,
    camera: &mut Camera2D,
    player: &mut Player,
    env_items: &EnvItems,
    dt: f32,
    width: i32,
    height: i32,
) {
    let width = width as f32;
    let height = height as f32;

    let bbox = Vector2::new(0.2, 0.2);

    let bbox_world_min = rl.get_screen_to_world2D(
        Vector2::new((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height),
        *camera,
    );
    let bbox_world_max = rl.get_screen_to_world2D(
        Vector2::new((1.0 + bbox.x) * 0.5 * width, (1.0 + bbox.y) * 0.5 * height),
        *camera,
    );

    camera.offset = Vector2::new((1.0 - bbox.x) * 0.5 * width, (1.0 - bbox.y) * 0.5 * height);

    if player.position.x < bbox_world_min.x {
        camera.target.x = player.position.x
    }
    if player.position.y < bbox_world_min.y {
        camera.target.y = player.position.y
    }
    if player.position.x > bbox_world_max.x {
        camera.target.x = bbox_world_min.x + (player.position.x - bbox_world_max.x)
    }
    if player.position.y > bbox_world_max.y {
        camera.target.y = bbox_world_min.y + (player.position.y - bbox_world_max.y)
    }
}
