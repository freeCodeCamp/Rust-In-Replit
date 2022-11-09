# freeCodeCamp - Risposte del corso Rust in Replit

## 1

### --description--

I principali strumenti all'interno dell'ecosistema Rust sono:

- rustc Il compilatore che prende il codice Rust e lo compila in binario (codice leggibile dalla macchina)
- rustup L'utilità da riga di comando per installare e aggiornare Rust
- cargo Il sistema di building e gestore di pacchetti di Rust (lavorerai con questo)

Attività: crea un nuovo progetto Rust eseguendo il seguente comando nel prompt:

```bash
    $ cargo new calculator
```

### --seed--

```rust

```

### --tests--

- Non ci sono test di RegEx per questa lezione.
- `null`

## 2

### --description--

Hai appena creato un nuovo progetto Rust nella cartella `calculator/`.

Cargo ha creato il boilerplate per un 'Hello World'.

Attività: apri il file `calculator/src/main.rs`.

Questo è il file di default che Cargo utilizza per il binario dell'applicazione.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Dovresti avere il file del progetto `calculator/src/main.rust`.
- `null`

## 3

### --description--

Questo file contiene una dichiarazione di funzione con l'handle `main`. Normalmente, rustc chiama prima la funzione `main` ogni volta che l'eseguibile viene eseguito.

`println` è una macro incorporata.

Una macro è simile a una funzione, ma può essere pensata come un pezzo di codice che scrive altro codice. Per ora, le principali differenze tra una funzione e una macro da tenere a mente sono:

    - Le macro sono chiamate usando un bang (!)
    - Le macro possono prendere un numero variabile di argomenti; le funzioni in Rust non possono

Attività: esegui il tuo codice, usando il seguente comando:

```bash
    $ cargo run --bin calculator
```

_NOTA:_ gli argomenti di `--bin calculator` sono necessari solo perché non sei all'interno della cartella `calculator`.

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- L'esecuzione del codice dovrebbe dare come output `Hello, world!`.
- `getCommandOutput(Hello, world!)`

## 4

### --description--

Le variabili sono dichiarate utilizzando la parola chiave `let`.

```rust
    let variable_name = value
```

Compito: all'interno della funzione `main`, dichiara una nuova variabile, chiamala `firstName` e dalle il valore del `"<tuo_nome>"`. Assicurati di dichiararla prima della chiamata `println!` e inserisci il tuo nome tra virgolette doppie.

_NOTA:_ le variabili possono anche essere dichiarate utilizzando le parole chiave const o static.

Attività: esegui il codice per vedere cosa dice il compilatore:

```bash
    $ cargo run --bin calculator
```

_SUGGERIMENTO:_ se rimani bloccato prova a seguire i consigli d'aiuto del compilatore.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile `firstName` e darle il valore del tuo nome tra virgolette doppie.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- Dovresti seguire il consiglio del compilatore di aggiungere un punto e virgola alla fine.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

Sopra, potresti notare che il compilatore rustc sta dando due suggerimenti per il tuo codice.

Attività: segui il consiglio del compilatore di convertire il nome della variabile in snake_case.

In Rust, per convenzione si usa lo snake_case per:

- Nomi di variabili
- Nomi di funzione
- Nomi di file

Lo SCREAMING_SNAKE_CASE è usato per costanti e static. Infine, il _PascalCase_ è usato per tipi, tratti ed enum (ne parleremo più tardi).

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti avere una variabile `first_name` e assegnarle il valore del tuo nome tra virgolette doppie.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

Attività: riesegui il codice. Dovresti avere un solo avviso ora.

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- Dovresti avere una variabile `first_name` e assegnarle il valore del tuo nome tra virgolette doppie.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

Il compilatore ti sta ancora dando un avviso riguardo al fatto che `first_name` è una variabile inutilizzata.

Attività: fai una correzione, cambiando la chiamata `println!` affinche sia:

```rust
    println!("Hello, {}!", first_name);
```

Le `'{}'` sono sostituite con il valore degli argomenti.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti cambiare la chiamata `println!` in `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

Ci sono molte cose che puoi fare con `println!`. Dai un'occhiata agli esempi presenti nella documentazione Rust e sperimenta con il codice:

- https://doc.rust-lang.org/rust-by-example/hello/print.html

Questo è ciò che rende la macro `println!` uno strumento eccellente per effettuare il debug sul tuo codice.

Attività: per vedere l'output esegui il codice con:

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

- Dovresti cambiare la chiamata `println!` in `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

Il tipo di `first_name` è `&str`. `str` è un tipo primitivo, e la _e commerciale (&)_ indica che il tipo è un _riferimento_

Un aspetto importante del linguaggio Rust è la proprietà. Cioè, uso e allocazione della memoria. Il concetto di proprietà emergerà durante questo corso.

Un altro tipo comune è `String`. Si tratta di un tipo utile, perché viene automaticamente allocato nella memoria heap. Ciò consente alle sue dimensioni di essere sconosciute al momento della compilazione.

Attività: converti `first_name` nel tipo `String`, usando il tratto from che è disponibile sulla struttura `Stringa`:

```rust
    let example = String::from("Hello, Camper!");
```

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti usare `String::from()` per creare una `String` con il tuo nome.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

Attività: subito dopo `first_name`, crea una nuova variabile denominata `name` e assegnale il valore di `first_name`. Poi, sostituisci il secondo argomento nella chiamata `println!` con la tua nuova variabile creata.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile `name` e assegnarle il valore di `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- Dovresti sostituire il secondo argomento di `println!` con `name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

Attività: copia la chiamata corrente di `println!` e posizionala immediatamente dopo la prima. Poi, sostituisci il secondo argomento con `first_name`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti avere due chiamate `println!` immediatamente una dopo l'altra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- La prima chiamata `println!` dovrebbe usare `name` e la seconda `println!` dovrebbe usare `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

Attività: esegui il codice. Vedrai un errore.

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

- Dovresti avere due chiamate `println!` immediatamente una dopo l'altra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- La prima chiamata `println!` dovrebbe usare `name` e la seconda `println!` dovrebbe usare `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

La tua app dà errore. La ragione di questo errore è che l'ultima chiamata `println!` prova a usare la variabile `first_name`. Tuttavia, questa variabile non è più disponibile, in quanto è stata _spostata_ in `name`.

Per evitare che `first_name` venga spostata, puoi assegnare a `name` il valore riferimento di `first_name`.

Attività: fallo, aggiungendo il simbolo di riferimento all'inizio del valore di `name`. Ecco un esempio:

```rust
    let value = String::from("");
    let referenced_value = &value;
```

Ciò impedisce che `value` venga spostato in `referenced_value` e, invece, utilizza un riferimento al valore di `value` in `referenced_value`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti avere due chiamate `println!` immediatamente una dopo l'altra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- La prima chiamata `println!` dovrebbe usare `name` e la seconda `println!` dovrebbe usare `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- Dovresti fare riferimento a `first_name` quando lo assegni a `name`, usando `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

Attività: esegui il codice. Non dovresti più vedere l'errore.

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

- Esegui il codice con `cargo run --bin calculator`. Non dovresti vedere errori.
- `null`

## 15

### --description--

Vuoi aggiungere il tuo cognome (Surname) a `name`.

Ci sono molti modi per farlo in Rust. Se provi a concatenare `" Surname"` a `&first_name`, Rust darà errore, perché non puoi concatenare a un valore di riferimento.

Potresti rimuovere &, ma poi il secondo `println!` manderà il programma in panic.

Per concatenare un riferimento a una `str (&str)`, il primo argomento deve essere posseduto. `String` può essere usato come un valore posseduto con il metodo `to_owned`:

```rust
    let owned_string = my_string.to_owned() + " Surname";
```

Attività: invece di spostare `first_name`, trasformalo in un valore posseduto e concatenagli il tuo cognome - assegnando il risultato a `name`.

Esegui il tuo codice. Se viene compilato e stampa due righe, hai completato questa lezione correttamente. In caso contrario, utilizza l'output per fare il debugging e correggere il codice.

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

- Dovresti trasformare `first_name` in un valore posseduto con `.to_owned()`.
- `first_name\.to_owned\(\)`
- Dovresti concatenare il tuo cognome al valore posseduto `first_name`.
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

Un modo più idiomatico per utilizzare il tipo `String`, è usare il metodo `push_str`:

```rust
    let mut my_string = String::from("String");
    my_string.push_str("a str");
```

Attività: elimina `name` e anche la prima chiamata `println!`. Quindi, usa il metodo `push_str` su `first_name` per aggiungervi il tuo cognome.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti usare il metodo `.push_str()`.
- `\.push_str\(`
- Dovresti aggiungere con push_str il tuo cognome a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

Attività: esegui il codice. Dovrebbe darti errore.

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

- Dovresti usare il metodo `.push_str()`.
- `\.push_str\(`
- Dovresti aggiungere con push_str il tuo cognome a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Dovresti rendere `first_name` mutabile con `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

Il tuo codice ha dato errore perché `first_name` non è _mutabile._

Attività: usa i suggerimenti del compilatore per rendere `first_name` mutabile.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti usare il metodo `.push_str()`.
- `\.push_str\(`
- Dovresti aggiungere con push_str il tuo cognome a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Dovresti rendere `first_name` mutabile con `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

Attività: esegui di nuovo il codice per assicurarti che venga compilato senza errori.

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

- Esegui il codice con `cargo run --bin calculator`. Non dovresti vedere errori.
- `null`

## 20

### --description--

Finora hai imparato a conoscere i tipi `str` e `String`, così come i riferimenti. Se non hai accidentalmente usato le virgolette singole ('), potresti non aver notato che, finora, tutto ciò che ha a che fare con le stringhe fa uso di virgolette doppie(").

Questo perché c'è un terzo tipo standard chiamato `char`.

Un `char` è un _USV (Unicode Scalar Value),_ che è rappresentato in unicode con valori come `U+221E` - il codice unicode per '∞'.

Le stringhe possono essere pensate come insiemi o array di `char`.

Attività: rimuovi tutto il codice dalla funzione `main`. Quindi, dichiara una nuova variabile `first` e assegnale la prima lettera del tuo nome - `first` dovrebbe essere di tipo `&str`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile chiamata `first`.
- `let\s+first`
- Dovresti assegnare a `first` un singolo carattere tra virgolette doppie.
- `first\s*=\s*"\w"`

## 21

### --description--

Attività: stampa sulla console il valore del metodo `.len()` su `first` e il valore di `first.chars().count()`.

Esegui il codice per vedere l'output. Se viene eseguito e stampa `'1 1'`, hai completato correttamente la lezione.

### --seed--

```rust
// Lesson #21
fn main() {
  let first = "T";
}
```

### --tests--

- Dovresti stampare la lunghezza di `first` e il numero di caratteri in `first`. Esempio di output: `1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

### --description--

Dovresti vedere l'output `1 1` nella console. Il metodo `len` restituisce la lunghezza di `str` in byte. Il metodo `chars` restituisce un iteratore sui `char` nella porzione di stringa, e il metodo `count` restituisce il numero di elementi nell'iteratore.

Attività: cambia il valore di `first` in modo che sia una porzione di stringa con il carattere dell'infinito: ∞

_SUGGERIMENTO:_ puoi copiare e incollare il carattere dalla riga precedente

Esegui il codice per vedere l'output. Se viene eseguito e stampa `'3 1'`, hai completato correttamente la lezione.

### --seed--

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Dovresti cambiare il valore di `first` in modo che sia una porzione di stringa con `∞`.
- `first\s*=\s*"∞"`
- Il codice dovrebbe stampare `3 1`.
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

Dovresti vedere l'output `3 1` nella console. Ciò è dovuto al fatto che il carattere `'∞'` ha una lunghezza di 3 byte.

Attività: sentiti libero di sperimentare con questi nuovi metodi e farti un'idea dei diversi valori che producono le stringhe.

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Non ci sono test per questa lezione.
- `null`

## 24

### --description--

Da questa lezione in poi, scriverai il codice avendo in mente il _TDD Test Driven Development_ o sviluppo guidato dai test. Ovvero, dovrai scrivere il tuo codice per passare i test esistenti, così come dovrai scrivere tu stesso dei test da superare.

Attività: esegui il seguente comando per inizializzare il tuo codice con i test per la prossima lezione:

```bash
    $ fcc reset 24
```

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Avresti dovuto eseguire il comando `fcc reset 15`.
- `mod tests`

## 25

### --description--

La configurazione di base per i test è già inclusa. La sintassi `#[]` sopra una dichiarazione è il modo in cui gli _attributi_ vengono aggiunti in Rust.

`cfg(test)` configura il tratto `test` per la dichiarazione sottostante, e la sintassi `#[test]` dichiara quali funzioni devono essere eseguite come test.

Attività: in cima allo script, aggiungi una funzione chiamata `main`. Poi, in cima al modulo `tests`, importa la funzione `main`, usando la sintassi:

```rust
    use crate::main;
```

La parola chiave `use` in Rust, è simile a 'import', 'require' o 'include' in altre lingue.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 26

### --description--

Come hai potuto notare dai test, le funzioni senza un return esplicito restituiscono una `tuple` vuota. Le tuple sono rappresentate con delle parentesi () - ecco perché il test asserisce che il valore di ritorno di `main` sia `()`.

Ci sono due modi per restituire qualcosa. Usando la parola chiave `return`, oppure omettendo il punto e virgola.

Se una funzione restituisce qualcosa di diverso da una tupla vuota, occorre digitarlo esplicitamente:

```rust
    fn my_func() -> String {
      let my_string: String = String::from("Nich");
      my_string
    }
```

_Nota:_ qui sopra è stato digitato esplicitamente per chiarezza.

Attività: supera il test restituendo `24` da `main` e scrivi il valore di ritorno della funzione con il tipo `usize`.

`usize` è il tipo predefinito per un intero positivo. La u sta per _unsigned,_ cioè senza segno, e size descrive la dimensione del sistema in bit. Comunemente i sistemi sono a 64- o 32-bit.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 27

### --description--

Ci sono molti tipi di numero, in Rust:

    - Interi senza segno: `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
    - Interi con segno: `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
    - Float: `f32`, `f64`

Gli interi senza segno rappresentano solo i numeri interi positivi. Gli interi con segno rappresentano sia i numeri interi positivi che negativi. I float rappresentano solo i numeri frazionari positivi e negativi.

Attività: supera i test cambiando il numero e il tipo del valore di ritorno della funzione `main`.

_NOTA:_ il primo test comprende il tratto `should_panic`. Ciò vuol dire che il codice dovrebbe dare errore.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 28

### --description--

Vuoi che la calcolatrice sia usata sulla riga di comando come:

```bash
    $ calculator <first_number> <operator> <second_number>
```

Con un output simile a:

```bash
    $ <first_number> <operator> <second_number> = <result>
```

Esempio:

```bash
    $ calculator 1 + 1
    $ 1 + 1 = 2
```

Attività: crea una nuova funzione chiamata `output` che accetta 4 argomenti. Il primo, il terzo e il quarto argomento dovrebbero essere interi con segno e il secondo argomento dovrebbe essere un `char`.

_SUGGERIMENTO:_ non dimenticare di importare la nuova funzione nel modulo dei test.

Ecco una funzione di esempio con argomenti di tipo specificato:

```rust
    fn example(first_arg: usize, second_arg: String) -> &str {
      "I return a reference to a string slice"
    }
```

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 29

### --description--

Ora, per far sì che `output` restituisca l'output corretto, userai la macro format.

La macro`format!` funziona quasi allo stesso modo della macro `println!` che stavi usando. Eccetto che invece di stampare l'output sulla console, restituisce l'output come `String`.

Attività: usa la macro `format!` per restituire un output con il seguente formato:

```bash
    <first_number> <operator> <second_number> = <result>
```

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 30

### --description--

Attività: all'interno della funzione `main`, stampa sulla console il risultato della chiamata di `output` con degli argomenti validi.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti stampare sulla console qualsiasi output valido.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

Attività: all'interno della funzione `main`, dichiara tre variabili: `first_number`, `operator` e `second_number`.

Quindi, assegna loro valori validi e passali come argomenti all'interno della chiamata `output`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile chiamata `first_number`.
- `let first_number`
- Dovresti dichiarare una variabile chiamata `second_number`.
- `let second_number`
- Dovresti dichiarare una variabile chiamata `operator`.
- `let operator`
- Dovresti chiamare `output` con le variabili che hai appena dichiarato.
- `output\(\s*first_number\s*,\s*operator\s*,\s*second_number`

## 32

### --description--

Potresti aver notato che ciò che viene stampato sulla console non è corretto. Devi eseguire un'operazione sui numeri di input, per risolvere questo problema.

Attività: dichiara una nuova funzione chiamata `operate` che accetta, in ordine, `operator`, `first_number` e `second_number`.

_SUGGERIMENTO:_ ricorda di importare la funzione nel modulo `tests`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 33

### --description--

Vuoi essere in grado di eseguire le quattro operazioni di base: addizione, sottrazione, divisione e moltiplicazione.

Attività: usa più istruzioni `if` per confrontare i casi in cui `operator` è uno tra: `'+' '-' '*' '/'`

Un'istruzione `if` segue questa sintassi:

```rust
    if my_var == "my str" {
      // Do stuff
    } else if my_var == "something else" {
      // Do more stuff
    } else {
      // Finally...
    }
```

Attività: restituisci il risultato dell'operazione su `first_number` e `second_number` per passare i test.

_SUGGERIMENTO:_ ricorda di includere una clausola `else`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 34

### --description--

Invece di restituire `0` come risultato quando viene utilizzato un operatore non valido, potrebbe avere più senso mandare l'applicazione in panic.

La macro `panic!` ha questo scopo e accetta un riferimento a una porzione di stringa come argomento, che contiene un messaggio di panico.

Attività: quando viene usato un operatore non valido manda il tuo codice in panic.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Non ci sono test Node per questa lezione
- `null`

## 35

### --description--

Invece di molte istruzioni `if...else`, puoi migliorare la leggibilità e la praticità del tuo codice con la struttura di controllo di Rust `match`. L'operatore `match` è simile all'istruzione `switch` di molti linguaggi. In ogni caso, consente di trovare la corrispondenza con un pattern.

Un esempio inventato di un'espressione che utilizza l'operatore `match`:

```rust
    let some_variable = 't';
    match some_variable {
      'a' => 'A',
      'b' => 'B',
      _ => 'Z',
    }
```

Dato che `'t'` non corrisponde ad `'a'` o `'b'`, l'espressione restituisce `'Z'`, seguendo il caso di base denotato dal trattino basso.

Attività: converti la logica if/else all'interno di `operate` in modo da usare l'operatore `match`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti usare l'operatore `match`.
- `match`
- Dovresti ancora superare tutti i test.
- `getTestOutput(7 passed)`

## 36

### --description--

Dovresti essere in grado di usare la calcolatrice con un input come: `calculator 3 x 3`

Uno pattern `match` può essere esteso usando una logica bit-wise come questa:

```rust
    match name {
      "Quincy" => "Hello, Quincy",
      "Tom" | "Nich" => "Hello, other",
      _ => panic!("Pattern not found"),
    }
```

Con `name` uguale a `"Nich"`, il secondo _braccio_ `match` risulta corrispondente.

Attività: estendi il braccio della moltiplicazione nell'operatore `match` in modo che corrisponda a valori di `operator` uguali a `'x'` e `'X'`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti usare l'operatore `|`.
- `|`

## 37

### --description--

Attualmente, l'argomento `result` per `output` ha codifica fissa.

Attività: all'interno di `main`, dichiara una nuova variabile chiamata `result` e assegnale come valore la chiamata di `operate` con le prime tre variabili. Quindi, passa `result` come quarto argomento ad `output`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile `result`
- `let\s+result`
- Il tuo codice dovrebbe stampare `1 - 10 = -9` sulla console.
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

Vuoi che questa applicazione legga i valori dagli argomenti dalla riga di comando. La libreria standard di Rust ha un modulo di ambiente che fornisce l'accesso agli argomenti passati attraverso la CLI.

I moduli nella libreria standard sono accessibili utilizzando la seguente sintassi:

```rust
    use std::*;
```

Questo importa tutti i moduli all'interno della libreria standard. Tuttavia, te ne serve solo uno.

Attività: alla radice dello script, usa la sintassi qui sopra per importare il modulo env dalla libreria standard.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti aggiungere `use std::env;` nella parte superiore del tuo script.
- `use\s+std::env\s*;`

## 39

### --description--

Ora che il modulo `env` è stato portato nell'ambito di visibilità, puoi fare riferimento ai suoi Struct, Enum e Funzioni.

Attività: in cima a `main` dichiara una nuova variabile chiamata `args` e assegnale il valore della chiamata della funzione `args`, che esiste all'interno del modulo `env`.

_SUGGERIMENTO:_ ricorda che per accedere a una funzione all'interno di un modulo occorre la sintassi `'::'`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una nuova variabile chiamata `args`
- `let\s+args`
- Dovresti assegnare ad `args` il valore `env::args()`
- `=\s*env::args\(\)`

## 40

### --description--

Attività: per avere un'idea di cosa contiene `args`, stampa il suo valore sulla console.

_SUGGERIMENTO:_ ricorda di seguire gli utili consigli del compilatore se hai difficoltà a stampare il valore.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Eseguendo `cargo run --bin calculator` dovrebbe stampare: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

Senza passare alcun argomento durante l'esecuzione del crate, il valore di `args` contiene ancora un argomento - il percorso relativo al binario.

Attività: vedi i diversi valori di `args` eseguendo comandi come:

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

- Non ci sono test Node per questa lezione.
- `null`

## 42

### --description--

Per accedere a un argomento specifico in `args`, è possibile utilizzare il metodo `nth`. Il metodo `nth` accetta un argomento numerico (n) per accedere all'n-esimo argomento - usando l'indicizzazione a base 0.

Attività: cambia il println di `args` in modo che stampi sulla console il primo argomento.

_SUGGERIMENTO:_ ricorda di seguire gli utili consigli del compilatore se rimani bloccato.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti accedere al primo (`0`) elemento dell'iteratore `env::args()`.
- `args\.nth\(0\)`
- Dovresti dichiarare `args` come mutabile con `let mut args =...`
- `let\s+mut\s+args`

## 43

### --description--

Seguendo il consiglio del compilatore, nella lezione precedente hai dovuto dichiarare `args` come mutabile. Questo perché il metodo `nth` itera mutabilmente sugli elementi.

Attività: rimuovi il println per 'args'. Poi, cambia `first_number`, `operator` e `second_number` in modo che siano uguali rispettivamente al primo, secondo e al terzo `args`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti assegnare `args.nth(0)` a `first_number`.
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- Dovresti assegnare `args.nth(1)` a `operator`.
- `let\s+operator\s*=\s*args\.nth\(1\)`
- Dovresti assegnare `args.nth(2)` a `second_number`.
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

Parte del codice è stato commentato, in modo che il programma venga compilato.

Se ora esegui il codice, vedrai che l'output contiene:

```bash
    $ Some("target/debug/calculator"), None, None
```

Questo perché `nth` non restituisce direttamente il valore, ma invece restituisce il valore racchiuso in un `Option`.

Un `Option` è un tipo che include `Some` avvolto intorno al valore, oppure `None` se il valore non esiste.

Per utilizzare il valore racchiuso in `Some`, l'`Option` può essere _spacchettato:_

```rust
    let my_option: Option<String> = env::args().nth(0);
    let my_value: String = my_option.unwrap();
```

Attività: spacchetta le variabili `first_number`, `operator` e `second_number` nelle dichiarazioni ed esegui il codice per vedere cosa accade.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti spacchettare `first_number` con `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- Dovresti spacchettare `operator` con `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- Dovresti spacchettare `second_number` con `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

Attualmente, l'esecuzione dell'applicazione con:

```bash
    $ cargo run --bin calculator
```

Provoca un panic. Questo perché provare a spacchettare un valore `None` dà un comportamento indefinito.

Ci sono modi per gestire gli errori in modo più grazioso, ma per ora, assicurati di chiamare la tua applicazione con abbastanza argomenti:

```bash
    $ cargo run --bin calculator -- 1 + 2
```

Attività: esegui di nuovo il tuo codice, continuando ad aggiungere argomenti dopo '--', finché non c'è più il panic.

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

- Non ci sono test Node per questa lezione.
- `null`

## 46

### --description--

Attualmente, sono necessari 5 argomenti per evitare che l'applicazione vada in panic. Sembra che tu stia solo cercando di spacchettare il terzo elemento, vero?

In realtà, a causa di `nth` che itera mutabilmente su `args`, il primo elemento viene rimosso dopo che è stato consultato. Quindi, cercare successivamente di accedere al secondo elemento, equivale ad aver cercato originariamente di accedere al terzo.

Attività: cambia l'argomento passato a `nth` in modo da accedere agli elementi corretti. Eseguire `cargo run --bin calculator -- 1 + 2` dovrebbe dare l'output: "1", "+", "2"

_SUGGERIMENTO:_ ricorda che il primo elemento è il percorso relativo al binario - non first_number.

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

- Non ci sono test Node per questa lezione.
- `null`

## 47

### --description--

Può essere utile annotare esplicitamente i tipi delle variabili. Ne hai già visto alcuni esempi, ma eccone un altro:

```rust
    let my_var: &str = "Mrugesh";
```

Attività: annota il tipo delle variabili `args`, `first_number`, `operator` e `second_number`.

_SUGGERIMENTO:_ dai ad alcune il tipo errato e segui i consigli del compilatore per correggerli. Dovrai importare un tipo dal modulo `env`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti importare la struct `Args` dal modulo `std::env` con `use std::env::Args`.
- `use\s+std::env::Args`
- Dovresti annotare `args` con il tipo `Args`.
- `let\s+mut\s+args:\s*Args`
- Dovresti annotare `first_number` con il tipo `String`.
- `let\s+first_number:\s*String`
- Dovresti annotare `operator` con il tipo `String`.
- `let\s+operator:\s*String`
- Dovresti annotare `second_number` con il tipo `String`.
- `let\s+second_number:\s*String`

## 48

### --description--

Invece di scrivere degli import non necessari, puoi combinarli con la seguente sintassi:

```rust
    use std::env::{var, Vars};
```

Quanto sopra importa la funzione `var` e la struct `Vars` dal modulo `env` nella libreria standard.

Attività: usa una dichiarazione import per importare sia la funzione `args` che la struct `Args`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti combinare entrambi gli import in una singola istruzione import con `use std::env::{args, Args};`.
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

Adesso devi sistemare il problema con `operate` e `output` che si aspettano dei tipi di input `i32` e `&str`.

Puoi farlo con il metodo `parse`, che esiste per il tipo `String`.

Il metodo `parse` converte `String` in un dato tipo. Il tipo può essere dato usando la sintassi _turbofish_:

```rust
    let my_string_number: String = String::from("Kris");
    let my_number_option: Option<usize> = my_string_number.parse::<usize>();
    let my_number: usize = my_number_option.unwrap();
```

Attività: all'interno di `main`, due nuove variabili - `first` e `second` - e usa la sintassi turbofish per assegnare i valori "parsati" e spacchettati di `first_number` e `second_number`, rispettivamente. Quindi sostituisci `first_number` e `second_number` con `first` e `second` nella chiamata `println!`.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Dovresti dichiarare una variabile chiamata `first`.
- `let\s+first`
- Dovresti dichiarare una variabile chiamata `second`.
- `let\s+second`
- Dovresti assegnare `first_number.parse::<i32>().unwrap()` a `first`.
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- Dovresti assegnare `second_number.parse::<i32>().unwrap()` a `second`.
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

Attualmente, `operator` è di tipo `String` mentre dovrebbe essere di tipo `char`. Una `String` può essere convertita in un `char` usando il metodo `chars` e spacchettando il primo `Option` restituito dalla chiamata del metodo `next`.

Attività: ritrasforma in codice i commenti e fai gli aggiustamenti necessari per far sì che il codice venga compilato.

Assicurati di seguire i suggerimenti del compilatore per fare in modo che il codice venga compilato. Quindi, rimuovi la prima chiamata `println!` per avere solo un output.

Puoi assicurarti che l'output sia corretto eseguendo:

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

- Non ci sono test Node per questa lezione.
- `null`

## 51

### --description--

Attualmente, la calcolatrice accetta solo interi come input.

Attività: cambia tutti i tipi necessari per consentire alla calcolatrice di accettare anche numeri in virgola mobile.

Puoi vedere se hai completato correttamente la lezione eseguendo:

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

- Suggerimento: dovresti cambiare i tipi da `i32` a `f32`.
- `null`

## 52

### --description--

Hai completato il codice per il tuo binario. Ora, è necessario compilarlo e distribuirlo per essere utilizzato.

Attività: esegui il seguente comando per fare il build del tuo codice in binario:

```bash
    $ cargo build --bin calculator
```

Se non vedi errori, hai completato con successo la lezione.

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

- Questa è l'ultima lezione. Congratulazioni!
- `null`

## 53

### --description--

Cargo ha appena compilato il tuo codice nella cartella `target/debug`.

Attività: esegui la tua applicazione, usando il seguente comando:

```bash
    $ target/debug/calculator 1 + 2
```

Se vedi l'output `'1 + 2 = 3'` hai completato con successo la lezione.

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

- Questa è l'ultima lezione. Congratulazioni!
- `null`

## 54

### --description--

Il compilatore Rust spesso compila con incredibili ottimizzazioni. Tuttavia, devi specificare a Cargo di eseguire un build _release_ del tuo codice, per trarne il massimo.

Attività: fai di nuovo il build della tua applicazione, stavolta usando il flag `release`:

```bash
    $ cargo build --release --bin calculator
```

Dovresti essere in grado di individuare il tuo binario ottimizzato nella cartella `target/release`.

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

- Questa è l'ultima lezione. Congratulazioni!
- `null`

## 55

### --description--

Congratulazioni. Hai completato il progetto `freeCodeCamp - Rust in Replit - CLI Calculator`.

Sei invitato ad ampliare il tuo progetto attuale - magari per accettare più operazioni...

Attività: esegui il seguente comando per iniziare il prossimo progetto:

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

- Questa è l'ultima lezione. Congratulazioni!
- `null`

## 56

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Segnaposto
- `null`
