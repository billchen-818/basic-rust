# 泛型

rust可以为多种元素声明为泛型类型，包括结构体、枚举、函数、特征、方法以及代码实现块。泛型的参数是由一对尖括号分隔，并包含于其中。

## 泛型函数

泛型函数的泛型参数在函数名之后，函数参数括号之前：

```rust
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn main() {
    let result = add(2, 2); // i32
    println!("result: {}", result);
    let result = add(2.0, 2.0); // f64
    println!("result: {}", result);
}
```

上面实现一个add泛型函数，其中泛型参数T实现了`std::ops::Add`特征。通过泛型函数，可以实现i32类型和f64类型两个数据的相加，避免了多次定义add函数。

## 泛型结构体

可以定义泛型元组结构体和普通结构体，在结构体名称后面用尖括号添加泛型参数。

```rust
struct GenericStruct<T>(T);

struct Container<T> {
    item: T
}
```

## 泛型枚举

```rust
enum Transmission<T> {
    Signal(T),
    NoSignal
}
```

上面定义的枚举Transmission包含一个名为Signal的变体（包含一个泛型值），还有一个名为NoSignal的变体（是一个无值变体）。

## 泛型实现

可以为泛型编写impl代码块。当为任何泛型实现impl代码块时，都需要在使用它之前声明泛型参数。T就像一个变量——一个类型变量，我们需要先声明它。

```rust
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

impl Container<u32> {
    fn set(item: u32) -> Self {
        Container { item }
    }
}
```

可以指定T的具体类型，来为`Container<T>`实现impl代码块，比如上面的`Container<u32>`。