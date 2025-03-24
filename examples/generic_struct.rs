// 泛型结构体

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

fn main() {
    let c = Container::new(42);
    println!("c.item = {}", c.item);
    let c = Container::new(3.14);
    println!("c.item = {}", c.item);
}
