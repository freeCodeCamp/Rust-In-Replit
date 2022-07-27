# freeCodeCamp - curso de Rust no Replit - respostas

## 1

### --description--

As principais ferramentas do ecossistema Rust são:

- rustc O compilador que pega seu código em Rust e o compila para binário (código legível pela máquina)
- rustup O utilitário de linha de comando para instalar e atualizar o Rust
- carga O sistema de criação e gerenciador de pacotes do Rust (você vai trabalhar com isto)

Tarefa: criar um projeto do Rust executando o seguinte comando no prompt:

```bash
    $ cargo new calculator
```

### --seed--

```rust

```

### --tests--

- Não há testes de RegEx para esta lição.
- `null`

## 2

### --description--

Você acabou de criar um projeto em Rust dentro do diretório `calculator/`.

O cargo criou o boilerplate para um 'Hello World'.

Tarefa: abrir o arquivo `calculator/src/main.rs`.

Este é o arquivo padrão que o cargo usa para o binário de sua aplicação.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Você deve ter o arquivo de projeto `calculator/src/main.rust`.
- `null`

## 3

### --description--

Este arquivo contém uma declaração da função com o identificador `main`. Por padrão, rustc chama primeiro a função `main` sempre que o executável é rodado.

`println` é uma macro incorporada.

Uma macro é semelhante a uma função, mas pode ser considerada um trecho de código que escreve outro código. Por enquanto, as principais diferenças entre uma função e uma macro a se ter em mente são:

    - As macros são chamadas usando um "bang" (!)
    - As macros podem ter um número variável de argumentos. As funções no Rust não podem.

Tarefa: executar seu código usando o seguinte comando:

```bash
    $ cargo run --bin calculator
```

_OBSERVAÇÃO:_ os argumentos da `--bin calculator` são necessários apenas porque você não está dentro do diretório da `calculator`.

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Executar seu código deve retornar `Hello, world!`.
- `getCommandOutput(Hello, world!)`

## 4

### --description--

As variáveis são declaradas usando a palavra-chave `let`.

```rust
    let variable_name = value
```

Tarefa: dentro da função `main`, declarar uma nova variável e nomeá-la `firstName`, dando a ela o valor de `"<your_name>"`. Certifique-se de declará-lo antes da chamada de `println!` e colocar seu nome entre aspas duplas.

_OBSERVAÇÃO:_ variáveis também podem ser declaradas usando as palavras-chave const ou static.

Tarefa: executar seu código para ver o que diz o compilador:

```bash
    $ cargo run --bin calculator
```

_DICA:_ se você ficar preso, tente seguir os conselhos úteis do compilador.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma variável `Nome` e atribuir a ela como valor o seu primeiro nome dentro de aspas duplas.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- Você deve seguir o conselho do compilador e adicionar um ponto e vírgula no final.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

Acima, você pode notar que o compilador rustc está dando duas sugestões para o código.

Tarefa: siga o conselho do compilador e converta o nome da variável em snake_case.

É uma convenção no Rust usar o caso snake_case para:

- Nomes de variáveis
- Nomes das funções
- Nomes de arquivos

SCREAMING_SNAKE_CASE é usado para constantes e estáticos. Por último, _PascalCase_ é usado para tipos, características e enums (abordaremos esse assunto mais tarde).

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve ter uma variável `first_name` e fornecer um valor do seu primeiro nome dentro das aspas duplas.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

Tarefa: execute o seu código novamente. Você deve ter apenas um aviso agora.

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- Você deve ter uma variável `first_name` e fornecer um valor do seu primeiro nome dentro das aspas duplas.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

O compilador ainda está dando um aviso sobre `first_name` ser uma variável não utilizada.

Tarefa: corrija isso, alterando a chamada de `println!` para:

```rust
    println!("Hello, {}!", first_name);
```

O `'{}'` é substituído pelo valor dos argumentos.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve mudar o `println!` para `println!("Olá, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

Há muitas coisas que você pode fazer com `println!`. Procure por exemplos na documentação do Rust e brinque com o seu código:

- https://doc.rust-lang.org/rust-by-example/hello/print.html

Isso é o que torna a macro `println!` uma excelente ferramenta para depurar seu código.

Tarefa: execute seu código para ver a saída de:

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

- Você deve mudar o `println!` para `println!("Olá, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

O tipo de `first_name` é `&str`. `str` é um tipo primitivo e o _"e" comercial (&)_ indica que o tipo é uma _referência._

Um aspecto importante da linguagem Rust é a propriedade. Ou seja, uso e alocação de memória. O conceito de propriedade surgirá ao longo deste curso.

Outro tipo comum é a `String`. Este é um tipo útil, pois é automaticamente alocado em pilhas. Isto permite que seu tamanho seja desconhecido no momento da compilação.

Tarefa: converta`first_name` para `String`, usando a característica que está disponível na estrutura `String`:

```rust
    let example = String::from("Hello, Camper!");
```

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve usar `String::from()` para criar uma `String` com seu primeiro nome.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

Tarefa: imediatamente após `first_name`, crie uma nova variável chamada `name` e atribua o valor de `first_name` a ela. Então, substitua o segundo argumento na chamada de `println!` por sua variável recém-criada.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma variável `name` e atribuir a ela o valor de `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- Você deve substituir o segundo argumento de `println!` por `name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

Tarefa: copie a chamada de `println!` atual e coloque-a imediatamente após a primeira. Então, substitua o segundo argumento por `first_name`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve ter duas chamadas de `println!`, uma após a outra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- A primeira `println!` deve usar `name` e a segunda `println!` usar `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

Tarefa: execute o seu código. Você verá um erro.

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

- Você deve ter duas chamadas de `println!`, uma após a outra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- A primeira `println!` deve usar `name` e a segunda `println!` usar `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

O aplicativo apresentou uma falha. O motivo dessa falha é que a última chamada `println!` tenta usar a variável `first_name`. Porém, essa variável não está mais disponível. Ela _mudou_ para `name`.

Para evitar que `first_name` mude, você pode atribuir `name` ao valor referenciado de `first_name`.

Tarefa: faça isso adicionando o símbolo de referência no começo do valor de `name`. Exemplo:

```rust
    let value = String::from("");
    let referenced_value = &value;
```

Isso impede que `value` mude para `referenced_value` e, ao invés disso, use uma referência ao valor de `value` em `referenced_value`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve ter duas chamadas de `println!`, uma após a outra.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- A primeira `println!` deve usar `name` e a segunda `println!` usar `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- Você deve referenciar `first_name` quando atribuir para `name`, usando `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

Tarefa: execute o seu código. Você não deve mais ver o erro.

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

- Execute seu código com `cargo run --bin calculator`. Você não deve ver erros.
- `null`

## 15

### --description--

Você quer adicionar o sobrenome (segundo nome) a `name`.

Há muitas maneiras de fazer isto em Rust. Se você tentar concatenar `" Surname"` com `&first_name`, o Rust retornará um erro, porque você não pode fazer concatenação com um valor referenciado.

Você pode remover o &, mas então o segundo `println!` causará problemas ao programa.

Para concatenar uma referência com um `str (&str)`, o primeiro argumento precisa ter um dono. Uma `String` pode ser usada como um valor que possui um dono, com o método `to_owned`:

```rust
    let owned_string = my_string.to_owned() + " Surname";
```

Tarefa: ao invés de mover `first_name`, transforme-o em um valor que possui dono, e concatene o sobrenome com ele - atribua o resultado em `name`.

Execute o código. Se ele compilar e mostrar as duas linhas, você completou a lição corretamente. Se não, use o resultado mostrado para debugar e modificar o código.

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

- Você deve transformar `first_name` em um valor que possui dono com `.to_owned()`.
- `first_name\.to_owned\(\)`
- Você deve concatenar seu sobrenome ao `first_name` que possui dono.
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

Uma maneira mais idiomática de fazer uso do tipo `String` é usando o método `push_str`:

```rust
    let mut my_string = String::from("String");
    my_string.push_str("a str");
```

Tarefa: exclua `name` e a primeira chamada de `println!`. Em seguida, use o método `push_str` no `first_name` para anexar seu sobrenome.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve usar o método `.push_str()`.
- `\.push_str\(`
- Você deve mover seu sobrenome para `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

Tarefa: execute o seu código. Deve dar erro.

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

- Você deve usar o método `.push_str()`.
- `\.push_str\(`
- Você deve mover seu sobrenome para `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Você deve tornar `first_name` mutável com `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

Seu código apresentou erro, porque `first_name` não é _mutável._

Tarefa: use as dicas do compilador para tornar `first_name` mutável.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve usar o método `.push_str()`.
- `\.push_str\(`
- Você deve mover seu sobrenome para `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Você deve tornar `first_name` mutável com `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

Tarefa: execute seu código novamente para garantir que ele seja compilado sem erro.

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

- Execute seu código com `cargo run --bin calculator`. Você não deve ver erros.
- `null`

## 20

### --description--

Até agora, você aprendeu sobre os tipos `str` e `String`, bem como sobre referências. Se você não tiver usado aspas simples acidentalmente ('), pode não ter notado que, até agora, tudo com strings usa aspas duplas (").

Isso ocorre porque há um terceiro tipo de padrão chamado `char`.

Um `char` é um _USV (Valor Escalar Unicode),_ que é representado em unicode com valores como `U+221E` - o unicode de '∞'.

Strings podem ser consideradas coleções ou arrays de `char`s.

Tarefa: remova todo o código de dentro da função `main`. Então, declare uma nova variável `first`, e atribua a ela a primeira letra do seu nome - `first` deve ser do tipo `&str`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma variável chamada `first`.
- `let\s+first`
- Você deve atribuir a `first` um único caractere entre aspas duplas.
- `first\s*=\s*"\w"`

## 21

### --description--

Tarefa: mostre no console o valor do método `.len()` com `first` e o valor de `first.chars().count()`.

Execute seu código para ver o resultado mostrado. Se ele executar e mostrar `'1 1'`, você completou a lição corretamente.

### --seed--

```rust
// Lesson #21
fn main() {
  let first = "T";
}
```

### --tests--

- Você deve mostrar o comprimento de `first` e o número de caracteres em `first`. Exemplo de resultado: `1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

### --description--

Você deve ver `1 1` como resultado no console. O método `len` retorna o comprimento em bytes da `str`. O método `chars` retorna um iterador de `char`s no pedaço de string, e o método `count` retorna o número de elementos no iterador.

Tarefa: mude o valor de `first` para que seja um pedaço de string do caractere infinito: ∞

_DICA:_ você pode copiar e colar o caractere da linha acima

Execute seu código para ver o resultado mostrado. Se ele executar e mostrar `'3 1'`, você completou a lição corretamente.

### --seed--

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Você deve mudar o valor de `first` para que seja um pedaço de string de `∞`.
- `first\s*=\s*"∞"`
- O código deve mostrar `3 1`.
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

Você deve ver `3 1` como resultado no console. O motivo é o fato de o caractere `'∞'` ocupar 3 bytes de comprimento.

Tarefa: sinta-se à vontade para brincar com esses novos métodos e veja valores diferentes as strings produzem.

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Não há testes para esta lição.
- `null`

## 24

### --description--

A partir desta lição, você escreverá o código com _TDD - desenvolvimento orientado a testes_ em mente. Ou seja, você terá que escrever códigos que passem nos testes existentes e também escrever alguns testes por conta própria.

Tarefa: execute o seguinte comando para inicializar o código com testes para a próxima lição:

```bash
    $ fcc reset 24
```

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve ter executado o comando `fcc reset 15`.
- `mod tests`

## 25

### --description--

O que já estiver incluído é a configuração básica para os testes. A sintaxe `#[]` acima de uma declaração é como os _atributos_ são adicionados no Rust.

`cfg(test)` configura `test` para a declaração abaixo. A sintaxe `#[test]` declara quais funções devem ser executadas como testes.

Tarefa: na parte superior do script, adicione uma função chamada `main`. Então, na parte superior do módulo `tests`, importe a função `main` usando esta sintaxe:

```rust
    use crate::main;
```

A palavra-chave `use`, no Rust, é similar a 'import', 'require' ou 'include' em outras linguagens.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 26

### --description--

Como você já deve ter notado, nos testes, funções sem retornos explícitos retornam uma `tuple` (tupla) vazia. Tuplas são representadas com parênteses () - o motivo de o teste avaliar o retorno de `main` é a existência desses `()`.

Há duas maneiras de retornar. Usando a palavra-chave `return` ou não colocando o ponto e vírgula.

Funções retornando qualquer coisa diferente de uma tupla vazia precisam ser explicitamente tipadas:

```rust
    fn my_func() -> String {
      let my_string: String = String::from("Nich");
      my_string
    }
```

_Observação:_ a função acima foi explicitamente tipada, para ficar claro.

Tarefa: passe no teste, retornando `24` da `main` e forneça o tipo de retorno da função como `usize`.

`usize` é o tipo padrão para um número inteiro positivo. O u significa _unsigned_ (sem sinal), enquanto size (tamanho) descreve o tamanho em bits do sistema. Comumente, são sistemas de 64 ou 32 bits.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 27

### --description--

Há muitos tipos de número, no Rust:

    - Inteiros sem sinal: `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
    - Inteiros com sinal: `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
    - Float: `f32`, `f64`

Inteiros sem sinal apenas representam números positivos inteiros. Inteiros com sinal representam números inteiros positivos e negativos. Floats apenas representam frações positivas e negativas.

Tarefa: passe nos testes mudando o número e o tipo de retorno da função `main`.

_Observação:_ o primeiro teste inclui a característica `should_panic`. Isso significa que o código deve mostrar erro.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 28

### --description--

Você quer que sua calculadora seja usada na linha de comando como:

```bash
    $ calculator <first_number> <operator> <second_number>
```

Com uma saída como:

```bash
    $ <first_number> <operator> <second_number> = <result>
```

Exemplo:

```bash
    $ calcualtor 1 + 1
    $ 1 + 1 = 2
```

Tarefa: crie uma função chamada `output`, que aceite 4 argumentos. O primeiro, o terceiro e o quatro argumentos devem ser inteiros signed, enquanto o segundo argumento deve ser um `char`.

_DICA:_ não se esqueça de importar a nova função no módulo de testes.

Aqui está um exemplo de uma função com argumentos tipados:

```rust
    fn example(first_arg: usize, second_arg: String) -> &str {
      "I return a reference to a string slice"
    }
```

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 29

### --description--

Agora, para `output` retornar o resultado correto, você usará o formato macro.

A macro `format!` funciona quase do mesmo jeito que a macro `println!`, que você está usando. A diferença é que, ao invés de mostrar o resultado no console, ela retornará o resultado como uma `String`.

Tarefa: use a macro `format!` para retornar um resultado seguindo este formato:

```bash
    <first_number> <operator> <second_number> = <result>
```

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 30

### --description--

Tarefa: dentro da função `main`, mostre no console o resultado que `output` retorna ao ser chamado com argumentos válidos.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve imprimir no console qualquer saída válida.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

Tarefa: dentro da função `main`, declare três variáveis: `first_number`, `operator` e `second_number`.

Então, atribua a elas valores válidos e as passe como argumentos ao chamar `output`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma variável chamada `first_number`.
- `let first_number`
- Você deve declarar uma variável chamada `second_number`.
- `let second_number`
- Você deve declarar uma variável chamada `operator`.
- `let operator`
- Você deve chamar `output` com as variáveis que você acabou de declarar.
- `output\(\s*first_number\s*,\s*operator\s*,\s*second_number`

## 32

### --description--

Você talvez tenha notado que o que é mostrado no console está errado. Você precisa fazer uma operação nos números fornecidos, para corrigir isso.

Tarefa: declare uma nova função nomeada `operate` que aceita, em ordem, `operator`, `first_number` e `second_number`.

_DICA:_ lembre-se de importar a função no módulo `tests`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 33

### --description--

Você quer ser capaz de executar as quatro operações básicas: adição, subtração, divisão e multiplicação.

Tarefa: use múltiplas declarações `if` para comparar os casos onde `operator` é um desses: `'+' '-' '*' '/'`

Uma declaração `if` segue esta sintaxe:

```rust
    if my_var == "my str" {
      // Do stuff
    } else if my_var == "something else" {
      // Do more stuff
    } else {
      // Finally...
    }
```

Tarefa: retorne o resultado da operação com `first_number` e `second_number` para passar nos testes.

_DICA:_ lembre-se de incluir uma declaração `else`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 34

### --description--

Ao invés de retornar um resultado `0`, quando um operador inválido é utilizado, pode fazer mais sentido fazer a aplicação entrar em pânico.

A macro `panic!` faz isso. Ela aceita uma referência para o pedaço de string como argumento, que pode conter uma mensagem de pânico.

Tarefa: crie pânico no seu código, quando um operador inválido é usado.

Você pode ver se você concluiu a lição corretamente executando:

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

- Não há testes de Node para esta lição
- `null`

## 35

### --description--

Em vez de muitas declarações `if...else`, você pode melhorar a legibilidade e a usabilidade do seu código com o fluxo de controle `match` do Rust. O operador `match` é similar a declaração `switch` de outras linguagens. Porém, ele permite checar se um padrão corresponde a outro.

Um exemplo inventado de uma expressão usando o operador `match`:

```rust
    let some_variable = 't';
    match some_variable {
      'a' => 'A',
      'b' => 'B',
      _ => 'Z',
    }
```

Como `'t'` não corresponde a `'a'` ou `'b'`, a expressão retorna `'Z'`, seguindo o caso base denotado pela sublinha.

Tarefa: converta a lógica de if/else dentro de `operate` para o operador `match`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve usar o operador `match`.
- `match`
- Você ainda deve passar nos testes.
- `getTestOutput(7 passed)`

## 36

### --description--

Você deve ser capaz de usar a calculadora com um valor de entrada assim: `calculator 3 x 3`

Um padrão de `match` pode ser estendido usando uma lógica bit-wise como esta:

```rust
    match name {
      "Quincy" => "Hello, Quincy",
      "Tom" | "Nich" => "Hello, other",
      _ => panic!("Pattern not found"),
    }
```

Com `name` sendo `"Nich"`, o segundo `match` _arm_ terá correspondência.

Tarefa: extenda a multiplicação com o operador `match` para corresponder aos valores de `operator` `'x'` e `'X'`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve usar o operador `|`.
- `|`

## 37

### --description--

Atualmente, o argumento `result` para `output` está dentro do código.

Tarefa: dentro de `main`, declare uma nova variável nomeada `result` e atribua nela um valor chamado `operate` com as primeiras três variáveis. Então, passe `result` como o quarto argumento para `output`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma nova variável `result`
- `let\s+result`
- O código deve mostrar `1 - 10 = -9` no console.
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

Você quer que esta aplicação leia valores dos argumentos da linha de comando. A biblioteca padrão do Rust possui um módulo de ambiente que fornece acesso aos argumentos passados através da linha de comando.

Módulos na biblioteca padrão são acessados usando a seguinte sintaxe:

```rust
    use std::*;
```

Isso importará todos os módulos dentro da biblioteca padrão. No entanto, só é necessário um.

Tarefa: na raíz do script, use a sintaxe acima para importar apenas o módulo env da biblioteca padrão.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve adicionar `use std::env;` ao topo do seu script.
- `use\s+std::env\s*;`

## 39

### --description--

Agora que o módulo `env` foi importado, você pode usar suas estruturas, enums e funções.

Tarefa: na parte superior de `main`, declare uma nova variável chamada `args` e atribua o valor resultante da função `args`, que existe no módulo `env`.

_DICA:_ lembre-se de que, para acessar uma função dentro de um módulo, usamos a sintaxe `'::'`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma nova variável chamada `args`
- `let\s+args`
- Você deve atribuir para `args` um valor de `env::args()`
- `=\s*env::args\(\)`

## 40

### --description--

Tarefa: para saber o que `args` contém, mostre seu valor no console.

_DICA:_ lembre-se de seguir os conselhos do compilador, se você estiver com dificuldades para mostrar o valor.

Você pode ver se você concluiu a lição corretamente executando:

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

- Executar `cargo run --bin calculator` deve mostrar: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

Sem passar nenhum argumento quando executar a crate, o valor de `args` ainda contém um argumento - o caminho do binário.

Tarefa: veja os diferentes valores de `args` executando comandos como:

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

- Não há testes de Node para esta lição.
- `null`

## 42

### --description--

Para acessar um argumento específico de `args`, você pode usar o método `nth`. O método `nth` aceita um argumento numérico (n) para acessar o próximo argumento 'nth' - usando uma indexação baseada em 0.

Tarefa: altere o println de `args` para mostrar o primeiro argumento no console.

_DICA:_ lembre-se de seguir os conselhos do compilador, se você estiver com dificuldades.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve acessar o primeiro elemento (`0`) do iterador `env::args()`.
- `args\.nth\(0\)`
- Você deve declarar `args` como mutável com `let mut args =...`
- `let\s+mut\s+args`

## 43

### --description--

Se você seguiu o conselho do compilador, na lição anterior, você precisava declarar `args` como mutável. O motivo é o método `nth` iterar de modo mutável pelos elementos.

Tarefa: remova o println de 'args'. Então, mude `first_number`, `operator` e `second_number` para que sejam iguais aos primeiro, segundo e terceiro `args`, respectivamente.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve atribuir `args.nth(0)` a `first_number`.
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- Você deve atribuir `args.nth(1)` a `operator`.
- `let\s+operator\s*=\s*args\.nth\(1\)`
- Você deve atribuir `args.nth(2)` a `second_number`.
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

Uma parte do código foi comentada para que o programa seja compilado.

Se você executar o código agora, verá que o resultado contém:

```bash
    $ Some("target/debug/calculator"), None, None
```

Isso ocorre porque `nth` não retorna o valor diretamente. Em vez disso, é retornado o valor encapsulado em uma `Option`.

Uma `Option` é um tipo que inclui `Some`, em torno de um valor, ou `None`, se o valor não existe.

Para usar o valor envolvido por `Some`, a `Option` pode estar _sem invólucro:_

```rust
    let my_option: Option<String> = env::args().nth(0);
    let my_value: String = my_option.unwrap();
```

Tarefa: remova as variáveis `first_number`, `operator` e `second_number` do invólucro na declaração e execute seu código para ver o que acontece.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve remover `first_number` de seu invólucro, com `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- Você deve remover `operator` de seu invólucro, com `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- Você deve remover `second_number` de seu invólucro, com `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

Atualmente, ao executar a aplicação com:

```bash
    $ cargo run --bin calculator
```

Causa um pânico. Isso é porque tentar remover o invólucro de um valor onde `None` existe é um comportamento indefinido.

Existem maneiras de lidar com erros de forma mais tranquila. Por enquanto, certifique-se de chamar a aplicação com argumentos suficientes:

```bash
    $ cargo run --bin calculator -- 1 + 2
```

Tarefa: execute o código de novo, mas continue adicionando argumentos depois de '--', até não haver mais pânico.

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

- Não há testes de Node para esta lição.
- `null`

## 46

### --description--

Atualmente, 5 argumentos são necessários para evitar a chamada de pânico. Parece que você só está tentando remover o terceiro elemento do invólucro?

Na verdade, devido ao `nth` iterar de modo mutável em `args`, depois de acessar o primeiro elemento, ele é removido. Então, tentar acessar o segundo elemento depois é equivalente a ter tentado acessar o terceiro.

Tarefa: mude os argumentos passados para `nth`. Assim, os elementos corretos são acessados. Executar `cargo run --bin calculator -- 1 + 2` deve ter o resultado: "1", "+", "2"

_DICA:_ lembre-se de que o primeiro elemento é o caminho relativo para o binário - não para first_number.

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

- Não há testes de Node para esta lição.
- `null`

## 47

### --description--

Pode ser útil anotar explicitamente os tipos das variáveis. Você já viu exemplos disso, mas aqui está mais um:

```rust
    let my_var: &str = "Mrugesh";
```

Tarefa: anote os tipos das variáveis `args`, `first_number`, `operator` e `second_number`.

_DICA:_ escreva o tipo incorreto em alguma e siga o conselho do compilador para corrigir. Você precisará importar um tipo do módulo `env`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve importar a estrutura `Args` do módulo `std::env` com `use std::env::Args`.
- `use\s+std::env::Args`
- Você deve anotar `args` com o tipo `Args`.
- `let\s+mut\s+args:\s*Args`
- Você deve anotar `first_number` com o tipo `String`.
- `let\s+first_number:\s*String`
- Você deve anotar `operator` com o tipo `String`.
- `let\s+operator:\s*String`
- Você deve anotar `second_number` com o tipo `String`.
- `let\s+second_number:\s*String`

## 48

### --description--

Em vez de escrever importações desnecessárias, você pode combiná-las com a seguinte sintaxe:

```rust
    use std::env::{var, Vars};
```

O código acima importa a função `var` e a estrutura `Vars` a partir do módulo `env` da biblioteca padrão.

Tarefa: use um comando de importação para importar tanto a função `args` como a estrutura `Args`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve combinar as duas importações em uma única declaração de importação com `use std::env::{args, Args};`.
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

Agora, você deve corrigir o problema de `operate` e `output`, esperando tipos de saída `i32` e `&str`.

Isso pode ser feito com o método `parse`, que existe no tipo `String`.

O método `parse` converte uma `String` em outro tipo. O tipo pode ser escrito usando a sintaxe _turbofish_:

```rust
    let my_string_number: String = String::from("Kris");
    let my_number_option: Option<usize> = my_string_number.parse::<usize>();
    let my_number: usize = my_number_option.unwrap();
```

Tarefa: dentro de `main`, declare duas novas variáveis - `first` e `second` - e use a sintaxe turbofish para atribuir os valores convertidos e sem invólucro de `first_number` e `second_number`, respectivamente. Então, substitua `first_number` e `second_number` por `first` e `second` na chamada de `println!`.

Você pode ver se você concluiu a lição corretamente executando:

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

- Você deve declarar uma variável chamada `first`.
- `let\s+first`
- Você deve declarar uma variável chamada `second`.
- `let\s+second`
- Você deve atribuir `first_number.parse::<i32>().unwrap()` para `first`.
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- Você deve atribuir `second_number.parse::<i32>().unwrap()` para `second`.
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

Atualmente, `operator` é do tipo `String` e precisa ser `char`. Uma `String` pode ser convertida em `char` usando o método `chars` e removendo a primeira `Option` retornada do invólucro ao chamar o método `next`.

Tarefa: descomente o código comentado e faça os ajustes necessários para permitir a compilação do código.

Certifique-se de seguir as dicas do compilador para fazer o código compilar. Então, remova o primeiro `println!`, para que haja apenas um resultado.

Você pode garantir que o resultado está correto executando:

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

- Não há testes de Node para esta lição.
- `null`

## 51

### --description--

Atualmente, a calculadora só aceita números inteiros como entrada.

Tarefa: altere todos os tipos necessários para permitir que a calculadora aceite também números decimais.

Você pode ver se você concluiu a lição corretamente executando:

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

- Dica: você deve alterar os tipos de `i32` para `f32`.
- `null`

## 52

### --description--

Você completou o código do binário. Agora, é necessário compilá-lo e entregá-lo para ser usado.

Tarefa: execute o seguinte comando para transformar o código em um binário:

```bash
    $ cargo build --bin calculator
```

Se não houver nenhum erro, você completou a lição com sucesso.

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

- Esta é a lição final. Parabéns!
- `null`

## 53

### --description--

Cargo acabou de compilar o código na pasta `target/debug`.

Tarefa: execute a aplicação usando o seguinte comando:

```bash
    $ target/debug/calculator 1 + 2
```

Se ver o resultado `'1 + 2 = 3'` você completou a lição com sucesso.

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

- Esta é a lição final. Parabéns!
- `null`

## 54

### --description--

O compilador do Rust frequentemente compila com incríveis otimizações. No entanto, você precisa especificar para que Cargo crie uma _versão_ do código, para tirar o máximo proveito dele.

Tarefa: transforme a aplicação novamente. Desta vez, use o marcador `release`:

```bash
    $ cargo build --release --bin calculator
```

Você deve conseguir localizar seu binário otimizado na pasta `target/release`.

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

- Esta é a lição final. Parabéns!
- `null`

## 55

### --description--

Parabéns. Você completou o projeto `freeCodeCamp - Rust no Replit - Calculadora na Linha de Comando`.

Encorajamos você a progredir no projeto atual - quem sabe aceitando múltiplas operações...

Tarefa: execute o seguinte comando para começar o próximo projeto:

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

- Esta é a lição final. Parabéns!
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
