mod iterator;
use iterator::Fib;

fn main() {
    let fib = Fib::new(10);

    // Keep only the even numbers from the first 10 fibonacci numbers.
    let v: Vec<usize> = fib.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", v);
}
