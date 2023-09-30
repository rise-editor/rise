use crate::{
    buffer::{mode::BufferMode, Buffer},
    core::{key::Key, Size},
    window::Window,
};

pub struct Editor {
    pub size: Size<u16>,
    pub windows: Vec<Window>,
    pub active_window: usize,
}

impl Editor {
    pub fn new(size: Size<u16>) -> Self {
        Self {
            size,
            windows: vec![],
            active_window: 0,
        }
    }

    pub fn create_new_window(&mut self) -> &mut Window {
        let window = Window::new(Size {
            width: self.size.width,
            height: self.size.height - 2,
        });

        self.windows.push(window);

        self.windows.last_mut().unwrap()
    }

    pub fn get_active_window(&self) -> &Window {
        self.windows.get(self.active_window).unwrap()
    }

    pub fn get_active_window_mut(&mut self) -> &mut Window {
        self.windows.get_mut(self.active_window).unwrap()
    }

    pub fn get_active_buffer(&self) -> &Buffer {
        self.get_active_window().get_active_buffer()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.get_active_window_mut().get_active_buffer_mut()
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.size.width = width;
        self.size.height = height;

        for window in self.windows.iter_mut() {
            window.set_size(width, height)
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        let buffer = self.get_active_buffer_mut();

        match buffer.mode {
            BufferMode::Normal => match buffer.actions_normal.get(&key.to_string().as_str()) {
                Some(action) => action(self),
                None => {}
            },
            BufferMode::Visual => match buffer.actions_visual.get(&key.to_string().as_str()) {
                Some(action) => action(self),
                None => {}
            },
            BufferMode::Insert => match buffer.actions_insert.get(&key.to_string().as_str()) {
                Some(action) => action(self),
                None => {
                    if !key.ctrl && !key.win && !key.alt && key.code.len() == 1 {
                        let ch = key.code.chars().nth(0).unwrap();
                        self.get_active_buffer_mut().insert_char(ch);
                    }
                }
            },
            BufferMode::Command => match buffer.actions_command.get(&key.to_string().as_str()) {
                Some(action) => action(self),
                None => {
                    if !key.ctrl && !key.win && !key.alt && key.code.len() == 1 {
                        let ch = key.code.chars().nth(0).unwrap();
                        buffer.command.insert_char(ch);
                    }
                }
            },
        }
    }
}
