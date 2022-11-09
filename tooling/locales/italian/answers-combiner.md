# freeCodeCamp - Rust in Replit - Combinatore di Immagini

## 1

### --description--

Inizia creando un nuovo progetto chiamato `combiner`.

Esegui `fcc test 1` per vedere se hai completato correttamente l'attività.

### --seed--

```rust
// Lesson #1
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Dovresti vedere una nuova cartella `combiner` creata nella root.
- `Hello, world!`

## 2

### --description--

In questo progetto, creerai una CLI (combinatore) che accetta tre argomenti:

```bash
    $ combiner image1.png image2.png output.png
```

I primi due argomenti sono i percorsi delle immagini che desideri combinare. Il terzo argomento è il percorso dell'immagine di output.

Attività: apri `combiner/src/main.rs` ed esegui `cargo run --bin combiner` per vedere se l'applicazione è impostata correttamente.

Dovresti vedere `Hello, world!` stampato sulla console.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- L'output del tuo codice dovrebbe essere `Hello, world!`
- `getCommandOutput(Hello, world!)`

## 3

### --description--

Attività: definisci una funzione chiamata `get_nth_arg` che accetta un argomento `usize`.

Ricorda di importare secondo necessità.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

- Esegui `cargo test --bin combiner` per vedere se hai completato correttamente il compito.
- `null`
- Suggerimento: ricordati di importare la funzione nel modulo `tests`.
- `null`

## 4

### --description--

I prossimi test fanno uso di un crate esterno chiamato `regex`.

Attività: apri il file `Cargo.toml` nella root e aggiungi le seguenti righe:

```rust
    [dependencies]
    regex = "1.5.4"
```

Ciò installerà nel tuo progetto il crate `regex`, usato nella funzione `reg_with_con`. Puoi saperne di più su questo crate all'indirizzo: https://crates.io/crates/regex

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

- Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività. Nessun errore significa che hai finito.
- `null`

## 5

### --description--

Per utilizzare argomenti da riga di comando, è necessario utilizzare il modulo `std::env`.

Attività: all'interno di `get_nth_arg`, restituisci il valore spacchettato della chiamata dal metodo `nth` sulla funzione `args` con l'argomento `n`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

- Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività. Nessun errore significa che hai finito.
- `null`

## 6

### --description--

Idealmente, desideri memorizzare solo gli argomenti della riga di comando a cui sei interessato in una singola variabile.

Attività: all'interno di`main`, crea una variabile chiamata `args` e assegnale il valore di `Args {}`.

Esegui `fcc test 6` per vedere se hai completato correttamente l'attività.

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

- Dovresti creare una nuova variabile chiamata `args`.
- `let args`
- Dovresti assegnare ad `args` il valore di `Args {}`.
- `let\s+args\s*=\s*Args\s*\{\};`

## 7

### --description--

La sintassi `Args {}` è un costruttore di una struct chiamata `Args`. Tuttavia, non abbiamo ancora definito la struct.

Ecco un esempio di una struct che hai già usato:

```rust
    struct String {
      vec: Vec<u8>,
    }
```

La struct `String` consiste di un campo `vec`, che è un `Vec` di `u8`.

Attività: definisci una struct chiamata `Args` con visibilità globale.

Ricorda di importare secondo necessità.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Invece di scrivere `use crate::` per ogni funzione o struct nel modulo `tests`, puoi usare la parola chiave `super` con il selettore wildcard `*` per selezionare tutto nel modulo corrente.

Attività: sostituisci le chiamate `use crate::` con `use super::*`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

- Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività. Nessun errore significa che hai finito.
- `null`

## 9

### --description--

Attività: aggiungi un campo chiamato `image_1` alla struct `Args` e dagli il tipo corretto per superare i test.

Ricordati di adeguare la dichiarazione in `main` secondo necessità.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: per farti un'idea più chiara della struct `Args`, stampa il valore di `my_arg` nel test `args_struct_has_image_1_field`.

Esegui `cargo test --bin combiner -- --show-output`. Se vedi un errore, hai completato correttamente l'attività.

Il flag `--show-output` mostra lo stdout dei test.

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

Il tuo codice non può essere compilato, perché la macro `println!` non sa come formattare la struct `Args`.

Attività: segui il consiglio del compilatore per estendere il formattatore all'interno di `println!`.

Esegui `cargo test --bin combiner -- --show-output`. Dovresti vedere ancora un errore.

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

Il compilatore ti sta dicendo che stai cercando di usare la macro `println!` su un tipo che non implementa il tratto `Debug`.

Di solito, i tratti devono essere implementati per una struttura utilizzando la parola chiave `impl`. Tuttavia, in questo caso, puoi utilizzare l'attributo `derive` per implementare automaticamente il tratto `Debug`:

```rust
    #[derive(Debug)]
    struct MyStruct {
      field_1: String,
    }
```

Attività: implementa il tratto `Debug` per la struct `Args`.

Esegui `cargo test --bin combiner -- --show-output`. Dovresti vedere la struct `Args` stampata sulla console.

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

Potresti notare che `String::new()` usato in `main`. La funzione `new` è un costruttore comune per le struct. Per `String`, ha un aspetto simile a questo:

```rust
    impl String {
      fn new() -> Self {
        String::from("")
      }
    }
```

Quanto sopra implementa la funzione `new` per `String`. Il tipo del valore di ritorno è `Self`, che è il tipo di struct.

Attività: implementa la funzione `new` per `Args`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: invece di creare manualmente la struct `Args` in `main`, usa la funzione `new` per creare la struct.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: in `new`, invece di assegnare una `String` vuota a `image_1`, usa la funzione `get_nth_arg` per assegnare il valore del primo argomento **valido**.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

La tua applicazione dovrebbe accettare tre argomenti: `image_1`, `image_2` e `output`.

Attività: aggiungi i due campi mancanti alla struct `Args`. Tutti i campi dovrebbero usare lo stesso tipo.

Esegui `fcc test 16` per vedere se hai completato correttamente l'attività.

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

- Dovresti definire `Args` con un campo `image_1` di tipo `String`.
- `image_1:\s*String`
- Dovresti definire `Args` con un campo `image_2` di tipo `String`.
- `image_2:\s*String`
- Dovresti definire `Args` con un campo `output` di tipo `String`.
- `output:\s*String`

## 17

### --description--

Attività: Aggiorna la funzione `new` per assegnare valori validi a tutti i campi attesi.

Esegui `cargo test --bin combiner --show-output` per vedere se hai completato correttamente l'attività.

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

Testare la tua applicazione con argomenti, ora dovrebbe stampare la struct `Args` con gli argomenti come valori per i campi.

Attività: modifica il `println` in `main` per stampare il valore di `args`.

Esegui `cargo run --bin combiner first_arg second_arg third_arg`. Se vedi quanto segue, hai completato correttamente l'attività:

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

Prima che il file `main.rs` diventi troppo affollato, dovresti spostare la logica degli argomenti in un file a parte.

Attività: crea il file `combiner/src/args.rs`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: sposta la struct `Args`, la sua implementazione e la funzione `get_nth_arg` nel file `args.rs`. Quindi, commenta il contenuto all'interno della funzione `main` in modo che la tua app venga compilata.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

In Rust, tutto è privato per impostazione predefinita. Quindi, per rendere pubblica una funzione o una struct, puoi utilizzare la parola chiave `pub`:

```rust
    pub MyStruct {}
```

Attività: all'interno di `args.rs` rendi pubbliche la struct e la funzione.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per utilizzare il contenuto di un file esterno, occorre dichiararlo come modulo:

```rust
    mod my_file_name
```

Attività: in cima a `main.rs`, dichiara il file `args.rs` come un modulo.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Ora che `args` è stato dichiarato come un modulo da usare in `main.rs`, puoi utilizzare la parola chiave `use` per importare la struct `Args`.

Attività: all'interno di `main.rs`, importa la struct `Args`. Quindi, ritrasforma in codice il commento nella funzione `main`.

Esegui `cargo test --bin combiner`. Dovresti vedere un errore.

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

L'errore si verifica perché la funzione `new` implementata per `Args` non è pubblica.

Attività: all'interno di `args.rs` dichiara la funzione `new` come pubblica.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Al fine di codificare e decodificare le immagini, userai il crate `image`.

Attività: all'interno della radice `Cargo.toml`, aggiungi quanto segue alla sezione `dependencies`:

```rust
    image = "0.23.14"
```

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main.rs`, definisci una funzione chiamata `find_image_from_path` che prende una `String` come argomento.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Il compilatore sta mostrando un avvertimento che la funzione `find_image_from_path` è inutilizzata. Ciò diventerà fastidioso nel corso di questo progetto. Fortunatamente, è possibile abilitare gli attributi globali per sopprimere l'avviso.

Gli attributi globali usano la sintassi `#![feature(feature_name)]` e dovrebbero essere posizionati in cima al file.

Attività: all'interno di `main.rs`, usa la feature `allow` per abilitare globalmente `unused_variables` e `dead_code`.

Esegui `cargo test --bin combiner`. Se non vedi più avvertimenti, hai completato con successo questa lezione.

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

Attività: importa la struct `Reader` da `image::io` e, all'interno di `find_image_from_path`, assegna il valore spacchettato della funzione `Reader::open`, passando `path` come argomento, a una variabile chiamata `image_reader`. Quindi, restituisci `image_reader`.

Suggerimento: segui il consiglio del compilatore e importa i tipi necessari.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `find_image_from_path`, assegna il valore spacchettato del metodo `format` su `image_reader` a una variabile chiamata `image_format` e restituiscila.

Suggerimento: segui il consiglio del compilatore e importa i tipi necessari.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: rimuovi gli import inutilizzati da `main.rs`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Finora, non hai decodificato l'immagine. `Reader` ha un metodo `decode` che restituisce un `DynamicImage` in un `Result`.

Attività: all'interno di `find_image_from_path`, assegna il valore spacchettato del metodo `decode` su `image_reader` a una variabile chiamata `image`. Quindi, restituisci `image`.

_Suggerimento:_ segui il consiglio del compilatore e importa i tipi necessari.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Hai imparato a conoscere il tipo tupla vuota `()`. Adesso, userai una tupla per restituire più valori. A differenza di altri tipi, una tupla può contenere più di un tipo.

```rust
    // The Vec type can only contain one type.
    let my_vec = vec![1u8, 2u16, 3u32];
    // Tuples can contain multiple types.
    let my_tuple = (1u8, 2u16, 3u32);
```

Attività: da `find_image_from_path`, restituisci una tupla contenente `DynamicImage` e `ImageFormat` dell'immagine, in questo ordine.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Le tuple possono essere destrutturate in variabili come questa:

```rust
    let (x, y, z) = (1, 2, 3);
```

Attività: all'interno di `main`, destruttura la tupla restituita da `find_image_from_path` nelle variabili `image_1` e `image_1_format`. Dovresti chiamare `find_image_from_path` con il valore del campo `image_1` uguale ad `args`.

Esegui `cargo test --bin combiner`. Dovresti vedere un errore.

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

Il tuo codice ha un errore, perché il campo `image_1` su `args` non è pubblico. Quindi, non può essere utilizzato attraverso i moduli.

Attività: all'interno di `args.rs`, cambia tutti i campi della struct `Args` in modo che siano pubblici.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main`, destruttura la tupla restituita da `find_image_from_path` nelle variabili `image_2` e `image_2_format`. Dovresti chiamare `find_image_from_path` con il valore del campo `image_2` uguale ad `args`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Finora, hai avuto a che fare con alcune funzioni che hanno restituito un `Result`. Ora creerai un nuovo `Result`.

Un `Result` è un tipo che può essere `Ok` o `Err`. È comune restituire una tupla vuota quando una funzione ha successo e restituire un messaggio di errore quando una funzione fallisce:

```rust
    fn function_returns_result() -> Result<(), String> {
      if (condition) {
        return Ok(());
      } else {
        return Err(String::from("Error message"));
      }
    }
```

Attività: all'interno di `main.rs`, converti la funzione `main` per restituire un `Result`. Per ora, restituisci una tupla vuota per `Ok`, ma imposta il tipo del valore di ritorno di `Err` su `String`.

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

La tua applicazione sarà in grado di combinare solo due immagini dello stesso tipo.

Attività: pertanto, se `image_1_format` non è uguale a `image_2_format`, restituisci un messaggio di errore.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Un altro errore può verificarsi quando le due immagini non hanno la stessa dimensione. Fortunatamente, c'è una funzione che può essere usata per ridimensionare le immagini.

Attività: inizia creando una funzione chiamata `standardise_size` che richiede `image_1` e `image_2` come parametri.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per semplificare la combinazione delle immagini, è possibile ridimensionare l'immagine più grande alla dimensione dell'immagine più piccola. Per fare ciò, è necessario ottenere le dimensioni più piccole tra le due immagini.

Attività: crea una funzione chiamata `get_smallest_dimensions` che accetta due tuple come parametri. Ogni tupla dovrebbe prendere due elementi, ciascuno di tipo `u32`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

All'interno di `get_smallest_dimensions`, dovrai restituire le dimensioni con il numero più piccolo di pixel. Il numero di pixel è il prodotto della larghezza per l'altezza.

Attività: restituisci `dim_1` se il numero di pixel in `dim_1` è minore del numero di pixel in `dim_2`. Altrimenti restituisci `dim_2`.

Ricorda che puoi usare la notazione con il punto per accedere agli elementi di una tupla.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `standardise_size`, destruttura la tupla restituita da `get_smallest_dimensions` in due variabili `width` e `height`. Usa il valore di ritorno della chiamata del metodo `dimensions` su ogni `DynamicImage` da passare come argomento per `get_smallest_dimensions`.

_Suggerimento:_ segui il consiglio del compilatore per ottenere le dimensioni delle immagini.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `standardise_size`, stampa `width` e `height` sulla console.

Esegui `cargo test --bin combiner -- --show-output`. Se vedi `'width: 10, height: 10'` stampato sulla console, hai completato con successo l'attività.

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

Attività: all'interno di `standardise_size`, scrivi un'istruzione `if` per verificare se le dimensioni di `image_2` sono uguali alle dimensioni più piccole determinate precedentemente. Se lo sono, restituisci una tupla contenente `image_1` e `image_2`. Altrimenti, restituisci la stessa tupla.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Invece di restituire le immagini senza modifiche, dovresti ridimensionare l'immagine più grande. Puoi usare il metodo `resize_exact` che esiste per la struct `DynamicImage`. Il metodo `resize_exact` ha la forma:

```rust
    image_to_resize.resize_exact(new_width: u32, new_height: u32, filter: image::imageops::FilterType);
```

Attività: all'interno di `standardise_size`, ridimensiona la variabile dell'immagine corretta con le giuste dimensioni, usando il filtro `Triangle`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main` prima del return `Ok`, usa la funzione `standardise_size` per ridichiarare `image_1` e `image_2` con le dimensioni corrette.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per gestire l'output, è possibile creare una struct temporanea per mantenere i metadati per l'immagine di output.

Attività: crea una struct chiamata `FloatingImage` che ha i seguenti campi:

```rust
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
```

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: implementa una funzione chiamata `new` per `FloatingImage`. La funzione `new` dovrebbe prendere tre argomenti: `width: u32`, `height: u32`, `name: String`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Al fine di scrivere in modo efficiente i dati combinati dell'immagine di output, devi creare un buffer abbastanza grande da contenere i dati, in modo che non sia necessario allocare spazio in più.

Le immagini di grandi dimensioni possono avere una grande quantità di dati, quindi puoi sfruttare la numerazione facile da leggere di Rust, che separa il numero in gruppi di tre cifre:

```rust
    let difficult_to_read_number = 1325364955;
    let easy_to_read_number = 1_325_364_955;
```

Attività: all'interno di `new`, dichiara una variabile `buffer_capacity` e assegnale il valore `3655744` usando il numero facile da leggere.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Ora che hai una dimensione del buffer, devi creare un buffer di `Vec<u8>`. La struct `Vec` implementa una funzione `with_capacity`, che prende una capacità come argomento e restituisce un nuovo `Vec` con tale capacità.

Attività: all'interno di `new`, dichiara una variabile chiamata `buffer` e assegnale il valore della chiamata della funzione`with_capacity` con `buffer_capacity`.

_Suggerimento:_ segui il consiglio del compilatore di scrivere esplicitamente il tipo della variabile `buffer`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `new`, usa le variabili disponibili per restituire un'istanza della struct `FloatingImage`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main`, dichiara una nuova variabile `output` usando la funzione `new` della struct `FloatingImage`. Usa i metodi `width` e `height` della variabile `image_1` per i primi due argomenti e il campo `output` della variabile `args` per il terzo argomento.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: definisci una funzione chiamata `combine_images` che prende due `DynamicImage` come argomenti.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per elaborare le immagini, le convertirai in un vettore di pixel RGBA. I pixel sono memorizzati come `u8`, perché i loro valori sono compresi tra 0 e 255.

La struct `DynamicImage` implementa il metodo `to_rgba8`, che restituisce un `ImageBuffer` contenente un `Vec<u8>`, e `ImageBuffer` implementa il metodo `into_vec`, che restituisce il `Vec<u8>`.

Attività: all'interno di `combine_images`, dichiara una variabile `vec_1` e usa i metodi qui sopra per assegnarle `Vec<u8>`. Restituisci `vec_1`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: fai lo stesso della lezione precedente, ma su `image_2`, e restituisci la nuova variabile chiamata `vec_2` invece di `vec_1`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Ora che hai i valori dei pixel di ogni immagine in `vec_1` e `vec_2`, puoi combinarli in una immagine singola.

Attività: definisci una funzione chiamata `alternate_pixels` che accetta due `Vec<u8>` come argomenti.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Dovrai memorizzare i dati dei pixel dell'immagine combinata in una variabile. Per creare questa variabile, puoi utilizzare la macro `vec`, fornendo il tipo e la lunghezza del vettore:

```rust
    let my_vec = vec![10u8; 5];
    assert_eq!(my_vec.len(), 5);
    assert_eq!(my_vec, [10, 10, 10, 10, 10]);
```

Attività: all'interno di `alternate_pixels`, dichiara una variabile `combined_data` e usa la macro `vec` per creare un `Vec<u8>` di `0` della stessa lunghezza di `vec_1`. Restituisci `combined_data`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per iterare sui pixel nei vettori, userai un loop `while`. Un loop `while` segue la seguente sintassi:

```rust
    while condition {
      // Do stuff
    }
```

Dove `condition` è una espressione booleana che può essere `true` o `false`.

Attività: all'interno di `alternate_pixels`, prima di restituire `combined_data`, dichiara una variabile `i` e assegnale il valore `0`. Quindi, dichiara un loop `while` che viene eseguito mentre `i` è minore della lunghezza di `vec_1`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per impostare correttamente i set di pixel RGBA per il vettore di output, dovrai sostituire i valori `0u8` con i valori corretti.

Attività: definisci una funzione chiamata `set_rgba` che richiede 3 argomenti: un `Vec<u8>` e due `usize`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `set_rgba`, definisci una variabile mutabile chiamata `rgba` e assegnale un `Vec<u8>` vuoto. Prova a farlo senza la macro `vec`. Quindi, restituisci `rgba`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per iterare su un intervallo di valori, puoi usare un loop `for...in` con l'operatore a _intervalli letterali inclusivo a destra_:

```rust
    for i in 1..=5 {
      println!("{}", i);
    }
```

Il `=` all'interno dell'intervallo letterale è l'operatore a intervalli _inclusivo a destra_, nel senso che la fine dell'intervallo è compresa.

Attività: all'interno di `set_rgba`, prima di restituire `rgba`, itera sull'intervallo `start..=end` e aggiungi ogni valore nell'intervallo a `rgba`.

Suggerimento: devi affermare il corretto tipo di valore da aggiungere a `rgba`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

A volte, il recupero di un valore dall'interno di un vettore causa un panic, perché l'indice è fuori dall'intervallo. Per evitarlo, puoi utilizzare il metodo `get` su un vettore:

```rust
    let my_vec = vec![1, 2, 3];
    assert_eq!(my_vec.get(0), Some(&1));
    assert_eq!(my_vec.get(3), None);
```

Il metodo `get` restituisce un riferimento al valore all'indice dato oppure `None` se l'indice è fuori dall'intervallo.

Attività: all'interno di `set_rgba`, nel loop `for`, dichiara una variabile `val` contenente il `match` del metodo `get` su `vec`, usando `i` come argomento.

Per `Some`, assegna il valore a `val`. Per `None`, restituisci panic.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `set_rgba`, modifica il valore aggiunto a `vec` in `val`.

Esegui `cargo test --bin combiner`. Dovresti vedere un errore.

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

L'errore si verifica perché il tipo di `val` è `&u8` - un riferimento a un intero senza segno a 8-bit. Tuttavia, il tipo di `vec` dovrebbe essere `Vec<u8>`, non `Vec<&u8>`.

Per risolvere questo problema, il valore restituito dal metodo `get` può essere _dereferenziato_. Una dereferenziazione viene effettuata annotando il valore con `*`.:

```rust
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
```

Attività: all'interno di `set_rgba`, dereferenzia il valore assegnato a `val`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attualmente, il loop `while` può potenzialmente essere eseguito per sempre, quando `vec` contiene qualsiasi elemento. Puoi risolvere questo problema incrementando `i` in ogni iterazione del loop. Ecco alcuni modi comuni per incrementare un intero:

```rust
    let mut a = 0;
    a += 1;
    a++;
    a = a + 1;
    assert_eq!(a, 3);
```

Attività: all'interno di `alternate_pixels`, nel loop `while`, incrementa `i` di `4`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Hai un `Vec<u8>` di `0` e due `Vec<u8>` di `0-255`. Per sostituire una porzione di un vettore con un altro, è possibile utilizzare il metodo `splice`:

```rust
    let original_vec = vec![0, 1, 2, 3];
    let mut vec_to_change = vec![0u8; 4];
    vec_to_change.splice(2..4, original_vec[2..4].iter().cloned());
    assert_eq!(vec_to_change, vec![0, 0, 2, 3]);
```

Il metodo `splice` richiede due argomenti: l'intervallo del vettore da sostituire e i valori con cui sostituirlo.

Attività: all'interno di `alternate_pixels`, nel loop `while`, usa il metodo `splice` su `combined_data` da `i` a `i+3` e usa la funzione `set_rgba` per inserire i valori corretti da `vec_1`.

Eseguendo `cargo test --bin combiner` dovresti ottenere un errore. Esegui `fcc test 65` per vedere se hai completato correttamente l'attività.

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

- Non dovresti cambiare la condizione del loop `while`.
- `while\s+i\s*<\s*vec_1\.len\(\)\s*\{`
- Dovresti incrementare `i` di 4 alla fine del loop.
- `i\s*\+=\s*4;\s*\}`
- Dovresti chiamare il metodo `splice` su `combined_data`.
- `combined_data\.splice\(`
- Dovresti passare l'intervallo `i..i+4` oppure `i..=i+3` come primo argomento del metodo `splice`.
- `splice\(\s*(i..i\s*\+\s*4)|(i..=i\s*\+\s*3)\s*,`
- Dovresti passare `set_rgba(vec_1, i, i+3)` come secondo argomento a `splice`.
- `splice\(\s*(i..i+4)|(i..=i\s*\+\s*3)\s*,\s*set_rgba\(\s*vec_1\s*,\s*i\s*,\s*i\s*\+\s*3\s*\)\s*\)`

## 66

### --description--

L'errore sta dicendo che il valore `vec_1` viene spostato in `set_rgba` alla prima iterazione del loop. Quindi, nella seconda iterazione, quando la condizione `while` dovrebbe valutare `i < vec_1.len()`, `vec_1` non ha la visibilità per essere utilizzata.

Attività: all'interno di `alternate_pixels`, risolvi il problema passando un riferimento a `vec_1` a `set_rgba` e sistema le annotazioni di tipo necessarie.

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

Attualmente, `alternate_pixels` sta effettuando lo splicing di ogni set RGBA da `vec_1` in `combined_data`. Tuttavia, vuoi che ogni secondo set provenga da `vec_2`. Per raggiungere questo obiettivo, puoi utilizzare l'operatore remainder:

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

Attività: all'interno di `alternate_pixels`, usa l'operatore remainder per effettuare lo splicing di ogni secondo set di valori RGBA da `vec_2` in `combined_data`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `combine_images`, invece di restituire `vec_2`, restituisci il risultato della chiamata di `alternate_pixels` con `vec_1` e `vec_2`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main`, dichiara una variabile `combined_data` e assegnale il valore della chiamata di `combine_images` con `image_1` e `image_2`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Ora vuoi impostare i dati di `combined_data` nell'immagine di `output`. Per fare ciò, definirai un metodo su `FloatingImage` per impostare i campi `data` di `output` con il valore di `combined_data`.

Finora, hai implementato solo funzioni su struct. I metodi sono definiti in modo simile, ma prendono come primo argomento un'istanza della struct:

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

Dato che il valore dell'istanza di `MyStruct` deve essere modificato, il metodo `change_name` prende un riferimento mutabile all'istanza come primo argomento. _Nota che il metodo è ancora chiamato con un solo argomento_.

Attività: implementa un metodo `set_data` su `FloatingImage` che prende un `Vec<u8>` come argomento.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `set_data`, imposta il campo `data` dell'istanza in modo che sia uguale al valore dell'argomento `data`. Quindi, restituisci un risultato `Ok` con una tupla vuota come risposta.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Per gestire gli errori in modo più chiaro, puoi creare un _enum_ per rappresentare i possibili errori che possono verificarsi:

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

Gli enum possono essere usati sia come valori che come tipi. Hai già incontrato l'enum `Option`.

Attività: crea un `enum` chiamato `ImageDataErrors` e le due varianti `BufferTooSmall` e `DifferentImageFormats`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: ricava il tratto `Debug` per l'enum `ImageDataErrors`.

Esegui `cargo test --bin combiner -- --show-output`. Dovresti vedere quanto segue stampato sullo console:

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

Ora puoi usare il tuo enum per dare degli errori più specifici.

Attività: all'interno di `set_data`, restituisci l'errore appropriato se `data.len()` è maggiore di `self.data.capacity()`.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main`, invece di restituire un errore `String`, usa `ImageDataErrors` per restituire l'errore appropriato.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Attività: all'interno di `main`, chiama `set_data` su `output` con `combined_data`.

_Suggerimento:_ segui il consiglio del compilatore per far sì che il codice venga compilato

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Il compilatore ti sta avvertendo che non stai usando il `Result` restituito dalla chiamata di `set_data`. Potresti causare un panic per l'errore usando il metodo `unwrap`. Tuttavia, dato che l'errore viene gestito con un enum, puoi propagarlo usando l'operatore di _propagazione dell'errore_:

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

Usare l'operatore `?` permette a `MyError` di propagarsi al chiamante.

Attività: segui i consigli del compilatore nell'avvertimento per propagare l'errore.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

Infine, puoi salvare la nuova immagine in un file. Il crate `image` ha una funzione `save_buffer_with_format` con la seguente forma:

```rust
    fn save_buffer_with_format(path: AsRef<Path>, buf: &[u8], width: u32, height: u32, color: image::ColorType, format: image::ImageFormat) -> image::ImageResult<()>;
```

Vedendo come `AsRef` è implementata per `String`, un argomento di tipo `String` può essere usato per `path`.

Attività: all'interno di `main`, usa le proprietà corrette di `output` come primi quattro argomenti, `Rgba8` come argomento di colore e `image_1_format` come argomento di formato. Spacchetta il risultato di `save_buffer_with_format`.

Esegui `cargo run --bin combiner -- ./images/fcc_glyph.png ./images/pro.png example.png`. Se il file `example.png` esiste ed è una combinazione della mia foto profilo con il logo di freeCodeCamp, hai completato l'attività.

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

Attività: esegui il build della versione release del tuo `combiner` CLI.

Esegui `cargo test --bin combiner` per vedere se hai completato correttamente l'attività.

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

- Puoi usare il comando `cargo build --bin combiner --release` per fare il build del binario.
- `null`

## 80

### --description--

Congratulazioni per aver terminato il corso **Rust in Replit**!

Adesso puoi sbizzarrirti con il tuo codice e il tuo nuovo strumento per combinare immagini.

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

- Questa è l'ultima lezione. Congratulazioni!
- `null`

## 81

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Segnaposto
- `null`
