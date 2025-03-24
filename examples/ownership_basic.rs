#[derive(Debug)]
struct Foo(u32);

fn main() {
    let foo = Foo(42);
    let bar = foo; // foo将所有权转移给了bar，foo不再有效
    //println!("{:?}", foo); // 这里使用foo将会报错
    println!("{:?}", bar);
}
