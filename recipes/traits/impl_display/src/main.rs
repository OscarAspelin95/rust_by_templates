mod schema;
use schema::MyStruct;
use std::fmt::Display;

fn print_struct<T: Display>(s: &T) {
    println!("{}", s);
}

fn main() {
    let my_struct = MyStruct::mock();
    print_struct(&my_struct);
}
