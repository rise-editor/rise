use crate::buffer::Buffer;

impl Buffer {
    pub fn move_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;

            if self.cursor.x < self.scroll.x {
                self.scroll.x -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.x < self.get_current_line_length() {
            self.cursor.x += 1;

            if self.cursor.x >= self.scroll.x + (self.visible_area.width as usize) {
                self.scroll.x += 1;
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;

            if self.cursor.y < self.scroll.y {
                self.scroll.y -= 1;
            }
        }

        if self.get_current_line_length() < self.cursor.x {
            self.move_line_end();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.y < self.get_line_count() - 1 {
            self.cursor.y += 1;

            if self.cursor.y >= self.scroll.y + (self.visible_area.height) as usize {
                self.scroll.y += 1;
            }
        }

        if self.get_current_line_length() < self.cursor.x {
            self.move_line_end();
        }
    }

    pub fn move_line_start(&mut self) {
        self.cursor.x = 0;

        if self.scroll.x > 0 {
            self.scroll.x = 0;
        }
    }

    pub fn move_line_end(&mut self) {
        self.cursor.x = self.get_current_line_length();

        if self.cursor.x < self.scroll.x {
            self.scroll.x = self.cursor.x;
        }
    }
}
