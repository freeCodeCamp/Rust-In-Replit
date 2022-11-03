# freeCodeCamp - Відповіді до курсу Rust на Replit

## 1

### --description--

Головними інструментами в екосистемі Rust є:

- rustc Компілятор, який бере ваш Rust-код і компілює його у двійковий (машиночитний) код
- rustup Утиліта командного рядка для того, щоб встановлювати та оновлювати Rust
- cargo Система збірки та керування пакунками для Rust (ви працюватимете з цим)

Завдання: створіть новий проєкт Rust, виконавши наступну команду в prompt:

```bash
    $ cargo new calculator
```

### --seed--

```rust

```

### --tests--

- Для цього уроку немає тестів RegEx.
- `null`

## 2

### --description--

Ви щойно створили новий проєкт Rust в межах директорії `calculator/`.

Cargo створив шаблонний код для «Hello World».

Завдання: відкрийте файл `calculator/src/main.rs`.

Це файл за замовчуванням, який Cargo використовує для двійкового файлу програми.

### --seed--

```rust
// Lesson #2
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Ви повинні мати файл `calculator/src/main.rust` для проєкту.
- `null`

## 3

### --description--

Цей файл містить оголошення функції з ідентифікатором `main`. За замовчуванням, rustc викликає функцію `main` першою під час кожного запуску програми.

`println` – це вбудований макрос.

Макрос схожий на функцію, але його можна розглядати як фрагмент коду, який пише інший код. Сьогодні до головних відмінностей між функцією та макросом, які слід пам'ятати, належать:

    - Макроси викликають за допомогою знаку оклику (!)
    - Макрос може взяти змінну кількість аргументів; функції в Rust не можуть

Завдання: запустіть код, використавши наступну команду:

```bash
    $ cargo run --bin calculator
```

_ПРИМІТКА:_ Аргументи `--bin calculator` необхідні лише тому, що ви не в директорії `calculator`.

### --seed--

```rust
// Lesson #3
fn main() {
  println!("Hello, world!");
}
```

### --tests--

- Виводом після запуску коду повинен бути `Hello, world!`.
- `getCommandOutput(Hello, world!)`

## 4

### --description--

Змінні оголошують за допомогою ключового слова `let`.

```rust
    let variable_name = value
```

Завдання: в межах функції `main` оголосіть нову змінну, назвіть її `firstName` та надайте їй значення `"<your_name>"`. Переконайтеся, що ви оголосили її перед викликом `println!` та помістіть своє ім’я в подвійні лапки.

_ПРИМІТКА:_ змінні також можна оголосити за допомогою ключових слів const та static.

Завдання: запустіть код, щоб побачити, що каже компілятор:

```bash
    $ cargo run --bin calculator
```

_ПІДКАЗКА:_ якщо ви застрягли, спробуйте дотриматись поради компілятора.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити змінну `firstName` та надати їй значення свого імені в подвійних лапках.
- `let\s+firstName\s*=\s*\"\w+\"\s*`
- Ви повинні слідувати порадам компілятора та додати крапку з комою в кінці.
- `let\s+firstName\s*=\s*\"\w+\"\s*;`

## 5

### --description--

Ви, мабуть, помітили, що компілятор rustc надає дві пропозиції до вашого коду.

Завдання: слідуйте порадам компілятора та змініть назву змінної на зміїний_регістр.

У Rust прийнято використовувати зміїний_регістр для:

- Назв змінних
- Назв функцій
- Назв файлів

ВЕРЕСКЛИВИЙ_ЗМІЇНИЙ_РЕГІСТР використовується для констант і статики. _ВерблюдячийРегістр_ використовується для типів, ознак та переліку (розберемо це пізніше).

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні мати змінну `first_name` та надати їй значення свого імені в подвійних лапках.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 6

### --description--

Завдання: перезапустіть свій код. Зараз ви повинні мати лише одне попередження.

### --seed--

```rust
// Lesson #6
fn main() {
  let first_name = "Quincy";
  println!("Hello, world!");
}
```

### --tests--

- Ви повинні мати змінну `first_name` та надати їй значення свого імені в подвійних лапках.
- `let\s+first_name\s*=\s*"\w+"\s*;`

## 7

### --description--

Компілятор досі надає попередження про те, що `first_name` є невикористаною змінною.

Завдання: виправте це, змінивши виклик `println!` на:

```rust
    println!("Hello, {}!", first_name);
```

`'{}'` замінюється значенням аргументів.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні змінити виклик `println!` на `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 8

### --description--

За допомогою `println!` можна зробити багато чого. Гляньте на документацію Rust by Example та потренуйтесь на своєму коді:

- https://doc.rust-lang.org/rust-by-example/hello/print.html

Ця риса робить макрос `println!` відмінним інструментом для налагодження коду.

Завдання: запустіть свій код, щоб побачити вивід із:

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

- Ви повинні змінити виклик `println!` на `println!("Hello, {}!", first_name)`.
- `println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 9

### --description--

Типом `first_name` є `&str`. `str` – це примітивний тип, а _амперсанд (&)_ вказує, що тип є _посиланням._

Важливим аспектом мови Rust є власність. Тобто використання пам'яті та її розподіл. Протягом цього курсу ще згадається концепція власності.

Іншим поширеним типом є `String`. Це корисний тип, оскільки він автоматично розподіляється. Це дозволяє його розміру бути невідомим під час компіляції.

Завдання: перетворіть `first_name` на тип `String`, використовуючи ознаку from, доступну в структурі `String`:

```rust
    let example = String::from("Hello, Camper!");
```

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні використати `String::from()`, щоб створити `String` з вашим ім'ям.
- `let\s+first_name\s*=\s*String::from\(\s*"\w+"\s*\)\s*;`

## 10

### --description--

Завдання: одразу після `first_name` створіть нову змінну з назвою `name` та призначте їй значення `first_name`. Потім змініть другий аргумент у виклику `println!` на щойно створену змінну.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити змінну `name` і призначити їй значення `first_name`.
- `let\s+name\s*=\s*first_name\s*;`
- Ви повинні замінити другий аргумент `println!` на `name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;`

## 11

### --description--

Завдання: скопіюйте поточний виклик `println!` та розмістіть його відразу після першого. Потім замініть другий аргумент на `first_name`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні мати два виклики `println!` відразу один за одним.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- Перший `println!` повинен використовувати `name`, а другий `println!` повинен використовувати `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 12

### --description--

Завдання: запустіть свій код. Ви побачите помилку.

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

- Ви повинні мати два виклики `println!` відразу один за одним.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- Перший `println!` повинен використовувати `name`, а другий `println!` повинен використовувати `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`

## 13

### --description--

У вашій програмі сталася помилка. Причиною цієї помилки є останній виклик `println!`, який намагається використати змінну `first_name`. Однак ця змінна більше не доступна, тому що була _переміщена_ в `name`.

Щоб запобігти переміщенню `first_name`, ви можете присвоїти вихідне значення `first_name` до `name`.

Завдання: зробіть це, додавши символ посилання на початку значення `name`. Ось приклад:

```rust
    let value = String::from("");
    let referenced_value = &value;
```

Це запобігає переміщенню `value` у `referenced_value`, а натомість використовує посилання на значення `value` у `referenced_value`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні мати два виклики `println!` відразу один за одним.
- `println!\("Hello,\s*{}!",\s*\w+\)\s*;\s*println!\("Hello,\s*{}!",\s*\w+\)\s*;`
- Перший `println!` повинен використовувати `name`, а другий `println!` повинен використовувати `first_name`.
- `println!\("Hello,\s*{}!",\s*name\)\s*;\s*println!\("Hello,\s*{}!",\s*first_name\)\s*;`
- Ви повинні посилатися на `first_name` при призначенні його до `name`, використовуючи `&first_name`.
- `let\s+name\s*=\s*&first_name\s*;`

## 14

### --description--

Завдання: запустіть свій код. Ви більше не повинні бачити помилку.

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

- Запустіть свій код за допомогою `cargo run --bin calculator`. Ви не повинні бачити жодних помилок.
- `null`

## 15

### --description--

Було б непогано додати своє прізвище до `name`.

У Rust існує багато способів зробити це. Якщо ви спробуєте просто приєднати `"Surname"` до `&first_name`, то Rust впаде з помилкою, тому що ви не можете приєднувати до вихідного значення.

Ви можете видалити &, але тоді другий `println!` призведе програму до паніки.

Для того, щоб приєднати посилання до `str (&str)`, перший аргумент повинен бути властним. `String` можна використовувати як властне значення за допомогою методу `to_owned`:

```rust
    let owned_string = my_string.to_owned() + " Surname";
```

Завдання: замість того, щоб переміщувати `first_name`, перетворіть його на властне значення та приєднайте своє прізвище до нього, присвоївши результат до `name`.

Запустіть свій код. Якщо він компілюється та друкує два рядки, то ви правильно виконали урок. Якщо ні, використайте вивід для налагодження і виправлення коду.

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

- Ви повинні перетворити `first_name` на властне значення за допомогою `.to_owned()`.
- `first_name\.to_owned\(\)`
- Ви повинні приєднати своє прізвище до властного `first_name`.
- `first_name\.to_owned\(\)\s*\+\s*"[\s\w]+"`

## 16

### --description--

Більш ідіоматичним способом використання типу `String` є метод `push_str`:

```rust
    let mut my_string = String::from("String");
    my_string.push_str("a str");
```

Завдання: видаліть `name`, а також перший виклик `println!`. Потім використайте метод `push_str` на `first_name`, щоб додати своє прізвище.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні використати метод `.push_str()`.
- `\.push_str\(`
- Ви повинні штовхнути своє прізвище до `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`

## 17

### --description--

Завдання: запустіть свій код. Повинна виникнути помилка.

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

- Ви повинні використати метод `.push_str()`.
- `\.push_str\(`
- Ви повинні штовхнути своє прізвище до `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Ви повинні зробити `first_name` мінливим за допомогою `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 18

### --description--

У вашому коді сталася помилка, оскільки `first_name` не є _мінливим._

Завдання: використайте підказки від компілятора, щоб зробити `first_name` мінливим.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні використати метод `.push_str()`.
- `\.push_str\(`
- Ви повинні штовхнути своє прізвище до `first_name`.
- `first_name\.push_str\(\s*"[\s\w]+"\s*\)`
- Ви повинні зробити `first_name` мінливим за допомогою `let mut first_name = ...`
- `let\s+mut\s+first_name\s*=`

## 19

### --description--

Завдання: запустіть свій код ще раз, щоб переконатися, що він компілюється без помилок.

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

- Запустіть свій код за допомогою `cargo run --bin calculator`. Ви не повинні бачити жодних помилок.
- `null`

## 20

### --description--

Наразі ви дізналися про типи `str` і `String`, а також про посилання. Якщо ви випадково не використали одинарні лапки ('), можливо, ви не помітили, що все, що стосується рядків, використовує подвійні лапки (").

Це тому що існує третій стандартний тип під назвою `char`.

`char` є _USV (юнікодовим скалярним значенням),_ який представлений в Юнікоді зі значеннями типу `U+221E` (юнікод для '∞').

Рядки можна вважати колекціями або масивами `char`.

Завдання: видаліть весь свій код із функції `main`. Потім оголосіть нову змінну `first` та призначте їй першу літеру свого імені: `first` повинен бути типом `&str`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити змінну під назвою `first`.
- `let\s+first`
- Ви повинні призначити `first` один символ в подвійних лапках.
- `first\s*=\s*"\w"`

## 21

### --description--

Завдання: надрукуйте в консолі значення методу `.len()` для `first` та значення `first.chars().count()`.

Запустіть свій код, щоб побачити вивід. Якщо він виконується і друкує `'1 1'`, ви правильно виконали урок.

### --seed--

```rust
// Lesson #21
fn main() {
  let first = "T";
}
```

### --tests--

- Ви повинні надрукувати довжину `first` і кількість символів у `first`. Приклад виводу: `1 1`
- `getCommandOutput(\s*1\s*1\s*)`

## 22

### --description--

Ви повинні бачити вивід `1 1` на консолі. Метод `len` повертає довжину в байтах для `str`. Метод `chars` повертає ітератор над `char` у зрізі рядка, а метод `count` повертає кількість елементів в ітераторі.

Завдання: змініть значення `first`, щоб воно було зрізом рядка нескінченного символу: ∞

_ПІДКАЗКА:_ ви можете скопіювати та вставити символ із рядка вище

Запустіть свій код, щоб побачити вивід. Якщо він виконується і друкує `'3 1'`, ви правильно виконали урок.

### --seed--

```rust
// Lesson #22
fn main() {
  let first = "T";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Ви повинні змінити значення `first` на зріз рядка `∞`.
- `first\s*=\s*"∞"`
- Ваш код повинен друкувати `3 1`.
- `getCommandOutput(\s*3\s*1\s*)`

## 23

### --description--

Ви повинні бачити вивід `3 1` на консолі. Це тому що символ `'∞'` займає 3 байти в довжину.

Завдання: попрактикуйтесь з новими методами, щоб отримати уявлення про значення, які створюють різні рядки.

### --seed--

```rust
// Lesson #23
fn main() {
  let first = "∞";
  println!("{} {}", first.len(), first.chars().count());
}
```

### --tests--

- Для цього уроку немає тестів.
- `null`

## 24

### --description--

Починаючи з цього уроку, ви будете писати свій код, маючи на увазі _керовану тестами розробку (англ. Test-driven development (TDD))_. Тобто вам доведеться написати свій код для проходження наявних тестів, а також написати декілька тестів самостійно.

Завдання: запустіть наступну команду, щоб ініціалізувати код з тестами на наступний урок:

```bash
    $ fcc reset 24
```

Перевірити правильність виконання уроку можна за допомогою:

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

- Вам потрібно було виконати команду `fcc reset 15`.
- `mod tests`

## 25

### --description--

Базові налаштування для тестів вже включено. Синтаксис `#[]` над оголошенням є способом додавання _атрибутів_ до Rust.

`cfg(test)` налаштовує ознаку `test` для наведеного нижче оголошення, а синтаксис `#[test]` оголошує, які функції потрібно виконати як тести.

Завдання: зверху скрипту додайте функцію під назвою `main`. Потім зверху модуля `tests` імпортуйте функцію `main`, використавши цей синтаксис:

```rust
    use crate::main;
```

Ключове слово `use` у Rust схоже на 'import', 'require' або 'include', як в інших мовах.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 26

### --description--

Як ви могли помітити з тестів, функції без явних повернень повертають порожній ` tuple` (кортеж). Кортежі представлені дужками () – причина, чому тест перевіряє повернення `main`, полягає в наявності `()`.

Є два способи повернення. Використовуючи ключове слово `return`, або залишивши крапку з комою.

Функції, що повертають будь-що, окрім порожнього кортежу, потрібно вказати чітко:

```rust
    fn my_func() -> String {
      let my_string: String = String::from("Nich");
      my_string
    }
```

_Примітка:_ вищевказане було чітко надруковано для ясності.

Завдання: пройдіть тест, повернувши `24` з `main` та введіть повернення функції з типом `usize`.

`usize` є типом за замовчуванням для додатних цілих чисел. Літера «u» означає _unsigned (беззнаковий)_, а size (розмір) описує бітовий розмір системи. Зазвичай це 64- або 32-бітні системи.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 27

### --description--

У Rust існує багато типів чисел:

    - Беззнакові: `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
    - Зі знаком: `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
    - Плаваючі: `f32`, `f64`

Беззнакові числа представляють лише додатні цілі числа. Числа зі знаком представляють додатні та від’ємні цілі числа. Плаваючі числа представляють лише додатні та від’ємні дроби.

Завдання: пройдіть тести, змінивши число і тип повернення функції `main`.

_ПРИМІТКА:_ перший тест включає ознаку `should_panic`. Це означає, що код повинен впасти з помилкою.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 28

### --description--

Було б непогано, якби ваш калькулятор використовувався в командному рядку ось так:

```bash
    $ calculator <first_number> <operator> <second_number>
```

З таким виводом:

```bash
    $ <first_number> <operator> <second_number> = <result>
```

Наприклад:

```bash
    $ calculator 1 + 1
    $ 1 + 1 = 2
```

Завдання: створіть нову функцію під назвою `output`, яка приймає 4 аргументи. Перший, третій та четвертий аргументи повинні бути цілими числами зі знаком, а другий аргумент повинен бути `char`.

_ПІДКАЗКА:_ не забудьте імпортувати нову функцію в модуль тестів.

Ось приклад функції з введеними аргументами:

```rust
    fn example(first_arg: usize, second_arg: String) -> &str {
      "I return a reference to a string slice"
    }
```

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 29

### --description--

Тепер, щоб `output` повернув правильний вивід, потрібно використати макрос format.

Макрос `format!` працює майже так само, як макрос `println!`, який ви вже використовували. Різниця полягає в тому, що замість друку на консоль, він повертає вивід у вигляді `String`.

Завдання: використайте макрос `format!`, щоб повернути результат у такому форматі:

```bash
    <first_number> <operator> <second_number> = <result>
```

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 30

### --description--

Завдання: у межах функції `main` надрукуйте на консолі результат виклику `output` з будь-якими дійсними аргументами.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні вивести на консоль будь-який дійсний вивід.
- `getCommandOutput(-?\d+ [\+\-\*\\\/xX] -?\d+ = -?\d)`

## 31

### --description--

Завдання: у межах функції `main` оголосіть три змінні: `first_number`, `operator` та `second_number`.

Потім призначте їм дійсні значення та передайте їх як аргументи під час виклику `output`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити змінну під назвою `first_number`.
- `let first_number`
- Ви повинні оголосити змінну під назвою `second_number`.
- `let second_number`
- Ви повинні оголосити змінну під назвою `operator`.
- `let operator`
- Ви повинні викликати `output` зі змінними, які ви щойно оголосили.
- `output\(\s*first_number\s*,\s*operator\s*,\s*second_number`

## 32

### --description--

Можливо, ви помітили, що на консолі неправильний друк. Вам потрібно виконати операцію над введеними числами, щоб виправити це.

Завдання: оголосіть нову функцію під назвою `operate`, яка приймає `operator`, `first_number` та `second_number`, в такому порядку.

_ПІДКАЗКА:_ не забудьте імпортувати функцію в модуль `tests`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 33

### --description--

Було б непогано мати можливість виконувати чотири основні операції: додавання, віднімання, ділення та множення.

Завдання: використайте декілька інструкцій `if`, щоб порівняти випадки, де `operator`</code> є одним з переліченого: `'+' '-' '*' '/'`

Інструкція `if` відповідає такому синтаксису:

```rust
    if my_var == "my str" {
      // Do stuff
    } else if my_var == "something else" {
      // Do more stuff
    } else {
      // Finally...
    }
```

Завдання: поверніть результат операції над `first_number` та `second_number`, щоб пройти тести.

_ПІДКАЗКА:_ не забудьте включити умову `else`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 34

### --description--

Замість того, щоб повертати результат `0`, коли використовується недійсний оператор, було б доцільніше змусити програму панікувати.

Макрос `panic!` робить саме це та приймає посилання на зріз рядка як аргумент, який може містити повідомлення для паніки.

Завдання: зробіть паніку зі свого коду, коли використовується недійсний оператор.

Перевірити правильність виконання уроку можна за допомогою:

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

- Для цього уроку немає тестів Node
- `null`

## 35

### --description--

Замість багатьох інструкцій `if...else` ви можете покращити читабельність та зручність використання коду за допомогою потоку керування `match` від Rust. Оператор `match` схожий на інструкцію `switch` багатьох мов. Однак він дозволяє перевірити, чи один шаблон відповідає іншому.

Вигаданий приклад виразу з використанням оператора `match`:

```rust
    let some_variable = 't';
    match some_variable {
      'a' => 'A',
      'b' => 'B',
      _ => 'Z',
    }
```

Оскільки `'t'` не відповідає `'a'` або `'b'`, то вираз повертає `'Z'`, слідуючи базовому випадку, позначеному підкресленням.

Завдання: конвертуйте логіку if/else в `operate`, щоб використовувався оператор `match`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні використати оператор `match`.
- `match`
- Ви все ще повинні пройти всі тести.
- `getTestOutput(7 passed)`

## 36

### --description--

Ви повинні мати можливість використовувати калькулятор з таким вводом: `calculator 3 x 3`

Шаблон `match` можна розширити за допомогою порозрядної логіки:

```rust
    match name {
      "Quincy" => "Hello, Quincy",
      "Tom" | "Nich" => "Hello, other",
      _ => panic!("Pattern not found"),
    }
```

Якщо `name` є `"Nich"`, то друга _гілка_ `match` буде зіставлена.

Завдання: розширте гілку множення в операторі `match`, щоб відповідати значенням `operator` `'x'` та `'X'`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні використати оператор `|`.
- `|`

## 37

### --description--

Наразі аргумент `result` для `output` жорстко закодований.

Завдання: у межах функції `main` оголосіть нову змінну під назвою `result` та призначить їй значення з виклику `operate` з першими трьома змінними. Потім передайте `result` як четвертий аргумент до `output`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити нову змінну під назвою `result`
- `let\s+result`
- Ваш код повинен друкувати `1 - 10 = -9` на консоль.
- `getCommandOutput(1 - 10 = -9)`

## 38

### --description--

Було б не погано, щоб ця програма читала значення з аргументів командного рядка. Стандартна бібліотека Rust має модуль середовища, який забезпечує доступ до аргументів, що передаються через інтерфейс командного рядка.

Модулі у стандартній бібліотеці доступні за допомогою наступного синтаксису:

```rust
    use std::*;
```

Це імпортує всі модулі зі стандартної бібліотеки. Однак вам потрібен лише один.

Завдання: у корені скрипту використайте наведений вище синтаксис для імпорту модуля env зі стандартної бібліотеки.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні додати `use std::env;` у верхній частині свого сценарію.
- `use\s+std::env\s*;`

## 39

### --description--

Тепер, коли модуль `env` включено в область застосування, ви можете посилатися на його структури, переліки та функції.

Завдання: на початку `main` оголосіть нову змінну під назвою `args` та призначте їй значення виклику функції `args`, що існує в модулі `env`.

_ПІДКАЗКА:_ пам’ятайте, що для доступу до функції всередині модуля використовуємо синтаксис `'::'`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити нову змінну під назвою `args`
- `let\s+args`
- Ви повинні призначити значення `env::args()` до `args`
- `=\s*env::args\(\)`

## 40

### --description--

Завдання: щоб мати уявлення про те, що містить `args`, надрукуйте його значення на консолі.

_ПІДКАЗКА:_ не забувайте дотримуватись порад компілятора, якщо у вас проблеми із друком значення.

Перевірити правильність виконання уроку можна за допомогою:

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

- Запуск `cargo run --bin calculator` повинен надрукувати: `Args { inner: ["target/debug/calculator"] }`
- `getCommandOutput("target/debug/calculator")`

## 41

### --description--

Без передачі жодних аргументів під час запуску ящика, значення `args` досі містить один аргумент – відносний шлях двійкового файлу.

Завдання: розгляньте різні значення `args`, запустивши такі команди:

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

- Для цього уроку немає тестів Node.
- `null`

## 42

### --description--

Щоб отримати доступ до конкретного аргументу в `args`, ви можете використати метод `nth`. Метод `nth` приймає один числовий аргумент (n) для доступу до наступного 'n-го' аргументу, використовуючи індексацію на основі 0.

Завдання: змініть println для `args`, щоб вивести перший аргумент на консоль.

_ПІДКАЗКА:_ не забувайте дотримуватись порад компілятора, якщо ви застрягли.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні отримати доступ до першого елемента (`0`) ітератора `env::args()`.
- `args\.nth\(0\)`
- Ви повинні оголосити `args` мінливим з `let mut args =...`
- `let\s+mut\s+args`

## 43

### --description--

Якщо ви дотримувалися порад компілятора в попередньому уроці, вам потрібно було оголосити `args` мінливим. Це пояснюється тим, що метод `nth` змінно перебирає елементи.

Завдання: видаліть println для 'args'. Потім змініть `first_number`, `operator` та `second_number`, щоб вони дорівнювали першому, другому та третьому `args`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні призначити `args.nth(0)` до `first_number`.
- `let\s+first_number\s*=\s*args\.nth\(0\)`
- Ви повинні призначити `args.nth(1)` до `operator`.
- `let\s+operator\s*=\s*args\.nth\(1\)`
- Ви повинні призначити `args.nth(2)` до `second_number`.
- `let\s+second_number\s*=\s*args\.nth\(2\)`

## 44

### --description--

Деякий код було закоментовано, щоб програма компілювалася.

Якщо ви запустите код зараз, то побачите, що вивід містить:

```bash
    $ Some("target/debug/calculator"), None, None
```

Це тому, що `nth` не повертає значення безпосередньо, але, замість цього, повертає значення, загорнуте в `Option`.

`Option` – це тип, який включає або `Some`, загорнутий навколо значення, або `None`, якщо значення не існує.

Для того, щоб використовувати загорнуте значення в `Some`, можна _розгорнути_ `Option`:

```rust
    let my_option: Option<String> = env::args().nth(0);
    let my_value: String = my_option.unwrap();
```

Завдання: розгорніть змінні `first_number`, `operator` і `second_number` в їхній декларації та запустіть свій код, щоб побачити, що станеться.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні розгорнути `first_number` за допомогою `args.nth(0).unwrap()`.
- `let\s+first_number\s*=\s*args\.nth\(0\)\.unwrap\(\)`
- Ви повинні розгорнути `operator` за допомогою `args.nth(1).unwrap()`.
- `let\s+operator\s*=\s*args\.nth\(1\)\.unwrap\(\)`
- Ви повинні розгорнути `second_number` за допомогою `args.nth(2).unwrap()`.
- `let\s+second_number\s*=\s*args\.nth\(2\)\.unwrap\(\)`

## 45

### --description--

Зараз запуск програми із:

```bash
    $ cargo run --bin calculator
```

Викликає паніку. Це тому що спроба розгорнути значення, де існує `None`, є невизначеною поведінкою.

Існують чепурніші способи обробки помилок, але зараз переконайтеся, що викликаєте програму з достатньою кількістю аргументів:

```bash
    $ cargo run --bin calculator -- 1 + 2
```

Завдання: запустіть код знову, але продовжуйте додавати аргументи після '--', доки не буде паніки.

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

- Для цього уроку немає тестів Node.
- `null`

## 46

### --description--

Зараз потрібно 5 аргументів, щоб запобігти паніці програми. Виглядає наче ви тільки намагаєтесь розгорнути третій елемент?

Насправді через те, що `nth` мінливо ітерує через `args`, після доступу до першого елемента він видаляється. Отже, спроба отримати доступ до другого елемента згодом еквівалентна початковій спробі отримати доступ до третього.

Завдання: змініть аргументи, які були передані до `nth`, щоб отримати доступ до правильних елементів. Запуск `cargo run --bin calculator -- 1 + 2` має вивести: "1", "+", "2"

_ПІДКАЗКА:_ пам’ятайте, що перший елемент є відносним шляхом до двійкового файлу, а не перше_число.

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

- Для цього уроку немає тестів Node.
- `null`

## 47

### --description--

Анотація типів ваших змінних може стати в пригоді. Ви вже бачили приклади, але ось ще один:

```rust
    let my_var: &str = "Mrugesh";
```

Завдання: анотуйте типи ваших змінних `args`, `first_number`, `operator` та `second_number`.

_Підказка:_ надайте якомусь неправильний тип та дотримуйтесь порад компілятора для виправлення. Вам буде потрібно імпортувати тип з модуля `env`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні імпортувати структуру `Args` з модуля `std:env` за допомогою `use std::env::Args`.
- `use\s+std::env::Args`
- Ви повинні анотувати `args` типом `Args`.
- `let\s+mut\s+args:\s*Args`
- Ви повинні анотувати `first_number` типом `String`.
- `let\s+first_number:\s*String`
- Ви повинні анотувати `operator` типом `String`.
- `let\s+operator:\s*String`
- Ви повинні анотувати `second_number` типом `String`.
- `let\s+second_number:\s*String`

## 48

### --description--

Замість того, щоб писати непотрібні імпорти, ви можете поєднувати їх наступним синтаксисом:

```rust
    use std::env::{var, Vars};
```

Наведений вище код імпортує функцію `var`, а також структуру `Vars` з модуля `env` в стандартній бібліотеці.

Завдання: використайте одну інструкцію для імпорту функції `args` та структури `Args`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні об'єднати обидва імпорти в єдину інструкцію імпорту за допомогою `use std::env::{args, Args};`.
- `use\s+std::env::\{args, Args\};`

## 49

### --description--

Зараз ви повинні виправити проблему `operate` та `output` які очікують типи `i32` та `&str` для вводу.

Цього можна досягти за допомогою методу `parse`, який існує в типі `String`.

Метод `parse` перетворює `String` у вказаний тип. Тип можна отримати за допомогою синтаксису _turbofish_:

```rust
    let my_string_number: String = String::from("Kris");
    let my_number_option: Option<usize> = my_string_number.parse::<usize>();
    let my_number: usize = my_number_option.unwrap();
```

Завдання: у межах `main` оголосіть дві нові змінні – `first` і `second` – та використайте синтаксис turbofish, щоб призначити проаналізовані та розгорнуті значення `first_number` і `second_number` відповідно. Потім замініть `first_number` та `second_number` на `first` та `second` у виклику `println!`.

Перевірити правильність виконання уроку можна за допомогою:

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

- Ви повинні оголосити змінну під назвою `first`.
- `let\s+first`
- Ви повинні оголосити змінну під назвою `second`.
- `let\s+second`
- Ви повинні призначити `first_number.parse::<i32>().unwrap()` до `first`.
- `first_number\.parse::<i32>\(\)\.unwrap\(\)`
- Ви повинні призначити `second_number.parse::<i32>().unwrap()` до `second`.
- `second_number\.parse::<i32>\(\)\.unwrap\(\)`

## 50

### --description--

Зараз `operator` є типом `String`, але повинен бути `char`. `String` можна перетворити на `char`, використовуючи метод `chars` та розгорнувши перший `Option`, який повернувся викликом метода `next`.

Завдання: розкоментуйте закоментований код та внесіть необхідні коригування, щоб дозволити компіляцію коду.

Обов’язково дотримуйтесь підказок компілятора, щоб довести код до компілювання. Потім видаліть перший `println!`, щоб мати лише один вивід.

Ви можете переконатися, що вивід правильний, виконавши команду:

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

- Для цього уроку немає тестів Node.
- `null`

## 51

### --description--

Наразі калькулятор приймає за ввід лише цілі числа.

Завдання: змініть усі необхідні типи, щоб дозволити калькулятору приймати й числа з плаваючою комою.

Перевірити правильність виконання уроку можна за допомогою:

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

- Підказка: потрібно змінити типи з `i32` на `f32`.
- `null`

## 52

### --description--

Ви завершили код для свого двійкового файлу. Тепер вам потрібно скомпілювати та відправити його для використання.

Завдання: запустіть наступну команду, щоб побудувати свій код у двійковий файл:

```bash
    $ cargo build --bin calculator
```

Якщо ви не бачите помилок, ви успішно завершили урок.

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

- Це останній урок. Вітаємо!
- `null`

## 53

### --description--

Cargo щойно скомпілював ваш код у директорію `target/debug`.

Завдання: запустіть програму, використавши наступну команду:

```bash
    $ target/debug/calculator 1 + 2
```

Якщо ви бачите вивід `'1 + 2 = 3'`, ви успішно завершили урок.

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

- Це останній урок. Вітаємо!
- `null`

## 54

### --description--

Компілятор Rust часто компілюється з неймовірними оптимізаціями. Однак, необхідно вказати Cargo, щоб він створив _фінальну_ збірку вашого коду, щоб отримати максимальну віддачу.

Завдання: перебудуйте свою програму, цього разу використовуючи прапорець `release`:

```bash
    $ cargo build --release --bin calculator
```

Ви зможете знайти свій оптимізований двійковий файл в директорії `target/release`.

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

- Це останній урок. Вітаємо!
- `null`

## 55

### --description--

Наші вітання. Ви завершили проєкт `freeCodeCamp - Rust на Replit - CLI Calculator`.

Ви можете розширити свій поточний проєкт. Можливо, щоб була можливість прийняти декілька операцій...

Завдання: запустіть нижченаведену команду, щоб розпочати наступний проєкт:

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

- Це останній урок. Вітаємо!
- `null`

## 56

### --description--

### --seed--

```rust
// Placeholder
```

### --tests--

- Заповнювач
- `null`
