pub mod find;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod paint_helper;
pub mod text;

use crate::core::{Point, Size};

use self::mode::BufferMode;

pub struct Buffer {
    pub mode: BufferMode,
    pub visible_area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
}
