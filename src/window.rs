use crate::buffer::Buffer;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Window {
    pub position: Point,
    pub size: Size,
    pub buffers: Vec<Buffer>,
}

impl Window {
    pub fn get_active_buffer(&self) -> &Buffer {
        self.buffers.get(0).unwrap()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(0).unwrap()
    }

    pub fn get_active_buffer_cursor_position(&self) -> Point {
        Point {
            x: self.position.x + self.get_active_buffer().cursor.column as u16,
            y: self.position.y + self.get_active_buffer().cursor.row as u16,
        }
    }
}
