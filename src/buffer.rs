pub mod handle_key;
pub mod move_cursor;
pub mod text;

use crate::cursor::Cursor;

pub struct Buffer {
    pub cursor: Cursor,
    pub lines: Vec<String>,
}
