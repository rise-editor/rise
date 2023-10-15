use crate::{buffer::Buffer, core::Rectangle};

pub struct Tab {
    pub area: Rectangle<u16>,
    pub buffers: Vec<Buffer>,
    pub active_buffer: usize,
}

impl Tab {
    pub fn new(area: Rectangle<u16>) -> Self {
        Self {
            area,
            buffers: vec![],
            active_buffer: 0,
        }
    }

    pub fn create_new_buffer(&mut self) {
        let buffer = Buffer::new(self.area.clone());

        self.buffers.push(buffer);
    }

    pub fn get_active_buffer(&self) -> &Buffer {
        self.buffers.get(self.active_buffer).unwrap()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(self.active_buffer).unwrap()
    }

    pub fn set_size(&mut self, area: Rectangle<u16>) {
        self.area = area;

        for buffer in self.buffers.iter_mut() {
            buffer.set_size(self.area.clone());
        }
    }

    pub fn get_active_buffer_visible_x(&self, column: usize) -> u16 {
        self.area.x + self.get_active_buffer().column_to_visible_x(column)
    }

    pub fn get_active_buffer_visible_y(&self, row: usize) -> u16 {
        self.area.y + self.get_active_buffer().row_to_visible_y(row)
    }
}
