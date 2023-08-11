use piston_window::{
    rectangle, rectangle::Shape, types::Color, Context, DrawState, G2d, Rectangle,
};

// 定义块的大小
const BLOCK_SIZE: f64 = 20.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

// 块图像绘制
pub fn draw_block(color: Color, shape: Shape, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let rec = Rectangle::new(color).color(color).shape(shape);
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let rectangle = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];
    rec.draw(rectangle, &DrawState::default(), con.transform, g)
}

// 绘制长方形
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    rectangle(
        color,
        [to_coord(x), to_coord(y), to_coord(width), to_coord(height)],
        con.transform,
        g,
    )
}
