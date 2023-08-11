use crate::{
    snake_snake::snake::{Direction, Snake},
    snake_window::draw::{draw_block, draw_rectangle},
};
use piston_window::{rectangle::Shape, types::Color, Context, G2d, Key};
use rand::{thread_rng, Rng};

/// 食物颜色
const FOOD_COLOR: Color = [255.0, 0.0, 255.0, 1.0];
/// 上边框颜色
const T_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 下边框颜色
const B_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 左边框颜色
const L_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 右边框颜色
const R_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
///游戏结束颜色
const GAME_OVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
/// 移动周期，每过多长时间进行一次移动
const MOVING_PERIOD: f64 = 0.3;

// 游戏主体
#[derive(Debug)]
pub struct Game {
    // 蛇
    snake: Snake,
    // 是否存在食物
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,

    // 暂停
    game_pause: bool,
    // 结束
    game_over: bool,
    // 等待时间
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),

            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            waiting_time: 0.0,
            game_pause: false,
            game_over: false,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        if self.food_exists {
            draw_block(
                FOOD_COLOR,
                Shape::Round(8.0, 16),
                self.food_x,
                self.food_y,
                con,
                g,
            )
        }

        //边框
        draw_rectangle(T_BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(B_BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(L_BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(R_BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // 游戏失败， 绘制失败画面
        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // 更新游戏数据
    pub fn update(&mut self, delta_time: f64) {
        // 暂停， 结束
        if self.game_pause || self.game_over {
            return;
        }

        // 增加游戏等待时间
        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food()
        }

        // 间隔一段时间后更新蛇的位置
        println!("waiting_time, {}, {}", self.waiting_time, MOVING_PERIOD);
        if self.waiting_time > MOVING_PERIOD {
            println!("{}", "update");
            self.update_snake(None);
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.over_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.game_pause {
            return;
        }
        if self.check_if_snake_alive(dir) {
            println!("update snake: keep alive");
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            println!("update snake: keep alive false");
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // 碰到自身了， 生命结束
        if self.snake.over_tail(next_x, next_y) {
            println!("update snake: over tail true");
            return false;
        }
        println!(
            "update snake: over tail false, next: {}, {}",
            next_x, next_y
        );

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    pub fn key_pressed(&mut self, key: Key) {
        // 输入R快速重新开始
        if key == Key::R {
            self.restart();
        }

        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::P => {
                self.game_pause = !self.game_pause;
                None
            }
            _ => None,
        };

        if let Some(d) = dir {
            // 如果方向相反，不处理
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        // 方向有效，刷新方向
        self.update_snake(dir);
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.game_pause = false;
    }
}
