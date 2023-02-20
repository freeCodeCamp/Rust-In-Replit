# freeCodeCamp - Rust in Replit 课程 - 图像组合

## 1

### --description--

首先创建一个名为 `combiner` 的新项目。

运行 `fcc test 1` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #1
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- 你应该看到在 root 创建的新目录 `combiner`。
- `Hello, world!`

## 2

### --description--

在这个项目中，你将创建一个 CLI（组合程序），它需要三个参数：

```bash
    $ combiner image1.png image2.png output.png
```

前两个参数是你想要组合的图像路径。 第三个参数是输出图像的路径。

任务：打开 `combiner/src/main.rs` 并运行 `cargo run --bin combiner` 以查看你的应用程序是否正确设置。

你应该看到 `Hello, world!` 被打印到控制台。

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- 你的代码应该输出 `Hello, world!`
- `getCommandOutput(Hello, world!)`

## 3

### --description--

任务：定义一个名为 `get_nth_arg` 的函数，这个函数需要一个 `usize` 参数。

请记住，必要时导入。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {

  #[test]
  fn get_nth_arg_is_defined() {
    let _ = get_nth_arg;
    assert!(true, "If this test panics, get_nth_arg is not defined.");
  }
  #[test]
  fn get_nth_arg_takes_usize() {
    assert_eq!(get_nth_arg(0usize), (), "Your get_nth_arg function should take a usize argument.");
  }
}
```

### --tests--

- 运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。
- `null`
- 提示：记住将函数导入到 `tests` 模块。
- `null`

## 4

### --description--

即将进行的测试使用了一个名为 `regex` 的外部 crate。

任务：在 root 中打开 `Cargo.toml` 文件，并添加以下代码：

```rust
    [dependencies]
    regex = "1.5.4"
```

这将安装 `regex` crate 到你的项目中，在 `reg_with_con` 函数中使用它。 你可以在 https://crates.io/crates/regex 了解关于这个 crate 的更多信息。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #4
fn main() {
  println!("Hello, world!");
}

fn get_nth_arg(n: usize) {
}

#[cfg(test)]
mod tests {
  use crate::get_nth_arg;

  #[test]
  fn get_nth_arg_takes_usize() {
    let _ = get_nth_arg(0usize);
    assert!(true, "Your get_nth_arg function should take a usize argument.");
  }

  #[test]
  fn regex_crate_is_installed() {
    let string_to_test = "println!(\"Hello, world!\")".to_string();
    assert!(reg_with_con(r"Hello, world!", string_to_test));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
}
```

### --tests--

- 运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。 没有报错就说明你已经完成了。
- `null`

## 5

### --description--

要使用命令行参数，你需要使用 `std::env` 模块。

任务：在 `get_nth_arg` 中，返回在具有参数 `n` 的 `args` 函数上调用 `nth` 方法的展开的值。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #5
fn main() {
  println!("Hello, world!");
}

fn get_nth_arg(n: usize) {
}

#[cfg(test)]
mod tests {
  use crate::get_nth_arg;

  #[test]
  fn get_nth_arg_takes_usize() {
    let _ = get_nth_arg(0usize);
    assert!(true, "Your get_nth_arg function should take a usize argument.");
  }
  #[test]
  fn get_nth_arg_returns_string() {
    let val = get_nth_arg(0);
    assert!(reg_with_con(r"target/debug/deps", val));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
}
```

### --tests--

- 运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。 没有报错就说明你已经完成了。
- `null`

## 6

### --description--

理想的情况是，你想在一个变量中只存储你感兴趣的命令行参数。

任务：在 `main`内，创建一个名为 `args`的变量，并给它赋值 `Args {}`。

运行 `fcc test 6` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #6
fn main() {
  println!("Hello, world!");
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use crate::get_nth_arg;

  #[test]
  fn get_nth_arg_takes_usize() {
    let _ = get_nth_arg(0usize);
    assert!(true, "Your get_nth_arg function should take a usize argument.");
  }
  #[test]
  fn get_nth_arg_returns_string() {
    let val = get_nth_arg(0);
    assert!(reg_with_con(r"target/debug/deps", val));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
}
```

### --tests--

- 你应该创建一个新的变量 `args`。
- `let args`
- 你应该给 `args` 赋值 `Args {}`。
- `let\s+args\s*=\s*Args\s*\{\};`

## 7

### --description--

语法 `Args {}` 是一个名为 `Args` 的结构体的构造器。 然而，我们还没有定义结构体。

这是一个你已经使用过的结构体的示例：

```rust
    struct String {
      vec: Vec<u8>,
    }
```

`String` 结构体包含一个 `vec` 字段，它是 `Vec` `u8`。

任务：在全局范围内，定义一个命名为 `Args` 的结构体。

请记住，必要时导入。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #7
fn main() {
  let args = Args {};
  println!("Hello, world!");
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use crate::get_nth_arg;

  #[test]
  fn get_nth_arg_takes_usize() {
    let _ = get_nth_arg(0usize);
    assert!(true, "Your get_nth_arg function should take a usize argument.");
  }
  #[test]
  fn get_nth_arg_returns_string() {
    let val = get_nth_arg(0);
    assert!(reg_with_con(r"target/debug/deps", val));
  }

  #[test]
  fn args_struct_exists() {
    let _my_arg = Args {};
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
}
```

### --tests--

## 8

### --description--

不用为 `tests` 模块中的每个函数或结构体写 `use crate::`，你可以使用 `super` 关键词和 `*` 通配符选择器来选择当前模块中的所有内容。

任务：用 `use super::*` 替换 `use crate::` 调用。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #8
fn main() {
  let args = Args {};
  println!("Hello, world!");
}

struct Args {}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use crate::get_nth_arg;
  use crate::Args;
  #[test]
  fn get_nth_arg_takes_usize() {
    let _ = get_nth_arg(0usize);
    assert!(true, "Your get_nth_arg function should take a usize argument.");
  }
  #[test]
  fn get_nth_arg_returns_string() {
    let val = get_nth_arg(0);
    assert!(reg_with_con(r"target/debug/deps", val));
  }

  #[test]
  fn args_struct_exists() {
    let _my_arg = Args {};
  }

  #[test]
  fn use_crate_is_not_used() {
    let main_contents = return_file_in_src("main.rs");
    assert!(!reg_with_con(r"use\s+crate", main_contents));
  }
  #[test]
  fn use_super_is_used() {
    let main_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"use\s+super::\*", main_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

- 运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。 没有报错就说明你已经完成了。
- `null`

## 9

### --description--

任务：添加一个名为 `image_1` 的字段到 `Args` 结构体，并给它一个正确的类型来通过测试。

记得在需要时调整 `main` 中的声明。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #9
fn main() {
  let args = Args {};
  println!("Hello, world!");
}

struct Args {}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let _my_arg = Args { image_1: String::new() };
  }
}
```

### --tests--

## 10

### --description--

任务：为了更好地了解 `Args` 结构体，在 `args_struct_has_image_1_field` 测试中打印 `my_arg` 的值。

运行 `cargo test --bin combiner -- --show-output`。 如果你看到一个错误，你正确地完成了任务。

`--show-output` 标志显示测试的标准输出结果。

### --seed--

```rust
// Lesson #10
fn main() {
  let args = Args { image_1: String::new() };
  println!("Hello, world!");
}

struct Args {
  image_1: String,
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let my_arg = Args { image_1: String::new() };

  }
}
```

### --tests--

## 11

### --description--

你的代码无法编译，因为 `println!` 宏不知道如何格式化 `Args` 结构体。

任务：按照编译器的建议在 `println!` 中扩展格式化程序。

运行 `cargo test --bin combiner -- --show-output`。 你仍然应该看到一个错误。

### --seed--

```rust
// Lesson #11
fn main() {
  let args = Args { image_1: String::new() };
  println!("Hello, world!");
}

struct Args {
  image_1: String,
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let my_arg = Args { image_1: String::new() };
    println!("{}", my_arg);
  }
}
```

### --tests--

## 12

### --description--

编译器告诉你你正在尝试在一个不能实现 `Debug` 特性的类型上使用 `println!` 宏。

通常，需要使用 `impl` 关键字在结构体实现特性。 然而，在这种情况下，你可以使用 `derive` 属性来自动实现 `Debug` 特性：

```rust
    #[derive(Debug)]
    struct MyStruct {
      field_1: String,
    }
```

任务：为你的 `Args` 结构体实现 `Debug` 特性。

运行 `cargo test --bin combiner -- --show-output`。 你应该看到控制台打印了 `Args` 结构体。

### --seed--

```rust
// Lesson #12
fn main() {
  let args = Args { image_1: String::new() };
  println!("Hello, world!");
}

struct Args {
  image_1: String,
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let my_arg = Args { image_1: String::new() };
    println!("{:?}", my_arg);
  }
}
```

### --tests--

## 13

### --description--

你可能注意到 `main` 中使用的 `String::new()`。 `new` 函数是结构体的常见构造函数。 对于 `String`，它看起来像这样：

```rust
    impl String {
      fn new() -> Self {
        String::from("")
      }
    }
```

以上代码为 `String` 执行了 `new` 函数。 返回类型是 `Self`，是结构体类型。

任务：为 `Args` 执行 `new` 函数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #13
fn main() {
  let args = Args { image_1: String::new() };
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
}



fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let new_arg = Args::new();
    let my_arg = Args { image_1: String::new() };
    assert!(new_arg.image_1 == my_arg.image_1);
  }
}
```

### --tests--

## 14

### --description--

任务：不要在 `main` 中手动创建 `Args` 结构体，使用 `new` 函数创建结构体。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #14
fn main() {
  let args = Args { image_1: String::new() };
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
}

impl Args {
  fn new() -> Self {
    Args { image_1: String::new() }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_has_image_1_field() {
    let new_arg = Args::new();
    let my_arg = Args { image_1: String::new() };
    assert!(new_arg.image_1 == my_arg.image_1);
  }
  #[test]
  fn args_var_uses_new() {
    let file_content = return_file_in_src("main.rs");
    assert!(reg_with_con(r"let\s+args\s*=\s*Args::new\(\)", file_content));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 15

### --description--

任务：在 `new` 里面，不要分配空的 `String` 给 `image_1`，而是使用 `get_nth_arg` 函数分配第一个**有效的**参数的值。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #15
fn main() {
  let args = Args::new();
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
}

impl Args {
  fn new() -> Self {
    Args { image_1: String::new() }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn args_struct_has_image_1_field() {
    let new_arg = Args::new();
    let my_arg = Args { image_1: get_nth_arg(1) };
    assert!(new_arg.image_1 == my_arg.image_1);
  }

  #[test]
  fn image_1_defined_with_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"image_1:\s*get_nth_arg\(1\)", file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 16

### --description--

你的应用程序应该需要三个参数：`image_1`、`image_2` 和 `output`。

任务：将缺少的两个字段添加到 `Args` 结构体。 所有字段都应该使用相同的类型。

运行 `fcc test 16` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #16
fn main() {
  let args = Args::new();
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
}

impl Args {
  fn new() -> Self {
    Args { image_1: get_nth_arg(1) }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn args_struct_expects_three_fields() {
    let new_arg = Args::new();
    let my_arg = Args { image_1: get_nth_arg(1) };
    assert!(new_arg.image_1 == my_arg.image_1);
  }

  #[test]
  fn image_1_defined_with_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"image_1:\s*get_nth_arg\(1\)", file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

- 你应该定义 `Args` 有一个类型为 `String` 的 `image_1` 字段。
- `image_1:\s*String`
- 你应该定义 `Args` 有一个类型为 `String` 的 `image_2` 字段。
- `image_2:\s*String`
- 你应该定义 `Args` 有一个类型为 `String` 的 `output` 字段。
- `output:\s*String`

## 17

### --description--

任务：更新 `new` 函数以分配有效的值给所有预期字段。

运行 `cargo test --bin combiner -- --show-output` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #17
fn main() {
  let args = Args::new();
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
  image_2: String,
  output: String,
}

impl Args {
  fn new() -> Self {
    Args { image_1: get_nth_arg(1) }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_expects_three_fields() {
    let my_arg = Args { image_1: String::new(), image_2: String::new(), output: String::new() };
    println!("{:?}", my_arg);
  }
  #[test]
  fn image_1_defined_with_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"image_1:\s*get_nth_arg\(1\)", file_contents));
  }
  #[test]
  fn image_2_defined_with_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"image_2:\s*get_nth_arg\(2\)", file_contents));
  }
  #[test]
  fn output_defined_with_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"output:\s*get_nth_arg\(3\)", file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 18

### --description--

用参数测试你的应用程序现在应该打印出 `Args` 结构体，参数是字段的值。

任务：在 `main` 中更改 `println` 以打印 `args` 的值。

运行 `cargo run --bin combiner first_arg second_arg third_arg`。 如果你看到以下代码，那你就正确地完成了任务：

```bash
    Args { image_1: "first_arg", image_2: "second_arg", output: "third_arg" }
```

### --seed--

```rust
// Lesson #18
fn main() {
  let args = Args::new();
  println!("Hello, world!");
}

#[derive(Debug)]
struct Args {
  image_1: String,
  image_2: String,
  output: String,
}

impl Args {
  fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3),
    }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn args_struct_expects_three_fields() {
    let my_arg = Args { image_1: String::new(), image_2: String::new(), output: String::new() };
    println!("{:?}", my_arg);
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 19

### --description--

在你的 `main.rs` 文件过于混乱之前，你应该将参数逻辑移动到它自己的文件。

任务：创建文件 `combiner/src/args.rs`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #19
fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

#[derive(Debug)]
struct Args {
  image_1: String,
  image_2: String,
  output: String,
}

impl Args {
  fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3),
    }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  #[test]
  fn args_dot_rs_file_exists() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con(r"File does not exist", file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 20

### --description--

任务：将 `Args` 结构体和实现以及 `get_nth_arg` 函数移动到 `args.rs` 文件。 然后，注释掉 `main` 函数中的内容，这样你的应用程序就可以编译了。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #20
fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

#[derive(Debug)]
struct Args {
  image_1: String,
  image_2: String,
  output: String,
}

impl Args {
  fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3),
    }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
  #[test]
  fn args_dot_rs_file_exists() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con("File does not exist", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_args() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("struct Args", &file_contents));
    assert!(reg_with_con("impl Args", &file_contents));
    assert!(reg_with_con("fn get_nth_arg", &file_contents));
  }

  #[test]
  fn main_dot_rs_has_no_args() {
    let file_contents = return_file_in_src("main.rs");
    assert!(!reg_with_con(r"\nstruct Args", &file_contents));
    assert!(!reg_with_con(r"\nimpl Args", &file_contents));
    assert!(!reg_with_con(r"\nfn get_nth_arg", &file_contents));
  }
  #[test]
  fn main_dot_rs_has_main_fn() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con("fn main()", &file_contents));
  }
  #[test]
  fn main_dot_rs_code_commented_out() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"//\s*let args", &file_contents));
    assert!(reg_with_con(r"//\s*println!", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 21

### --description--

在 Rust 中，默认情况下，一切都是私密的。 因此，为了使函数或结构体公开，你可以使用 `pub` 关键字：

```rust
    pub MyStruct {}
```

任务：在 `args.rs` 内使结构体和函数公开。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #21
fn main() {
  // let args = Args::new();
  // println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  #[test]
  fn args_dot_rs_file_exists() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con("File does not exist", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_args() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("struct Args", &file_contents));
    assert!(reg_with_con("impl Args", &file_contents));
    assert!(reg_with_con("fn get_nth_arg", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_pub_fn() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("pub fn get_nth_arg()", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_pub_struct() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("pub struct Args", &file_contents));
  }
  #[test]
  fn args_dot_rs_not_has_pub_impl() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con("pub impl Args", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 22

### --description--

要使用外部文件的内容，需要声明它为模块：

```rust
    mod my_file_name
```

任务：在 `main.rs` 的顶端，声明 `args.rs` 文件为一个模块。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #22
fn main() {
  // let args = Args::new();
  // println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  #[test]
  fn main_declares_args_as_module() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con("(\n|^)mod args;", &file_contents));
  }
  #[test]
  fn args_dot_rs_file_exists() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con("File does not exist", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_args() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("struct Args", &file_contents));
    assert!(reg_with_con("impl Args", &file_contents));
    assert!(reg_with_con("fn get_nth_arg", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_pub_fn() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("pub fn get_nth_arg()", &file_contents));
  }
  #[test]
  fn args_dot_rs_has_pub_struct() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("pub struct Args", &file_contents));
  }
  #[test]
  fn args_dot_rs_not_has_pub_impl() {
    let file_contents = return_file_in_src("args.rs");
    assert!(!reg_with_con("pub impl Args", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 23

### --description--

现在 `args` 已被声明为在 `main.rs` 内使用的模块，你可以使用 `use` 关键字导入 `Args` 结构体。

任务：在 `main.rs` 内，导入 `Args` 结构体。 然后，在 `main` 函数内取消注释被注释掉的代码。

运行 `cargo test --bin combiner`。 你应该看到一个错误。

### --seed--

```rust
// Lesson #23
mod args;

fn main() {
  // let args = Args::new();
  // println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  #[test]
  fn main_declares_args_as_module() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con("(\n|^)mod args;", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 24

### --description--

发生错误是因为为 `Args` 执行的 `new` 函数不是公开的。

任务：在 `args.rs` 内声明 `new` 函数为公开的。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #24
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  #[test]
  fn main_declares_args_as_module() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con("(\n|^)mod args;", &file_contents));
  }

  #[test]
  fn args_dot_rs_declares_new_as_pub() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con("pub fn new()", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 25

### --description--

为了对图像进行编码和解码，你将使用 `image` crate。

任务：在 root `Cargo.toml` 内，添加以下代码到 `dependencies` 部分：

```rust
    image = "0.23.14"
```

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #25
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  #[test]
  fn image_crate_is_in_root_cargo_toml() {
    use image;
    assert!(true, "If there is an error, the crate has not been correctly added")
  }
}
```

### --tests--

## 26

### --description--

任务：在 `main.rs` 中，定义一个名为 `find_image_from_path` 的函数，以 `String` 作为一个参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #26
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_is_defined() {
    assert_eq!(find_image_from_path("".to_string()), ());
  }
}
```

### --tests--

## 27

### --description--

编译器显示了一个警告，`find_image_from_path` 函数未被使用。 在这个项目的进行过程中，这将令人烦恼。 幸运的是，你可以启用全局属性来抑制警告。

全局属性使用语法 `#![feature(feature_name)]`，并且应该被放在文件的顶部。

任务：在 `main.rs`内，使用 `allow` 功能全局启用 `unused_variables` 和 `dead_code`。

运行 `cargo test --bin combiner`。 如果你没有看到警告，说明你已经成功地完成了这一节课程。

### --seed--

```rust
// Lesson #27
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) {

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_is_defined() {
    assert_eq!(find_image_from_path("".to_string()), ());
  }
}
```

### --tests--

## 28

### --description--

任务：从 `image::io` 导入 `Reader` 结构体，并在 `find_image_from_path` 内分配 `Reader::open` 函数展开的值，传递 `path` 作为参数，给一个名为 `image_reader` 的变量。 然后，返回 `image_reader`。

提示：遵循编译器的建议，并导入必要的类型。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #28
#![allow(unused_variables, dead_code)]
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) {

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_reader() {
    use std::{fs::File, io::BufReader};
    let _image_reader: Reader<BufReader<File>> =
      find_image_from_path("./images/fcc_glyph.png".to_string());
  }
}
```

### --tests--

## 29

### --description--

任务：在 `find_image_from_path` 内，将 `image_reader` 上 `format` 方法的解包的值分配给一个名为 `image_format` 的变量，并返回它。

提示：遵循编译器的建议，并导入必要的类型。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #29
#![allow(unused_variables, dead_code)]
mod args;
use std::{fs::File, io::BufReader};

use args::Args;
use image::io::Reader;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) -> Reader<BufReader<File>> {
  let image_reader = Reader::open(path).unwrap();
  image_reader
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_format() {
    use image::ImageFormat;
    let image_format: ImageFormat = find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(ImageFormat::Png, image_format);
  }
  #[test]
  fn image_reader_var_declared() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"let\s+image_reader", file_contents));
  }
  #[test]
  fn image_format_var_declared() {
    let file_contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"let\s+image_format", file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 30

### --description--

任务：从 `main.rs` 中删除未使用的导入。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #30
#![allow(unused_variables, dead_code)]
mod args;
use std::{fs::File, io::BufReader};

use args::Args;
use image::{io::Reader, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) -> ImageFormat {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  image_format
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_format() {
    use image::ImageFormat;
    let image_format: ImageFormat = find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(ImageFormat::Png, image_format);
  }
  #[test]
  fn std_lib_not_imported() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(!reg_with_con(r"use\s+std::", file_contents));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 31

### --description--

到目前为止，你还没有解码图像。 `Reader` 有一个 `decode` 方法，在 `Result` 中返回一个 `DynamicImage`。

任务：在 `find_image_from_path` 内，将 `image_reader` 上 `decode` 方法的解包的值分配给一个名为 `image` 的变量。 然后，返回 `image`。

_提示：_遵循编译器的建议，并导入必要的类型。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #31
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) -> ImageFormat {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  image_format
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_image() {
    use image::DynamicImage;
    let _image: DynamicImage = find_image_from_path("./images/fcc_glyph.png".to_string());
  }
  #[test]
  fn image_var_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let image", file_contents));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 32

### --description--

你已经学习了空元组类型 `()`。 现在，你将使用元组返回多个值。 与其他类型不同，元组可以包含多个类型。

```rust
    // The Vec type can only contain one type.
    let my_vec = vec![1u8, 2u16, 3u32];
    // Tuples can contain multiple types.
    let my_tuple = (1u8, 2u16, 3u32);
```

任务：从 `find_image_from_path` 返回一个包含 `DynamicImage` 和 `ImageFormat` 的元组，按该顺序排列。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #32
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) -> DynamicImage {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  image
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_a_tuple() {
    use image::{DynamicImage, ImageFormat};
    let (_image, image_format): (DynamicImage, ImageFormat) =
      find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
}
```

### --tests--

## 33

### --description--

元组可以被解构为这样的变量：

```rust
    let (x, y, z) = (1, 2, 3);
```

任务：在 `main` 内，解构从 `find_image_from_path` 返回的元组为变量 `image_1` 和 `image_1_format`。 你应该调用 `find_image_from_path`，传入 `args` 的 `image_1` 字段的值。

运行 `cargo test --bin combiner`。 你应该看到一个错误。

### --seed--

```rust
// Lesson #33
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_a_tuple() {
    use image::{DynamicImage, ImageFormat};
    let (_image, image_format): (DynamicImage, ImageFormat) =
      find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
  #[test]
  fn find_image_from_path_called_with_args() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"find_image_from_path\(args.image_1\)",
        file_contents
      ));
    }
  }
  #[test]
  fn tuple_destructured() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+\(\s*image_1\s*,\s*image_1_format\s*\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

```

### --tests--

## 34

### --description--

你的代码有一个错误，因为 `args` 的 `image_1` 字段不是公开的。 因此，它可能不会被用于不同模块。

任务：在 `args.rs` 内，更改所有 `Args` 结构体的字段为公开。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #34
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_a_tuple() {
    use image::{DynamicImage, ImageFormat};
    let (_image, image_format): (DynamicImage, ImageFormat) =
      find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
  #[test]
  fn find_image_from_path_called_with_args() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"find_image_from_path\(args.image_1\)",
        file_contents
      ));
    }
  }
  #[test]
  fn tuple_destructured() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+\(\s*image_1\s*,\s*image_1_format\s*\)",
        file_contents
      ));
    }
  }
  #[test]
  fn all_args_struct_fields_public() {
    let file_contents = return_file_in_src("args.rs");
    assert!(reg_with_con(r"pub\s+image_1:\s*String", &file_contents));
    assert!(reg_with_con(r"pub\s+image_2:\s*String", &file_contents));
    assert!(reg_with_con(r"pub\s+output:\s*String", &file_contents));
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

```

### --tests--

## 35

### --description--

任务：在 `main` 内，解构从 `find_image_from_path` 返回的元组为变量 `image_2` 和 `image_2_format`。 你应该调用 `find_image_from_path`，传入 `args` 的 `image_2` 字段的值。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #35
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_image_from_path_returns_a_tuple() {
    use image::{DynamicImage, ImageFormat};
    let (_image, image_format): (DynamicImage, ImageFormat) =
      find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
  #[test]
  fn find_image_from_path_called_with_args() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"find_image_from_path\(args.image_2\)",
        file_contents
      ));
    }
  }
  #[test]
  fn tuple_destructured() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+\(\s*image_2\s*,\s*image_2_format\s*\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 36

### --description--

到目前为止，你已经处理过一些返回一个 `Result` 的函数。 现在，你将创建一个新的 `Result`。

`Result` 是一种类型，可以是 `Ok` 或者 `Err`。 通常在函数运行成功时返回一个空元组，并在函数运行失败时返回一个错误信息：

```rust
    fn function_returns_result() -> Result<(), String> {
      if (condition) {
        return Ok(());
      } else {
        return Err(String::from("Error message"));
      }
    }
```

任务：在 `main.rs` 中，转换 `main` 函数返回一个 `Result`。 对于 `Ok` 上返回一个空元组，而对于 `Err` 类型返回 `String`。

### --seed--

```rust
// Lesson #36
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_result() {
    let _res: Result<(), String> = main();
  }
  #[test]
  fn no_err_condition() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(!reg_with_con(r"Err\(", file_contents));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 37

### --description--

你的应用程序将只能合并两个同样类型的图像。

任务：如果 `image_1_format` 不等于 `image_2_format`，则返回一个错误消息。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #37
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);
  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_result() {
    let _res: Result<(), String> = main();
  }
  #[test]
  fn err_condition() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"Err\(", file_contents));
    }
  }
  #[test]
  fn image_format_comparison() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(
        reg_with_con(r"image_1_format\s*!=\s*image_2_format", file_contents)
          | reg_with_con(r"image_2_format\s*!=\s*image_1_format", file_contents)
          | reg_with_con(r"else\s*\{\s*(return\s+)?Err\(", file_contents)
      );
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 38

### --description--

当两张图像大小不同时，可能发生另一个错误。 幸运的是，有一个能够用来调整图像大小的函数。

任务：先创建一个名为 `standardise_size` 的函数，这个函数以 `image_1` 和 `image_2` 作为参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #38
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn standardise_size_defined() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"fn\s+standardise_size", file_contents));
    }
  }
  #[test]
  fn standardise_size_accepts_correct_args() {
    let image_1 = DynamicImage::new_rgb8(1, 1);
    let image_2 = DynamicImage::new_rgb8(1, 1);
    let _res = standardise_size(image_1, image_2);
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

```

### --tests--

## 39

### --description--

为了更简单地合并图像，你可以将最大的图像缩放到最小图像的大小。 为了做到这一点，你需要得到两个图像中最小的尺寸。

任务：创建一个名为 `get_smallest_dimensions` 的函数，这个函数需要两个元组作为参数。 每个元组应该包含两个元素，类型为 `u32`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #39
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn get_smallest_dimensions_defined() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"fn\s+get_smallest_dimensions", file_contents));
    }
  }
  #[test]
  fn get_smallest_dimensions_accepts_correct_args() {
    let dim_1 = (0u32, 0u32);
    let dim_2 = (1u32, 1u32);
    let _res = get_smallest_dimensions(dim_1, dim_2);
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

```

### --tests--

## 40

### --description--

在 `get_smallest_dimensions` 内，你需要返回最小像素数的尺寸。 像素数量是宽度和高度的结合。

任务：如果 `dim_1` 中的像素数小于 `dim_2` 中的像素数，则返回 `dim_1`。 否则返回 `dim_2`。

请记住，你可以使用点符号来访问元组的元素。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #40
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) {}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) {

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn get_smallest_dimensions_defined() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"fn\s+get_smallest_dimensions", file_contents));
    }
  }
  #[test]
  fn get_smallest_dimensions_accepts_correct_args() {
    let dim_1 = (0u32, 0u32);
    let dim_2 = (1u32, 1u32);
    let _res = get_smallest_dimensions(dim_1, dim_2);
  }
  #[test]
  fn get_smallest_dimensions_returns_correct_dim() {
    let smaller_tup = (10, 10);
    let larger_tup = (10, 11);
    assert_eq!(get_smallest_dimensions(smaller_tup, larger_tup), (10, 10));
    assert_eq!(get_smallest_dimensions(larger_tup, smaller_tup), (10, 10));
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}

```

### --tests--

## 41

### --description--

任务：在 `standardise_size` 内，解构从 `get_smallest_dimensions` 返回的元组为两个变量 `width` 和 `height`。 使用在每个 `DynamicImage` 上调用 `dimensions` 的返回值，作为参数传递给 `get_smallest_dimensions`。

_提示：_遵循编译器的建议，以获取图像的尺寸。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #41
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) {

}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn width_and_height_destructured() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+\(\s*width\s*,\s*height\s*\)",
        file_contents
      ));
    }
  }
  #[test]
  fn image_dimensions_method_used() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"image_1\.dimensions\(\)", file_contents));
      assert!(reg_with_con(r"image_2\.dimensions\(\)", file_contents));
    }
  }
  #[test]
  fn generic_image_view_imported() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"GenericImageView", file_contents));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 42

### --description--

任务：在 `standardise_size` 内，打印 `width` 和 `height` 到控制台。

运行 `cargo test --bin combiner -- --show-output`。 如果你看到 `'width: 10, height: 10'` 打印到控制台，就说明你已经成功完成了任务。

### --seed--

```rust
// Lesson #42
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn width_and_height_print() {
    let image_1 = DynamicImage::new_rgb8(10, 11);
    let image_2 = DynamicImage::new_rgb8(10, 10);
    let _res = standardise_size(image_1, image_2);
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 43

### --description--

任务：在 `standardise_size` 内， 写一个 `if` 语句来检查 `image_2` 的尺寸是否等于先前确定的最小尺寸。 如果等于，返回一个包含 `image_1` 和 `image_2` 的元组。 否则，返回相同的元组。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #43
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn image_2_dims_compared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(
        reg_with_con(
          r"image_2\.dimensions\(\)\s*==\s*\(\s*width\s*,\s*height\s*\)",
          file_contents
        ) | reg_with_con(
          r"\(\s*width\s*,\s*height\s*\)\s*==\s*image_2\.dimensions\(\)",
          file_contents
        ) | (reg_with_con(r"image_2\.dimensions\(\)\.0\s*==\s*width", file_contents)
          & reg_with_con(r"image_2\.dimensions\(\)\.1\s*==\s*height", file_contents))
      );
    }
  }
  #[test]
  fn standardise_size_returns_dynamic_image_tuple() {
    let image_1 = DynamicImage::new_bgr8(8, 9);
    let image_2 = DynamicImage::new_bgr8(11, 1);
    let _res: (DynamicImage, DynamicImage) = standardise_size(image_1, image_2);
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 44

### --description--

你应该调整较大的图像的大小，而不是返回图像不作改变。 你可以使用 `DynamicImage` 结构体上的 `resize_exact` 方法。 `resize_exact` 方法的形式如下：

```rust
    image_to_resize.resize_exact(new_width: u32, new_height: u32, filter: image::imageops::FilterType);
```

任务：在 `standardise_size` 内，使用 `Triangle` 过滤器将正确的图像变量调整为正确的尺寸。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #44
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1, image_2)
  } else {
    (image_1, image_2)
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn standardise_size_returns_resized_images() {
    let image_1 = DynamicImage::new_bgr8(8, 9);
    let image_2 = DynamicImage::new_bgr8(11, 1);
    let (image_1, image_2): (DynamicImage, DynamicImage) = standardise_size(image_1, image_2);
    assert_eq!(image_1.dimensions(), (11, 1));
    assert_eq!(image_2.dimensions(), (11, 1));
  }
  #[test]
  fn standardise_size_correct_image() {
    let image_1 = DynamicImage::new_bgr8(10, 11);
    let image_2 = DynamicImage::new_bgr8(10, 10);
    let (image_1, image_2) = standardise_size(image_1, image_2);
    assert_eq!(image_1.dimensions(), (10, 10));
    assert_eq!(image_2.dimensions(), (10, 10));
  }
}
```

### --tests--

## 45

### --description--

任务：在 `main` 内，在 `Ok` 返回之前，使用 `standardise_size` 函数重新声明 `image_1` 和 `image_2` 为正确的大小。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #45
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  #[test]
  fn main_calls_standardise_size() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"standardise_size\(\s*image_1\s*,\s*image_2\s*\)",
        file_contents
      ));
    }
  }
  #[test]
  fn image_vars_redeclared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+\(\s*image_1\s*,\s*image_2\s*\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 46

### --description--

为了处理输出，可以创建一个临时的结构体来保存输出图像的元数据。

任务：创建一个名为 `FloatingImage` 的结构体，其中包含以下字段：

```rust
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
```

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #46
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_exists() {
    struct _A {
      a: FloatingImage,
    }
  }
  #[test]
  fn floating_image_has_all_fields() {
    let _float = FloatingImage {
      width: 1u32,
      height: 2u32,
      data: vec![1u8],
      name: "test".to_string(),
    };
  }
}
```

### --tests--

## 47

### --description--

任务：为 `FloatingImage` 写一个名为 `new` 的函数。 `new` 函数应该有三个参数：`width: u32`、`height: u32`、`name: String`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #47
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_impl_new() {
    let _float = FloatingImage::new(1u32, 2u32, "test".to_string());
  }
}

```

### --tests--

## 48

### --description--

为了有效地将合并图像的数据写入输出图像， 你需要创建一个足够大的缓冲区来保存数据，因此不需要分配额外的空间。

大图像可能有大量的数据，因此你可以利用 Rust 的易读编号， 它将数字分成每三个数字为一组：

```rust
    let difficult_to_read_number = 1325364955;
    let easy_to_read_number = 1_325_364_955;
```

任务：在 `new` 内，声明一个名为 `buffer_capacity` 的变量，并使用易读号码分配值 `3655744` 给它。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #48
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) {

  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_impl_new() {
    let _float = FloatingImage::new(1u32, 2u32, "test".to_string());
  }
  #[test]
  fn buffer_capacity_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+buffer_capacity",
        file_contents
      ));
    }
  }
  #[test]
  fn buffer_capacity_assigned() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"=\s+3_655_744",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 49

### --description--

现在你有一个缓冲尺寸了，你需要创建一个 `Vec<u8>` 的缓冲区。 `Vec` 结构体实现了一个 `with_capacity` 函数。它接收一个容量作为参数，并返回一个具有该容量的新 `Vec`。

任务：在 `new` 里面，声明一个名为 `buffer` 的变量，并将传入 `buffer_capacity` 调用 `with_capacity` 函数的值赋给它。

_提示：_遵循编译器的建议，键入 `buffer` 变量。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #49
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) {
    let buffer_capacity = 3_655_744;
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_impl_new() {
    let _float = FloatingImage::new(1u32, 2u32, "test".to_string());
  }
  #[test]
  fn buffer_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+buffer",
        file_contents
      ));
    }
  }
  #[test]
  fn buffer_assigned() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"=\s*Vec::with_capacity\(\s*buffer_capacity\s*\)",
        file_contents
      ));
    }
  }
  #[test]
  fn buffer_typed() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"let\s+buffer\s*:\s*Vec<u8>",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 50

### --description--

任务：在 `new` 里面，使用可用的变量返回 `FloatingImage` 结构体的实例。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #50
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_width() {
    let float = FloatingImage::new(1u32, 2u32, "test".to_string());
    assert_eq!(float.width, 1);
  }
  #[test]
  fn floating_image_struct_height() {
    let float = FloatingImage::new(0u32, 10u32, "test".to_string());
    assert_eq!(float.height, 10);
  }
  #[test]
  fn floating_image_struct_name() {
    let float = FloatingImage::new(0u32, 0u32, "output".to_string());
    assert_eq!(float.name, String::from("output"));
  }
  #[test]
  fn floating_image_struct_data() {
    let float = FloatingImage::new(0u32, 0u32, "test".to_string());
    assert_eq!(float.data.capacity(), 3_655_744);
  }
}
```

### --tests--

## 51

### --description--

任务：在 `main` 里面，声明一个新变量 `output`，使用 `FloatingImage` 结构体的 `new` 函数。 使用 `image_1` 变量的 `width` 和 `height` 方法作为前两个参数，`args` 变量的 `output` 字段作为第三个参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #51
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn floating_image_struct_width() {
    let float = FloatingImage::new(1u32, 2u32, "test".to_string());
    assert_eq!(float.width, 1);
  }
  #[test]
  fn floating_image_struct_height() {
    let float = FloatingImage::new(0u32, 10u32, "test".to_string());
    assert_eq!(float.height, 10);
  }
  #[test]
  fn floating_image_struct_name() {
    let float = FloatingImage::new(0u32, 0u32, "output".to_string());
    assert_eq!(float.name, String::from("output"));
  }
  #[test]
  fn floating_image_struct_data() {
    let float = FloatingImage::new(0u32, 0u32, "test".to_string());
    assert_eq!(float.data.capacity(), 3_655_744);
  }
  #[test]
  fn output_var_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let\s+output", file_contents));
      assert!(reg_with_con(
        r"FloatingImage::new\(\s*image_1\.width\(\)\s*,\s*image_1\.height\(\)\s*,\s*args\.output",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 52

### --description--

任务：定义一个名为 `combine_images` 的函数，这个函数以两个 `DynamicImage` 作为参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #52
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn combine_images_defined() {
    let image_1 = DynamicImage::new_bgr8(10, 10);
    let image_2 = DynamicImage::new_bgr8(10, 10);
    let img = combine_images(image_1, image_2);
    assert_eq!(img, ());
  }
}
```

### --tests--

## 53

### --description--

为了处理图像，你将把它们转换成 RGBA 像素的矢量。 像素储存为 `u8`，因为它们的值在 0 到 255 之间。

`DynamicImage` 结构体执行 `to_rgba8` 方法，返回一个 `ImageBuffer`，其中包含 `Vec<u8>`， `ImageBuffer` 执行 `into_vec` 方法，返回 `Vec<u8>`。

任务：在 `combine_images` 里面，声明一个变量 `vec_1`，并使用上述方法给它分配 `Vec<u8>`。 返回 `vec_1`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #53
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) {

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn combine_images_defined() {
    let image_1 = DynamicImage::new_bgr8(2, 2);
    let image_2 = DynamicImage::new_bgr8(2, 2);
    let img: Vec<u8> = combine_images(image_1, image_2);
    assert_eq!(
      img,
      vec![0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]
    );
  }
}
```

### --tests--

## 54

### --description--

任务：与上一节课相同，但是在 `image_2` 上操作，返回名为 `vec_2` 而不是 `vec_1` 的新变量。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #54
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  vec_1
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn combine_images_return_vec_2() {
    let image_1 = DynamicImage::new_bgr8(2, 2);
    let image_2 = DynamicImage::new_bgr8(1, 1);
    let img = combine_images(image_1, image_2);
    assert_eq!(img, vec![0, 0, 0, 255]);
  }
}
```

### --tests--

## 55

### --description--

现在你有了在 `vec_1` 和 `vec_2` 中每个图像的像素值，你可以将它们合并为一个图像。

任务：定义一个名为 `alternate_pixels` 的函数，这个函数以两个 `Vec<u8>` 作为参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #55
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_defined() {
    let vec_1: Vec<u8> = vec![0, 0, 0, 0];
    let vec_2: Vec<u8> = vec![1, 1, 1, 1];
    alternate_pixels(vec_1, vec_2);
  }
}
```

### --tests--

## 56

### --description--

你需要将合并的图像像素数据存储在一个变量中。 要创建此变量，你可以使用 `vec` 宏，提供矢量的类型和长度：

```rust
    let my_vec = vec![10u8; 5];
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec, [10, 10, 10, 10, 10]);
```

任务：在 `alternate_pixels` 里面，声明一个变量 `combined_data`，并使用 `vec` 宏创建一个 `Vec<u8>` 为 `0`，与 `vec_1` 长度相同。 返回 `combined_data`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #56
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_defined() {
    let vec_1: Vec<u8> = vec![0, 0, 0, 0];
    let vec_2: Vec<u8> = vec![1, 1, 1, 1];
    alternate_pixels(vec_1, vec_2);
  }
  #[test]
  fn alternate_pixels_returns_combined_data() {
    let vec_1 = vec![0, 1, 2, 3, 4];
    let vec_2 = vec![4, 3, 2, 1];
    let a = alternate_pixels(vec_1.clone(), vec_2);
    assert_eq!(a.len(), vec_1.len());
    assert_eq!(a, [0u8, 0u8, 0u8, 0u8, 0u8]);
  }
}
```

### --tests--

## 57

### --description--

要在矢量中遍历像素，你将使用 `while` 循环。 `while` 循环的语法是：

```rust
    while condition {
      // Do stuff
    }
```

`condition` 是一个布尔表达式，它能评估 `true` 或 `false`。

任务：在 `alternate_pixels` 内，返回 `combined_data` 之前，声明一个变量 `i`，并为它分配值 `0`。 然后，声明一个 `while` 循环，在 `i` 小于 `vec_1` 长度时运行。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #57
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  combined_data
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_defined() {
    let vec_1: Vec<u8> = vec![];
    let vec_2: Vec<u8> = vec![];
    alternate_pixels(vec_1, vec_2);
  }
  #[test]
  fn alternate_pixels_returns_combined_data() {
    let vec_1 = vec![];
    let vec_2 = vec![];
    let a = alternate_pixels(vec_1.clone(), vec_2);
    assert_eq!(a.len(), vec_1.len());
    assert_eq!(a, []);
  }
  #[test]
  fn i_var_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let\s+i\s*=\s*0", file_contents));
    }
  }
  #[test]
  fn while_loop_defined() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"while\s+i\s*<\s*vec_1\.len\(\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 58

### --description--

为了正确设置输出向量的 RGBA 像素集，你将用正确的值替换 `0u8` 值。

任务：定义一个名为 `set_rgba` 的函数，这个函数接收 3 个参数：一个 `Vec<u8>` 和两个 `usize`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #58
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}
  combined_data
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_defined() {
    let _a = set_rgba(vec![0u8], 0usize, 3usize);
  }
}
```

### --tests--

## 59

### --description--

任务：在 `set_rgba` 内，定义一个可变变量，名为 `rgba`，并将其分配为空的 `Vec<u8>`。 试着不使用 `vec` 宏来实现它。 然后，返回 `rgba`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #59
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) {

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_empty_vec() {
    let a = set_rgba(vec![0u8], 0usize, 3usize);
    assert_eq!(a, Vec::new());
  }
  #[test]
  fn rgba_is_mut() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let\s+mut\s+rgba", file_contents));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 60

### --description--

要对一个范围的值进行迭代，你可以使用 `for...in` 循环，使用_右包含_运算符：

```rust
    for i in 1..=5 {
      println!("{}", i);
    }
```

在范围内的 `=` 是_右包含_运算符，这意味着末尾会被包含在内。

任务：在 `set_rgba` 内，返回 `rgba` 之前，遍历范围 `start..=end`，并将范围内的每个值推到 `rgba`。

提示：你将需要断言被推到 `rgba` 的值的正确类型。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #60
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();

  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_start_to_end() {
    let a = set_rgba(vec![0u8], 0usize, 3usize);
    assert_eq!(a, vec![0, 1, 2, 3]);
  }
}
```

### --tests--

## 61

### --description--

有时候从矢量内部检索值会引起 panic，因为索引超出范围。 为了避免这种情况，你可以在矢量上使用 `get` 方法：

```rust
    let my_vec = vec![1, 2, 3];
    assert_eq!(my_vec.get(0), Some(&1));
    assert_eq!(my_vec.get(3), None);
```

`get` 方法返回对给定索引的值的引用，如果索引超出范围，则返回 `None`。

任务：在 `set_rgba` 内，在 `for` 循环中，声明一个变量 `val`，为 `vec` 上的 `get` 方法的 `match`，使用 `i` 作为参数。

对于 `Some`，赋值 `val`。 对于 `None`，返回 panic。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #61
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {

    rgba.push(i as u8);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_start_to_end() {
    let a = set_rgba(vec![0u8, 1u8, 2u8, 3u8, 4u8], 0usize, 3usize);
    assert_eq!(a, vec![0, 1, 2, 3]);
  }
  #[test]
  #[should_panic]
  fn set_rgba_func_panics_on_invalid_index() {
    let a = set_rgba(vec![10u8, 12u8, 32u8, 34u8, 54u8], 0usize, 5usize);
  }
}
```

### --tests--

## 62

### --description--

任务：在 `set_rgba` 中，更改推到 `vec` 的值为 `val`。

运行 `cargo test --bin combiner`。 你应该看到一个错误。

### --seed--

```rust
// Lesson #62
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(i as u8);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_start_to_end() {
    let a = set_rgba(vec![0u8, 1u8, 2u8, 3u8, 4u8], 0usize, 3usize);
    assert_eq!(a, vec![0, 1, 2, 3]);
  }
  #[test]
  #[should_panic]
  fn set_rgba_func_panics_on_invalid_index() {
    let a = set_rgba(vec![10u8, 12u8, 32u8, 34u8, 54u8], 0usize, 5usize);
  }
}
```

### --tests--

## 63

### --description--

出现错误是因为 `val` 的类型是 `&u8` - 引用 8 位无符号整数。 然而， `vec` 的类型应该是 `Vec<u8>`，而不是 `Vec<&u8>`。

要解决这个问题，从 `get` 方法返回的值可以_解引用_。 通过给值加上 `*` 来解引用：

```rust
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
```

任务：在 `set_rgba` 内，解引用分配给 `val` 的值。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #63
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {}

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_rgba_func_returns_start_to_end() {
    let a = set_rgba(vec![0u8, 1u8, 2u8, 3u8, 4u8], 0usize, 3usize);
    assert_eq!(a, vec![0, 1, 2, 3]);
  }
  #[test]
  #[should_panic]
  fn set_rgba_func_panics_on_invalid_index() {
    let a = set_rgba(vec![10u8, 12u8, 32u8, 34u8, 54u8], 0usize, 5usize);
  }
}
```

### --tests--

## 64

### --description--

现在，当 `vec` 包含有元素时，`while` 循环有可能永久运行下去。 你可以通过在每次迭代时递增 `i` 来修复这个问题。 下面是一些常见的递增整数的方式：

```rust
    let mut a = 0;
    a += 1;
    a++;
    a = a + 1;
    assert_eq!(a, 3);
```

任务：在 `alternate_pixels` 内，在 `while` 循环中，给 `i` 递增 `4`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #64
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let i = 0;
  while i < vec_1.len() {

  }
  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_not_infinite() {
    let _a = alternate_pixels(vec![1, 2, 3], vec![3, 2, 1]);
  }
  #[test]
  fn i_incremented_by_four() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"i\s*\+=\s*4",
        file_contents
      ) | reg_with_con(
        r"i\s*=\s*i\s*\+\s*4",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 65

### --description--

你有一个 `Vec<u8>` `0` 和两个 `Vec<u8>` `0-255`。 要将矢量的一个切片替换为另一个切片，你可以使用 `splice` 方法：

```rust
    let original_vec = vec![0, 1, 2, 3];
    let mut vec_to_change = vec![0u8; 4];
    vec_to_change.splice(2..4, original_vec[2..4].iter().cloned());
    assert_eq!(vec_to_change, vec![0, 0, 2, 3]);
```

`splice` 方法需要两个参数：要替换的矢量范围和替换后的值。

任务：在 `alternate_pixels` 内，在 `while` 循环中，对 `combined_data` 使用 `splice` 方法，从 `i` 到 `i+3`，使用 `set_rgba` 函数从 `vec_1` 插入正确的值。

运行 `cargo test --bin combiner` 应该产生一个错误。 运行 `fcc test 65` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #65
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_not_infinite() {
    let _a = alternate_pixels(vec![1, 2, 3], vec![3, 2, 1]);
  }
}
```

### --tests--

- 你不应更改 `while` 循环条件。
- `while\s+i\s*<\s*vec_1\.len\(\)\s*\{`
- 你应该在循环结束时给 `i` 递增 4。
- `i\s*\+=\s*4;\s*\}`
- 你应该在 `combined_data` 调用 `splice` 方法。
- `combined_data\.splice\(`
- 你应该将范围 `i..i+4` 或 `i..=i+3` 传递给 `splice` 方法的第一个参数。
- `splice\(\s*(i..i\s*\+\s*4)|(i..=i\s*\+\s*3)\s*,`
- 你应该将 `set_rgba(vec_1, i, i+3)` 传递给 `splice` 作为第二个参数。
- `splice\(\s*(i..i+4)|(i..=i\s*\+\s*3)\s*,\s*set_rgba\(\s*vec_1\s*,\s*i\s*,\s*i\s*\+\s*3\s*\)\s*\)`

## 66

### --description--

错误是说 `vec_1` 值在循环第一次迭代时被移动到 `set_rgba`。 所以，在第二次迭代时，当 `while` 条件应该计算 `i < vec_1.len()` 时，`vec_1` 不在使用范围内。

任务：在 `alternate_pixels` 内，修复问题，通过传递一个 `vec_1` 的引用到 `set_rgba`，并修复必要的类型注释。

### --seed--

```rust
// Lesson #66
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    combined_data.splice(i..=i + 3, set_rgba(vec_1, i, i + 3));
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_not_infinite() {
    let _a = alternate_pixels(vec![1, 2, 3, 4], vec![4, 3, 2, 1]);
  }
  #[test]
  fn alternate_pixels_returns_spliced_data() {
    let vec_1 = vec![0, 2, 4, 6];
    let vec_2 = vec![1, 3, 5, 7];
    let a = alternate_pixels(vec_1, vec_2);
    assert_eq!(a, vec![0, 2, 4, 6]);
  }
  #[test]
  fn set_rgba_expects_vec_ref() {
    let a = set_rgba(&vec![1, 2, 3, 4], 2, 3);
    assert_eq!(a, vec![3, 4]);
  }
}
```

### --tests--

## 67

### --description--

目前，`alternate_pixels` 正在将每一个 RGBA 集从 `vec_1` 切片到 `combined_data`。 然而，你想要每个第二个集合是来自 `vec_2`。 要实现这一点，你可以使用取模运算符：

```rust
    let mut my_vec = vec![0u8; 6];
    for i in 0..6 {
      if i % 2 == 0 {
        my_vec[i] = 2
      } else {
        my_vec[i] = 1
      }
    }
    assert_eq!(my_vec, vec![2, 1, 2, 1, 2, 1]);
```

任务：在 `alternate_pixels` 内，使用取模运算符将 RGBA 值的每一个第二个集合从 `vec_2` 切片到 `combined_data`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #67
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn alternate_pixels_not_infinite() {
    let _a = alternate_pixels(vec![1, 2, 3, 4], vec![4, 3, 2, 1]);
  }
  #[test]
  fn alternate_pixels_returns_spliced_data() {
    let vec_1 = vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
    let vec_2 = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23];
    let a = alternate_pixels(vec_1, vec_2);
    assert_eq!(a, vec![0, 2, 4, 6, 9, 11, 13, 15, 16, 18, 20, 22]);
  }
  #[test]
  fn set_rgba_expects_vec_ref() {
    let a = set_rgba(&vec![1, 2, 3, 4], 2, 3);
    assert_eq!(a, vec![3, 4]);
  }
}
```

### --tests--

## 68

### --description--

任务：在 `combine_images` 内，不返回 `vec_2`，而是返回调用 `alternate_pixels` 并传入 `vec_1` 和 `vec_2` 的结果。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #68
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();
  vec_2
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use image::RgbaImage;

  use super::*;
  #[test]
  fn combine_images_returns_spliced_data() {
    let mut image_1 = RgbaImage::new(2, 2);
    image_1.put_pixel(0, 0, image::Rgba([0, 2, 4, 6]));
    image_1.put_pixel(1, 0, image::Rgba([8, 10, 12, 14]));
    image_1.put_pixel(0, 1, image::Rgba([16, 18, 20, 22]));
    image_1.put_pixel(1, 1, image::Rgba([24, 26, 28, 30]));
    let mut image_2 = RgbaImage::new(2, 2);
    image_2.put_pixel(0, 0, image::Rgba([1, 3, 5, 7]));
    image_2.put_pixel(1, 0, image::Rgba([9, 11, 13, 15]));
    image_2.put_pixel(0, 1, image::Rgba([17, 19, 21, 23]));
    image_2.put_pixel(1, 1, image::Rgba([25, 27, 29, 31]));
    let a = combine_images(
      DynamicImage::ImageRgba8(image_1),
      DynamicImage::ImageRgba8(image_2),
    );
    assert_eq!(
      a,
      vec![0, 2, 4, 6, 9, 11, 13, 15, 16, 18, 20, 22, 25, 27, 29, 31]
    );
  }
}
```

### --tests--

## 69

### --description--

任务：在 `main` 内，声明一个变量 `combined_data`，把调用 `combine_images` 并传入 `image_1` 和 `image_2` 的值赋值给它。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #69
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn combined_data_var_declared() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"let\s+combined_data", file_contents));
    }
  }
  #[test]
  fn combine_images_func_called() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"combine_images\(\s*image_1\s*,\s*image_2\s*\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 70

### --description--

现在，你想要将 `combined_data` 的数据设置为 `output` 图像。 要做到这一点，你将在 `FloatingImage` 上定义一个方法来设置 `output` 的 `data` 字段为 `combined_data` 的值。

到目前为止，你只在结构体上执行了函数。 方法也是通过类似方法定义的，但它们将结构体的一个实例作为它们的第一个参数：

```rust
    struct MyStruct {
      name: String,
    }
    impl MyStruct {
      fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
      }
    }

    let mut my_struct = MyStruct { name: String::from("Shaun") };
    assert_eq!(my_struct.name, "Shaun".to_string);
    my_struct.change_name("Tom");
    assert_eq!(my_struct.name, "Tom".to_string);
```

因为 `MyStruct` 实例的值需要修改，方法 `change_name` 接收实例的可变引用作为它的第一个参数。 _注意这个方法在调用时仍然只有一个参数_。

任务：在 `FloatingImage` 上执行一个 `set_data` 方法，它需要一个 `Vec<u8>` 作为参数。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #70
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_data_method_defined() {
    let mut float = FloatingImage::new(1, 2, "freeCodeCamp".to_string());
    float.set_data(vec![0u8, 1u8, 2u8]);
  }
}
```

### --tests--

## 71

### --description--

任务：在 `set_data` 中，设置实例的 `data` 字段等于 `data` 参数的值。 然后，返回一个 `Ok` 结果和一个元组。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #71
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) {}
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_data_method_returns_result() {
    let mut float = FloatingImage::new(1, 2, "freeCodeCamp".to_string());
    let res: Result<(), ()> = float.set_data(vec![0u8, 1u8, 2u8]);
    assert_eq!(res.unwrap(), ());
    assert_eq!(float.data, vec![0u8, 1u8, 2u8]);
  }
}
```

### --tests--

## 72

### --description--

要更清楚地处理错误，你可以创建_枚举_来表示可能发生的错误：

```rust
    enum MyErrors {
      BrainTooTired,
      TimeOfDay(String),
      CoffeeCupEmpty,
    }

    fn work() -> Result<(), MyErrors> {
      match state {
        "missing semi-colon" => Err(MyErrors::BrainTooTired),
        "06:00" => Err(MyErrors::TImeOfDay("It's too early to work".to_string())),
        "22:00" => Err(MyErrors::TimeOfDay("It's too late to work".to_string())),
        "empty" => Err(MyErrors::CoffeeCupEmpty),
        _ => Ok(()),
      }
    }
```

枚举既可以作为值，也可以作为类型。 你已经遇到了 `Option` 枚举。

任务：创建一个名为 `ImageDataErrors` 的 `enum`，有两个变体 `BufferTooSmall` 和 `DifferentImageFormats`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #72
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ()> {
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn enum_is_defined_with_variants() {
    let _a = ImageDataErrors::BufferTooSmall;
    let _b = ImageDataErrors::DifferentImageFormats;
  }
}
```

### --tests--

## 73

### --description--

任务：为 `ImageDataErrors` 枚举添加 `Debug` 特性。

运行 `cargo test --bin combiner -- --show-output`。 你应该看到控制台打印了以下内容：

```bash
    BufferTooSmall
    DifferentImageFormats
```

### --seed--

```rust
// Lesson #73
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ()> {
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn enum_derives_debug() {
    println!(
      "{:?}\n{:?}",
      ImageDataErrors::BufferTooSmall,
      ImageDataErrors::DifferentImageFormats
    );
  }
}
```

### --tests--

## 74

### --description--

现在你可以使用你的枚举来给出更多的特定错误。

任务：在 `set_data` 内，如果 `data.len()` 大于 `self.data.capacity()`，返回适当的错误。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #74
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ()> {
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn set_data_errors_if_buffer_too_small() {
    let mut float = FloatingImage::new(1, 1, "freeCodeCamp".to_string());
    let res: Result<(), ImageDataErrors> = float.set_data(vec![0u8; 4_000_000]);
    let is_buffer_too_small = match res {
      Ok(_) => false,
      Err(e) => match e {
        ImageDataErrors::BufferTooSmall => true,
        ImageDataErrors::DifferentImageFormats => false,
      },
    };
    assert!(is_buffer_too_small);
    assert_eq!(
      float.data.capacity(),
      3_655_744,
      "You should not set the data, if the buffer is too small"
    );
  }
}
```

### --tests--

## 75

### --description--

任务：在 `main` 中，使用 `ImageDataErrors` 返回适当的错误，而不是返回一个错误 `String`。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #75
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), String> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(String::from("Image formats must match"));
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_enum_on_err() {
    let _a: Result<(), ImageDataErrors> = main();
  }
}
```

### --tests--

## 76

### --description--

任务：在 `main` 里面，在 `output` 上调用 `set_data`，并传入 `combined_data`。

_提示：_遵循编译器的建议，以编译代码。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #76
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);
  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_enum_on_err() {
    let _a: Result<(), ImageDataErrors> = main();
  }
  #[test]
  fn set_data_called_on_output() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(r"output.set_data\(", file_contents));
    }
  }
  #[test]
  fn set_data_called_with_combined_data() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"set_data\(\s*combined_data\s*\)",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 77

### --description--

编译器正在向你发出警告，你没有使用调用 `set_data` 返回的 `Result`。 使用 `unwrap` 方法，你可能引起 panic。 然而，由于错误是用枚举处理的，因此你可以通过使用_错误传播_运算符来传播错误：

```rust
    enum MyError {
      Oops
    }
    fn first() -> Result<(), MyError> {
      Err(MyError::Oops)
    }
    fn second() -> Result<(), MyError> {
      first()?
    }
    fn last() -> String {
      match second() {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
      }
    }
    assert_eq!(last(), "Err".to_string());
```

使用 `?` 运算符允许 `MyError` 传播到调用者。

任务：遵循编译器警告中的建议传播错误。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #77
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);

  output.set_data(combined_data);

  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_enum_on_err() {
    let _a: Result<(), ImageDataErrors> = main();
  }
  #[test]
  fn set_data_propagates_error() {
    if let Some((file_contents, _)) = return_file_in_src("main.rs").split_once("#[cfg(test)]") {
      assert!(reg_with_con(
        r"set_data\(\s*combined_data\s*\)\?",
        file_contents
      ));
    }
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("combiner/src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

## 78

### --description--

最后，你可以将新图像保存到一个文件。 `image` crate 有一个 `save_buffer_with_format` 函数采用以下形式：

```rust
    fn save_buffer_with_format(path: AsRef<Path>, buf: &[u8], width: u32, height: u32, color: image::ColorType, format: image::ImageFormat) -> image::ImageResult<()>;
```

因为对 `String` 应用了 `AsRef`，一个类型为 `String` 的参数可以被用于 `path`。

任务：在 `main` 中，使用正确的 `output` 属性作为前四个参数、`Rgba8` 为颜色参数、`image_1_format` 为格式参数。 解包 `save_buffer_with_format` 的结果。

运行 `cargo run --bin combiner -- ./images/fcc_glyph.png ./images/pro.png example.png`。 如果 `example.png` 文件存在，并且是我的个人资料图片与 freeCodeCamp 标志的组合，就说明你已经完成了这项任务。

### --seed--

```rust
// Lesson #78
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);

  output.set_data(combined_data)?;

  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn main_returns_enum_on_err() {
    let _a: Result<(), ImageDataErrors> = main();
  }
}
```

### --tests--

## 79

### --description--

任务：构建你的 `combiner` CLI 的发行版本。

运行 `cargo test --bin combiner` 来查看你是否正确地完成了任务。

### --seed--

```rust
// Lesson #79
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);

  output.set_data(combined_data)?;

  image::save_buffer_with_format(
    output.name,
    &output.data,
    output.width,
    output.height,
    image::ColorType::Rgba8,
    image_1_format,
  )
  .unwrap();
  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}

#[cfg(test)]
mod tests {
  #[test]
  fn release_build_built() {
    let a = return_file("./target/release/combiner.d");
    assert!(!reg_with_con(r"File does not exist", &a));
  }

  fn reg_with_con(regex: &str, file_contents: &str) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(file_contents)
  }
  fn return_file(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from("File does not exist"),
    }
  }
}
```

### --tests--

- 你可以使用 `cargo build --bin combiner --release` 命令来构建二进制。
- `null`

## 80

### --description--

恭喜你完成了 **Rust in Replit** 课程！

你现在可以自己写代码，并使用你的新的命令行工具来制作你自己的组合图像。

### --seed--

```rust
// Lesson #80
#![allow(unused_variables, dead_code)]
mod args;

use args::Args;
use image::{
  imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (image_1, image_2) = standardise_size(image_1, image_2);
  let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

  let combined_data = combine_images(image_1, image_2);

  output.set_data(combined_data)?;

  image::save_buffer_with_format(
    output.name,
    &output.data,
    output.width,
    output.height,
    image::ColorType::Rgba8,
    image_1_format,
  )
  .unwrap();
  Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferTooSmall,
  DifferentImageFormats,
}

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).unwrap();
  let image_format = image_reader.format().unwrap();
  let image = image_reader.decode().unwrap();
  (image, image_format)
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);
  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  let vec_1 = image_1.to_rgba8().into_vec();
  let vec_2 = image_2.to_rgba8().into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  let mut combined_data = vec![0u8; vec_1.len()];

  let mut i = 0;
  while i < vec_1.len() {
    if i % 8 == 0 {
      combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
    } else {
      combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
    }
    i += 4;
  }

  combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
  let mut rgba = Vec::new();
  for i in start..=end {
    let val = match vec.get(i) {
      Some(d) => *d,
      None => panic!("Index out of bounds"),
    };
    rgba.push(val);
  }
  rgba
}
```

### --tests--

- 这是最后一节课程。 祝贺你！
- `null`

## 81

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Placeholder
- `null`
