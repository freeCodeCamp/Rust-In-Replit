# freeCodeCamp - Rust in Replit Course Answers

## 2

```rust
fn main() {
  println!("Hello, world!");
}
```

- You should have the project file `calculator/src/main.rust`.
- `null`

## 3

```rust
fn main() {
  println!("Hello, world!");
}
```

- Running your code should output `Hello, world!`.
- `getCommandOutput(Hello, world!)`

## 4

```rust
fn main() {
  println!("Hello, world!");
}
```

- You should declare a variable `firstName` and give it a value of your first name within double quotes.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- You should follow the compiler's advice to add a semi-colon at the end.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

```rust
fn main() {
  let firstName = "Quincy";
  println!("Hello, world!");
}
```

- You should have a variable `first_name` and give it a value of your first name within double quotes.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

```rust
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

- You should change the `println!` call to `println!("Hello, {}!", first_name)`.
- `println!("Hello,\s*{}!",\s*first_name)\s*;`

## 7

```rust
fn main() {
  let first_name = "Quincy";
  println!("Hello, {}!", first_name);
}
```

- You should use `String::from()` to create a `String` with your first name.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 8

```rust
fn main() {
  let first_name = String::from("Quincy");
  println!("Hello, {}!", first_name);
}
```

- You should declare a variable `name` and assign it the value of `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- You should replace the second argument of `println!` with `name`.
- `println!("Hello, {}!",\*name)\s*;`

## 9

```rust
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name;
  println!("Hello, {}!", name);
}
```

- You should have two `println!` calls immediately after one another.
- `println!("Hello, {}!",\s*\w+)\s*;\s*println!("Hello, {}!",\s*\w+)\s*;`
- You should have the first `println!` use `name` and the second `println!` use `first_name`.
- `println!("Hello, {}!",\s*name)\s*;\s*println!("Hello, {}!",\s*first_name)\s*;`
- You should reference `first_name` when assigning it to `name`, by using `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 10

```rust
fn main() {
  let first_name = String::from("Tom");
  let name = &first_name;
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

- You should not turn `first_name` into an owned value with `.to_owned()`.
- `first_name\.to_owned()`
- You should concatenate your surname to the owned `first_name`.
- `first_name\.to_owned()\s*+\s*"\w+"`

## 11

```rust
fn main() {
  let first_name = String::from("Quincy");
  let name = first_name.to_owned() + " Larson";
  println!("Hello, {}!", name);
  println!("Hello, {}!", first_name);
}
```

- You should use the `.push_str()` method.
- `\.push_str\(`
- You should push your surname to `first_name`.
- `first_name\.push_str\(\s*"\w+"\s*\)`
- You should make `first_name` mutable with `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 12

```rust
fn main() {
  let mut first_name = String::from("Nicholas");
  first_name.push_str(" Carrigan");
  println!("Hello, {}!", first_name);
}
```

- You should declare a variable named `first`.
- `let\s+first`
- You should assign `first` a single character in double quotes.
- `first\s*=\s*"\w"`
- You should print the length of `first` and the number of characters in `first`. Example Output: `1 1`
- `getCommandOutput(1\s*1);`

## 13

```rust
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

- You should change the value of `first` to be a string slice of `∞`.
- `first\s*=\s*"∞"`
- Your code should print `3 1`.
- `getCommandOutput(3\s*1);`

## 14

```rust
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

- There are no tests for this lesson.
- `null`

## 15

```rust


#[cfg(test)]
mod tests {

  #[test]
  fn main_exists() {
    assert_eq!(main(), ());
  }
}
```

- You should have run the command `fcc reset 15`.
- `mod tests`

## 16

```rust

#[cfg(test)]
mod tests {

  #[test]
  fn main_exists() {
    assert_eq!(main(), ());
  }
}
```

- There are no Node tests for this lesson
- `null`

## 17

```rust
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

- There are no Node tests for this lesson
- `null`

## 18

```rust
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

- There are no Node tests for this lesson
- `null`

## 19

```rust
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, ());
  }
}
```

- There are no Node tests for this lesson
- `null`

## 20

```rust
fn main() -> f32 {
  24.5
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) {

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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

- There are no Node tests for this lesson
- `null`

## 21

```rust
fn main() {

}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

- You should print to the console any valid output.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 22

```rust
fn main() {

  println!("{}", output(10, "+", 10, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

- You should declare a variable named `first_number`.
- `let first_number`
- You should declare a variable named `second_number`.
- `let second_number`
- You should declare a variable named `operator`.
- `let operator`

## 23

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_expects_three_args() {
    let op = operate("-", -5, 200);
    assert_eq!(op, ());
  }
}
```

- There are no Node tests for this lesson
- `null`

## 24

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) {

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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_returns_zero_on_invalid_op() {
    let op = operate("invalid", 1, 1);
    assert_eq!(op, 0);
  }
}
```

- There are no Node tests for this lesson
- `null`

## 25

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  if operator == "+" {
    first_number + second_number
  } else if operator == "-" {
    first_number - second_number
  } else if operator == "/" {
    first_number / second_number
  } else if operator == "*" {
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- There are no Node tests for this lesson
- `null`

## 26

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  if operator == "+" {
    first_number + second_number
  } else if operator == "-" {
    first_number - second_number
  } else if operator == "/" {
    first_number / second_number
  } else if operator == "*" {
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should use the `match` operator.
- `match`

## 27

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should use an `|` operator.
- `|`

## 28

```rust
fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  println!("{}", output(first_number, operator, second_number, 0));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should declare a new variable `result`
- `let\s+result`
- Your code should print `1 - 10 = -9` to the console.
- `getCommandOutput(1 - 10 = -9)`

## 29

```rust


fn main() {
  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should add `use std::env;` to the top of your script.
- `use\s+std::env\s*;`

## 30

```rust
use std::env;

fn main() {


  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should declare a new variable named `args`
- `let\s+args`
- You should assign `args` a value of `env::args()`
- `=\s*env::args()`

## 31

```rust
use std::env;

fn main() {
  let args = env::args();

  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- Running `cargo run --bin calculator` should print: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 32

```rust
use std::env;

fn main() {
  let args = env::args();
  println!("{:?}", args);

  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

## 33

```rust
use std::env;

fn main() {
  let args = env::args();
  println!("{:?}", args);

  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should access the first (`0`) element of the `env::args()` iterator.
- `args\.nth(0)`
- You should declare `args` as mutable with `let mut args =...`
- `let\s+mut\s+args`

## 34

```rust
use std::env;

fn main() {
  let mut args = env::args();
  println!("{:?}", args.nth(0));

  let first_number = 1;
  let operator = "-";
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should assign `args.nth(0)` to `first_number`.
- `let\s+first_number\s*=\s*args.nth(0)`
- You should assign `args.nth(1)` to `operator`.
- `let\s+operator\s*=\s*args.nth(1)`
- You should assign `args.nth(2)` to `second_number`.
- `let\s+second_number\s*=\s*args.nth(2)`

## 35

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should unwrap `first_number` with `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args.nth(0).unwrap()`
- You should unwrap `operator` with `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args.nth(1).unwrap()`
- You should unwrap `second_number` with `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args.nth(2).unwrap()`

## 36

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- There are no Node tests for this lesson.
- `null`

## 37

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- There are no Node tests for this lesson.
- `null`

## 38

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should import the `Args` struct from the `std::env` module with `use std::env::Args`.
- `use\s+std::env::Args`
- You should annotate `args` with the type `Args`.
- `let\s+args:\s*Args`
- You should annotate `first_number` with the type `String`.
- `let\s+first_number:\s*String`
- You should annotate `operator` with the type `String`.
- `let\s+operator:\s*String`
- You should annotate `second_number` with the type `String`.
- `let\s+second_number:\s*String`

## 39

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should combine both imports into a single import statement with `use std::env::{args, Args};`.
- `use\s+std::env::{args, Args};`

## 40

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- You should declare a variable named `first`.
- `let\s+first`
- You should declare a variable named `second`.
- `let\s+second`
- You should assign `first_number.parse::<i32>().unwrap()` to `first`.
- `first_number.parse::<i32>().unwrap()`
- You should assign `second_number.parse::<i32>().unwrap()` to `second`.
- `second_number.parse::<i32>().unwrap()`

## 41

```rust
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

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- There are no Node tests for this lesson.
- `null`

## 42

```rust
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<i32>().unwrap();
  let second = second_number.parse::<i32>().unwrap();
  let result = operate(&operator, first, second);

  println!("{}", output(first, &operator, second, result));
}

fn output(first_number: i32, operator: &str, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "/" => first_number / second_number,
    "*" | "X" | "x" => first_number * second_number,
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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1, 1);
  }
}
```

- This is the final lesson. Congrats!
- `null`
