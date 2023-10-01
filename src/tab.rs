use crate::{
    buffer::Buffer,
    core::{Point, Size},
};

pub struct Tab {
    pub position: Point<u16>,
    pub size: Size<u16>,
    pub buffers: Vec<Buffer>,
    pub active_buffer: usize,
}

impl Tab {
    pub fn new(size: Size<u16>) -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            size,
            buffers: vec![],
            active_buffer: 0,
        }
    }

    pub fn create_new_buffer(&mut self) {
        let buffer = Buffer::new(Size {
            width: self.size.width,
            height: self.size.height,
        });

        self.buffers.push(buffer);
    }

    pub fn get_active_buffer(&self) -> &Buffer {
        self.buffers.get(self.active_buffer).unwrap()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(self.active_buffer).unwrap()
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.size.width = width;
        self.size.height = height;

        for buffer in self.buffers.iter_mut() {
            buffer.area.width = width;
            buffer.area.height = height;
        }
    }

    pub fn get_active_buffer_visible_x(&self, column: usize) -> u16 {
        self.position.x + self.get_active_buffer().column_to_visible_x(column)
    }

    pub fn get_active_buffer_visible_y(&self, row: usize) -> u16 {
        self.position.y + 1 + self.get_active_buffer().row_to_visible_y(row)
    }
}
