#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key {
    pub ctrl: bool,
    pub alt: bool,
    pub win: bool,
    pub code: String,
}

impl Key {
    pub fn to_string(&self) -> String {
        if self.ctrl || self.win || self.alt {
            let ctrl = if self.ctrl { "c-" } else { "" };
            let win = if self.win { "w-" } else { "" };
            let alt = if self.alt { "a-" } else { "" };

            return format!("<{}{}{}{}>", ctrl, win, alt, self.code);
        }

        self.code.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::key::Key;

    fn test(ctrl: bool, win: bool, alt: bool, code: &str, expected: &str) {
        let key = Key {
            ctrl,
            win,
            alt,
            code: String::from(code),
        };

        assert_eq!(key.to_string(), String::from(expected));
    }

    #[test]
    fn to_string() {
        test(true, true, true, "a", "<c-w-a-a>");
        test(true, true, true, "Z", "<c-w-a-Z>");
        test(true, true, false, "b", "<c-w-b>");
        test(true, false, true, "c", "<c-a-c>");
        test(false, false, true, "d", "<a-d>");
        test(false, true, false, "e", "<w-e>");
        test(false, false, false, "a", "a");
        test(false, false, false, "esc", "esc");
    }
}
