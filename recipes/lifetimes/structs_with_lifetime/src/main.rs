mod schema;
use schema::MyStruct;

/// The main problem with having borrowed data in a struct is that
/// the lifetime of the struct is bound to the lifetime of the field(s).
fn main() {
    let some_str = "some_str".to_string();

    {
        let some_vec = vec![1, 2, 3];

        let my_struct = MyStruct::new(some_str.as_str(), &some_vec[..]);
        println!("{:?}", &my_struct);
    }

    // println!("{:?}", &my_struct); // won't work
}
