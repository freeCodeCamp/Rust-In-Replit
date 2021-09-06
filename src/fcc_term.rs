// cargo test -q welcome -- --show-output | awk '$0 ~ /:|unning/mg {print $0}'

#[allow(dead_code)]
pub struct Term<'a> {
  font_style: Styles,
  colour: Colours,
  text: &'a str,
}

#[allow(dead_code)]
impl Term<'_> {
  pub fn new(text: &str, colour: Colours, style: Styles) -> Term {
    Term {
      font_style: style,
      text,
      colour,
    }
  }
  pub fn default(text: &str) -> Term {
    Term {
      font_style: Styles::Normal,
      colour: Colours::White,
      text,
    }
  }
  // pub fn task(text: &str) -> Term {
  //   let with_task = String::from("Task: ")
  //   Term {
  //     font_style: Styles::Bold,
  //     colour: Colours::Pink,
  //     with_task,
  //   }
  // }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Styles {
  Normal = 0,
  Bold = 1,
  Faint = 2,
  Italic = 3,
  Underline = 4,
  Strike = 9,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Colours {
  Black = 0,
  Red = 9,
  Green = 10,
  Pink = 13,
  Turquoise = 14,
  Yellow = 11,
  White = 255,
}

impl Copy for Styles {}
impl Copy for Colours {}

impl Clone for Styles {
  fn clone(&self) -> Self {
    *self
  }
}
impl Clone for Colours {
  fn clone(&self) -> Self {
    *self
  }
}

#[allow(dead_code)]
pub fn term(t_arr: &[Term]) {
  let mut output = "".to_owned();
  for t in t_arr.iter() {
    output.push_str(&format!(
      "\x1B[{:?};38;5;{:?}m{}\x1B[0m",
      t.font_style as u8, t.colour as u8, t.text
    ));
  }
  println!("{}", output);
}

// pub fn term(t: Term) {
//   // println!("{:#?}", t.font_style as u8);
//   for i in 0..255 {
//     println!("\x1B[{:?};38;5;{:?}m{}{}\x1B[0m", Styles::Normal as u8, i, t.text, i);
//   }
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use Colours::{Green, Red};
//   use Styles::{Bold, Normal};

//   #[test]
//   fn welcome() {
//     term(&[
//       Term::default("Welcome to the "),
//       Term::new("freeCodeCamp ", Green, Bold),
//       Term::new("Rust in Replit ", Red, Normal),
//       Term::default("course!"),
//     ]);
//     assert!(true);
//   }

//   #[test]
//   fn instruction_1() {
//     term(&[Term::default(
//       "The instructions will appear here - in the console",
//     )]);
//     assert!(true);
//   }
// }
