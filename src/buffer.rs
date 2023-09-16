pub mod command;
pub mod find;
pub mod handle_key;
pub mod maps;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod paint;
pub mod select;
pub mod text;

use std::collections::HashMap;

use crate::buffer::{
    maps::{
        get_default_command_maps,
        get_default_insert_maps,
        get_default_normal_maps,
        get_default_visual_maps,
    },
    mode::BufferMode,
    select::Select,
};
use crate::core::{editable::Editable, Point, Size};

pub struct Buffer {
    pub file_name: Option<String>,
    pub mode: BufferMode,
    pub area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
    pub select: Select,
    pub command: Editable,
    pub actions_command: HashMap<&'static str, fn(&mut Buffer)>,
    pub actions_insert: HashMap<&'static str, fn(&mut Buffer)>,
    pub actions_normal: HashMap<&'static str, fn(&mut Buffer)>,
    pub actions_visual: HashMap<&'static str, fn(&mut Buffer)>,
}

impl Buffer {
    pub fn new(area: Size<u16>) -> Buffer {
        Buffer {
            file_name: None,
            mode: BufferMode::Normal,
            area,
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
            actions_command: get_default_command_maps(),
            actions_insert: get_default_insert_maps(),
            actions_normal: get_default_normal_maps(),
            actions_visual: get_default_visual_maps(),
        }
    }
}
