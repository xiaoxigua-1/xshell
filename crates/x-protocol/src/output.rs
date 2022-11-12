pub struct Output {
    pub string: String,
}

impl Output {
    pub fn new(string: String) -> Self {
        Output { string }
    }
}

impl Default for Output {
    fn default() -> Self {
        Output {
            string: String::new(),
        }
    }
}
