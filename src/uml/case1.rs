struct Light {
    is_on: bool,
}

impl Light {
    fn new() -> Self {
        Self { is_on: false }
    }
    fn on(&mut self) {
        self.is_on = true;
    }
    fn off(&mut self) {
        self.is_on = false
    }
    fn status(&self) -> bool {
        self.is_on
    }
}
