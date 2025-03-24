use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn main() {
    let result = add(2, 2);
    println!("result: {}", result);
    let result = add(2.0, 2.0);
    println!("result: {}", result);
}
