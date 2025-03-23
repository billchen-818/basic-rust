## 类型

### 基本数据类型

- bool
- char
- i8/u8/i16/u16/i32/u32/i64/u64/i128/u128
- isize/usize
- f32/f64
- ([T;N]:固定大小的数组，T表示类型，N表示长度)
- ([T]:动态大小的连续序列)
- str:字符串切片
- (T,U...):有限序列
- fn(i32) -> i32:函数类型(一个接收i32类型参数并返回i32类型参数的函数)

以上是rust里面的基本数据类型。

### 变量

rust中定义变量通过`let`关键字，默认定义的变量是不可变变量，对它进行二次赋值会报错。

```rust
let name = "rust";
```

添加`mut`关键字可以让变量具有可变属性：

```rust
let mut name = "golang";
name = "rust";
```

### 函数

通过关键字`fn`定来定义函数:

```rust
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

### 闭包

rust支持闭包，闭包与函数类似，但是具有声明它们的环境或者作用域等信息，可以将闭包分配给变量。闭包定义：let xxx = || ();

用两个竖线，竖线中间是参数；

```rust
fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    println!(doubler(value));
}
```

闭包主要用作一个高阶函数的参数，高阶函数是一个以两一个函数或者闭包作为参数的函数。

### 字符串

rust中字符串主要有两种类型：`&str`和`String`类型。

`String`类型数据是在堆上分配的，`&str`类型数据通常是指向现有字符串的指针，这些字符串可以在栈上，也可以在堆上，也可以上已编译对象代码的数据段中的字符串。

`&`是一个运算符，用于创建指向任何类型的指针。

### 集合类型

#### 数组

数组具有固定长度，可以存储相同类型的元素。用`[T;N]`表示，T表示类型，N表示长度（元素数量）。

```rust
let arr: [u8;8] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
```

#### 元组

元组中，类型可以是不同的，从函数返回多个值时，可以使用它。

```rust
let num_and_str: (u8, &str) = (40, "have a good day!");
```

#### vec

与数组类型，但是内容和长度不需要事先指定，并且可以按需增长。在堆上分配。创建使用函数`Vec::new()`或者`vec![]`宏。

```rust
let v1: Vec<u8> = vec![1];
v1.push(2);
let v2: Vec<u8> = Vec::new();
v2.push(3);
```

#### 键值对

rust标准库提供了键值对，可以存储键值对。在`std::collections`模块，名为HashMap。使用构造函数new创建。

```rust
use std::collections::HashMap;

let mut f = HashMap::new();
f.insert("apple", 3);
```

#### 切片

切片用`&[T]`表示，T表示任意类型，用法与数组类似。

```rust
let mut n: [u8;4] = [1,2,3,4];
{
    let all: &[u8] = &n[..];
}

{
    let first_two: &[u8] = &n[0..2];
}
```

### 自定义数据类型

#### 结构体struct

结构体用`struct`关键字定义，结构体分为单元结构体、元组结构体和类C语言结构体。单元结构体不占用任何空间，因为没有与之关联的数据。元组结构体具有关联数据，每个字段都没有命名，而是根据定义的位置进行引用。对于5个以下的数据建模时，元组结构体是首选。类C结构体通过`对象.类型名`来访问。

```rust

// 单元结构体
struct Dummy;
let value = Dummy;

// 元组结构体
struct Color(u8, u8, u8);
let white = Color(255, 255, 255);
let red = white.0;
let green = white.1;
let blue = white.2;

// 类C结构体
struct Player {
    name: String,
    iq: u8,
    score: u16,
    age: u8
}
let name = "Bob".to_string();
let player = Player{
    name,
    iq: 171,
    score: 123,
    age: 18
};
let score = player.score;
```

#### 枚举enum

采用`enum`关键字来定义枚举。

```rust
enum Transportation {
    Bus,
    Motorcycle,
    Walk
}
```

#### 结构体的impl

通过impl代码块，可以为结构体实现方法，其中一种方法没有self类型作为参数的叫做关联方法，关联方法智能通过`类型::方法名`来调用，另一种方法采用self作为第一个参数，叫做实例方法，通过`实例.方法名`来调用。

实例方法的self参数说明：
- `self`:不允许后续使用self类型
- `&self`:提供对类型实例的读取访问权限
- `&mut self`:提供对类型实例的可变访问
