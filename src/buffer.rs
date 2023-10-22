pub mod highlight;
pub mod maps;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod options;
pub mod paint;
pub mod select;
pub mod text;

use std::collections::HashMap;

use crate::{
    buffer::{
        highlight::Highlight,
        maps::{
            get_default_command_maps, get_default_insert_maps, get_default_normal_maps,
            get_default_visual_maps,
        },
        mode::BufferMode,
        options::BufferOptions,
        select::Select,
    },
    core::{Point, Rectangle, Style},
    editor::Editor,
};

pub type ActionMap = HashMap<&'static str, fn(&mut Editor)>;

pub struct Buffer {
    pub file_name: Option<String>,
    pub mode: BufferMode,
    pub area: Rectangle<u16>,
    pub info_area: Rectangle<u16>,
    pub text_area: Rectangle<u16>,
    pub scroll: Point<usize>,
    pub cursor: Point<usize>,
    pub lines: Vec<String>,
    pub select: Select,
    pub actions_command: ActionMap,
    pub actions_insert: ActionMap,
    pub actions_normal: ActionMap,
    pub actions_visual: ActionMap,
    pub popups: Vec<Buffer>,
    pub active_popup: Option<usize>,
    pub options: BufferOptions,
    pub styles: HashMap<&'static str, Style>,
    pub highlights: Vec<Highlight>,
}

impl Buffer {
    pub fn new(area: Rectangle<u16>) -> Buffer {
        let mut buffer = Buffer {
            file_name: None,
            mode: BufferMode::Normal,
            area: area.clone(),
            info_area: Rectangle::zero(),
            text_area: Rectangle::zero(),
            scroll: Point { x: 0, y: 0 },
            cursor: Point { x: 0, y: 0 },
            lines: vec![String::new()],
            select: Select {
                start: Point { x: 0, y: 0 },
            },
            actions_command: get_default_command_maps(),
            actions_insert: get_default_insert_maps(),
            actions_normal: get_default_normal_maps(),
            actions_visual: get_default_visual_maps(),
            popups: vec![],
            active_popup: None,
            options: BufferOptions::default(),
            styles: HashMap::new(),
            highlights: vec![],
        };

        buffer.set_size(area);
        buffer
    }

    pub fn set_size(&mut self, area: Rectangle<u16>) {
        self.info_area = area.clone();
        self.text_area = area.clone();
        self.info_area.width = 2 + self.lines.len().to_string().len() as u16;
        if !self.options.show_info_column {
            self.info_area.width = 0;
        }
        self.text_area.x = self.info_area.x + self.info_area.width;
        self.text_area.width = area.width - self.info_area.width;
        self.move_cursor(self.cursor.y, self.cursor.x);
    }
}
