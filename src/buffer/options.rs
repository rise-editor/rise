pub struct BufferOptions {
    pub show_info_column: bool,
    pub show_border: bool,
}

impl Default for BufferOptions {
    fn default() -> Self {
        Self {
            show_info_column: true,
            show_border: false,
        }
    }
}
