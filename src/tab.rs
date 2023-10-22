use crate::{buffer::Buffer, core::rectangle::Rectangle};

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

    pub fn create_new_buffer(&mut self) -> &mut Buffer {
        let buffer = Buffer::new(self.area.clone());

        self.buffers.push(buffer);
        self.active_buffer = self.buffers.len() - 1;

        self.buffers.last_mut().unwrap()
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
}
