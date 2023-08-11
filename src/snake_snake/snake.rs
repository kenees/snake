use std::collections::LinkedList;

use piston_window::{rectangle::Shape, types::Color, Context, G2d};

use crate::snake_window::draw::draw_block;

/// 蛇身体的颜色
const SNAKE_BODY_COLOR: Color = [0.5, 0.0, 0.0, 1.0];
/// 蛇头的颜色
const SNAKE_HEAD_COLOR: Color = [1.0, 0.00, 0.00, 1.0];

// 方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // 方向合法性校验，不能输入相反方向
    pub fn opposite(&self) -> Direction {
        // Why
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

// 定义蛇的数据
#[derive(Debug)]
pub struct Snake {
    // 朝向
    direction: Direction,
    // 身体
    body: LinkedList<Block>,
    // 尾巴
    tail: Option<Block>,
}

impl Snake {
    // 初始化蛇
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        // 初始塞3个坐标
        body.push_back(Block { x: x + 2, y: y });
        body.push_back(Block { x: x + 1, y: y });
        body.push_back(Block { x: x, y: y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut is_head = true;
        for block in &self.body {
            if is_head {
                is_head = false;
                draw_block(
                    SNAKE_HEAD_COLOR,
                    Shape::Round(10.0, 16),
                    block.x,
                    block.y,
                    con,
                    g,
                );
            } else {
                draw_block(
                    SNAKE_BODY_COLOR,
                    Shape::Round(12.5, 16),
                    block.x,
                    block.y,
                    con,
                    g,
                );
            }
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        //
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        // 判断是否有传入方向，默认为蛇当前的方向
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // 自身碰撞检测
    pub fn over_tail(&self, x: i32, y: i32) -> bool {
        // let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            // ch += 1;
            // if ch == self.body.len() - 1 {
            //     break;
            // }
        }
        false
    }

    // 移动到下一个位置
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (x, y) = self.next_head(dir);
        self.body.push_front(Block { x, y });
        let remove_block = self.body.pop_back().unwrap();
        self.tail = Some(remove_block);
        println!("update snake: move end");
    }

    // 增加蛇的长度
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap(); // 克隆蛇尾
        self.body.push_back(blk);
    }
}
