#[derive(Debug, Clone)]
pub struct Branches {
    pub while_counter: u32,
    pub if_counter: u32,
}

impl Branches {
    pub fn new() -> Self {
        Branches {
            while_counter: 0,
            if_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.while_counter = 0;
        self.if_counter = 0;
    }
}
