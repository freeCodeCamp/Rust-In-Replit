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
    term(&[Term::new("NOTE: ", Red, Normal), Term::new("If your session with Replit disconnects (timeout or refresh), you will need to run the following in the prompt\n", Yellow, Underline)]);
    term(&[cmd("\t$ source ~/.bashrc\n")]);
    term(&[d(
      "To get your first lesson, type the following in the prompt\n",
    )]);
    term(&[cmd("\t$ source ~/.bashrc\n")]);
    term(&[cmd("\t$ fcc 1\n")]);
    term(&[
      d("If at any point you need a reminder, click the "),
      Term::new("Run", Green, Normal),
      d(" button. Or, type the following in the prompt\n"),
    ]);
    term(&[cmd("\t$ fcc help\n")]);
    assert!(true);
  }
  #[test]
  fn help() {
    term(&[d("\n\nHere is some helpful information\n")]);
    term(&[
      cmd("\t$ source ~/.bashrc"),
      d("\t- refreshes the Nix prompt to use custom freeCodeCamp settings\n"),
    ]);
    term(&[
      cmd("\t$ fcc <n>"),
      d("\t- shows the instructions for the nth lesson\n"),
    ]);
    term(&[
      cmd("\thttps://doc.rust-lang.org/std/index.html"),
      d("\t- Rust documentation\n"),
    ]);
    term(&[
      cmd("\thttps://doc.rust-lang.org/book/title-page.html"),
      d("\t- Rust book\n"),
    ]);
    assert!(true);
  }
  #[test]
  fn one() {
    term(&[
      d("\n\nFollow the instructions given in "),
      Term::new("this console", Green, Bold),
      d(" to complete the lessons.\n"),
    ]);
    term(&[
      d("The main tools within the Rust ecosystem are\n"), 
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
    term(&[
      d("\n\nYou have just created a new Rust crate within the "),
      file("calculator/ "),
      d("directory\n"),
    ]);
    term(&[
      d("Cargo has created the boilerplate for a 'Hello World'.\n\n"),
      task(),
      d("Navigate to the "),
      file("calculator/src/main.rs "),
      d("file.\n\n"),
      d("This is the default file Cargo uses for your application binary\n\n"),
    ]);

    next(3);
    assert!(true);
  }
  #[test]
  fn three() {
    term(&[
      d("\n\nThis file contains a "),
      emph("function declaration "),
      d("with the handle "),
      file("main"),
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
    term(&[d("A macro is similar to a function, but can be thought of as a piece of code which writes other code.\n"), d("For now, the main differences between a function and a macro to keep in mind are\n")]);
    term(&[
      d("\t- Macros are called using a "),
      emph("bang (!)\n"),
      d("\t- Macros can take a variable number of arguments; functions in Rust cannot\n\n"),
    ]);
    term(&[
      task(),
      d("Run your code, using the following command\n"),
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
    term(&[
      d("\n\nVariables are declared using the "),
      kw("let "),
      d("keyword.\n\n"),
    ]);
    term(&[
      task(),
      d("Within the "),
      file("main "),
      d("function, declare a new variable, and name it "),
      kw("myVar "),
      d("and give it a value of "),
      kw("\"<your_name>\""),
      d(". Ensure to declare it before the "),
      kw("println! "),
      d("call, and place your name within double quotes.\n\n"),
    ]);
    term(&[
      emph("NOTE; "),
      d("Variables can also be declared using the "),
      kw("const "),
      d("or "),
      kw("static "),
      d("keywords, but we will cover these later.\n\n"),
    ]);
    term(&[
      emph("HINT; "),
      d("If you get stuck, try to follow the compiler's helpful advice\n\n"),
    ]);

    next(5);
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
}
