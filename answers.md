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
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

```rust
fn main() {
  let firstName = "Quincy";
  println!("Hello, world!");
}
```

- You should have a variable `first_name` and give it a value of your first name within double quotes.
- `let\s+first_name\s*=\*+\"\w+\"\s*;`

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
fn main() {

}

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
fn main() -> f32 {
  24.5
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
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}
```

- There are no Node tests for this lesson
- `null`

## Final

```rust
fn main() {
  let mut args = std::env::args();
  let first_number = args.nth(1).unwrap();
  let operator = args.nth(0).unwrap();
  let second_number = args.nth(0).unwrap();

  let first = first_number.parse::<i32>().unwrap();
  let second = second_number.parse::<i32>().unwrap();
  let result = operate(operator.as_str(), first, second);

  let output = output(first, operator, second, result);
  println!("{}", output);
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
    _ => 0,
  }
}

#[cfg(test)]
mod tests {
  use crate::operate;
  #[test]
  fn adition_of_integers() {
    assert_eq!(operate("+", 1, 1), 2);
  }
}
```
