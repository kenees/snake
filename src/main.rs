// extern crate piston_window;
// Cargo.toml 的 depencies 表让你无需再写一遍 extern crate， 版本问题，现在不需要了

use piston_window::types::Color;
use piston_window::{clear, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

mod snake_game;
mod snake_snake;
mod snake_window;

// use crate::snake_game::game::Game;
use snake_game::game::Game;
use snake_window::draw::to_coord_u32;

// 定义背景颜色
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    println!("Hello, world!");
    // 定义窗口大小
    let (width, height) = (30, 30);

    // 定义游戏窗口
    let mut window: PistonWindow =
        WindowSettings::new("snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        // 监听输入
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            // update game..
            game.update(arg.dt);
        });
    }
}
