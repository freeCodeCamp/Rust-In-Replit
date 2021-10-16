# freeCodeCamp - Rust in Replit Course Answers

## 1

### --description--

### --seed--

```rust

```

### --tests--

- There are no RegEx tests for this lesson.
- `null`

## 2

### --description--

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- You should have the project file `calculator/src/main.rust`.
- `null`

## 3

### --description--

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Running your code should output `Hello, world!`.
- `getCommandOutput(Hello, world!)`

## 4

### --description--

### --seed--

```rust
// Lesson #4
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- You should declare a variable `firstName` and give it a value of your first name within double quotes.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- You should follow the compiler's advice to add a semi-colon at the end.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

### --seed--

```rust
// Lesson #5
fn main() {
  let firstName = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- You should have a variable `first_name` and give it a value of your first name within double quotes.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- You should have a variable `first_name` and give it a value of your first name within double quotes.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

### --seed--

```rust
// Lesson #7
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- You should change the `println!` call to `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

### --seed--

```rust
// Lesson #8
fn main() {
  let first_name = "Quincy";
  println!("Hello, {}!", first_name);
}
```

### --tests--

- You should change the `println!` call to `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

### --seed--

```rust
// Lesson #9
fn main() {
  let first_name = "Quincy";
  println!("Hello, {}!", first_name);
}
```

### --tests--

- You should use `String::from()` to create a `String` with your first name.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

### --seed--

```rust
// Lesson #10
fn main() {
  let first_name = String::from("Quincy");
  println!("Hello, {}!", first_name);
}
```

### --tests--

- You should declare a variable `name` and assign it the value of `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- You should replace the second argument of `println!` with `name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

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

- You should have two `println!` calls immediately after one another.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- You should have the first `println!` use `name` and the second `println!` use `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

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

- You should have two `println!` calls immediately after one another.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- You should have the first `println!` use `name` and the second `println!` use `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

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

- You should have two `println!` calls immediately after one another.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- You should have the first `println!` use `name` and the second `println!` use `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- You should reference `first_name` when assigning it to `name`, by using `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

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

- Run your code with `cargo run --bin calculator`. You should see no errors.
- `null`

## 15

### --description--

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

- You should turn `first_name` into an owned value with `.to_owned()`.
- `first_name\.to_owned\(\)`
- You should concatenate your surname to the owned `first_name`.
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

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

- You should use the `.push_str()` method.
- `\.push_str\(`
- You should push your surname to `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

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

- You should use the `.push_str()` method.
- `\.push_str\(`
- You should push your surname to `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- You should make `first_name` mutable with `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

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

- You should use the `.push_str()` method.
- `\.push_str\(`
- You should push your surname to `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- You should make `first_name` mutable with `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

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

- Run your code with `cargo run --bin calculator`. You should see no errors.
- `null`

## 20

### --description--

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

- You should declare a variable named `first`.
- `let\s+first`
- You should assign `first` a single character in double quotes.
- `first\s*=\s*"\w"`

## 21

### --description--

### --seed--

```rust
fn main() {
  let first = "T";
}
```

### --tests--

- You should print the length of `first` and the number of characters in `first`. Example Output: `1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- You should change the value of `first` to be a string slice of `∞`.
- `first\s*=\s*"∞"`
- Your code should print `3 1`.
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- There are no tests for this lesson.
- `null`

## 24

### --description--

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

- You should have run the command `fcc reset 15`.
- `mod tests`

## 25

### --description--

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

- There are no Node tests for this lesson
- `null`

## 26

### --description--

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

- There are no Node tests for this lesson
- `null`

## 27

### --description--

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

- There are no Node tests for this lesson
- `null`

## 28

### --description--

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
    let out = output(-10, "+", 10, 0);
    assert_eq!(out, ());
  }
}
```

### --tests--

- There are no Node tests for this lesson
- `null`

## 29

### --description--

### --seed--

```rust
// Lesson #29
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

### --tests--

- There are no Node tests for this lesson
- `null`

## 30

### --description--

### --seed--

```rust
// Lesson #30
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

### --tests--

- You should print to the console any valid output.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

### --seed--

```rust
// Lesson #31
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

### --tests--

- You should declare a variable named `first_number`.
- `let first_number`
- You should declare a variable named `second_number`.
- `let second_number`
- You should declare a variable named `operator`.
- `let operator`

## 32

### --description--

### --seed--

```rust
// Lesson #32
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

### --tests--

- There are no Node tests for this lesson
- `null`

## 33

### --description--

### --seed--

```rust
// Lesson #33
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

### --tests--

- There are no Node tests for this lesson
- `null`

## 34

### --description--

### --seed--

```rust
// Lesson #34
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

### --tests--

- There are no Node tests for this lesson
- `null`

## 35

### --description--

### --seed--

```rust
// Lesson #35
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

### --tests--

- You should use the `match` operator.
- `match`

## 36

### --description--

### --seed--

```rust
// Lesson #36
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

### --tests--

- You should use an `|` operator.
- `|`

## 37

### --description--

### --seed--

```rust
// Lesson #37
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

### --tests--

- You should declare a new variable `result`
- `let\s+result`
- Your code should print `1 - 10 = -9` to the console.
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

### --seed--

```rust
// Lesson #38


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

### --tests--

- You should add `use std::env;` to the top of your script.
- `use\s+std::env\s*;`

## 39

### --description--

### --seed--

```rust
// Lesson #39
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

### --tests--

- You should declare a new variable named `args`
- `let\s+args`
- You should assign `args` a value of `env::args()`
- `=\s*env::args\(\)`

## 40

### --description--

### --seed--

```rust
// Lesson #40
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

### --tests--

- Running `cargo run --bin calculator` should print: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

### --seed--

```rust
// Lesson #41
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

### --tests--

- There are no Node tests for this lesson.
- `null`

## 42

### --description--

### --seed--

```rust
// Lesson #42
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

### --tests--

- You should access the first (`0`) element of the `env::args()` iterator.
- `args\.nth\(0\)`
- You should declare `args` as mutable with `let mut args =...`
- `let\s+mut\s+args`

## 43

### --description--

### --seed--

```rust
// Lesson #43
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

### --tests--

- You should assign `args.nth(0)` to `first_number`.
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- You should assign `args.nth(1)` to `operator`.
- `let\s+operator\s*=\s*args\.nth\(1\)`
- You should assign `args.nth(2)` to `second_number`.
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

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

### --tests--

- You should unwrap `first_number` with `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- You should unwrap `operator` with `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- You should unwrap `second_number` with `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- There are no Node tests for this lesson.
- `null`

## 46

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- There are no Node tests for this lesson.
- `null`

## 47

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- You should import the `Args` struct from the `std::env` module with `use std::env::Args`.
- `use\s+std::env::Args`
- You should annotate `args` with the type `Args`.
- `let\s+mut\s+args:\s*Args`
- You should annotate `first_number` with the type `String`.
- `let\s+first_number:\s*String`
- You should annotate `operator` with the type `String`.
- `let\s+operator:\s*String`
- You should annotate `second_number` with the type `String`.
- `let\s+second_number:\s*String`

## 48

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- You should combine both imports into a single import statement with `use std::env::{args, Args};`.
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- You should declare a variable named `first`.
- `let\s+first`
- You should declare a variable named `second`.
- `let\s+second`
- You should assign `first_number.parse::<i32>().unwrap()` to `first`.
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- You should assign `second_number.parse::<i32>().unwrap()` to `second`.
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

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
  use crate::output;
  use crate::operate;

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

### --tests--

- There are no Node tests for this lesson.
- `null`

## 51

### --description--

### --seed--

```rust
// Lesson #51
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
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10.0 + 10.0 = 0.0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate("+", -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5.0, 200.0);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12.0, -12.0);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12.0, -1.0);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12.0, -2.0);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12.0, 2.0);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12.0, 2.0);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1.0, 1.0);
  }
}
```

### --tests--

- You should change the types from `i32` to `f32`.
- `null`

## 52

### --description--

### --seed--

```rust
// Lesson #52
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(&operator, first, second);

  println!("{}", output(first, &operator, second, result));
}

fn output(first_number: f32, operator: &str, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: f32, second_number: f32) -> f32 {
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
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, "*", 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate("+", -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1.0, 1.0);
  }
}
```

### --tests--

- This is the final lesson. Congrats!
- `null`

## 53

### --description--

### --seed--

```rust
// Lesson #53
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(&operator, first, second);

  println!("{}", output(first, &operator, second, result));
}

fn output(first_number: f32, operator: &str, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: f32, second_number: f32) -> f32 {
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
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, "*", 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate("+", -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1.0, 1.0);
  }
}
```

### --tests--

- This is the final lesson. Congrats!
- `null`

## 54

### --description--

### --seed--

```rust
// Lesson #54
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(&operator, first, second);

  println!("{}", output(first, &operator, second, result));
}

fn output(first_number: f32, operator: &str, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: f32, second_number: f32) -> f32 {
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
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, "*", 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate("+", -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1.0, 1.0);
  }
}
```

### --tests--

- This is the final lesson. Congrats!
- `null`

## 55

### --description--

### --seed--

```rust
// Lesson #55
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first_number: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second_number: String = args.nth(0).unwrap();

  let first = first_number.parse::<f32>().unwrap();
  let second = second_number.parse::<f32>().unwrap();
  let result = operate(&operator, first, second);

  println!("{}", output(first, &operator, second, result));
}

fn output(first_number: f32, operator: &str, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: &str, first_number: f32, second_number: f32) -> f32 {
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
  use crate::output;
  use crate::operate;

  #[test]
  fn output_accepts_floating_point_numbers() {
    let out = output(-10.0, "*", 10.0, 0.0);
    assert_eq!(out, String::from("-10 * 10 = 0"));
  }
  #[test]
  fn operate_accepts_floating_point_numbers() {
    let op = operate("+", -10.5, 10.0);
    assert_eq!(op, -0.5);
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10.0, "+", 10.0, 0.0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate("+", -5.0, 200.0);
    assert_eq!(op, 195.0);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate("-", -12.0, -12.0);
    assert_eq!(op, 0.0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate("/", -12.0, -1.0);
    assert_eq!(op, 12.0);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate("*", -12.0, -2.0);
    assert_eq!(op, 24.0);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate("x", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  fn operate_handles_multiplcaiton_cap_x() {
    let op = operate("X", -12.0, 2.0);
    assert_eq!(op, -24.0);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate("invalid", 1.0, 1.0);
  }
}
```

### --tests--

- This is the final lesson. Congrats!
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
