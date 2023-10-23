#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextPosition {
    pub row: usize,
    pub start: usize,
    pub end: usize,
}
