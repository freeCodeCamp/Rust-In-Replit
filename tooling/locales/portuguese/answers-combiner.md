# freeCodeCamp - curso de Rust no Replit - Combinador de imagens

## 1

### --description--

Comece criando um projeto chamado `combiner`.

Execute `fcc test 1` para ver se você completou a tarefa corretamente.

### --seed--

```rust
// Lesson #1
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Você deve ver um novo diretório `combiner` criado na raiz do projeto.
- `Hello, world!`

## 2

### --description--

Neste projeto, você criará uma interface de linha de comando (o combiner) que precisa de três argumentos:

```bash
    $ combiner image1.png image2.png output.png
```

Os primeiros dois argumentos são os caminhos para as imagens que você deseja combinar. O terceiro argumento é o caminho para a imagem resultante.

Tarefa: abra o arquivo `combiner/src/main.rs` e execute `cargo --bin combiner` para ver se a aplicação está configurada corretamente.

Você deve ver `Hello, world!` mostrado no console.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- O código deve retornar `Hello, world!`
- `getCommandOutput(Hello, world!)`

## 3

### --description--

Tarefa: defina uma função chamada `get_nth_arg` que recebe um argumento `usize`.

Lembre-se de importar conforme necessário.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

- Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.
- `null`
- Dica: Lembre-se de importar a função no módulo `tests`.
- `null`

## 4

### --description--

Os próximos testes fazem uso de uma crate externa chamada `regex`.

Tarefa: abra o arquivo `Cargo.toml` no diretório raiz do projeto e adicione as seguintes linhas:

```rust
    [dependencies]
    regex = "1.5.4"
```

Isso instalará a crate `regex` no projeto, que será usada na função `reg_with_con`. Você pode saber mais sobre esta crate em: https://crates.io/crates/regex

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

- Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente. Se não houver nenhum erro, significa que tudo está pronto.
- `null`

## 5

### --description--

Para usar os argumentos da linha de comando, você precisará usar o módulo `std::env`.

Tarefa: dentro de `get_nth_arg`, retorne o valor retirado de seu invólucro ao chamar o método `nth` na função `args` com o argumento `n`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

- Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente. Se não houver nenhum erro, significa que tudo está pronto.
- `null`

## 6

### --description--

Idealmente, você quer armazenar apenas os argumentos da linha de comando em que você está interessado em uma única variável.

Tarefa: dentro de `main`, crie uma variável chamada `args` e atribua a ela o valor de `Args {}`.

Execute `fcc test 6` para ver se você completou a tarefa corretamente.

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

- Você deve criar uma variável chamada `args`.
- `let args`
- Você deve atribuir para `args` o valor de `Args {}`.
- `let\s+args\s*=\s*Args\s*\{\};`

## 7

### --description--

A sintaxe `Args {}` é um construtor para uma estrutura chamada `Args`. No entanto, ainda não definimos a estrutura.

Aqui está um exemplo de uma estrutura que você já usou:

```rust
    struct String {
      vec: Vec<u8>,
    }
```

A estrutura `String` consiste de um campo de `vec`, que é um `Vec` de `u8`s.

Tarefa: no escopo global, defina uma estrutura chamada `Args`.

Lembre-se de importar conforme necessário.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Em vez de escrever `use crate::` para cada função ou estrutura no módulo `tests`, você pode usar a palavra-chave `super` com o seletor `*` para selecionar tudo no módulo atual.

Tarefa: substitua as chamadas para `use crate::` por `use super::*`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

- Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente. Se não houver nenhum erro, significa que tudo está pronto.
- `null`

## 9

### --description--

Tarefa: adicione um campo chamado `image_1` na estrutura `Args` e indique o tipo correto para passar nos testes.

Lembre-se de ajustar a declaração em `main` conforme necessário.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: para entender melhor a estrutura `Args`, mostre o valor de `my_arg` no teste `args_struct_has_image_1_field`.

Execute `cargo test --bin combiner -- --show-output`. Se aparecer um erro, você concluiu a tarefa corretamente.

A opção `--show-output` mostra o stdout dos testes.

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

O código não pôde compilar, porque a macro `println!` não sabe como formatar a estrutura `Args`.

Tarefa: siga o conselho do compilador para estender o formatador dentro de `println!`.

Execute `cargo test --bin combiner -- --show-output`. Ainda deve mostrar um erro.

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

O compilador está dizendo que você está tentando usar a macro `println!` em um tipo que não implementa o traço `Debug`.

Geralmente, traços precisam ser implementados em uma estrutura usando a palavra-chave `impl`. No entanto, neste caso, você pode usar o atributo `derive` para implementar automaticamente o traço `Debug`:

```rust
    #[derive(Debug)]
    struct MyStruct {
      field_1: String,
    }
```

Tarefa: implemente o traço `Debug` na estrutura `Args`.

Execute `cargo test --bin combiner -- --show-output`. Você deve ver a estrutura `Args` mostrada no console.

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

Você pode notar a `String::new()` usada em `main`. A função `new` é um construtor comum para estruturas. Para `String`, é algo assim:

```rust
    impl String {
      fn new() -> Self {
        String::from("")
      }
    }
```

O código acima implementa a função `new` para `String`. O tipo de retorno é `Self`, que é o tipo da estrutura.

Tarefa: implemente a função `new` para `Args`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: em vez de criar manualmente a estrutura `Args` em `main`, use a função `new` para criar a estrutura.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `new`, em vez de atribuir uma `String` vazia para `image_1`, use a função `get_nth_arg` para atribuir o valor do primeiro argumento **válido**.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

A aplicação deve esperar três argumentos: `image_1`, `image_2` e `output`.

Tarefa: adicione os dois campos que estão faltando na estrutura `Args`. Todos os campos devem usar o mesmo tipo.

Execute `fcc test 16` para ver se você completou a tarefa corretamente.

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

- Você deve definir `Args` com um campo `image_1` do tipo `String`.
- `image_1:\s*String`
- Você deve definir `Args` com um campo `image_2` do tipo `String`.
- `image_2:\s*String`
- Você deve definir `Args` com um `output` do tipo `String`.
- `output:\s*String`

## 17

### --description--

Tarefa: atualize a função `new` para atribuir valores válidos para todos os campos esperados.

Execute `cargo test --bin combiner -- --show-output` para ver se você completou a tarefa corretamente.

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

Testar sua aplicação com argumentos agora deve mostrar a estrutura `Args` com os argumentos como valores para os campos.

Tarefa: altere `println` em `main` para mostrar o valor de `args`.

Execute `cargo run --bin combiner first_arg second_arg third_arg`. Se ver o seguinte, você completou a tarefa corretamente:

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

Antes do arquivo `main.rs` ficar muito desordenado, você deve mover a lógica do argumento para um arquivo próprio.

Tarefa: crie o arquivo `combiner/src/args.rs`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: mova a estrutura `Args`, a implementação e a função `get_nth_arg` para o arquivo `args.rs`. Em seguida, comente o conteúdo dentro da função `main` para que a aplicação seja compilada.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

No Rust, tudo é privado por padrão. Então, para deixar a função e a estrutura públicas, você pode usar a palavra-chave `pub`:

```rust
    pub MyStruct {}
```

Tarefa: dentro de `args.rs` deixe a estrutura e a função públicas.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para utilizar o conteúdo de um arquivo externo, ele precisa ser declarado como um módulo:

```rust
    mod my_file_name
```

Tarefa: no topo de `main.rs`, declare o arquivo `args.rs` como um módulo.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Agora que `args` foi declarado como um módulo para uso dentro de `main.rs`, você pode utilizar a palavra-chave `use` para importar a estrutura `Args`.

Tarefa: dentro de `main.rs`, importe a estrutura `Args`. Então, descomente o código comentado na função `main`.

Execute `cargo test --bin combiner`. Você deve ver um erro.

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

O erro ocorre porque a função `new` implementada para `Args` não é pública.

Tarefa: dentro de `args.rs` declare a função `new` como pública.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

A fim de codificar e decodificar as imagens, você usará a crate `image`.

Tarefa: dentro da raiz do arquivo `Cargo.toml`, adicione o seguinte código à seção `dependencies`:

```rust
    image = "0.23.14"
```

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main.rs`, defina uma função chamada `find_image_from_path`, que recebe uma `String` como argumento.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

O compilador está mostrando um aviso de que a função `find_image_from_path` não está sendo utilizada. Isso será um incômodo, ao longo deste projeto. Felizmente, você pode habilitar atributos globais para ocultar o aviso.

Atributos globais usam a sintaxe `#![feature(feature_name)]` e devem ser colocados no topo do arquivo.

Tarefa: dentro de `main.rs`, use o recurso `allow` para ativar globalmente `unused_variables` e `dead_code`.

Execute `cargo test --bin combiner`. Se você não ver mais o aviso, concluiu esta aula com sucesso.

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

Tarefa: importe a estrutura `Reader` de `image::io` e, dentro de `find_image_from_path`, atribua o valor removido do invólucro da função `Reader::open`, passando `path` como argumento para uma variável chamada `image_reader`. Em seguida, retorne `image_reader`.

Dica: siga o conselho do compilador e importe os tipos necessários.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `find_image_from_path`, atribua o valor removido do invólucro do método `format` em `image_reader` para uma variável chamada `image_format` e retorne-o.

Dica: siga o conselho do compilador e importe os tipos necessários.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: remova as importações não utilizadas de `main.rs`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Até agora, você não decodificou a imagem. O `Reader` tem um método `decode` que retorna uma `DynamicImage` em um `Result`.

Tarefa: dentro de `find_image_from_path`, atribua o valor removido do invólucro do método `decode` em `image_reader` para uma variável chamada `image`. Em seguida, retorne `image`.

_Dica:_ siga o conselho do compilador e importe os tipos necessários.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Você aprendeu sobre o tipo tupla vazia `()`. Agora, você usará uma tupla para retornar vários valores. Diferente de outros tipos, uma tupla pode conter mais de um tipo.

```rust
    // The Vec type can only contain one type.
    let my_vec = vec![1u8, 2u16, 3u32];
    // Tuples can contain multiple types.
    let my_tuple = (1u8, 2u16, 3u32);
```

Tarefa: a partir de `find_image_from_path`, retorne uma tupla que contenha a `DynamicImage` e o `ImageFormat` da imagem, nesta ordem.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tuplas podem ser destruturadas em variáveis como esta:

```rust
    let (x, y, z) = (1, 2, 3);
```

Tarefa: dentro de `main`, desestruture a tupla retornada de `find_image_from_path` nas variáveis `image_1` e `image_1_format`. Você deve chamar `find_image_from_path` com o valor do campo `image_1` de `args`.

Execute `cargo test --bin combiner`. Você deve ver um erro.

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

O código tem um erro, pois o campo `image_1` em `args` não é público. Assim, não pode ser utilizado nos outros módulos.

Tarefa: dentro de `args.rs`, altere todos os campos da estrutura `Args` para público.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main`, desestruture a tupla retornada de `find_image_from_path` nas variáveis `image_2` e `image_2_format`. Você deve chamar `find_image_from_path` com o valor do campo `image_2` de `args`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Até agora, você lidou com poucas funções que retornaram um `Result`. Agora, você vai criar um novo `Result`.

Um `Result` é um tipo que pode ser `Ok` ou `Err`. É comum retornar uma tupla vazia quando uma função foi bem-sucedida e retornar uma mensagem de erro quando uma função falhar:

```rust
    fn function_returns_result() -> Result<(), String> {
      if (condition) {
        return Ok(());
      } else {
        return Err(String::from("Error message"));
      }
    }
```

Tarefa: dentro de `main.rs`, converta a função `main` para retornar um `Result`. Por enquanto, apenas retorne uma tupla vazia no `Ok`, mas defina o tipo do retorno de `Err` para `String`.

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

A aplicação será capaz de combinar apenas duas imagens do mesmo tipo.

Tarefa: assim, se `image_1_format` não é igual a `image_2_format`, retorne uma mensagem de erro.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Outro erro que pode ocorrer é quando as duas imagens não são do mesmo tamanho. Felizmente, há uma função que pode ser usada para redimensionar as imagens.

Tarefa: comece criando uma função chamada `standardise_size` que recebe `image_1` e `image_2` como parâmetros.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para tornar a combinação de imagens mais simples, você pode redimensionar a maior imagem para o tamanho da menor imagem. Para fazer isso, você precisa obter as menores dimensões das duas imagens.

Tarefa: crie uma função chamada `get_smallest_dimensions` que recebe duas tuplas como parâmetros. Cada tupla deve receber dois elementos, cada um com tipo `u32`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Dentro de `get_smallest_dimensions`, você precisará retornar as dimensões com o menor número de pixels. O número de pixels é o produto da largura e da altura.

Tarefa: retorne `dim_1` se o número de pixels em `dim_1` for menor que o número de pixels em `dim_2`. Caso contrário, retorne `dim_2`.

Lembre-se de que você pode usar a notação de ponto para acessar os elementos de uma tupla.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `standard _size`, desestruture a tupla retornada de `get_smallest_dimensions` em duas variáveis `width` e `height`. Use o retorno do método `dimensions` em cada `DynamicImage` para passar como argumentos para `get_smallest_dimensions`.

_Dica:_ siga o conselho do compilador para conseguir as dimensões das imagens.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `standardise_size`, mostre `width` e `height` no console.

Execute `cargo test --bin combiner -- --show-output`. Se você ver `'width: 10, height: 10'` no console, a tarefa foi concluída com sucesso.

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

Tarefa: dentro de `standardise_size`, escreva uma instrução `if` para verificar se as dimensões de `image_2` são iguais às dimensões menores previamente determinadas. Se forem, retorne uma tupla contendo `image_1` e `image_2`. Caso contrário, retorne a mesma tupla.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Ao invés de retornar as imagens inalteradas, você deve redimensionar a imagem maior. Você pode usar o método `resize_exact` que existe na estrutura `DynamicImage`. O método `resize_exact` assume a forma:

```rust
    image_to_resize.resize_exact(new_width: u32, new_height: u32, filter: image::imageops::FilterType);
```

Tarefa: dentro de `standardise_size`, redimensione a variável da imagem correta para as dimensões corretas, usando o filtro `Triangle`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main` antes do retorno de `Ok`, use a função `standardise_size` para redeclarar `image_1` e `image_2` como o tamanho correto.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para lidar com o resultado, uma estrutura temporária pode ser criada para guardar os metadados para a imagem resultante.

Tarefa: crie uma estrutura chamada `FloatingImage` com os seguintes campos:

```rust
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
```

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: implemente uma função chamada `new` para `FloatingImage`. A função `new` deve receber três argumentos: `width: u32`, `height: u32`, `name: String`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para escrever eficientemente os dados da imagem combinada na imagem resultante, você precisa criar um buffer de tamanho suficiente para guardar os dados, então nenhum espaço extra precisará ser alocado.

Imagens grandes podem ter uma grande quantidade de dados, assim você pode tirar proveito da numeração fácil de ler do Rust, que separa o número em grupos de três algarismos:

```rust
    let difficult_to_read_number = 1325364955;
    let easy_to_read_number = 1_325_364_955;
```

Tarefa: dentro de `new`, declare uma variável chamada `buffer_capacity` e atribua o valor de `3655744` usando o número fácil de ler.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Agora que você tem um tamanho de buffer, precisa criar um buffer de `Vec<u8>`. A estrutura `Vec` implementa uma função `with_capacity`, que precisa de uma capacidade como argumento e retorna um novo `Vec` com essa capacidade.

Tarefa: dentro de `new`, declare uma variável chamada `buffer`, e atribua a ela o valor retornado da função `with_capacity` com `buffer_capacity`.

_Dica:_ siga o conselho do compilador para tipar explicitamente a variável `buffer`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `nova`, use as variáveis disponíveis para retornar uma instância da estrutura `FloatingImage`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main`, declare uma nova variável `output` usando a função `new` da estrutura `FloatingImage`. Use os métodos `width` e `height` da variável `image_1` para os dois primeiros argumentos e o campo `output` da variável `args` para o terceiro argumento.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: defina uma função chamada `combine_images` que recebe dois `DynamicImage`s como argumentos.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para processar as imagens, você as converterá em um vetor de pixels RGBA. Os pixels são armazenados como `u8`s, porque seus valores estão entre 0 e 255.

A estrutura `DynamicImage` implementa o método `to_rgba8`, que retorna um `ImageBuffer` contendo um `Vec<u8>` e o `ImageBuffer` implementa o método `into_vec`, que retorna o `Vec<u8>`.

Tarefa: dentro de `combine_images`, declare uma variável `vec_1` e use os métodos abaixo para atribuir `Vec<u8>` a ela. Retorne `vec_1`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: faça o mesmo que na lição anterior, mas em `image_2`, e retorne a nova variável chamada `vec_2` em vez de `vec_1`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Agora que você tem os valores de pixel de cada imagem em `vec_1` e `vec_2`, você pode combiná-los em uma única imagem.

Tarefa: defina uma função chamada `alternate_pixels` que recebe dois `Vec<u8>`s como argumentos.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Você precisará armazenar os dados de pixel da imagem combinada em uma variável. Para criar essa variável, você pode usar a macro `vec`, fornecendo o tipo e o comprimento do vetor:

```rust
    let my_vec = vec![10u8; 5];
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec, [10, 10, 10, 10, 10]);
```

Tarefa: dentro de `alternate_pixels`, declare uma variável `combined_data`, e use a macro `vec` para criar um `Vec<u8>` de `0` do mesmo tamanho que `vec_1`. Retorne `combined_data`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para iterar sobre os pixels nos vetores, você usará um laço de repetição `while`. Um laço `while` segue essa sintaxe:

```rust
    while condition {
      // Do stuff
    }
```

Onde `condition` é uma expressão booleana que é avaliada como `true` ou `false`.

Tarefa: dentro de `alternate_pixels`, antes de retornar `combined_data`, declare uma variável `i` e atribua o valor de `0`. Em seguida, declare um laço `while` que é executado enquanto `i` é menor que o comprimento de `vec_1`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para definir corretamente os conjuntos de pixels RGBA para o vetor resultante, você substituirá os valores `0u8` pelos valores corretos.

Tarefa: defina uma função chamada `set_rgba`, que recebe 3 argumentos: Um `Vec<u8>` e dois `usize`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `set_rgba`, defina uma variável mutável chamada `rgba` e atribua a ela um `Vec<u8>` vazio. Tente fazer isso sem a macro `vec`. Então, retorne `rgba`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para iterar sobre um intervalo de valores, você pode usar um laço `for...in` com o operador _literal de intervalo que inclui o número à direita_:

```rust
    for i in 1..=5 {
      println!("{}", i);
    }
```

O `=` dentro do intervalo literal é o operador de intervalo _que inclui à direita_, ou seja, o final está incluído.

Tarefa: dentro de `set_rgba`, antes de retornar `rgba`, itere ao longo do intervalo `start..=end` e armazene cada valor do intervalo em `rgba`.

Dica: você precisará confirmar o tipo correto do valor que está sendo armazenado em `rgba`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Às vezes, recuperar um valor de dentro do vetor causa pânico, porque o índice está fora dos limites. Para evitar isso, você pode usar o método `get` em um vetor:

```rust
    let my_vec = vec![1, 2, 3];
    assert_eq!(my_vec.get(0), Some(&1));
    assert_eq!(my_vec.get(3), None);
```

O método `get` retorna uma referência ao valor no índice especificado ou `None` se o índice estiver fora dos limites.

Tarefa: dentro de `set_rgba`, dentro do laço `for`, declare uma variável `val` que é o `match` do método `get` em `vec` usando `i` como argumento.

Em `Some`, atribua o valor para `val`. Em `None`, retorne pânico.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `set_rgba`, altere o valor armazenado para `vec` para estar em `val`.

Execute `cargo test --bin combiner`. Você deve ver um erro.

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

O erro acontece por causa que o tipo de `val` é `&u8` - uma referência a um inteiro unsigned de 8 bits. No entanto, o tipo de `vec` deve ser `Vec<u8>`, não `Vec<&u8>`.

Para corrigir isso, o valor retornado do método `get` pode ser _desreferenciado_. Uma desreferência é feita anotando o valor com `*`.:

```rust
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
```

Tarefa: dentro de `set_rgba`, desfaça a referência ao valor atribuído a `val`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Atualmente, o laço `while` tem o potencial de ser executado para sempre, quando `vec` contém quaisquer elementos. Você pode corrigir isso incrementando `i` em cada iteração do laço. Aqui estão algumas maneiras comuns de incrementar um número inteiro:

```rust
    let mut a = 0;
    a += 1;
    a++;
    a = a + 1;
    assert_eq!(a, 3);
```

Tarefa: dentro de `alternate_pixels`, no laço `while`, incremente `i` em `4`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Você tem um `Vec<u8>` de `0` e dois `Vec<u8>`s de `0-255`. Para substituir um pedaço de um vetor por outro, você pode usar o método `splice`:

```rust
    let original_vec = vec![0, 1, 2, 3];
    let mut vec_to_change = vec![0u8; 4];
    vec_to_change.splice(2..4, original_vec[2..4].iter().cloned());
    assert_eq!(vec_to_change, vec![0, 0, 2, 3]);
```

O método `splice` recebe dois argumentos: o intervalo do vetor para substituir, e os valores para substituí-lo.

Tarefa: dentro de `alternate_pixels`, no laço `while`, use o método `splice` em `combined_data`, de `i` para `i+3`, e use a função `set_rgba` para inserir os valores corretos de `vec_1`.

Executar `cargo test --bin combiner` deve mostrar um erro. Execute `fcc test 65` para ver se você completou a tarefa corretamente.

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

- Você não deve alterar a condição do laço `while`.
- `while\s+i\s*<\s*vec_1\.len\(\)\s*\{`
- Você deve incrementar `i` em 4 ao final do laço.
- `i\s*\+=\s*4;\s*\}`
- Você deve chamar o método `splice` em `combined_data`.
- `combined_data\.splice\(`
- Você deve passar o intervalo `i..i+4` ou `i..=i+3` para o primeiro argumento do método `splice`.
- `splice\(\s*(i..i\s*\+\s*4)|(i..=i\s*\+\s*3)\s*,`
- Você deve passar `set_rgba(vec_1, i, i+3)` como o segundo argumento de `splice`.
- `splice\(\s*(i..i+4)|(i..=i\s*\+\s*3)\s*,\s*set_rgba\(\s*vec_1\s*,\s*i\s*,\s*i\s*\+\s*3\s*\)\s*\)`

## 66

### --description--

O erro está dizendo que o valor `vec_1` está sendo movido para `set_rgba` na primeira iteração do laço. Então, na segunda iteração, quando a condição do `while` devia ser `i < vec_1. en()`, `vec_1` não está no escopo a ser usado.

Tarefa: dentro de `alternate_pixels`, corrija o problema passando uma referência para `vec_1` para `set_rgba` e corrija as anotações de tipo necessárias.

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

Atualmente, `alternate_pixels` está dividindo cada conjunto RGBA de `vec_1` em `combined_data`. No entanto, você quer que cada segundo conjunto faça parte de `vec_2`. Para fazer isso, você pode usar o operador de resto:

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

Tarefa: dentro de `alternate_pixels`, use o operador de resto para dividir cada segundo conjunto de valores RGBA de `vec_2` para `combined_data`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `combine_images`, em vez de retornar `vec_2`, retorne o resultado de chamar `alternate_pixels` com `vec_1` e `vec_2`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main`, declare uma variável `combined_data`, e atribua a ela o valor da chamada de `combine_images` com `image_1` e `image_2`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Agora, você quer definir os dados de `combined_data` em `output` da imagem. Para fazer isso, você vai definir um método em `FloatingImage` para definir o campo `data` de `output` para o valor de `combined_data`.

Até agora, você implementou apenas funções em estruturas. Métodos são definidos de maneira similar, mas eles precisam de uma instância da estrutura como primeiro argumento:

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

Como o valor da instância de `MyStruct` precisa ser alterado, o método `change_name` recebe uma referência mutável para a instância como seu primeiro argumento. _Observe que o método ainda é chamado apenas com um argumento_.

Tarefa: implemente um método `set_data` em `FloatingImage` que leva um `Vec<u8>` como argumento.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `set_data`, defina o campo de `data` da instância para que seja igual ao valor do argumento `data`. Em seguida, retorne um resultado `Ok` com uma tupla vazia como resposta.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Para lidar com erros de forma mais clara, você pode criar um _enum_ para representar os possíveis erros que podem ocorrer:

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

Enums podem ser usados como valores ou tipos. Você já encontrou o enum `Option`.

Tarefa: crie um `enum` chamado `ImageDataErrors` e que tenha duas variantes `BufferTooSmall` e `DifferentImageFormats`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: derive o traço `Debug` para o enum `ImageDataErrors`.

Execute `cargo test --bin combiner -- --show-output`. Você deve ver o seguinte mostrado no console:

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

Agora você pode usar o enum para indicar erros mais específicos.

Tarefa: dentro de `set_data`, retorne o erro apropriado, se `data.len()` for maior que `self.data.capacity()`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main`, em vez de retornar um erro `String`, use `ImageDataErrors` para retornar o erro apropriado.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Tarefa: dentro de `main`, chame `set_data` no `output` com `combined_data`.

_Dica:_ siga o conselho do compilador para fazer o código compilar

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

O compilador está avisando que você não está usando o `Result` retornado ao chamar `set_data`. Você pode gerar o pânico no erro usando o método `unwrap`. No entanto, como o erro está sendo tratado com um enum, você pode propagar o erro usando o operador de _propagação de erro_:

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

Usar o operador `?` permite que `MyError` propague para a função que o chamou.

Tarefa: siga o conselho do compilador para propagar o erro.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

Por fim, você pode salvar a nova imagem em um arquivo. A crate `image` tem uma função `save_buffer_with_format` com o seguinte formato:

```rust
    fn save_buffer_with_format(path: AsRef<Path>, buf: &[u8], width: u32, height: u32, color: image::ColorType, format: image::ImageFormat) -> image::ImageResult<()>;
```

Como `AsRef` está implementado para `String`, um argumento do tipo `String` pode ser usado para `path`.

Tarefa: dentro de `main`, use as propriedades de `output` corretas como os primeiros quatro argumentos, `Rgba8` como argumento de cores, e `image_1_format` como argumento de formato. Remova do invólucro o resultado de `save_buffer_with_format`.

Execute `cargo run --bin combiner -- ./images/fcc_glyph.png ./images/pro.png example.png`. Se o arquivo `exemplo.png` existe e se ele for uma combinação da minha imagem do perfil com o logotipo do freeCodeCamp, você completou a tarefa.

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

Tarefa: crie uma versão de lançamento da interface de linha de comando do `combiner`.

Execute `cargo test --bin combiner` para ver se você completou a tarefa corretamente.

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

- Você pode usar o comando `cargo build --bin combiner --release` para gerar o binário.
- `null`

## 80

### --description--

Parabéns por terminar o curso de **Rust no Replit**!

Agora você pode brincar com o código e com a nova ferramenta de linha de comando para combinar suas próprias imagens.

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

- Esta é a lição final. Parabéns!
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
