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
  let first_name = String::from("Quincy");
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
