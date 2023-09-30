use crate::{core::Size, window::Window};

pub struct Editor {
    pub size: Size<u16>,
    pub windows: Vec<Window>,
    pub active_window: usize,
}

impl Editor {
    pub fn new(size: Size<u16>) -> Self {
        Self {
            size,
            windows: vec![],
            active_window: 0,
        }
    }

    pub fn create_new_window(&mut self) -> &mut Window {
        let window = Window::new(Size {
            width: self.size.width,
            height: self.size.height - 2,
        });

        self.windows.push(window);

        self.windows.last_mut().unwrap()
    }

    pub fn get_active_window(&self) -> &Window {
        self.windows.get(self.active_window).unwrap()
    }

    pub fn get_active_window_mut(&mut self) -> &mut Window {
        self.windows.get_mut(self.active_window).unwrap()
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.size.width = width;
        self.size.height = height;

        for window in self.windows.iter_mut() {
            window.set_size(width, height)
        }
    }
}
