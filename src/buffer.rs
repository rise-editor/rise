pub mod clipboard;
pub mod find;
pub mod helper;
pub mod highlight;
pub mod maps;
pub mod mode;
pub mod movement;
pub mod operations;
pub mod options;
pub mod paint;
pub mod text;
pub mod visual_mode;

use std::collections::HashMap;

use crate::{
    buffer::{
        clipboard::Clipboard,
        highlight::Highlight,
        maps::{get_default_insert_maps, get_default_normal_maps, get_default_visual_maps},
        mode::BufferMode,
        options::BufferOptions,
        visual_mode::Selection,
    },
    core::{point::Point, rectangle::Rectangle, style::Style, text_position::TextPosition},
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
    pub selection: Selection,
    pub actions_insert: ActionMap,
    pub actions_normal: ActionMap,
    pub actions_visual: ActionMap,
    pub popups: Vec<Buffer>,
    pub active_popup: Option<usize>,
    pub options: BufferOptions,
    pub styles: HashMap<&'static str, Style>,
    pub highlights: Vec<Highlight>,
    pub finds: Vec<TextPosition>,
    pub clipboard: Option<Clipboard>,
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
            selection: Selection {
                start: Point { x: 0, y: 0 },
            },
            actions_insert: get_default_insert_maps(),
            actions_normal: get_default_normal_maps(),
            actions_visual: get_default_visual_maps(),
            popups: vec![],
            active_popup: None,
            options: BufferOptions::default(),
            styles: HashMap::new(),
            highlights: vec![],
            finds: vec![],
            clipboard: None,
        };
        buffer.set_static_highlights();
        buffer.set_size(area);
        buffer
    }

    pub fn set_size(&mut self, area: Rectangle<u16>) {
        self.area = area.clone();
        self.info_area = area.clone();
        self.text_area = area.clone();
        self.info_area.width = 2 + self.get_line_count().to_string().len() as u16;
        if !self.options.show_info_column {
            self.info_area.width = 0;
        }
        self.text_area.x = self.info_area.x + self.info_area.width;
        self.text_area.width = area.width - self.info_area.width;
        self.move_cursor(self.cursor.y, self.cursor.x);
    }
}
