use crate::{
    buffer::{mode::BufferMode, Buffer},
    core::{key::Key, Size},
    tab::Tab,
};

pub struct Editor {
    pub size: Size<u16>,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

impl Editor {
    pub fn new(size: Size<u16>) -> Self {
        Self {
            size,
            tabs: vec![],
            active_tab: 0,
        }
    }

    pub fn create_new_tab(&mut self) -> &mut Tab {
        let tab = Tab::new(Size {
            width: self.size.width,
            height: self.size.height - 2,
        });

        self.tabs.push(tab);

        self.tabs.last_mut().unwrap()
    }

    pub fn get_active_tab(&self) -> &Tab {
        self.tabs.get(self.active_tab).unwrap()
    }

    pub fn get_active_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(self.active_tab).unwrap()
    }

    pub fn get_active_buffer(&self) -> &Buffer {
        self.get_active_tab().get_active_buffer()
    }

    pub fn get_active_buffer_mut(&mut self) -> &mut Buffer {
        self.get_active_tab_mut().get_active_buffer_mut()
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.size.width = width;
        self.size.height = height;

        for tab in self.tabs.iter_mut() {
            tab.set_size(width, height)
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
