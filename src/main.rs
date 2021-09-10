mod fcc_term;

fn main() {}

#[cfg(test)]
mod tests {
  use super::fcc_term;
  use fcc_term::{term, Colours, Styles, Term};
  use Colours::{Green, Pink, Red, Turquoise, White, Yellow};
  use Styles::{Bold, Italic, Normal, Underline};

  #[test]
  fn welcome() {
    // std::process::Command::new("source").arg("~/.bashrc").spawn().unwrap();
    hr();
    term(&[
      d("\n\nWelcome to the "),
      Term::new("freeCodeCamp ", Green, Bold),
      Term::new("Rust in Replit ", Red, Normal),
      d("course!\n"),
    ]);

    term(&[
      d("You will be using this console "),
      Term::new("(or the Shell) ", White, Italic),
      d("to read majority of the instructions, throughout this course."),
    ]);
    term(&[d("So, be sure to keep an eye on this area.\n\n")]);
    term(&[Term::new("NOTE: ", Red, Normal), Term::new("If your session with Replit disconnects (timeout or refresh), you will need to run the following in the prompt:\n", Yellow, Underline)]);
    term(&[cmd("\t$ source ~/.bashrc\n")]);
    term(&[d(
      "To get your first lesson, type the following in the prompt:\n",
    )]);
    term(&[cmd("\t$ source ~/.bashrc\n")]);
    term(&[cmd("\t$ fcc 1\n")]);
    term(&[
      d("If at any point you need a reminder, click the "),
      Term::new("Run", Green, Normal),
      d(" button. Or, type the following in the prompt:\n"),
    ]);
    term(&[cmd("\t$ fcc help\n")]);
    hr();
    assert!(true);
  }
  #[test]
  fn help() {
    hr();
    term(&[d("\n\nHere is some helpful information:\n")]);
    term(&[
      cmd("\t$ source ~/.bashrc"),
      d("\t- refreshes the Nix prompt to use custom freeCodeCamp settings\n"),
    ]);
    term(&[
      cmd("\t$ fcc <n>"),
      d("\t- shows the instructions for the nth lesson\n"),
    ]);
    term(&[
      cmd("\t$ cargo run --bin calculator"),
      d("\t- runs the "),
      file("calculator/src/main.rs "),
      d("binary\n"),
    ]);
    term(&[
      cmd("\thttps://doc.rust-lang.org/std/index.html"),
      d("\t- Rust documentation\n"),
    ]);
    term(&[
      cmd("\thttps://doc.rust-lang.org/book/title-page.html"),
      d("\t- Rust book\n"),
    ]);
    hr();
    assert!(true);
  }
  #[test]
  fn one() {
    hr();
    term(&[
      d("\n\nFollow the instructions given in "),
      Term::new("this console", Green, Bold),
      d(" to complete the lessons.\n"),
    ]);
    term(&[
      d("The main tools within the Rust ecosystem are:\n"), 
      file("\t- rustc"),
      d("\tThe compiler which takes your Rust code and compiles it into binary (machine readable code)\n"),
      file("\t- rustup"),
      d("\tThe command line utility to install and update Rust\n"),
      file("\t- cargo"),
      d("\tThe Rust build system and package manager (we will work with this)\n\n")
    ]);
    term(&[
      task(),
      d("Create a new Rust crate, by running the following command in the prompt:\n"),
      cmd("\t$ cargo new calculator\n\n"),
    ]);

    next(2);
    assert!(true);
  }
  #[test]
  fn two() {
    hr();
    term(&[
      d("\n\nYou have just created a new Rust crate within the "),
      file("calculator/ "),
      d("directory.\n"),
    ]);
    term(&[
      d("Cargo has created the boilerplate for a 'Hello World'.\n\n"),
      task(),
      d("Navigate to the "),
      file("calculator/src/main.rs "),
      d("file.\n\n"),
      d("This is the default file Cargo used for your application binary.\n\n"),
    ]);

    next(3);
    assert!(true);
  }
  #[test]
  fn three() {
    hr();
    term(&[
      d("\n\nThis file contains a "),
      emph("function declaration "),
      d("with the handle "),
      file("main."),
    ]);
    term(&[
      d("By default, rustc calls the "),
      file("main "),
      d("function first whenever the the executable is run.\n"),
    ]);
    term(&[
      d("Something important to notice about the contents of the function is the "),
      file("println!() "),
      d("call.\n"),
    ]);
    term(&[
      file("println "),
      d("is a built-in "),
      emph("macro"),
      d(".\n"),
    ]);
    term(&[d("A macro is similar to a function, but can be thought of as a piece of code which writes other code.\n"), d("For now, the main differences between a function and a macro to keep in mind are:\n")]);
    term(&[
      d("\t- Macros are called using a "),
      emph("bang (!)\n"),
      d("\t- Macros can take a variable number of arguments; functions in Rust cannot\n\n"),
    ]);
    term(&[
      task(),
      d("Run your code, using the following command:\n"),
      cmd("\t$ cargo run --bin calculator\n"),
    ]);
    term(&[
      emph("NOTE: "),
      d("The "),
      cmd("--bin calculator "),
      d("arguments are only necessary, because you are not within the "),
      file("calculator "),
      d("directory\n"),
    ]);

    next(4);
    assert!(true);
  }
  #[test]
  fn four() {
    hr();
    term(&[
      d("\n\nVariables are declared using the "),
      kw("let "),
      d("keyword.\n\n"),
    ]);
    term(&[d("\tlet variable_name = value\n\n")]);
    term(&[
      task(),
      d("Within the "),
      file("main "),
      d("function, declare a new variable, and name it "),
      kw("firstName "),
      d("and give it a value of "),
      kw("\"<your_name>\""),
      d(". Ensure to declare it before the "),
      kw("println! "),
      d("call, and place your name within double quotes.\n\n"),
    ]);
    term(&[
      emph("NOTE: "),
      d("Variables can also be declared using the "),
      kw("const "),
      d("or "),
      kw("static "),
      d("keywords, but we will cover these later.\n\n"),
    ]);
    term(&[
      task(),
      d("Run your code to see what the compiler says:\n"),
      cmd("\t$ cargo run --bin calculator\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("If you get stuck, try to follow the compiler's helpful advice.\n\n"),
    ]);

    next(5);
    assert!(true);
  }
  #[test]
  fn five() {
    hr();
    term(&[d(
      "\n\nAbove, you might notice the rustc compiler is giving two suggestions for your code.\n",
    )]);
    term(&[
      task(),
      d("Follow the compiler's advice to convert the variable name into "),
      emph("snake_case.\n\n"),
    ]);
    term(&[
      d("It is convention in Rust to use snake_case for\n"),
      d("\t- Variable names\n\t- Function names\n\t- File names\n\n"),
    ]);
    term(&[
      emph("SCREAMING_SNAKE_CASE "),
      d("is used for constants and statics. Lastly, "),
      emph("PascalCase "),
      d("is used for types, traits, and enums (we will cover these later).\n\n"),
    ]);

    next(6);
    assert!(true);
  }
  #[test]
  fn six() {
    hr();
    term(&[
      d("\n\nThe compiler is still giving us a warning about "),
      kw("first_name "),
      d("being an unused variable.\n"),
    ]);
    term(&[
      task(),
      d("Fix that, by changing the "),
      kw("println! "),
      d("call to be\n"),
      cmd("\tprintln!(\"Hello, {}!\", first_name);\n\n"),
    ]);
    term(&[
      d("The '{}' are replaced with the value of the arguments. There are many things you can do with "),
      kw("println"),
      d(". Look at the Rust by Example docs, and play around with your code:\n"),
      d("\t- https://doc.rust-lang.org/rust-by-example/hello/print.html\n\n"),
      d("This is what makes the "),
      kw("println "),
      d("macro an excellent tool to debug your code.\n\n")
    ]);
    term(&[
      d("Run your code to see the output with:\n"),
      cmd("\t$ cargo run --bin calculator\n\n"),
    ]);

    next(7);
    assert!(true);
  }
  #[test]
  fn seven() {
    hr();
    term(&[
      d("\n\nThe type of "),
      kw("first_name"),
      d(" is "),
      kw("&str"),
      d(".\n"),
      kw("str"),
      d(" is a primitive type, and the "),
      emph("ampersand (&)"),
      d(" indicates the type is a "),
      kw("reference.\n"),
    ]);
    term(&[
      d("An important aspect of the Rust langauge is "),
      emph("ownership"),
      d(". That is, memory use and allocation."),
      d("\nThe concept of ownership will come up, throughout this course.\n\n"),
    ]);
    term(&[
      d("Another common type is "),
      kw("String"),
      d(". This is a useful type, because it is automatically heap allocated. This allows its size to be unknown at compile time.\n")
    ]);
    term(&[
      task(),
      d("Convert "),
      kw("first_name"),
      d(" into the "),
      kw("String"),
      d(" type, by using the "),
      kw("from"),
      emph(" trait"),
      d(" which is available on the "),
      kw("String"),
      emph(" struct:\n"),
      cmd("\tString::from(\"my &str\")\n"),
    ]);
    term(&[d(
      "Do not worry about understanding all these new terms just yet\n\n",
    )]);

    next(8);
    assert!(true);
  }
  #[test]
  fn eight() {
    hr();
    term(&[
      d("\n\n"),
      task(),
      d("Immediately after "),
      kw("first_name"),
      d(", create a new variable named "),
      kw("name"),
      d(", and assign the value of "),
      kw("first_name"),
      d(" to it. Then, replace the second argument in the "),
      kw("println"),
      d(" call with your newly created variable.\n\n"),
    ]);

    next(9);
    assert!(true);
  }
  #[test]
  fn nine() {
    hr();
    term(&[
      d("\n\n"),
      task(),
      d("Copy the current "),
      kw("println"),
      d(" call, and place it immediately after the first. Then, replace the second argument with "),
      kw("first_name\n\n"),
    ]);
    term(&[
      d("If you try to run your code now, your app will panic."),
      emph(" Panicking"),
      d(" is Rust's way of throwing an error a.k.a. 'erroring out'."),
    ]);
    term(&[
      d("The reason for this error is the last "),
      kw("println"),
      d(" call tries to use the "),
      kw("first_name"),
      d(" variable. However, this variable is no longer availble, as it was "),
      emph("moved"),
      d(" into "),
      kw("name\n"),
    ]);
    term(&[
      d("To prevent "),
      kw("first_name "),
      d("from being moved, you can assign "),
      kw("name"),
      d(" to the referenced value of "),
      kw("first_name\n"),
    ]);
    term(&[
      task(),
      d("Do this, by adding the reference symbol mentioned in lesson 7 to the beginning of the "),
      kw("name"),
      d(" value. Here is an example:\n"),
    ]);
    term(&[
      d("\tlet value = String::from(\"\");\n"),
      d("\tlet referenced_value = &value;\n"),
    ]);
    term(&[
      d("This prevents "),
      kw("value"),
      d(" from being moved into "),
      kw("referenced_value"),
      d(", and, instead, uses a reference to the value of "),
      kw("value"),
      d(" in "),
      kw("referenced_value\n\n"),
    ]);

    next(10);
    assert!(true);
  }
  #[test]
  fn ten() {
    term(&[]);
    next(11);
    assert!(true);
  }

  fn d(text: &str) -> Term {
    Term::default(text)
  }
  fn task() -> Term<'static> {
    Term::new("Task: ", Pink, Bold)
  }
  fn next(num: usize) {
    term(&[
      d("When you are done, type the following for the next lesson\n"),
      Term::new(format!("\t$ fcc {}\n", num).as_str(), Turquoise, Normal),
    ]);
    hr();
  }
  fn cmd(text: &str) -> Term {
    Term::new(text, Turquoise, Normal)
  }
  fn file(text: &str) -> Term {
    Term::new(text, Yellow, Normal)
  }
  fn emph(text: &str) -> Term {
    Term::new(text, White, Italic)
  }
  fn kw(text: &str) -> Term {
    Term::new(text, Green, Normal)
  }
  fn hr() {
    term(&[d("--fcc--")]);
  }
}
