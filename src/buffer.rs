pub mod command;
pub mod find;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod paint;
pub mod select;
pub mod text;

use crate::buffer::{mode::BufferMode, select::Select};
use crate::core::{editable::Editable, Point, Size};

pub struct Buffer {
    pub file_name: Option<String>,
    pub mode: BufferMode,
    pub visible_area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
    pub select: Select,
    pub command: Editable,
}

impl Buffer {
    pub fn new(area: Size<u16>) -> Buffer {
        Buffer {
            file_name: None,
            mode: BufferMode::Normal,
            visible_area: area,
            scroll: Point { x: 0, y: 0 },
            cursor: Point { x: 0, y: 0 },
            lines: vec![String::new()],
            select: Select {
                start: Point { x: 0, y: 0 },
            },
            command: Editable {
                text: String::new(),
                cursor_x: 0,
            },
        }
    }
}
