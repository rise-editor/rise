use crate::{buffer::Buffer, core::key::Key};

use super::mode::BufferMode;

impl Buffer {
    pub fn handle_key(&mut self, key: Key) {
        match self.mode {
            BufferMode::Normal => {
                match self.actions_normal.get(&key.to_string().as_str()) {
                    Some(action) => action(self),
                    None => {}
                }
            }
            BufferMode::Visual => {
                match self.actions_visual.get(&key.to_string().as_str()) {
                    Some(action) => action(self),
                    None => {}
                }
            }
            BufferMode::Insert => {
                match self.actions_insert.get(&key.to_string().as_str()) {
                    Some(action) => action(self),
                    None => {
                        if !key.ctrl && !key.win && !key.alt && key.code.len() == 1 {
                            let ch = key.code.chars().nth(0).unwrap();
                            self.insert_char(ch);
                        }
                    }
                }
            }
            BufferMode::Command => {
                match self.actions_command.get(&key.to_string().as_str()) {
                    Some(action) => action(self),
                    None => {
                        if !key.ctrl && !key.win && !key.alt && key.code.len() == 1 {
                            let ch = key.code.chars().nth(0).unwrap();
                            self.command.insert_key(ch);
                        }
                    }
                }
            }
        }
    }
}
