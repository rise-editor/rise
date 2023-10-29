use std::collections::HashMap;

pub enum TabMode {
    Space,
    Tab,
}

pub struct BufferOptions {
    pub show_info_column: bool,
    pub show_border: bool,
    pub chars: HashMap<char, char>,
    pub tab_mode: TabMode,
    pub tabstop: u8,
}

impl Default for BufferOptions {
    fn default() -> Self {
        let mut options = Self {
            show_info_column: true,
            show_border: false,
            chars: HashMap::new(),
            tab_mode: TabMode::Space,
            tabstop: 4,
        };
        options.chars.insert(' ', '•');
        options
    }
}

impl BufferOptions {
    pub fn get_whitespace_chars(&self) -> String {
        let mut chars = String::new();
        match self.tab_mode {
            TabMode::Space => {
                for _ in 0..self.tabstop {
                    chars.push(' ');
                }
            }
            TabMode::Tab => chars.push('\t'),
        }
        chars
    }
}
