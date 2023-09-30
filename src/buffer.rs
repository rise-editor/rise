pub mod actions;
pub mod command;
pub mod maps;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod paint;
pub mod select;
pub mod text;

use std::collections::HashMap;

use crate::core::{editable_text::EditableText, Point, Size};
use crate::{
    buffer::{
        maps::{
            get_default_command_maps, get_default_insert_maps, get_default_normal_maps,
            get_default_visual_maps,
        },
        mode::BufferMode,
        select::Select,
    },
    editor::Editor,
};

pub type ActionMap = HashMap<&'static str, fn(&mut Editor)>;

pub struct Buffer {
    pub file_name: Option<String>,
    pub mode: BufferMode,
    pub area: Size<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
    pub select: Select,
    pub command: EditableText,
    pub actions_command: ActionMap,
    pub actions_insert: ActionMap,
    pub actions_normal: ActionMap,
    pub actions_visual: ActionMap,
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
            command: EditableText {
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
