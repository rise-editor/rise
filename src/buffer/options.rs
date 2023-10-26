use std::collections::HashMap;

pub struct BufferOptions {
    pub show_info_column: bool,
    pub show_border: bool,
    pub chars: HashMap<char, char>,
}

impl Default for BufferOptions {
    fn default() -> Self {
        let mut options = Self {
            show_info_column: true,
            show_border: false,
            chars: HashMap::new(),
        };
        options.chars.insert(' ', '•');
        options
    }
}
