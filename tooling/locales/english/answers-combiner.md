# freeCodeCamp - Rust in Replit - Image Combiner

## 1

### --description--

Start by creating a new project called `combiner`.

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
- You should assign `args` the value of `Args {}`.
- `let\s+args\s*=\s*Args\s*\{\};`

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

Task: Follow the compiler's advice to extend the formatter within the `println!`.

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

Run `cargo run --bin combiner first_arg second_arg third_arg`. If you see the following, you correctly completed the task:

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

Task: Move the `Args` struct and implementation, and the `get_nth_arg` function to the `args.rs` file. Then, comment out the content within the `main` function so your app compiles.

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

In order to encode and decode the images, you will use the `image` crate.

Task: Within the root `Cargo.toml`, add the following to the `dependencies` section:

```rust
    image = "0.23.14"
```

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `main.rs`, define a function named `find_image_from_path` that takes a `String` as an argument.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

The compiler is showing a warning that the `find_image_from_path` function is unused. This is going to become annoying, over the course of this project. Fortunately, you can enable global attributes to suppress the warning.

Global attributes use the syntax `#![feature(feature_name)]`, and should be placed at the top of the file.

Task: Within `main.rs`, use the `allow` feature to globally enable `unused_variables` and `dead_code`.

Run `cargo test --bin combiner`. If you no longer see the warning, you have successfully completed this lesson.

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

Task: Import the `Reader` struct from `image::io`, and, within `find_image_from_path`, assign the unwrapped value of the `Reader::open` function, passing `path` as the argument, to a variable named `image_reader`. Then, return `image_reader`.

Hint: Follow the compiler's advice, and import the necessary types.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `find_image_from_path`, assign the unwrapped value of the `format` method on `image_reader` to a variable named `image_format`, and return it.

Hint: Follow the compiler's advice, and import the necessary types.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Remove the unused imports from `main.rs`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

So far, you have not decoded the image. The `Reader` has a `decode` method which returns a `DynamicImage` in a `Result`.

Task: Within `find_image_from_path`, assign the unwrapped value of the `decode` method on `image_reader` to a variable named `image`. Then, return `image`.

_Hint:_ Follow the compiler's advice, and import the necessary types.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

You have learnt about the empty tuple type `()`. Now, you will use a tuple to return multiple values. Unlike other types, a tuple can contain more than one type.

```rust
    // The Vec type can only contain one type.
    let my_vec = vec![1u8, 2u16, 3u32];
    // Tuples can contain multiple types.
    let my_tuple = (1u8, 2u16, 3u32);
```

Task: From `find_image_from_path`, return a tuple containing the `DynamicImage` and `ImageFormat` of the image, in that order.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Tuples can be destructured into variables like this:

```rust
    let (x, y, z) = (1, 2, 3);
```

Task: Within `main`, destructure the tuple returned from `find_image_from_path` into the variables `image_1` and `image_1_format`. You should call `find_image_from_path` with the value of the `image_1` field of `args`.

Run `cargo test --bin combiner`. You should see an error.

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

Your code has an error, because the `image_1` field on `args` is not public. So, it may not be used across modules.

Task: Within `args.rs`, change all the `Args` struct's fields to be public.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `main`, destructure the tuple returned from `find_image_from_path` into the variables `image_2` and `image_2_format`. You should call `find_image_from_path` with the value of the `image_2` field of `args`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

So far, you have been dealt with a few functions which returned a `Result`. Now, you are going to create a new `Result`.

A `Result` is a type that can either be `Ok` or `Err`. It is common to return an empty tuple when a function is successful, and return an error message when a function fails:

```rust
    fn function_returns_result() -> Result<(), String> {
      if (condition) {
        return Ok(());
      } else {
        return Err(String::from("Error message"));
      }
    }
```

Task: Within `main.rs`, convert the `main` function to return a `Result`. For now, just return an empty tuple on `Ok`, but set the return `Err` type to `String`.

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

Your application will only be able to combine two images of the same type.

Task: As such, if `image_1_format` is not equal to `image_2_format`, return an error message.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Another error that can occur is when the two images are not the same size. Fortunately, there is a function that can be used to resize the images.

Task: Start by creating a function named `standardise_size` which takes `image_1` and `image_2` as parameters.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

To make combining the images simpler, you can resize the largest image to the size of the smallest image. In order to do this, you need to get the smallest dimensions of the two images.

Task: Create a function named `get_smallest_dimensions` which takes two tuples as parameters. Each tuple should take two elements, each of type `u32`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Within `get_smallest_dimensions`, you will need to return the dimensions with the smallest number of pixels. The number of pixels is the product of the width and height.

Task: Return `dim_1` if the number of pixels in `dim_1` is less than the number of pixels in `dim_2`. Return `dim_2` otherwise.

Remember, you can use dot notation to access the elements of a tuple.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `standardise_size`, destructure the tuple returned from `get_smallest_dimensions` into two variables `width` and `height`. Use the return of calling the `dimensions` method on each `DynamicImage` to pass as arguments for `get_smallest_dimensions`.

_Hint:_ Follow the compiler's advice to get the dimensions of the images.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `standardise_size`, print `width` and `height` to the console.

Run `cargo test --bin combiner -- --show-output`. If you see the `'width: 10, height: 10'` printed to the console, you have successfully completed the task.

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

Task: Within `standardise_size`, write an `if` statement to check if `image_2`'s dimensions are equal to the previously determined smallest dimensions. If they are, return a tuple containing `image_1` and `image_2`. Otherwise, return the same tuple.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Instead of returning the images unchanged, you should resize the larger image. You can use the `resize_exact` method which exists on the `DynamicImage` struct. The `resize_exact` method takes the form:

```rust
    image_to_resize.resize_exact(new_width: u32, new_height: u32, filter: image::imageops::FilterType);
```

Task: Within `standardise_size`, resize the correct image variable to the correct dimensions, using the `Triangle` filter.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `main` before the `Ok` return, use the `standardise_size` function to redeclare `image_1` and `image_2` as the correct size.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

To handle the output, a temporary struct can be created to hold the meta data for the output image.

Task: Create a struct called `FloatingImage` that has the following fields:

```rust
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
```

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Implement a function named `new` for `FloatingImage`. The `new` function should take three arguments: `width: u32`, `height: u32`, `name: String`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

In order to efficiently write the combined image data to the output image, you need to create a buffer large enough to hold the data so no extra space needs to be allocated.

Large images can have a large amount of data, so you can take advantage of Rust's easy-to-read numbering, which separates the number into groups of three digits:

```rust
    let difficult_to_read_number = 1325364955;
    let easy_to_read_number = 1_325_364_955;
```

Task: Within `new`, declare a variable named `buffer_capacity`, and assign it the value of `3655744` using the easy-to-read number.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Now that you have a buffer size, you need to create a buffer of `Vec<u8>`. The `Vec` struct implements a `with_capacity` function, which takes a capacity as an argument and returns a new `Vec` with that capacity.

Task: Within `new`, declare a variable named `buffer`, and assign it the value of calling the `with_capacity` function with `buffer_capacity`.

_Hint:_ Follow the compiler's advice to explicitly type the `buffer` variable.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `new`, use the variables available to return an instance of the `FloatingImage` struct.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `main`, declare a new variable `output` using the `new` function of the `FloatingImage` struct. Use the `width` and `height` methods of the `image_1` variable for the first two arguments, and the `output` field of the `args` variable for the third argument.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Define a function named `combine_images` which takes two `DynamicImage`s as arguments.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

In order to process the images, you will convert them into a vector of RGBA pixels. The pixels are stored as `u8`s, because their values are between 0 and 255.

The `DynamicImage` struct implements the `to_rgba8` method, which returns an `ImageBuffer` containing a `Vec<u8>`, and the `ImageBuffer` implements the `into_vec` method, which returns the `Vec<u8>`.

Task: Within `combine_images`, declare a variable `vec_1`, and use the above methods to assign the `Vec<u8>` to it. Return `vec_1`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Do the same as in the previous lesson, but on `image_2`, and return the new variable named `vec_2` instead of `vec_1`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Now that you have the pixel values of each image in `vec_1` and `vec_2`, you can combine them into a single image.

Task: Define a function named `alternate_pixels` which takes two `Vec<u8>`s as arguments.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

You will need to store the combined image pixel data in a variable. To create this variable, you can use the `vec` macro, providing the type and length of the vector:

```rust
    let my_vec = vec![10u8; 5];
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec, [10, 10, 10, 10, 10]);
```

Task: Within `alternate_pixels`, declare a variable `combined_data`, and use the `vec` macro to create a `Vec<u8>` of `0` the same length as `vec_1`. Return `combined_data`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

To iterate over the pixels in the vectors, you will use a `while` loop. A `while` loop follows this syntax:

```rust
    while condition {
      // Do stuff
    }
```

Where `condition` is a boolean expression that evaluates to `true` or `false`.

Task: Within `alternate_pixels`, before returning `combined_data`, declare a variable `i`, and assign it the value of `0`. Then, declare a `while` loop that runs whilst `i` is less than the length of `vec_1`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

In order to correctly set the RGBA pixel sets for the output vector, you will replace the `0u8` values with the correct values.

Task: Define a function named `set_rgba` which takes 3 arguments: A `Vec<u8>`, and two `usize`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `set_rgba`, define a mutable variable named `rgba`, and assign it to be an empty `Vec<u8>`. Try to do this without the `vec` macro. Then, return `rgba`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

To iterate over a range of values, you can use a `for...in` loop with the _right-inclusive range literal_ operator:

```rust
    for i in 1..=5 {
      println!("{}", i);
    }
```

The `=` within the range literal is the _right-inclusive_ range operator, meaning the end is included.

Task: Within `set_rgba`, before returning `rgba`, iterate over the range `start..=end`, and push each value in the range to `rgba`.

Hint: You will need to assert the correct type of the value being pushed to `rgba`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Sometimes, retrieving a value from within a vector causes a panic, because the index is out of bounds. To avoid this, you can use the `get` method on a vector:

```rust
    let my_vec = vec![1, 2, 3];
    assert_eq!(my_vec.get(0), Some(&1));
    assert_eq!(my_vec.get(3), None);
```

The `get` method returns a reference to the value at the given index, or `None` if the index is out of bounds.

Task: Within `set_rgba`, within the `for` loop, declare a variable `val` which is the `match` of the `get` method on `vec` using `i` as the argument.

On `Some`, assign the value to `val`. On `None`, return panic.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Task: Within `set_rgba`, change the value pushed onto the `vec` to be `val`.

Run `cargo test --bin combiner`. You should see an error.

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

The error comes about, because the type of `val` is `&u8` - a reference to an 8-bit unsigned integer. However, the type of `vec` should be `Vec<u8>`, not `Vec<&u8>`.

To fix this, the value returned from the `get` method can be _dereferenced_ . A dereference is done by annotating the value with `*`.:

```rust
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
```

Task: Within `set_rgba`, dereference the value assigned to `val`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Currently, the `while` loop has the potential to run forever, when `vec` contains any elements. You can fix this by incrementing `i` on each iteration of the loop. Here are some common ways to increment an integer:

```rust
    let mut a = 0;
    a += 1;
    a++;
    a = a + 1;
    assert_eq!(a, 3);
```

Task: Within `alternate_pixels`, in the `while` loop, increment `i` by `4`.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

You have a `Vec<u8>` of `0`s, and two `Vec<u8>`s of `0-255`. To replace one slice of a vector with another, you can use the `splice` method:

```rust
    let original_vec = vec![0, 1, 2, 3];
    let mut vec_to_change = vec![0u8; 4];
    vec_to_change.splice(2..4, original_vec[2..4].iter().cloned());
    assert_eq!(vec_to_change, vec![0, 0, 2, 3]);
```

The `splice` method takes two arguments: the range of the vector to replace, and the values to replace it with.

Task: Within `alternate_pixels`, in the `while` loop, use the `splice` method on `combined_data` from `i` to `i+3`, and use the `set_rgba` function to insert the correct values from `vec_1`.

Running `cargo test --bin combiner` should produce an error. Run `fcc test 65` to see if you correctly completed the task.

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

- You should not change the `while` loop condition.
- `while\s+i\s*<\s*vec_1\.len\(\)\s*\{`
- You should increment `i` by 4 at the end of the loop.
- `i\s*\+=\s*4;\s*\}`
- You should call the `splice` method on `combined_data`.
- `combined_data\.splice\(`
- You should either pass the range `i..i+4` or `i..=i+3` to the `splice` method's first argument.
- `splice\(\s*(i..i\s*\+\s*4)|(i..=i\s*\+\s*3)\s*,`
- You should pass `set_rgba(vec_1, i, i+3)` as the second argument to `splice`.
- `splice\(\s*(i..i+4)|(i..=i\s*\+\s*3)\s*,\s*set_rgba\(\s*vec_1\s*,\s*i\s*,\s*i\s*\+\s*3\s*\)\s*\)`

## 66

### --description--

The error is saying the `vec_1` value is moved into `set_rgba` on the first iteration of the loop. So, on the second iteration, when the `while` condition is supposed to evaluate `i < vec_1.len()`, `vec_1` is not in scope to be used.

Task: Within `alternate_pixels`, fix the issue, by passing a reference to `vec_1` to `set_rgba`, and fix the necessary type annotations.

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

Currently, `alternate_pixels` is splicing every RGBA set from `vec_1` into `combined_data`. However, you want every second set to be from `vec_2`. To achieve this, you can use the remainder operator:

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

Task: Within `alternate_pixels`, use the remainder operator to splice every second set of RGBA values from `vec_2` into `combined_data`.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Within `combine_images`, instead of returning `vec_2`, return the result of calling `alternate_pixels` with `vec_1` and `vec_2`.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Within `main`, declare a variable `combined_data`, and assign it the value of calling `combine_images` with `image_1` and `image_2`.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Now, you want to set the data of `combined_data` into the `output` image. To do this, you are going to define a method on `FloatingImage` to set the `data` field of `output` to the value of `combined_data`.

So far, you have only implemented functions on structs. Methods are defined in a similar way, but they take an instance of the struct as their first argument:

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

As the value of the instance of `MyStruct` needs to be changed, the method `change_name` takes a mutable reference to the instance as its first argument. _Notice the method is still only called with one argument_ .

Task: Implement a method `set_data` on `FloatingImage` which takes a `Vec<u8>` as an argument.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Within `set_data`, set the instance's `data` field to be equal to the value of the `data` argument. Then, return an `Ok` result with an empty tuple as a response.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

To handle errors more clearly, you can create an _enum_ to represent the possible errors that can occur:

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

Enums can be used as both values as well as types. You have already encountered the `Option` enum.

Task: Create an `enum` called `ImageDataErrors`, and has two variants `BufferTooSmall` and `DifferentImageFormats`.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Derive the `Debug` trait for the `ImageDataErrors` enum.

Run `cargo test --bin combiner -- --show-output`. You should see the following printed to the console:

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

Now you can use your enum to give more specific errors.

Task: Within `set_data`, return the appropriate error, if `data.len()` is greater than `self.data.capacity()`.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Within `main`, instead of returning an error `String`, use `ImageDataErrors` to return the appropriate error.

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

Task: Within `main`, call `set_data` on `output` with `combined_data`.

_Hint:_ Follow the compiler's advice to get the code to compile

Run `cargo test --bin combiner` to see if you correctly completed this task.

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

The compiler is giving you a warning that you are not using the `Result` returned from calling `set_data`. You could panic on the error using the `unwrap` method. However, as the error is being handled with an enum, you can propagate the error by using the _error propagation_ operator:

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

Using the `?` operator allows the `MyError` to propagate to the caller.

Task: Follow the compiler's advice in the warning to propagate the error.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

Finally, you can save the new image to a file. The `image` crate has a `save_buffer_with_format` function taking the following form:

```rust
    fn save_buffer_with_format(path: AsRef<Path>, buf: &[u8], width: u32, height: u32, color: image::ColorType, format: image::ImageFormat) -> image::ImageResult<()>;
```

Seeing as `AsRef` is implemented for `String`, an argument of type `String` can be used for the `path`.

Task: Within `main`, use the correct `output` properties as the first four arguments, `Rgba8` as the colour argument, and `image_1_format` as the format argument. Unwrap the result of `save_buffer_with_format`.

Run `cargo run --bin combiner -- ./images/fcc_glyph.png ./images/pro.png example.png`. If the `example.png` file exists, and is a combination of my profile picture with the freeCodeCamp logo, you have completed the task.

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

Task: Build a release version of your `combiner` CLI.

Run `cargo test --bin combiner` to see if you correctly completed the task.

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

- You can use the `cargo build --bin combiner --release` command to build the binary.
- `null`

## 80

### --description--

Congratulations on finishing the **Rust in Replit** course!

You may now play around with your code and your new command line tool to make combined images of your own.

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

- This is the final lesson. Congrats!
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
