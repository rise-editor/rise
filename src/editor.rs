use crate::{
    buffer::{mode::BufferMode, Buffer},
    commands::{
        explorer::ExplorerCommand, find_file::FindFileCommand, read_file::ReadFileCommand,
        write_file::WriteFileCommand,
    },
    core::{editable_text::EditableText, key::Key, Rectangle},
    tab::Tab,
};

pub struct Editor {
    pub area: Rectangle<u16>,
    pub tabs_area: Rectangle<u16>,
    pub document_area: Rectangle<u16>,
    pub status_area: Rectangle<u16>,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub command: EditableText,
}

impl Editor {
    pub fn new(area: Rectangle<u16>) -> Self {
        let mut editor = Self {
            area: area.clone(),
            tabs_area: Rectangle::zero(),
            document_area: Rectangle::zero(),
            status_area: Rectangle::zero(),
            tabs: vec![],
            active_tab: 0,
            command: EditableText {
                text: String::new(),
                cursor_x: 0,
            },
        };

        editor.set_size(area);
        editor
    }

    pub fn create_new_tab(&mut self) -> &mut Tab {
        let tab = Tab::new(self.document_area.clone());

        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;

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

    pub fn get_active_buffer_or_popup(&self) -> &Buffer {
        let buffer = self.get_active_tab().get_active_buffer();

        match buffer.active_popup {
            Some(i) => buffer.popups.get(i).unwrap(),
            None => buffer,
        }
    }

    pub fn get_active_buffer_or_popup_mut(&mut self) -> &mut Buffer {
        let buffer = self.get_active_tab_mut().get_active_buffer_mut();

        match buffer.active_popup {
            Some(i) => buffer.popups.get_mut(i).unwrap(),
            None => buffer,
        }
    }

    pub fn set_size(&mut self, area: Rectangle<u16>) {
        self.area = area.clone();
        self.tabs_area = area.clone();
        self.tabs_area.height = 1;
        self.document_area = area.clone();
        self.document_area.y += self.tabs_area.height;
        self.document_area.height -= 2;
        self.status_area = area.clone();
        self.status_area.y = self.document_area.y + self.document_area.height;
        self.status_area.height = 1;

        for tab in self.tabs.iter_mut() {
            tab.set_size(self.document_area.clone());
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        let main_buffer = self.get_active_buffer_mut();
        let buffer = match main_buffer.active_popup {
            Some(active_popup_index) => main_buffer.popups.get_mut(active_popup_index).unwrap(),
            None => main_buffer,
        };

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
                        buffer.insert_char(ch);
                    }
                }
            },
            BufferMode::Command => match buffer.actions_command.get(&key.to_string().as_str()) {
                Some(action) => action(self),
                None => {
                    if !key.ctrl && !key.win && !key.alt && key.code.len() == 1 {
                        let ch = key.code.chars().nth(0).unwrap();
                        self.command.insert_char(ch);
                    }
                }
            },
        }
    }

    pub fn run_command(&mut self) {
        let command = self.command.text.trim();

        if command == "w" || command.starts_with("w ") {
            WriteFileCommand::run(self);
        } else if command == "e" {
            ExplorerCommand::run(self);
        } else if command.starts_with("e ") {
            ReadFileCommand::run(self);
        } else if command == "f" {
            FindFileCommand::run(self);
        }

        self.command.reset();
        self.get_active_buffer_mut().enter_normal_mode();
    }
}
