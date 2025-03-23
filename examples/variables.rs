fn main() {
    let target = "world"; // 定义一个不可变变量
    let mut greeting = "Hello"; // 可变变量
    println!("{}, {}!", greeting, target);
    greeting = "How are you doing";
    //target = "mate"; // 这边赋值会报错，因为target是不可变变量
    println!("{}, {}!", greeting, target);
}
