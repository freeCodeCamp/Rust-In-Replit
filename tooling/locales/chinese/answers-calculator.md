# freeCodeCamp - Rust in Replit 课程

## 1

### --description--

Rust 生态的主要工具是：

- rustc 编译器用于将你的 Rust 代码编译成二进制（机器可以识别的代码）
- rustup 是一种命令行工具，用于安装和更新 Rust
- cargo 是 Rust 构建系统和软件包管理器（你会使用它）

任务：在提示中运行以下命令来创建一个新的 Rust 项目：

```bash
    $ cargo new calculator
```

### --seed--

```rust

```

### --tests--

- 此课程没有 RegEx 测试。
- `null`

## 2

### --description--

你刚刚在 `calculator/` 目录下创建了一个新的 Rust 项目

Cargo 已经创建了一个 “Hello World” 模版

任务：打开 `calculator/src/main.rs` 文件

这是 Cargo 为你的应用程序二进制文件使用的默认文件。

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- 你应该有项目文件 `calculator/src/main.rust`。
- `null`

## 3

### --description--

这个文件包含一个 handle 为 `main` 的函数声明。 默认情况下，rustc 在运行可执行文件时首先调用 `main` 函数。

`println` 是内置宏。

宏类似于函数，但可以被认为是一段写其他代码的代码。 目前需要记住的函数和宏的主要区别是：

    - 使用这个符号（!）调用宏
    - 宏可以使用可变数量的参数；Rust 中的函数不行

任务：使用以下命令运行你的代码

```bash
    $ cargo run --bin calculator
```

_注意：_ `--bin calculator` 参数是必需的，因为你不在 `calculator` 目录中。

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- 运行你的代码应该输出 `Hello, world!`。
- `getCommandOutput(Hello, world!)`

## 4

### --description--

变量使用 `let` 关键字声明。

```rust
    let variable_name = value
```

任务：在 `main` 函数内，声明一个新的变量， 命名为 `firstname`，并给它赋值 `"<your_name>"`。 请务必在调用 `println` 函数前声明，并将你的名字放在双引号内。

_注意：_ 变量也可以使用 const 或 static 关键字进行声明。

任务：运行你的代码来查看编译器的输出内容

```bash
    $ cargo run --bin calculator
```

_提示：_ 如果你被卡住，请尝试遵循编译器提供的建议。

你可以通过运行以下代码来看到你是否正确地完成了课程：

```bash
    $ fcc test 4
```

### --seed--

```rust
// Lesson #4
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- 你应该声明一个变量 `firstName` ，并把你的名字作为值放在双引号内。
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- 你应该采纳编译器的建议，在末尾加一个分号。
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

以上，你可能注意到 rustc 编译器为你的代码提供了两个建议。

任务：按照编译器的建议，将变量名称转换为 snake_case。

在 Rust 中，对以下情况使用 snake_case 是一种惯例：

- 变量名称
- 函数名称
- 文件名称

SCREAMING_SNAKE_CASE 用于常量和静态变量。 最后，_PascalCase_ 用于类型、特性和枚举（我们将在后面介绍这些）。

你可以通过运行以下代码来看到你是否正确地完成了课程：

```bash
    $ fcc test 5
```

### --seed--

```rust
// Lesson #5
fn main() {
  let firstName = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- 你应该声明一个变量 `first_Name` ，并把你的名字作为值放在双引号内。
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

任务：重新运行你的代码。 你现在应该只有一个警告。

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- 你应该声明一个变量 `first_Name` ，并把你的名字作为值放在双引号内。
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

编译器仍在警告你 `first_name` 是一个未使用的变量。

任务：通过修改 `println!` 调用为以下代码来修复它

```rust
    println!("Hello, {}!", first_name);
```

`'{}'` 被替换成参数的值。

你可以通过运行以下代码来看到你是否正确地完成了课程：

```bash
    $ fcc test 7
```

### --seed--

```rust
// Lesson #7
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- 你应该修改 `println!` 调用为 `println!("Hello, {}!", first_name)`。
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

你可以用 `println!` 做很多事情。 看看 Rust by Example 的文档，然后用你的代码尝试运行：

- https://doc.rust-lang.org/rust-by-example/hello/print.html

这就是为什么 `println!` 宏能够成为调试代码的优秀工具。

任务：运行代码以查看输出

```bash
    $ cargo run --bin calculator
```

### --seed--

```rust
// Lesson #8
fn main() {
  let first_name = "Quincy";
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该修改 `println!` 调用为 `println!("Hello, {}!", first_name)`。
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

`first_name` 的类型是 `&str`。 `str` 是一个原始类型，_符号（&）_ 表示该类型是一个_引用_。

Rust 语言的一个重要方面是所有权。 也就是说，内存的使用和分配。 在整个课程中都会出现所有权的概念。

另一个常见的类型是 `String`。 这是一个有用的类型，因为它是自动堆分配的。 这使得它的大小在编译时是未知的。

任务：使用 `String` 中的 from 特性将 `first_name` 转换为 `String` 类型：

```rust
    let example = String::from("Hello, Camper!");
```

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 9
```

### --seed--

```rust
// Lesson #9
fn main() {
  let first_name = "Quincy";
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该使用 `String::from()` 来创建一个 `String`，使用你的名字。
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

任务：在 `first_name`之后立即创建一个新变量，名为 `name`，并将值 `first_name` 分配给它。 然后，用你新创建的变量替换 `println!` 调用中的第二个参数。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 10
```

### --seed--

```rust
// Lesson #10
fn main() {
  let first_name = String::from("Quincy");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该声明一个变量 `name`，并将值 `first_name` 分配给它。
- `let\s+name\s*=\s*first_name\s*;`
- 你应该用 `name` 替换 `println!` 的第二个参数。
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

任务：复制当前 `println!` 调用，然后将它放置在第一个之后。 再用 `first_name` 替换第二个参数。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 11
```

### --seed--

```rust
// Lesson #11
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name;
  println!("Hello, {}!", name);
}
```

### --tests--

- 你应该有连着的两个 `println!` 调用。
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- 你应该在第一个 `println!` 使用 `name`，在第二个 `println!` 使用 `first_name`。
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

任务：运行你的代码。 你将看到一个错误。

### --seed--

```rust
// Lesson #12
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name;
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该有连着的两个 `println!` 调用。
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- 你应该在第一个 `println!` 使用 `name`，在第二个 `println!` 使用 `first_name`。
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

你的应用程序出错了。 这个错误的原因是上一次 `println!` 调用试图使用 `first_name` 变量。 但是，这个变量不再可用了，因为它被_移动_到 `name`。

为了防止 `first_name` 被移动，你可以将 `name` 分配给 `first_name` 的引用值。

任务：通过将引用符号添加到 `name` 值的开头来实现这一点。 这是一个例子：

```rust
    let value = String::from("");
    let referenced_value = &value;
```

这将防止 `value` 被移动到 `referenced_value`，然后，在 `referenced_value` 中使用 `value` 的引用值。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 13
```

### --seed--

```rust
// Lesson #13
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name;
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该有连着的两个 `println!` 调用。
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- 你应该在第一个 `println!` 使用 `name`，在第二个 `println!` 使用 `first_name`。
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- 你应该在把 `first_name` 分配给 `name` 时通过 `&first_name` 引用它。
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

任务：运行你的代码。 你不应该再看到错误了。

### --seed--

```rust
// Lesson #14
fn main() {
  let first_name = String::from("Tom");
  let name = &first_name;
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 用 `cargo run --bin calculator` 运行你的代码。 你不应该再看到任何错误了。
- `null`

## 15

### --description--

你想在 `name` 中添加你的姓氏（第二个名字）。

在 Rust 中，有很多方法可以做到这一点。 如果你只是尝试将 `" Surname"` 拼接到 `&first_name`，Rust 会出错，因为你不能拼接到一个引用值中。

你可以删除 &，但接着第二个 `println!` 会使程序出错。

为了拼接一个引用到一个 `str (&str)`，第一个参数需要被拥有。 通过 `to_owned` 方法，可以拥有一个 `String`：

```rust
    let owned_string = my_string.to_owned() + " Surname";
```

任务：不移动 `first_name`，而是将其变成一个被拥有的值，并将你的姓拼接到它——将结果分配给 `name`。

运行你的代码。 如果它编译和打印这两行，你就正确地完成了课程。 否则，请使用输出调试并修复你的代码。

### --seed--

```rust
// Lesson #15
fn main() {
  let first_name = String::from("Tom");
  let name = &first_name;
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该使用 `.to_owned()` 将 `first_name` 转换为被拥有的值。
- `first_name\.to_owned\(\)`
- 你应该将你的姓与所拥有的 `first_name` 连起来。
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

使用 `String` 类型的一个更加习惯性的方法是使用 `push_str` 方法：

```rust
    let mut my_string = String::from("String");
    my_string.push_str("a str");
```

任务：删除 `name` 以及第一个 `println!` 调用。 然后，在 `first_name` 上使用 `pub_str` 方法添加你的姓。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 16
```

### --seed--

```rust
// Lesson #16
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name.to_owned() + " Larson";
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该使用 `.push_str()` 方法。
- `\.push_str\(`
- 你应该把你的姓推到 `first_name`。
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

任务：运行你的代码。 它应该报错并退出。

### --seed--

```rust
// Lesson #17
fn main() {
  let first_name = String::from("Quincy");
  first_name.push_str(" Larson");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该使用 `.push_str()` 方法。
- `\.push_str\(`
- 你应该把你的姓推到 `first_name`。
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- 你应该使用 `let mut first_name = ...` 让 `first_name` 是可变的。
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

你的代码报错而退出，因为 `first_name` 不是_可变的_。

任务：使用编译器的提示使 `first_name` 成为可变的。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 18
```

### --seed--

```rust
// Lesson #18
fn main() {
  let first_name = String::from("Quincy");
  first_name.push_str(" Larson");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该使用 `.push_str()` 方法。
- `\.push_str\(`
- 你应该把你的姓推到 `first_name`。
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- 你应该使用 `let mut first_name = ...` 让 `first_name` 是可变的。
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

任务：再次运行你的代码来确保编译时没有错误。

### --seed--

```rust
// Lesson #19
fn main() {
  let mut first_name = String::from("Nicholas");
  first_name.push_str(" Carrigan");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 用 `cargo run --bin calculator` 运行你的代码。 你不应该再看到任何错误了。
- `null`

## 20

### --description--

到目前为止，你已经了解到 `str` 和 `String` 类型以及引用。 如果你没有意外使用单引号（'），你可能没有注意到：到目前为止，与字符串有关的一切都使用双引号（"）。

这是因为有第三种标准类型称为 `char`。

`char` 是一个 _USV (Unicode Scalar 值)_，表示为 unicode 和值，例如 `U+221E`——“∞” 的 unicode。

字符串可以被认为是 `char` 的集合或数组。

任务：从你的 `main` 函数中删除所有代码。 然后，声明一个新变量 `first`， 并将你的名字的第一个字母分配给它——`first` 应该是类型 `&str`。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 20
```

### --seed--

```rust
// Lesson #20
fn main() {
  let mut first_name = String::from("Nicholas");
  first_name.push_str(" Carrigan");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- 你应该声明一个变量 `first`。
- `let\s+first`
- 你应该给 `first` 赋值，格式为一个双引号中的单个字符。
- `first\s*=\s*"\w"`

## 21

### --description--

任务：首先在控制台打印对 `first` 使用 `.len()` 方法的值，以及 `first.chars().count()` 的值。

任务：运行代码以查看输出。 如果它运行并打印 `'1 1'`，你已经正确地完成了课程。

### --seed--

```rust
// Lesson #21
fn main() {
  let first = "T";
}
```

### --tests--

- 你应该首先打印 `first` 的长度，以及 `first` 中字符的数量。 输出示例：`1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

### --description--

你应该在控制台看到 `1 1` 输出。 `len` 方法返回 `str` 的字节长度。 `chars` 方法返回一个在字符串切片中 `char` 迭代器，`count`方法返回迭代器中元素的数量。

任务：将 `first` 的值更改为无限字符 ∞ 的字符串切片。

_提示：_ 你可以从上面一行复制粘贴字符。

运行你的代码来查看输出。 如果它运行并打印 `'3 1'`，你已经正确地完成了课程。

### --seed--

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- 你应该将 `first` 的值更改为 `∞` 的字符串切片。
- `first\s*=\s*"∞"`
- 你的代码应该打印 `3 1`。
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

你应该在控制台看到 `3 1` 输出。 这是因为 `'∞'` 字符长度为 3 字节。

任务：随时尝试使用这些新方法，并了解不同字符串产生什么值。

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- 此课程没有测试。
- `null`

## 24

### --description--

从这一课开始，你在写代码时要注意 _TDD 测试驱动开发_。 也就是说，你需要写代码来通过现有的测试，以及自己写一些测试来通过课程。

任务：运行下面的命令来为下一节课程初始化你的代码和测试。

```bash
    $ fcc reset 24
```

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 24
```

### --seed--

```rust
// Lesson #24


#[cfg(test)]
mod tests {

  #[test]
  fn main_exists() {
    assert_eq!(main(), ());
  }
}
```

### --tests--

- 你应该运行命令 `fcc reset 15`。
- `mod tests`

## 25

### --description--

已经包括了测试的基本设置。 声明上方的 `#[]` 语法是_属性_如何添加到 Rust。

`cfg(test)` 将 `test` 特性配置为下面的声明，`#[test]` 语法声明了哪些函数将被作为测试运行。

任务：在脚本的顶端添加一个名为 `main` 的函数。 然后，在 `tests` 模块的顶端，导入 `main` 函数，使用此语法：

```rust
    use crate::main;
```

`use` 关键字，在 Rust 中，类似于其他语言中的 “import”、“require” 或 “include”。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #25

#[cfg(test)]
mod tests {

  #[test]
  fn main_exists() {
    assert_eq!(main(), ());
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 26

### --description--

你可能会从测试中注意到，没有明确返回的函数返回了一个空的 `tuple`。 元组用括号（）表示——为什么测试断定 `main` 的返回是 `()`。

有两种返回方式。 使用 `return` 关键字，或使用分号。

返回除空元组以外的任何值的函数需要明确进行类型定义：

```rust
    fn my_func() -> String {
      let my_string: String = String::from("Nich");
      my_string
    }
```

_注意：_上面的内容已经被明确定义类型，以求清晰。

任务：通过测试，从 `main` 返回 `24` 并将函数的返回的类型设置为 `usize` 类型。

`usize` 是一个正整数的默认类型。 u 代表_unsigned 无符号的_，size 大小表示系统的比特大小。 这通常是 64 或 32 位系统。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #26
fn main() {

}

#[cfg(test)]
mod tests {
  use crate::main;
  #[test]
  fn main_returns_24() {
    assert_eq!(main(), 24);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 27

### --description--

在 Rust 中有许多类型的数字：

    - 无符号的整数：`u8`、`u16`、`u32`、`u64`、`usize`、`u128`
    - 有符号的整数：`i8`、`i16`、`i32`、`i64`、`isize`、`i128`
    - 浮点数：`f32`、`f64`

无符号的整数只表示正整数。 有符号的整数表示正整数和负整数。 浮点数只表示正和负分数。

任务：通过更改 `main` 函数的数字和返回类型来通过测试。

_注意：_ 第一个测试包含 `should_panic` 特性。 这意味着代码应该报错而退出。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #27
fn main() -> usize {
  24
}

#[cfg(test)]
mod tests {
  use crate::main;
  #[test]
  #[should_panic]
  fn main_panics_with_i() {
    assert_eq!(main() as usize as f32, main() as f32);
  }
  #[test]
  fn main_returns_f() {
    assert_eq!(main() as f32, 24.5);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 28

### --description--

你想要你的计算器在命令行上使用，比如：

```bash
    $ calculator <first_number> <operator> <second_number>
```

输出如下：

```bash
    $ <first_number> <operator> <second_number> = <result>
```

例如：

```bash
    $ calculator 1 + 1
    $ 1 + 1 = 2
```

任务：创建一个名为 `output` 的新函数 ，它接受 4 个参数。 第一个、第三个和第四个参数应该是有符号的整数，而第二个参数应该是一个 `char`。

_提示：_不要忘记将新函数导入测试模块。

这是一个具有定义类型的参数的函数示例：

```rust
    fn example(first_arg: usize, second_arg: String) -> &str {
      "I return a reference to a string slice"
    }
```

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #28
fn main() -> f32 {
  24.5
}




#[cfg(test)]
mod tests {
  use crate::main;

  #[test]
  #[should_panic]
  fn main_panics_with_i() {
    assert_eq!(main() as usize as f32, main() as f32);
  }
  #[test]
  fn main_returns_f() {
    assert_eq!(main() as f32, 24.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, ());
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 29

### --description--

现在，要使 `output` 返回正确的输出，你将使用宏。

`format!` 宏的运行与 `println!` 宏几乎完全一样，你一直在使用。 只有一点，它返回输出为 `String`，而不是把输出打印到控制台。

任务：使用 `format!` 宏返回这个格式的输出：

```bash
    <first_number> <operator> <second_number> = <result>
```

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #29
fn main() -> f32 {
  24.5
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) {

}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  #[test]
  #[should_panic]
  fn main_panics_with_i() {
    assert_eq!(main() as usize as f32, main() as f32);
  }
  #[test]
  fn main_returns_f() {
    assert_eq!(main() as f32, 24.5);
  }

  #[test]
  fn output_returns_the_correct_string() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 30

### --description--

任务：在 `main` 函数内，将调用 `output` 和有效参数的结果打印到控制台。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 30
```

### --seed--

```rust
// Lesson #30
fn main() {

}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

### --tests--

- 你应该把有效的输出打印到控制台。
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

任务：在 `main` 函数内，声明三个变量：`first_num`、`operator` 和 `second_number`。

然后分配它们有效的值，并在 `output` 调用中传递它们作为参数。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 31
```

### --seed--

```rust
// Lesson #31
fn main() {

  println!("{}", output(10, '+', 10, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

### --tests--

- 你应该声明一个变量 `first_number`。
- `let first_number`
- 你应该声明一个变量 `second_number`。
- `let second_number`
- 你应该声明一个变量 `operator`。
- `let operator`
- 你应该以你刚刚声明的变量调用 `output`。
- `output\(\s*first_number\s*,\s*operator\s*,\s*second_number`

## 32

### --description--

你可能已经注意到打印到控制台的内容不正确。 你需要在输入数字上执行操作才能修复这个问题。

任务：声明一个名为 `operate` 的新函数，按顺序接受 `operator`、`first_number` 和 `second_number`。

_提示：_记住将函数导入到 `tests` 模块。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #32
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}



#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;

  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_expects_three_args() {
    let op = operate('-', -5, 200);
    assert_eq!(op, ());
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 33

### --description--

你想要能够执行四个基本操作：加、 减、 除和乘。

任务：使用多个 `if` 语句来比较 `operator` 是 `'+' '-' '*' '/'` 其中之一的情况。

`if` 语句的语法是：

```rust
    if my_var == "my str" {
      // Do stuff
    } else if my_var == "something else" {
      // Do more stuff
    } else {
      // Finally...
    }
```

任务：返回对于 `first_number` 和 `second_number` 的操作结果，以通过测试。

_提示：_记住要包含一个 `else` 语句。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #33
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) {

}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -10, -12);
    assert_eq!(op, 2);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_returns_zero_on_invalid_op() {
    let op = operate('a', 1, 1);
    assert_eq!(op, 0);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 34

### --description--

如果使用无效的运算符，不会返回 `0` 的结果，而是会更容易引起应用程序 panic。

`panic!` 宏就是这样，它接受一个字符串切片作为参数，它可以包含一个引起 panic 的消息。

任务：当使用无效运算符时，你的代码发生 panic。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #34
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  if operator == '+' {
    first_number + second_number
  } else if operator == '-' {
    first_number - second_number
  } else if operator == '/' {
    first_number / second_number
  } else if operator == '*' {
    first_number * second_number
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 35

### --description--

你可以使用 Rust 的 `match` 控制流提高代码的可读性和可用性，而不是使用大量 `if...else` 语句。 `match` 运算符类似于其他语言中的 `switch` 语句。 然而，它允许模式匹配。

一个使用 `match` 运算符的表达式的例子：

```rust
    let some_variable = 't';
    match some_variable {
      'a' => 'A',
      'b' => 'B',
      _ => 'Z',
    }
```

因为 `'t'` 不匹配 `'a'` 或 `'b'`，表达式返回 `'Z'`，即下划线指示的基本情况。

任务：转换 `operate` 内的 if/else 逻辑为使用 `match` 运算符。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 35
```

### --seed--

```rust
// Lesson #35
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  if operator == '+' {
    first_number + second_number
  } else if operator == '-' {
    first_number - second_number
  } else if operator == '/' {
    first_number / second_number
  } else if operator == '*' {
    first_number * second_number
  } else {
    panic!("Invalid operator used.");
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该使用 `match` 运算符。
- `match`
- 你仍然需要通过所有测试。
- `getTestOutput(7 passed)`

## 36

### --description--

你应该能够使用计算器，输入是这样的：`calculator 3 x 3`。

一个 `match` 模式可以使用这样的比特逻辑扩展：

```rust
    match name {
      "Quincy" => "Hello, Quincy",
      "Tom" | "Nich" => "Hello, other",
      _ => panic!("Pattern not found"),
    }
```

`name` 为 `"Nich"`，第二个 `match` _分支_ 应该被匹配。

任务：扩展 `match` 运算符中的分支以匹配 `operator` 值 `'x'` 和 `'X'`。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #36
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该使用 `|` 运算符。
- `|`

## 37

### --description--

目前， `output` 的 `result` 参数是硬编码的。

任务：在 `main` 里面，声明一个新变量，名为 `result`，并赋予它一个调用 `operate` 传入前三个变量的值。 然后，将 `result` 传递给 `output`，作为第四个参数。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 37
```

### --seed--

```rust
// Lesson #37
fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该声明一个变量 `result`。
- `let\s+result`
- 你的代码应该打印 `1 - 10 = -9` 到控制台。
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

你想要此应用程序从命令行参数中读取值。 Rust 的标准库有一个环境模块，可以访问通过 CLI 传递的参数。

使用以下语法访问标准库中的模块：

```rust
    use std::*;
```

这将导入标准库中的所有模块。 然而，你只需要一个。

任务：在脚本的根目录，使用上面的语法只从标准库中导入 env 模块。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 38
```

### --seed--

```rust
// Lesson #38


fn main() {
  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该添加 `use std::env;` 到你的脚本顶部。
- `use\s+std::env\s*;`

## 39

### --description--

现在 `env` 模块已经放入范围内，你可以引用它的结构体、枚举和函数。

任务： `main` 顶部声明一个新变量，名为 `args`，并分配给它调用 `env` 模块中 `args` 函数的值。

_提示：_记住，在模块中访问函数使用 `'::'` 语法。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 39
```

### --seed--

```rust
// Lesson #39
use std::env;

fn main() {


  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该声明一个新的变量 `args`。
- `let\s+args`
- 你应该分配给 `args` 一个值 `env::args()`。
- `=\s*env::args\(\)`

## 40

### --description--

任务：要了解 `args` 的内容，将其值打印到控制台。

_提示：_记住，如果你正在努力打印该值的话，请遵循编译器的有用建议。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 40
```

### --seed--

```rust
// Lesson #40
use std::env;

fn main() {
  let args = env::args();

  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 运行 `cargo run --bin calculator` 应该打印：`Args { inner: ["target/debug/calculator"] }`。
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

在运行 crate 时不传递任何参数，`args` 的值仍然包含一个参数——二进制文件的相对路径。

任务：通过运行命令来查看 `args` 的不同值：

```bash
    $ cargo run --bin calculator -- 1 + 2
```

### --seed--

```rust
// Lesson #41
use std::env;

fn main() {
  let args = env::args();
  println!("{:?}", args);

  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 42

### --description--

要在 `args` 中访问一个特定的参数，你可以使用 `nth` 方法。 `nth` 方法接受一个数字参数（n）以访问下一个 “第 n 个” 参数——使用基于 0 的索引。

任务：更改 `args` println 来打印第一个参数到控制台。

_提示：_记住，如果你遇到困难，请遵循编译器的有用建议。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 42
```

### --seed--

```rust
// Lesson #42
use std::env;

fn main() {
  let args = env::args();
  println!("{:?}", args);

  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该访问 `env::args()` 迭代器的第一个（`0`）元素。
- `args\.nth\(0\)`
- 你应该使用 `let mut args =...` 声明 `args` 是可变的。
- `let\s+mut\s+args`

## 43

### --description--

如果你遵循编译器的建议，在前一课中，你需要声明 `args` 为可变的。 这是因为 `nth` 方法在元素上反复迭代。

任务：移除 “args” println。 然后，更改 `first_number`、`operator` 和 `second_number` 分别等于第一个、第二个和第三个 `args`。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 43
```

### --seed--

```rust
// Lesson #43
use std::env;

fn main() {
  let mut args = env::args();
  println!("{:?}", args.nth(0));

  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该将 `args.nth(0)` 赋值给 `first_number`。
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- 你应该将 `args.nth(1)` 赋值给 `operator`。
- `let\s+operator\s*=\s*args\.nth\(1\)`
- 你应该将 `args.nth(2)` 赋值给 `second_number`。
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

一些代码已被注释掉，因此程序会被编译。

如果你现在运行代码，你将看到输出包含：

```bash
    $ Some("target/debug/calculator"), None, None
```

这是因为 `nth` 不会直接返回值，而是返回以 `Option` 包裹的值。

`Option` 是一个类型，包含 `Some` 包裹着一个值，或者是当值不存在的时候是 `None`。

为了使用包裹在 `Some` 中的值，`Option` 可以被_解包：_

```rust
    let my_option: Option<String> = env::args().nth(0);
    let my_value: String = my_option.unwrap();
```

任务：在声明变量 `first_number`、`operator` 和 `second_number` 时解包它们，并运行你的代码来查看发生什么。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 44
```

### --seed--

```rust
// Lesson #44
use std::env;

fn main() {
  let mut args = env::args();

  let first_number = args.nth(0);
  let operator = args.nth(1);
  let second_number = args.nth(2);

  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该使用 `args.nth(0).unwrap()` 解包 `first_number`。
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- 你应该使用 `args.nth(1).unwrap()` 解包 `operator`。
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- 你应该使用 `args.nth(2).unwrap()` 解包 `second_number`。
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

现在，通过以下代码运行应用程序：

```bash
    $ cargo run --bin calculator
```

出现 panic。 这是因为在 `None` 存在的情况下试图解包一个值是未定义的行为。

有一些方法可以更优雅地处理错误，但是，现在要确保在调用你的应用程序时有足够的参数：

```bash
    $ cargo run --bin calculator -- 1 + 2
```

任务：再次运行你的代码，但在 “--” 之后继续添加参数，直到没有 panic。

### --seed--

```rust
// Lesson #45
use std::env;

fn main() {
  let mut args = env::args();

  let first_number = args.nth(0).unwrap();
  let operator = args.nth(1).unwrap();
  let second_number = args.nth(2).unwrap();

  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 46

### --description--

现在，需要 5 个参数来防止应用程序出现 panic。 看起来你只是在试图解包第三个元素？

实际上，由于 `nth` 迭代了 `args`，访问第一个元素后，它将被删除。 因此，在之后试图访问第二个元素就等于最初试图访问第三个元素。

任务：更改传递给 `nth` 的参数，以便访问正确的元素。 运行 `cargo run --bin calculator -- 1 + 2` 应该输出："1", "+", "2"。

_提示：_记住，第一个元素是二进制的相对路径 - 而不是 first_number。

### --seed--

```rust
// Lesson #46
use std::env;

fn main() {
  let mut args = env::args();

  let first_number = args.nth(0).unwrap();
  let operator = args.nth(1).unwrap();
  let second_number = args.nth(2).unwrap();

  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 47

### --description--

明确注明你的变量类型是有用的。 你已经看到了这个例子，但这里还有另外一个例子：

```rust
    let my_var: &str = "Mrugesh";
```

任务：为你的 `args`、`first_number`、`operator` 和 `second_number` 变量注明类型。

_提示：_如果类型不正确，则按照编译器的建议进行纠正。 你需要从 `env` 模块导入类型。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 47
```

### --seed--

```rust
// Lesson #47
use std::env;

fn main() {
  let mut args = env::args();

  let first_number = args.nth(1).unwrap();
  let operator = args.nth(0).unwrap();
  let second_number = args.nth(0).unwrap();

  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该通过 `use std::env::Args` 从 `std::env` 模块导入 `Args` 结构体。
- `use\s+std::env::Args`
- 你应该注明 `args` 的类型为 `Args`。
- `let\s+mut\s+args:\s*Args`
- 你应该注明 `first_number` 的类型为 `String`。
- `let\s+first_number:\s*String`
- 你应该注明 `operator` 的类型为 `String`。
- `let\s+operator:\s*String`
- 你应该注明 `second_number` 的类型为 `String`。
- `let\s+second_number:\s*String`

## 48

### --description--

不要写入不必要的导入，你可以用以下语法合并它们：

```rust
    use std::env::{var, Vars};
```

以上导入标准库中的 `var` 函数和 `env` 模块中的 `Vars` 结构体。

任务：使用一个 import 语句导入 `args` 函数，以及 `Args` 结构体。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 48
```

### --seed--

```rust
// Lesson #48
use std::env;
use std::env::Args;

fn main() {
  let mut args: Args = env::args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该使用 `use std::env::{args, Args};` 将这两次导入合并成一个单一的导入语句。
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

现在，你需要修复 `operate` and `output` 的问题，输出是 `i32` 和 `&str` 类型。

使用 `parse` 方法缓存，这个方法适用于 `String` 类型。

`parse` 方法将 `String` 转换为一个指定的类型。 类型可以使用 _turbofish（涡轮鱼）_语法提供：

```rust
    let my_string_number: String = String::from("Kris");
    let my_number_option: Option<usize> = my_string_number.parse::<usize>();
    let my_number: usize = my_number_option.unwrap();
```

任务：在 `main` 里面，声明两个新变量 - `first` 和 `second` - 并使用涡轮鱼语法分别赋予解析的和解包的值 `first_number` 和 `second_number`。 然后，在 `println!` 调用中将 `first_number` 和 `second_number` 替换为 `first` 和 `second`。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ fcc test 49
```

### --seed--

```rust
// Lesson #49
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();



  println!("{:?}, {:?}, {:?}", first_number, operator, second_number);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 你应该声明一个变量 `first`。
- `let\s+first`
- 你应该声明一个变量 `second`。
- `let\s+second`
- 你应该将 `first_number.parse::<i32>().unwrap()` 赋给 `first`。
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- 你应该将 `second_number.parse::<i32>().unwrap()` 赋给 `second`。
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

目前，`operator` 是 `String` 类型，它需要是 `char`。 `String` 可以转换成 `char`，使用 `chars` 方法，并通过调用 `next` 方法将返回的第一个 `Option` 解包。

任务：取消注释被注释掉的代码，并且进行必要的调整以允许代码编译。

请确保遵循编译器的提示来编译代码。 然后，删除第一个 `println!` 调用，所以只有一个输出。

你可以通过运行以下代码来确保输出正确：

```bash
    $ cargo run --bin calculator -- 1 + -1
```

### --seed--

```rust
// Lesson #50
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<i32>().unwrap();
  let second = second_number.parse::<i32>().unwrap();

  println!("{:?}, {:?}, {:?}", first, operator, second);
  // let result = operate(operator, first_number, second_number);

  // println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}
```

### --tests--

- 此课程没有 Node 测试。
- `null`

## 51

### --description--

目前，计算器只接受整数作为输入。

任务：更改所有必要类型以允许计算器也接受浮点数。

你可以通过运行以下代码查看你是否正确完成了这一课：

```bash
    $ cargo test --bin calculator
```

### --seed--

```rust
// Lesson #51
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<i32>().unwrap();
  let second = second_number.parse::<i32>().unwrap();
  let result = operate(operator, first, second);

  println!("{}", output(first, operator, second, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate('+', -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1.0, 1.0);
  }
}
```

### --tests--

- 提示：你应该将类型从 `i32` 更改为 `f32`。
- `null`

## 52

### --description--

你已完成二进制代码。 现在，你需要编译和发布它以便使用它。

任务：运行以下命令将你的代码编译成二进制版本：

```bash
    $ cargo build --bin calculator
```

如果你看到没有错误，就说明你已经成功地完成了课程。

### --seed--

```rust
// Lesson #52
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(operator, first, second);

  println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, '*', 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate('+', -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1.0, 1.0);
  }
}
```

### --tests--

- 这是最后一节课程。 祝贺你！
- `null`

## 53

### --description--

Cargo 刚刚将你的代码编译到 `target/debug` 目录。

任务：使用以下命令运行你的代码：

```bash
    $ target/debug/calculator 1 + 2
```

如果你看到输出 `'1 + 2 = 3'`，就说明你已经成功地完成了课程。

### --seed--

```rust
// Lesson #53
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(operator, first, second);

  println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, '*', 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate('+', -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1.0, 1.0);
  }
}
```

### --tests--

- 这是最后一节课程。 祝贺你！
- `null`

## 54

### --description--

Rust 编译器在编译时经常会进行令人难以置信的优化。 然而，为了最大限度地利用它，你需要使用 Cargo 构建你的代码的_发行_版。

任务：重新构建你的应用程序，这次使用 `release` 标志：

```bash
    $ cargo build --release --bin calculator
```

你应该能够在 `target/release` 目录中找到优化的二进制文件。

### --seed--

```rust
// Lesson #54
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(operator, first, second);

  println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, '*', 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate('+', -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1.0, 1.0);
  }
}
```

### --tests--

- 这是最后一节课程。 祝贺你！
- `null`

## 55

### --description--

恭喜！ 你已完成 `freeCodeCamp - Rust in Replit - CLI Calculator` 项目。

欢迎你拓展你当前的项目——也许是接受多个操作......

任务：运行以下命令来开始下一个项目：

```bash
    $ fcc switch combiner
```

### --seed--

```rust
// Lesson #55
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(operator, first, second);

  println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, '*', 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate('+', -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, '+', 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1.0, 1.0);
  }
}
```

### --tests--

- 这是最后一节课程。 祝贺你！
- `null`

## 56

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Placeholder
- `null`
