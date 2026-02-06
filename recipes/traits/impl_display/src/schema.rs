use std::fmt::{self, Display};

#[derive(Debug)]
pub struct MyStruct {
    pub field_1: String,
    pub field_2: usize,
}

impl MyStruct {
    pub fn mock() -> Self {
        Self {
            field_1: "some_string".to_string(),
            field_2: 0,
        }
    }
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.field_1, self.field_2)
    }
}
