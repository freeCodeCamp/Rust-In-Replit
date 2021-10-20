# freeCodeCamp - Rust in Replit - Image Combiner

## 1

### --description--

Start by creating a new crate called `combiner`.

Run `fcc test 1` to see if you correctly completed the task.

### --seed--

```rust
// Lesson #1
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- You should see a new directory `combiner` created in the root.
- `Hello, world!`

## 2

### --description--

In this project, you will be creating a CLI (combiner) which expects three arguments:

```bash
  $ combiner image1.png image2.png output.png
```

The first two arguments are the paths to the images you want to combine. The third argument is the path to the output image.

Task: Open `combiner/src/main.rs` and run `cargo run --bin combiner` to see if your application is correctly set up.

You should see `Hello, world!` printed to the console.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Your code should output `Hello, world!`
- `getCommandOutput(Hello, world!)`

## 3

### --description--

Task: Define a function called `get_nth_arg` which takes one `usize` argument.

Remember to import as necessary.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

- Run `cargo test --bin combiner` to see if you correctly completed the task.
- `null`
- Hint: Remember to import the function into the `tests` module.
- `null`

## 4

### --description--

Upcoming tests make use of an external crate called `regex`.

Task: Open the `Cargo.toml` file in the root, and add the following lines:

```rust
[dependencies]
regex = "1.5.4"
```

This will install the `regex` crate into your project which is used in the `reg_with_con` function. You can find out more about this crate at: https://crates.io/crates/regex

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

- Run `cargo test --bin combiner` to see if you correctly completed the task. No errors means you're done.
- `null`

## 5

### --description--

To make use of command line arguments, you will need to use the `std::env` module.

Task: Within `get_nth_arg`, return the unwrapped value of calling the `nth` method on the `args` function with the argument `n`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

- Run `cargo test --bin combiner` to see if you correctly completed the task. No errors means you're done.
- `null`

## 6

### --description--

Ideally, you want to store only the command line arguments you are interested in in a single variable.

Task: Within `main`, create a variable called `args`, and assign it the value of `Args {}`.

Run `fcc test 6` to see if you correctly completed the task.

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

- You should create a new variable called `args`.
- `let args`
- You should assign `args` the value of `Args::new()`.
- `let\s+args\s*=\s*Args::new()`

## 7

### --description--

The syntax `Args {}` is a constructor for a struct named `Args`. However, we have not defined the struct yet.

Here is an example of a struct you have already used:

```rust
struct String {
  vec: Vec<u8>,
}
```

The `String` struct consists of a `vec` field, which is a `Vec` of `u8`s.

Task: At the global scope, define a struct named `Args`.

Remember to import as necessary.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Instead of writing `use crate::` for every function or struct in the `tests` module, you can use the `super` keyword with the `*` wildcard selector to select everything in the current module.

Task: Replace the `use crate::` calls with `use super::*`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

- Run `cargo test --bin combiner` to see if you correctly completed the task. No errors means you're done.
- `null`

## 9

### --description--

Task: Add a field named `image_1` to the `Args` struct, and give it the correct type to pass the tests.

Remember to adjust the declaration in `main` as necessary.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: To get a better idea of the `Args` struct, print the value of `my_arg` in the `args_struct_has_image_1_field` test.

Run `cargo test --bin combiner -- --show-output`. If you see an error, you correctly completed the task.

The `--show-output` flag shows the stdout of the tests.

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

Your code could not compile, because the `println!` macro does not know how to format the `Args` struct.

Task: Follow the compiler's advice to extend the formatter.

Run `cargo test --bin combiner -- --show-output`. You should still see an error.

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

The compiler is telling you that you are trying to use the `println!` macro on a type that does not implement the `Debug` trait.

Usually, traits need to be implemented for a struct using the `impl` keyword. However, in this case, you can use the `derive` attribute to automatically implement the `Debug` trait for you:

```rust
#[derive(Debug)]
struct MyStruct {
  field_1: String,
}
```

Task: Implement the `Debug` trait for your `Args` struct.

Run `cargo test --bin combiner -- --show-output`. You should see the `Args` struct printed to the console.

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

You may notice the `String::new()` used in `main`. The `new` function is a common constructor for structs. For `String`, it looks something like this:

```rust
impl String {
  fn new() -> Self {
    String::from("")
  }
}
```

The above implements the `new` function for `String`. The return type is `Self`, which is the type of the struct.

Task: Implement the `new` function for `Args`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Instead of manually creating the `Args` struct in `main`, use the `new` function to create the struct.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `new`, instead of assigning an empty `String` to `image_1`, use the `get_nth_arg` function to assign the value of the first **valid** argument.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Your application should expect three arguments: `image_1`, `image_2`, and `output`.

Task: Add the missing two fields to the `Args` struct. All of the fields should use the same type.

Run `fcc test 16` to see if you correctly completed the task.

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

- You should define `Args` with an `image_1` field of type `String`.
- `image_1:\s*String`
- You should define `Args` with an `image_2` field of type `String`.
- `image_2:\s*String`
- You should define `Args` with an `output` field of type `String`.
- `output:\s*String`

## 17

### --description--

Task: Update the `new` function to assign valid values to all expected fields.

Run `cargo test --bin combiner -- --show-output` to see if you correctly completed the task.

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

Testing your application with arguments should now print out the `Args` struct with the arguments as values for the fields.

Task: Change the `println` in `main` to print the value of `args`.

Run `cargo run --bin combiner first_arg second_arg third_arg`. If you see to see the following, you correctly completed the task:

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

Before your `main.rs` file gets too cluttered, you should move the argument logic to its own file.

Task: Create the file `combiner/src/args.rs`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Move the `Args` struct and implementation, and the `get_nth_arg` function to the `args.rs` file. Then, uncomment the content within the `main` function so your app compiles.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

In Rust, everything is private by default. So, in order to make a function or struct public, you can use the `pub` keyword:

```rust
pub MyStruct {}
```

Task: Within `args.rs` make the struct and the function public.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

In order to use the contents of an external file, it needs to be declared as a module:

```rust
mod my_file_name
```

Task: At the top of `main.rs`, declare the `args.rs` file as a module.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Now that `args` has been declared as a module for use within `main.rs`, you can use the `use` keyword to import the `Args` struct.

Task: Within `main.rs`, import the `Args` struct. Then, uncomment the commented out code in the `main` function.

Run `cargo test --bin combiner`. You should see an error.

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

The error comes about because the `new` function implemented for `Args` is not public.

Task: Within `args.rs` declare the `new` function as public.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

## Final

### --description--

### --seed--

```rust
// Lesson #Final
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat};
use std::convert::TryInto;
mod args;
pub use args::{get_nth_arg, Args};

struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}

#[derive(Debug)]
enum ImageDataErrors {
  BufferToSmall,
  DifferentImageFormats,
}

impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> FloatingImage {
    // Initialise image with buffer based on RGBA data
    let buffer_capacity = (width * height * 4).try_into().unwrap();
    let buffer = Vec::with_capacity(buffer_capacity);

    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
  // TODO: Turn into Result?
  fn set_data(&mut self, data: Vec<u8>) -> Result<&str, ImageDataErrors> {
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferToSmall);
    }
    // TODO: Un-hard-code expected_len
    let expected_len = self.width * self.height * 4;
    let mut temp = data.clone();
    temp.resize(expected_len.try_into().unwrap(), 0);
    self.data = temp;
    Ok("Data has been set")
  }
}

fn main() -> Result<(), ImageDataErrors> {
  // Get command line arguments - paths to images
  let args: Args = Args::new();

  // Create Image objects
  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  let (width, height) = get_largest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {} , height: {}\n", width, height);
  let mut output = FloatingImage::new(width, height, args.output);

  let comb = combine_images(&image_1, image_2);

  output.set_data(comb)?;

  image::save_buffer_with_format(
    output.name,
    &output.data,
    output.width,
    output.height,
    image::ColorType::Rgba8,
    image_1_format,
  )
  .expect("Unable to save combined image");
  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader = Reader::open(path).expect("Image not found");
  let image_format = image_reader.format().expect("Image format undeterminable");
  let image = image_reader.decode().expect("Image data invalid");
  (image, image_format)
}

fn combine_images(image_1: &DynamicImage, image_2: DynamicImage) -> Vec<u8> {
  // Turn image into vector of RGBA u8 -> one pixel == [u8, u8, u8, u8]
  let data_1 = image_1.to_rgba8();
  let data_2 = image_2.to_rgba8();
  let vec_1: Vec<u8> = data_1.into_vec();
  let vec_2: Vec<u8> = data_2.into_vec();

  alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
  // Set buffer for combined pixel data
  let largest_len = vec_1.len().max(vec_2.len());
  let mut combined_data = vec![0u8; largest_len];

  println!("{:?}, {:?}", &vec_1.len(), &vec_2.len());

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
      None => 0,
    };
    rgba.push(val);
  }
  rgba
}

fn get_largest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  return dim_1.max(dim_2);
}

#[cfg(test)]
mod tests {
  use image::GenericImageView;
  use image::ImageFormat;

  use crate::alternate_pixels;
  use crate::find_image_from_path;
  use crate::get_largest_dimensions;
  use crate::set_rgba;

  // TODO: teach use super::*;

  #[test]
  fn main_contains_arg_declaration() {
    let contents = return_file_in_src("main.rs");
    assert!(reg_with_con(r"pub use arg", contents));
  }
  #[test]
  fn finding_fcc_glyph_format_is_png() {
    let (_, image_format) = find_image_from_path("./images/fcc_glyph.png".to_string());
    assert_eq!(image_format, ImageFormat::Png);
  }
  #[test]
  fn finding_fcc_glyph_image_dimensions() {
    let (image, _) = find_image_from_path("./images/fcc_glyph.png".to_string());
    let dimensions = image.dimensions();
    println!("{:?}", dimensions);
    assert_eq!(dimensions, (712, 484));
  }
  #[test]
  #[should_panic]
  fn finding_image_from_non_existant_path_panics() {
    find_image_from_path("path/does/not/exist".to_string());
  }

  #[test]
  fn rgba_set_returned() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(set_rgba(&vec, 4, 4 + 3), vec![4, 5, 6, 7]);
  }

  #[test]
  fn vector_members_are_alternated() {
    let vec_1 = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23];
    let vec_2 = vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
    assert_eq!(
      alternate_pixels(vec_1, vec_2),
      vec![1, 3, 5, 7, 8, 10, 12, 14, 17, 19, 21, 23]
    );
  }
  #[test]
  fn largest_tuple() {
    let smaller_tup = (10, 10);
    let larger_tup = (10, 11);
    assert_eq!(get_largest_dimensions(smaller_tup, larger_tup), (10, 11));
  }

  fn reg_with_con(regex: &str, file_contents: String) -> bool {
    use regex::Regex;

    Regex::new(regex).unwrap().is_match(&file_contents)
  }

  fn return_file_in_src(filename: &str) -> String {
    use std::fs::read_to_string;

    match read_to_string(String::from("src/") + filename) {
      Ok(file_contents) => file_contents,
      Err(_) => String::from(""),
    }
  }
}
```

### --tests--

## 100
