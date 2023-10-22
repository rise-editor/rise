pub type Color = (u8, u8, u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}
