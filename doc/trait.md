# trait

trait是rust中的特征（可以类比go中的接口）,使用`trait`关键字可以定义一个特征，使用花括号括起来，并且里面包含零个或多个方法。可以在特征中定义常量，所有实现者都可以共享它。

```rust
pub trait Playable {
    fn play(&self);

    fn pause(&self) {
        println!("Pausing...");
    }
}

pub use basic_rust::playable::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Playing audio: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Playing video: {}", self.0);
    }
}
```

trait支持继承。

```rust
// 测试trait继承

trait Vehicle {
    fn get_price(&self) -> u64;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn nee(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200000
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

fn main() {
    let tesla = TeslaRoadster::nee("Tesla Roadster I", 2008);
    println!("Model: {}", tesla.model());
    println!("Release date: {}", tesla.release_date);
    println!("Price: {}", tesla.get_price());
}
```

## 特征的形式

### 标记特征

标准库中的`std::marker`中定义的特征称为标记特征，这类特征不包含任何方法，声明时只是提供特征名称和空的函数体。

- Copy
- Send
- Sync

这些都是标记特征，因为它们用于简单地将类型标记为属于特定的组群，以获得一定程度的编译期保障。

### 简单特征

特征定义的最简单形式。

标准库中的一个简单示例是`Default`特征，主要是针对可以使用默认值初始化的类型实现的。

### 泛型特征

特征也可以是泛型的，能让用户为多种类型实现特征。

```rust
pub trait From<T> {
    fn from(T) -> Self;
}
```

泛型特征的例子是标准库里面的`From<T>`和`Into<T>`。它们允许从某种类型转换为类型T，反之亦然。

### 关联类型特征

```rust
trait Foo {
    type Out;

    fn get_value(self) -> Self::Out;
}
```

在特征中声明相关类型，比如上面的在特征中声明Out类型。最佳用例是`Iterator`特征。

### 继承特征

```rust
trait Bar {
    fn bar();
}

trait Foo: Bar {
    fn foo();
}
```

上面的代码声明了一个特征Foo，它依赖于父级特征Bar。

标准库中有这样的一个特征，Copy特征，它依赖于Clone特征。

### 特征区间

```rust
struct Game;
struct Enemy;
struct Hero;

impl Game {
    fn load<T>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
```

上面代码无法编译通过，因为无法知道类型为T的泛型函数，是否拥有init方法。

我们可以定义一个trait，trait里面包含init方法：

```rust
struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
```

现在就可以编译通过了，在Game的load方法中，我们指定了T的特征范围。这就是特征区间。

Rust中指定特征区间可以是上面的方式，放在尖括号里面，也可以使用Where语句来指定：

```rust
impl Game {
    fn load<T>(&self, entity: T) 
    Where T: Loadable
    {
        entity.init();
    }
}
```

## 标准库特征介绍

**1、Default特征**

为类型提供了默认初始化的功能。

```rust
#[derive(Default)]
struct Complex<T> {
    re: T,
    im: T
}
```

> 只有成员或者字段都实现了Default特征的结构体、枚举、联合体等才能实现Defalut特征

**2、Debug**

在控制台可以输出类型，便于调试

**3、PartialEq和Eq**

用于元素比较，验证是否相等

**4、Copy和Clone**

定义了类型的复制方式，Copy隐式创建，Clone通过调用clone方法显式创建。

Copy依赖于在类型上实现的Clone。

**5、std::ops模块的Add**

可以实现在类型上`+`运算符

**6、std::conver模块的Into和From特征**

用户能够更具其它类型创建或者转化类型。

**7、Display特征**

输出可读的显示。