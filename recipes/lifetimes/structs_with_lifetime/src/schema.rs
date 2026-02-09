#[derive(Debug)]
#[allow(unused)]
pub struct MyStruct<'a, 'b> {
    pub field_1: &'a str,
    pub field_2: &'b [u8],
}

impl<'a, 'b> MyStruct<'a, 'b> {
    pub fn new(field_1: &'a str, field_2: &'b [u8]) -> Self {
        Self {
            field_1: field_1,
            field_2: field_2,
        }
    }
}
