pub mod write_file;

use crate::buffer::Buffer;

trait Command {
    fn run(buffer: &Buffer);
}
