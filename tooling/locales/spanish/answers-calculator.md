# freeCodeCamp - Respuestas del curso de Rust en Replit

## 1

### --description--

Las principales herramientas dentro del ecosistema Rust son:

- rustc - el compilador que toma tu código Rust y lo compila en binario (código legible por máquina
- rustup - La utilidad de línea de comandos para instalar y actualizar Rust
- cargo - El sistema de construcción de Rust y el admistrador de paquetes (trabajará con esto)

Tarea: Crear un nuevo proyecto Rust ejecutando el siguiente comando en el prompt:

```bash
    $ cargo new calculator
```

### --seed--

```rust

```

### --tests--

- No hay pruebas de RegEx para esta lección.
- `null`

## 2

### --description--

Acabas de crear un nuevo proyecto de Rust dentro de el directorio `calculator/`.

Cargo ha creado el boilerplate para un 'Hola Mundo'.

Tarea: Abre el archivo `calculator/src/main.rs`.

Este es el archivo por defecto que Cargo utiliza para el binario de su aplicación.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Deberías tener el archivo de proyecto `calculator/src/main.rust`.
- `null`

## 3

### --description--

Este archivo contiene una declaración de función con el manejador `main`. Por defecto, rustc llama primero a la función `main` cada vez que se ejecuta el ejecutable.

`println` es una macro incorporada.

Un macro es similar a una función, pero se puede considerar como un fragmento de código que escribe otro código. Por ahora, las principales diferencias entre una función y una macro a tener en cuenta son:

    - Las macros se llaman con un bang (!)
    - Macros pueden tomar un número variable de argumentos; las funciones en Rust no pueden

Tarea: Ejecute el código con el siguiente comando:

```bash
    $ cargo run --bin calculator
```

_NOTA:_ Los `--bin calculator` argumentos solo son necesarios, porque no están dentro de la `calculator` directorio.

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- La ejecución de su código debería dar como resultado `¡Hola, mundo!`.
- `getCommandOutput(Hello, world!)`

## 4

### --description--

Las variables se declaran utilizando la palabra clave `let`.

```rust
    let variable_name = value
```

Tarea: Dentro de la `main` función, declara una nueva variable y nómbrala `firstName` y dale un valor de `"<your_name>"`. Asegúrate de declararlo antes de la llamada `println!` y coloca tu nombre entre commillas dobles.

_NOTA:_ Las variables también pueden ser declaradas usando la const o palabras clave estáticas.

Tarea: Ejecuta tu código para ver lo que dice el compilador:

```bash
    $ cargo run --bin calculator
```

_SUGERENCIA:_ Si te quedas atascado, trata de seguir los consejos útiles del compilador.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debe declarar una variable `firstName` y darle un valor de su nombre dentro de comillas dobles.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- Deberías seguir el consejo del compilador para añadir un punto y coma al final.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

Para empezar, puede notar que el compilador rustc está dando dos sugerencias para su código.

Tarea: Seguir los consejos del compilador para convertir el nombre de la variable en snake_case.

Es una convención en Rust utilizar snake_case para:

- Nombres variables
- Nombres de función
- Nombre de archivos

SCREAMING_SNAKE_CASE se utiliza para constantes y estáticas. Por último, _PascalCase_ se utiliza para tipos, rasgos y enums (los cubriremos luego).

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debe tener una variable `first_name` y darle un valor de su nombre dentro de comillas dobles.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

Tarea: Vuelva a ejecutar su código. Sólo debería tener una advertencia, ahora.

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- Debe tener una variable `first_name` y darle un valor de su nombre dentro de comillas dobles.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

El compilador todavía le está dando una advertencia de que `first_name` es una variable no utilizada.

Tarea: Corregir eso, cambiando la llamada de `println!` para ser:

```rust
    println!("Hello, {}!", first_name);
```

Los `'{}'` son reemplazados por el valor de los argumentos.

Puedes ver si completaste la lección correctamente ejecutando:

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

- ¡Deberías cambiar la `println!` llamada a `println!("Hola, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

Hay muchas cosas que puedes hacer con `println!`. Mira la documentación de Rust by Example, y juega con tu código:

- https://doc.rust-lang.org/rust-by-example/hello/print.html

¡Esto es lo que hace que la macro `println!` sea una herramienta excelente para depurar tu código.

Tarea: Ejecuta tu código para ver la salida con:

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

- ¡Deberías cambiar la `println!` llamada a `println!("Hola, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

El tipo de `first_name` es `&str`. `str` es un tipo primitivo, y el _ampersand (&)_ indica que el tipo es una _referencia._

Un aspecto importante del lenguaje Rust es la propiedad. Es decir, el uso de memoria y la asignación. A lo largo de este curso surgirá el concepto de propiedad.

Otro tipo común es `String`. Este es un tipo útil, ya que se asigna automáticamente. Esto permite que su tamaño sea desconocido en tiempo de compilación.

Tarea: Convierte `first_name` en el tipo `String`, usando el trait que está disponible en la estructura `String`:

```rust
    let example = String::from("Hello, Camper!");
```

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías usar `String::from()` para crear una `String` con tu nombre.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

Tarea: Inmediatamente después de `first_name`, cree una nueva variable llamada `name`, y asignarle el valor de `first_name` a él. Luego, reemplaza el segundo argumento en la llamada `println!` con tu variable recién creada.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debe declarar una variable `name` y asignarle el valor de `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- Deberías reemplazar el segundo argumento de `println!` por `name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

Misión: Copia la llamada actual `println!` y colócala inmediatamente después de la primera. Luego, reemplace el segundo argumento con `first_name`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- ¡Deberías tener dos llamadas `println!` inmediatamente después del otro.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- Debe hacer que el primer `println!` utilice `name` y el segundo `println!` utilice `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

Tarea: Ejecute su código. Verá un error.

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

- ¡Deberías tener dos llamadas `println!` inmediatamente después del otro.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- Debe hacer que el primer `println!` utilice `name` y el segundo `println!` utilice `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

Error en tu aplicación. La razón de este error es la última llamada `println!` intenta usar la variable `first_name`. Sin embargo, esta variable ya no está disponible, ya que fue _movido_ a `name`.

Para evitar que `first_name` se mueva, puede asignar `name` al valor referenciado de `first_name`.

Tarea: Hacer esto, agregando el símbolo de referencia al principio del valor `name`. Aquí hay un ejemplo:

```rust
    let value = String::from("");
    let referenced_value = &value;
```

Esto evita que el `value` se mueva a `referenced_value` y, en su lugar, utiliza una referencia al valor de `value` en `referenced_value`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- ¡Deberías tener dos llamadas `println!` inmediatamente después del otro.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- ¡Debería tener la primera `println!` use `name` y la segunda `println!` use `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- Debe hacer referencia a `first_name` al asignarlo a `name`, usando `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

Tarea: Ejecute su código. Ya no debería ver el error.

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

- Ejecuta tu código con `cargo run --bin calculator`. No debería ver errores.
- `null`

## 15

### --description--

Quieres añadir tu apellido (segundo nombre) a `name`.

Hay muchas maneras de hacerlo en Rust. Si intenta simplemente concatenar `" Apellido"` a `&first_name`, No se puede concatenar con un valor referenciado.

Podrías eliminar el &, pero entonces el segundo `println!` hará que el programa entre en pánico.

Para concatenar una referencia a un `str (&str)`, el primer argumento necesita ser dueño. Un `String` puede ser usado como un valor propiedad con el método `to_owned`:

```rust
    let owned_string = my_string.to_owned() + " Surname";
```

Tarea: En lugar de mover `first_name`, conviértalo en un valor propio, y concatena tu apellido - asignando el resultado a `name`.

Tarea: Ejecute su código. Si compila e imprime las dos líneas, has completado la lección correctamente. Si no, utilice la salida para depurar y arreglar su código.

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

- Debe convertir `first_name` en un valor propiedad con `.to_owned()`.
- `first_name\.to_owned\(\)`
- Usted debe concatenar su apellido a la propiedad `first_name`.
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

Una forma más idiomática de hacer uso del tipo `String`, es utilizando el método `push_str`:

```rust
    let mut my_string = String::from("String");
    my_string.push_str("a str");
```

Tarea: Elimina `name` así como la primera llamada `println!`. Luego, utilice el método `push_str` en `first_name` para anexar su apellido.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debería utilizar el método `.push_str()`.
- `\.push_str\(`
- Debe enviar su apellido a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

Tarea: Ejecute su código. Debería ser un error.

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

- Debería utilizar el método `.push_str()`.
- `\.push_str\(`
- Debe enviar su apellido a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Debe hacer `first_name` mutable con  `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

Su código fue erróneo, porque `first_name` no es _mutable._

Tarea: Utilice las pistas del compilador para hacer mutable `first_name`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debería utilizar el método `.push_str()`.
- `\.push_str\(`
- Debe enviar su apellido a `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Debe hacer `first_name` mutable con `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

Tarea: Ejecute su código de nuevo para asegurarse de que compila sin error.

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

- Ejecuta tu código con `cargo run --bin calculator`. No debería ver errores.
- `null`

## 20

### --description--

Hasta ahora, has aprendido sobre los tipos `str`y `String`, así como sobre referencias. Si no has usado comillas simples ('), puede que no hayas notado eso, hasta ahora, todo lo relacionado con las cadenas usan comillas dobles (").

Esto se debe a que hay un tercer tipo estándar llamado `char`.

Un `char` es un _USV (Valor Escalar Unicode),_ que está representado en unicode con valores como `U+221E` - el unicode para '∞'.

Las cadenas pueden ser pensadas como colecciones o arreglos de `char`s.

Tarea: Elimina todo tu código de la función `main`. Luego, declare una nueva variable `first`, y asignarle la primera letra de su nombre - `first` debería ser tipo `&str`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías declarar una variable llamada `first`.
- `let\s+first`
- Deberías asignar `first` un solo carácter entre comillas dobles.
- `first\s*=\s*"\w"`

## 21

### --description--

Tarea: Imprime en la consola el valor del método `.len()` en `first` y el valor de `first.chars().count()`.

Tarea: Ejecuta tu código para ver la salida con. Si se ejecuta y imprime `'1 1'`, has completado correctamente la lección.

### --seed--

```rust
// Lesson #21
fn main() {
  let first = "T";
}
```

### --tests--

- Deberías imprimir la longitud de `first` y el número de caracteres en `first`. Ejemplo de salida: `1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

### --description--

Deberías ver `1 1` salida en la consola. El método `len` retorna la longitud en bytes para el `str`. El método `chars` devuelve un iterador sobre el `char`en string, y el método `count` retorna el número de elementos en el iterador.

Tarea: Cambia el valor de `first` para que sea una porción de cadena del carácter: ∞

_SUGERENCIA:_ Puede copiar y pegar el carácter de la línea anterior

Ejecute su código para ver la salida. Si se ejecuta e imprime `'3 1'`, has completado correctamente la lección.

### --seed--

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Deberías cambiar el valor de `first` para ser una porción de string de `∞`.
- `first\s*=\s*"∞"`
- Tu código debe imprimir `3 1`.
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

Deberías ver `3 1` salida en la consola. Esto se debe a que el caracter `'∞'` ocupa 3 bytes de longitud.

Tarea: Siéntete libre de jugar con estos nuevos métodos, así como de tener una idea de qué valores producen diferentes strings.

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- No hay pruebas para esta lección.
- `null`

## 24

### --description--

A partir de esta lección, escribirás tu código con _TDD Test Driven Development (Desarrollo guiado por pruebas)_ en mente. Es decir, tendrás que escribir tu código para pasar las pruebas existentes, así como escribir algunas pruebas para pasarte a ti mismo.

Tarea: Ejecuta el siguiente comando para inicializar tu código con pruebas para la siguiente lección:

```bash
    $ fcc reset 24
```

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debería haber ejecutado el comando `fcc reset 15`.
- `mod tests`

## 25

### --description--

Ya se incluye la configuración básica para las pruebas. La sintaxis `#[]` arriba de una declaración es cómo se añaden _atributos_ en Rust.

`cfg(test)` configura el trait de la prueba `` a la declaración de abajo, y la sintaxis `#[test]` declara qué funciones deben ser ejecutadas como pruebas.

Tarea: En la parte superior del script, agregue una función llamada `main`. Luego, en la parte superior del módulo de `tests`, importa la función `main`, usando esta sintaxis:

```rust
    use crate::main;
```

La palabra clave `use`, en Rust, es similar a 'import', 'require', o 'include' como en otros idiomas.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 26

### --description--

Como se puede observar en las pruebas, las funciones sin retornos explícitos devuelven una `tuple` vacía. Las tuples se representan con paréntesis () - por lo que la prueba afirma que el retorno de `main` es `()`.

Hay dos maneras de regresar. Usando la palabra clave `return`, o dejando el punto y coma.

Las funciones que devuelvan cualquier cosa que no sea una tupla vacía deben ser escritas explícitamente:

```rust
    fn my_func() -> String {
      let my_string: String = String::from("Nich");
      my_string
    }
```

_Nota:_ Lo anterior ha sido escrito explícitamente, para mayor claridad.

Tarea: Pasar la prueba, devolviendo `24` desde `main`, y escribir el retorno de la función con el tipo `usize`.

`usize` es el tipo por defecto para un entero positivo. El u significa _sin signo,_ y tamaño describe el tamaño de bits del sistema. Esto es comúnmente sistemas de 64 o 32 bits.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 27

### --description--

Hay muchos tipos de números, en Rust:

    - Enteros sin firmar: `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
    - Entero firmado: `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
    - Flotante: `f32`, `f64`

Los enteros sin signo representan sólo números enteros positivos. Los enteros firmados representan números enteros positivos y negativos. Los floats sólo representan fracciones positivas y negativas.

Tarea: Pasar las pruebas, cambiando el número y el tipo de retorno de la función `main`.

_NOTA:_ La primera prueba incluye el trait `should_panic`. Esto significa que el código debería erradicarse.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 28

### --description--

Quieres que tu calculadora sea usada en la línea de comandos como:

```bash
    $ calculator <first_number> <operator> <second_number>
```

Con una salida como:

```bash
    $ <first_number> <operator> <second_number> = <result>
```

Por ejemplo:

```bash
    $ calculator 1 + 1
    $ 1 + 1 = 2
```

Tarea: Crea una nueva función llamada `output` que acepta 4 argumentos. Los primeros, terceros, y cuartos argumentos deben ser enteros con signo, y el segundo argumento debe ser un `char`.

_SUGERENCIA:_ No olvide importar la nueva función en el módulo de pruebas.

Esta es una función de ejemplo con argumentos escritos:

```rust
    fn example(first_arg: usize, second_arg: String) -> &str {
      "I return a reference to a string slice"
    }
```

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 29

### --description--

Ahora, para obtener `output` para devolver la salida correcta, vas a usar el formato macro.

La macro `format!` funciona de forma casi idéntica a la macro `println!` que ha estado utilizando. Excepto, en lugar de imprimir la salida a la consola, devuelve la salida como un `String`.

Tarea: Utilizar la macro `formato!` para devolver una salida siguiendo este formato:

```bash
    <first_number> <operator> <second_number> = <result>
```

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 30

### --description--

Tarea: Dentro de la función `main`, imprimir en la consola el resultado de llamar a `output` con cualquier argumento válido.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debería imprimir en la consola cualquier salida válida.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

Tarea: Dentro de la función `main`, declare tres variables: `first_number`, `operator`y `second_number`.

Luego, asígales valores válidos, y pasarlos como argumentos dentro de la llamada `output`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías declarar una variable llamada `first_number`.
- `let first_number`
- Debe declarar una variable llamada `second_number`.
- `let second_number`
- Debe declarar una variable llamada `operador`.
- `let operator`
- Deberías llamar a `ouput` con las variables que acabas de declarar.
- `output\(\s*first_number\s*,\s*operator\s*,\s*second_number`

## 32

### --description--

Puede que hayas notado que lo que se imprime en la consola no es correcto. Necesitas realizar una operación en los números de entrada, para arreglar esto.

Tarea: Declarar una nueva función llamada `operate` que acepte, por orden, el `operator`, `first_number`, y `second_number`.

_SUGERENCIA:_ Recuerda importar la función en el módulo `tests`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 33

### --description--

Quieres ser capaz de realizar las cuatro operaciones básicas: suma, resta, división y multiplicación.

Tarea: Utilice múltiples `if` enunciados para comparar los casos donde `operator` es uno de: `'+' '-' '*' '/'`

Una sentencia `if` sigue esta sintaxis:

```rust
    if my_var == "my str" {
      // Do stuff
    } else if my_var == "something else" {
      // Do more stuff
    } else {
      // Finally...
    }
```

Tarea: Devuelve el resultado de la operación en `first_number` y `second_number` para pasar las pruebas.

_SUGERENCIA:_ Recuerde incluir una cláusula `else`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 34

### --description--

En lugar de devolver un resultado de `0`, cuando se utiliza un operador no válido, podría tener más sentido hacer entrar en pánico a la aplicación.

La macro `panic!` hace justamente eso, y acepta como argumento una referencia a un trozo de cadena, que puede contener un mensaje para entrar en pánico.

Tarea: Que tu código entre en pánico cuando se utilice un operador no válido.

Puedes ver si completaste la lección correctamente ejecutando:

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

- No hay pruebas de Nodo para esta lección
- `null`

## 35

### --description--

En lugar de muchas sentencias de `if...else`, puedes mejorar la legibilidad y la usabilidad de tu código con el flujo de control `match` de Rust. El operador `match` es similar a la instrucción `switch` de muchos lenguajes. Sin embargo, permite la coincidencia de patrones.

Un ejemplo obtenido de una expresión usando el operador `match`:

```rust
    let some_variable = 't';
    match some_variable {
      'a' => 'A',
      'b' => 'B',
      _ => 'Z',
    }
```

Como `'t'` no coincide con `'a'` o `'b'`, la expresión devuelve `'Z'`, siguiendo el caso base indicado por el guión bajo.

Tarea: Convertir la lógica if/else dentro de `operate` para utilizar el operador `match`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías usar el operador `match`.
- `match`
- Deberías pasar todas las pruebas.
- `getTestOutput(7 passed)`

## 36

### --description--

Deberías poder usar la calculadora con una entrada como:`calculator 3 x 3`

Un patrón `match` puede ampliarse utilizando una lógica de bits como ésta:

```rust
    match name {
      "Quincy" => "Hello, Quincy",
      "Tom" | "Nich" => "Hello, other",
      _ => panic!("Pattern not found"),
    }
```

Con un `name` de `"Nich"`, el segundo `match` _arm_ sería igualado.

Tarea: Ampliar el arm de multiplicación en el operador `match` para que coincida en los valores del `operator` de `'x'` y `'X'`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debes usar un operador `|`.
- `|`

## 37

### --description--

Actualmente, el argumento `result` para `output` está codificado.

Tarea: dentro de `main`, declare una nueva variable llamada `result` y asígnele un valor de llamar a `operate` con las primeras tres variables. Luego, pasa `result` como cuarto argumento a `output`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debes declarar una nueva variable `result`
- `let\s+result`
- Tu código debe imprimir `1 - 10 = -9` en la consola.
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

Quieres que esta aplicación lea los valores de los argumentos de la línea de comandos. La biblioteca estándar de Rust tiene un módulo de entorno que proporciona acceso a los argumentos pasados a través del CLI.

Se accede a los módulos de la biblioteca estándar utilizando la siguiente sintaxis:

```rust
    use std::*;
```

Esto importa todos los módulos dentro de la biblioteca estándar. Sin embargo, solo se necesita uno.

Tarea: En la raíz del script, utilice la sintaxis anterior para importar sólo el módulo env de la biblioteca estándar.

Puedes ver si has completado la lección correctamente ejecutando:

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

- Deberías añadir ` use std::env;` en la parte superior de tu script.
- `use\s+std::env\s*;`

## 39

### --description--

Ahora que el módulo `env` se ha incluido en el alcance, puede hacer referencia a sus estructuras, enumeraciones y funciones.

Tarea: En la parte superior de `main` declare una nueva variable llamada `args`, y asígnele el valor de llamar a la función `args`, que existe dentro del Módulo `env`.

_SUGERENCIA:_ Recuerde, acceder a una función dentro de un módulo usa la sintaxis `'::'`.

Puede ver si completó la lección correctamente ejecutando:

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

- Debe declarar una nueva variable llamada `args`
- `let\s+args`
- Debe asignar a `args` un valor de `env::args()`
- `=\s*env::args\(\)`

## 40

### --description--

Tarea: para tener una idea de lo que contiene `args`, imprima su valor en la consola.

_SUGERENCIA:_ Recuerde, siga los útiles consejos del compilador si tiene dificultades para imprimir el valor.

Puede ver si completó la lección correctamente ejecutando:

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

- Ejecutar `cargo run --bin calculator` debería imprimir: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

Sin pasar ningún argumento al ejecutar la caja, el valor de `args` todavía contiene un argumento - la ruta relativa del binario.

Tarea: Ver los diferentes valores de `args` ejecutando comandos como:

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

- No hay pruebas de Nodo para esta lección.
- `null`

## 42

### --description--

Para acceder a un argumento específico en `args`, puede utilizar el método `nth`. El método `nth` acepta un argumento numérico (n) para acceder al siguiente argumento 'nth' - usando 0 basado en indexación.

Tarea: Cambia el println de `args` para imprimir el primer argumento a la consola.

_SUGERENCIA:_ Recuerde seguir los consejos útiles del compilador, si se queda atascado.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Debes acceder al primer elemento (`0`) del iterador `env::args()`.
- `args\.nth\(0\)`
- Debe declarar `args` como mutable con `let mut args =...`
- `let\s+mut\s+args`

## 43

### --description--

Si has seguido el consejo del compilador, en la lección anterior, necesitaste declarar `args` como mutable. Esto se debe a que el método `nth` iterará mutablemente sobre los elementos.

Tarea: Eliminar la println para 'args'. Luego, cambia `first_number`, `operator`, y `second_number` para ser igual al primer, segundo y tercero `args` con respeto.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías asignar `args.nth(0)` a `first_number`.
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- Deberías asignar `args.nth(1)` al `operator`.
- `let\s+operator\s*=\s*args\.nth\(1\)`
- Deberías asignar `args.nth(2)` a `second_number`.
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

Algún código ha sido comentado, por lo que el programa compila.

Si ejecuta el código ahora, verá que la salida contiene:

```bash
    $ Some("target/debug/calculator"), None, None
```

Esto es porque `nth` no devuelve el valor directamente, pero, en su lugar, devuelve el valor envuelto en una `Option`.

Una `Option` es un tipo que incluye `Some` envuelto alrededor de un valor, o `None` si el valor no existe.

Para usar el valor envuelto en `Some`, la `Option` se puede _unwrapped:_

```rust
    let my_option: Option<String> = env::args().nth(0);
    let my_value: String = my_option.unwrap();
```

Tarea: desenvuelva las variables `first_number`, `operator` y `second_number` en su declaración y ejecute su código para ver qué sucede.

Puede ver si completó la lección correctamente ejecutando:

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

- Debe desenvolver `first_number` con `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- Debe desenvolver `operator` con `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- Debería desenvolver `second_number` con `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

Actualmente ejecutando la aplicación con:

```bash
    $ cargo run --bin calculator
```

Causa un pánico. Esto es porque intentar desenvolver un valor donde `None` existe es un comportamiento indefinido.

Hay formas de manejar los errores elegantemente, pero, por ahora, asegúrese de llamar a su aplicación con suficientes argumentos:

```bash
    $ cargo run --bin calculator -- 1 + 2
```

Tarea: Ejecute de nuevo su código, pero siga añadiendo argumentos después de '--', hasta que no haya pánico.

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

- No hay pruebas de Nodo para esta lección.
- `null`

## 46

### --description--

Actualmente, se necesitan 5 argumentos para evitar que la aplicación caiga en pánico. Parece que sólo estás tratando de desenvolver el tercer elemento, no es asi?

En realidad, debido a `nth` iterando mutablemente sobre `args`, después de acceder al primer elemento, es eliminado. Asi que, intentar acceder al segundo elemento después es equivalente a haber intentado acceder al tercer elemento originalmente.

Tarea: Cambia los argumentos anteriores a `nth` para que se acceda a los elementos correctos. Ejecutando `argo run --bin calculator -- 1 + 2` debería salir: "1", "+", "2"

_SUGERENCIA:_ Recuerda, el primer elemento es la ruta relativa al binario - no el first_number.

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

- No hay pruebas de Nodo para esta lección.
- `null`

## 47

### --description--

Puede ser útil para anotar explícitamente tus tipos de variables. Ya ha visto ejemplos de esto, pero aquí hay uno más:

```rust
    let my_var: &str = "Mrugesh";
```

Tarea: Anota tus `args`, `first_number`, `operator`y `second_number` variables.

_SUGERENCIA:_ Da a algo el tipo incorrecto y sigue los consejos del recopilador para corregirlo. Necesitará importar un tipo desde el módulo `env`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías importar la estructura `Args` del módulo `std::env` con `use std::env::Args`.
- `use\s+std::env::Args`
- Deberías anotar `argumentos` con el tipo `Args`.
- `let\s+mut\s+args:\s*Args`
- Deberías anotar `argumentos` con el tipo `Args`.
- `let\s+first_number:\s*String`
- Deberías anotar `argumentos` con el tipo `Args`.
- `let\s+operator:\s*String`
- Deberías anotar `argumentos` con el tipo `Args`.
- `let\s+second_number:\s*String`

## 48

### --description--

En lugar de escribir importaciones innecesarias, puede combinarlas con la siguiente sintaxis:

```rust
    use std::env::{var, Vars};
```

Lo anterior, importa la función `var`, y la estructura `Vars` del módulo `env` en la biblioteca estándar.

Misión: Utilice una instrucción de importación para importar tanto la función `args`, como la estructura `Args`.

Puedes ver si has completado la lección correctamente ejecutando:

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

- Deberías combina ambas importaciones en un solo extracto de importación con `use std::env::{args, Args}`.
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

Ahora, necesitas arreglar el problema de que `operate` y `output` esperan entradas de tipo `i32` y `&str`.

Esto se puede activar con el método `parse`, que existe en el tipo `String`.

El método `parse` convierte un `String` en un tipo determinado. El tipo puede ser dado usando la sintaxis _turbofish_:

```rust
    let my_string_number: String = String::from("Kris");
    let my_number_option: Option<usize> = my_string_number.parse::<usize>();
    let my_number: usize = my_number_option.unwrap();
```

Tarea: Dentro de `main`, declarar dos nuevas variables - `first` y `second` - y utilizar la sintaxis turbofish para asignar los valores analizados y no analizados de `first_number` y `second_number` respectivamente. Luego, reemplaza `first_number` y `second_number` por `first` y `second` en la llamada `println!`.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Deberías declarar una llamada variable `first`.
- `let\s+first`
- Deberías declarar una llamada variable `second`.
- `let\s+second`
- Deberías asignar `first_number.parse::<i32>().unwrap()` a `first`.
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- Deberías asignar `second_number.parse::<i32>().unwrap()` a `second`.
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

Actualmente, `operator` es de tipo `String` cuando debe ser `char`. Un `String` puede convertirse en un `char`usando el método `chars`, y desenvolviendo la primera `Option` devuelta llamando al `next` método.

Tarea: Descomente el código comentado y haga los ajustes necesarios para permitir que el código se unifique.

Asegúrese de seguir los consejos del compilador para obtener la compilación del código. Luego, elimina la primera llamada `println!` para que solo haya una salida.

Puede asegurarse de que la salida es correcta ejecutando:

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

- No hay pruebas de Nodo para esta lección.
- `null`

## 51

### --description--

Actualmente, la calculadora sólo acepta enteros como entradas.

Tarea: Cambie todos los tipos necesarios para que la calculadora acepte también números de punto flotante.

Puedes ver si completaste la lección correctamente ejecutando:

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

- Sugerencia: Deberías cambiar los tipos de `i32` a `f32`.
- `null`

## 52

### --description--

Has completado el código de tu binario. Ahora, necesita compilarlo y enviarlo para que sea usado.

Tarea: Ejecuta el siguiente comando para construir tu código en un binario:

```bash
    $ cargo build --bin calculator
```

Si no ves errores, has completado la lección con éxito.

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

- Esta es la última lección. ¡Felicitaciones!
- `null`

## 53

### --description--

Cargo acaba de compilar tu código en el directorio `target/debug`.

Tarea: Ejecuta tu aplicación, usando el siguiente comando:

```bash
    $ target/debug/calculator 1 + 2
```

Si ves la salida `'1 + 2 = 3'` has completado con éxito la lección.

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

- Esta es la última lección. ¡Felicitaciones!
- `null`

## 54

### --description--

El compilador Rust a menudo compila con increíbles optimizaciones. Sin embargo, necesitas especificar para que Cargo construya una versión de _lanzamiento_ de tu código, para sacar el máximo provecho de ella.

Tarea: Reconstruir tu aplicación, esta vez usando la bandera `release`:

```bash
    $ cargo build --release --bin calculator
```

Deberías ser capaz de localizar tu binario optimizado dentro del directorio `target/release`.

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

- Esta es la última lección. ¡Felicitaciones!
- `null`

## 55

### --description--

Felicidades. Has completado el proyecto `freeCodeCamp - Rust in Replit - CLI Calculator`.

Usted es bienvenido a ampliar su proyecto actual - quizás, para aceptar múltiples operaciones...

Tarea: Ejecutar el siguiente comando para comenzar el siguiente proyecto:

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

- Esta es la última lección. ¡Felicitaciones!
- `null`

## 56

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Texto provisional
- `null`
