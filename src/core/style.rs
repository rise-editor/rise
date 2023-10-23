pub type Color = (u8, u8, u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Style {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self {
            fg,
            bg,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
