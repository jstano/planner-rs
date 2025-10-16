#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Environment {
    id: i32,
}

impl Environment {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}
