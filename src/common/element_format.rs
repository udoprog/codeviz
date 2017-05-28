pub trait ElementFormat {
    fn push(&mut self, value: &str);

    fn concat(&mut self, value: &str);

    fn end(self) -> String;
}

impl ElementFormat for String {
    fn push(&mut self, value: &str) {
        if self.len() > 0 {
            self.push('\n');
        }

        self.push_str(value);
    }

    fn concat(&mut self, value: &str) {
        self.push_str(value);
    }

    fn end(mut self) -> String {
        if self.len() > 0 {
            self.push('\n');
        }

        self
    }
}
