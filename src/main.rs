mod fcc_term;

fn main() {}

#[cfg(test)]
mod tests {
  use super::fcc_term;
  use fcc_term::{term, Colours, Styles, Term};
  use Colours::{Green, Pink, Purple, Red, Turquoise, White, Yellow};
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
      d("to read majority of the instructions, throughout this course.\n"),
    ]);
    term(&[d(
      "To get your first lesson, type the following in the prompt:\n",
    )]);
    term(&[cmd("\t$ fcc 1\n")]);
    term(&[
      d("You can click the "),
      Term::new("Run", Green, Normal),
      d(" button at any time to get back to this screen.\n\nOnce the course is set up, you can get help and see options by running:\n"),
    ]);
    term(&[cmd("\t$ fcc help\n\n")]);
    term(&[
      d("If at any point you get stuck, you can either:\n"),
      d("\t- Reset your code with: "),
      cmd("$ fcc reset <n>\n"),
      d("\t- View the answer to the code with: "),
      cmd("$ fcc solution <n>\n"),
    ]);

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
      cmd("\t$ chmod +x ./tooling/fcc.js"),
      d("\t- gives the shell permission to run the fcc Node module\n"),
    ]);
    term(&[
      cmd("\t$ chmod +x ./tooling/fcc"),
      d("\t- gives the shell permission to run the fcc binary used for the 'cli-calculator' project\n"),
    ]);
    term(&[
      cmd("\t$ fcc <n>"),
      d("\t- shows the instructions for the nth lesson\n"),
    ]);
    term(&[
      cmd("\t$ fcc test <n>"),
      d("\t- runs the node tests for the nth lesson\n"),
    ]);
    term(&[
      cmd("\t$ fcc reset <n>"),
      d("\t- resets the code to the beginning of the nth lesson\n"),
    ]);
    term(&[
      cmd("\t$ fcc solution <n>"),
      d("\t- shows the solution for the nth lesson\n"),
    ]);
    term(&[
      cmd("\t$ fcc switch <project>"),
      d("\t- switch between the available project lessons\n"),
    ]);
    term(&[
      cmd("\t$ cargo run --bin <project>"),
      d("\t- runs the "),
      file("project/src/main.rs "),
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
    n(1);
    term(&[
      d("\n\nThe main tools within the Rust ecosystem are:\n"), 
      file("\t- rustc"),
      d("\tThe compiler which takes your Rust code and compiles it into binary (machine readable code)\n"),
      file("\t- rustup"),
      d("\tThe command line utility to install and update Rust\n"),
      file("\t- cargo"),
      d("\tThe Rust build system and package manager (you will work with this)\n\n")
    ]);
    term(&[
      task(),
      d("Create a new Rust project by running the following command in the prompt:\n"),
      cmd("\t$ cargo new calculator\n\n"),
    ]);

    next(2);
    assert!(true);
  }
  #[test]
  fn two() {
    hr();
    n(2);
    term(&[
      d("\n\nYou have just created a new Rust project within the "),
      file("calculator/ "),
      d("directory.\n"),
    ]);
    term(&[
      d("Cargo has created the boilerplate for a 'Hello World'.\n\n"),
      task(),
      d("Open the "),
      file("calculator/src/main.rs "),
      d("file.\n\n"),
      d("This is the default file Cargo uses for your application binary.\n\n"),
    ]);

    next(3);
    assert!(true);
  }
  #[test]
  fn three() {
    hr();
    n(3);
    term(&[
      d("\n\nThis file contains a "),
      emph("function declaration "),
      d("with the handle "),
      file("main."),
    ]);
    term(&[
      d("By default, rustc calls the "),
      file("main "),
      d("function first whenever the executable is run.\n"),
    ]);
    term(&[
      file("println "),
      d("is a built-in "),
      emph("macro"),
      d(".\n"),
    ]);
    term(&[
      d("A macro is similar to a function, but can be thought of as a piece of code which writes other code.\n"),
      d("For now, the main differences between a function and a macro to keep in mind are:\n")
    ]);
    term(&[
      d("\t- Macros are called using a "),
      emph("bang (!)\n"),
      d("\t- Macros can take a variable number of arguments; functions in Rust cannot\n"),
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
    n(4);
    term(&[
      d("\n\nVariables are declared using the "),
      kw("let "),
      d("keyword.\n"),
    ]);
    term(&[code("\tlet variable_name = value\n\n")]);
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
      kw("const"),
      d(" or "),
      kw("static"),
      d(" keywords.\n"),
    ]);
    term(&[
      task(),
      d("Run your code to see what the compiler says:\n"),
      cmd("\t$ cargo run --bin calculator\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("If you get stuck, try to follow the compiler's helpful advice.\n"),
    ]);

    test(4);
    next(5);
    assert!(true);
  }
  #[test]
  fn five() {
    hr();
    n(5);
    term(&[d(
      "\n\nAbove, you might notice the rustc compiler is giving two suggestions for your code.\n",
    )]);
    term(&[
      task(),
      d("Follow the compiler's advice to convert the variable name into "),
      emph("snake_case.\n"),
    ]);
    term(&[
      d("It is convention in Rust to use snake_case for\n"),
      d("\t- Variable names\n\t- Function names\n\t- File names\n"),
    ]);
    term(&[
      emph("SCREAMING_SNAKE_CASE "),
      d("is used for constants and statics. Lastly, "),
      emph("PascalCase "),
      d("is used for types, traits, and enums (we will cover these later).\n"),
    ]);

    test(5);
    next(6);
    assert!(true);
  }
  #[test]
  fn six() {
    hr();
    n(6);
    term(&[
      d("\n\n"),
      task(),
      d("Re-run your code. You should only have one warning, now.\n"),
    ]);

    next(7);
    assert!(true);
  }
  #[test]
  fn seven() {
    hr();
    n(7);
    term(&[
      d("\n\nThe compiler is still giving you a warning about "),
      kw("first_name "),
      d("being an unused variable.\n"),
    ]);
    term(&[
      task(),
      d("Fix that, by changing the "),
      kw("println! "),
      d("call to be\n"),
      code("\tprintln!(\"Hello, {}!\", first_name);\n"),
    ]);
    term(&[d(
      "The '{}' are replaced with the value of the arguments.\n",
    )]);
    test(7);
    next(8);
    assert!(true);
  }
  #[test]
  fn eight() {
    hr();
    n(8);
    term(&[
      d("\nThere are many things you can do with "),
      kw("println"),
      d(". Look at the Rust by Example docs, and play around with your code:\n"),
      d("\t- https://doc.rust-lang.org/rust-by-example/hello/print.html\n\n"),
      d("This is what makes the "),
      kw("println "),
      d("macro an excellent tool to debug your code.\n"),
    ]);
    term(&[
      task(),
      d("Run your code to see the output with:\n"),
      cmd("\t$ cargo run --bin calculator\n"),
    ]);

    next(9);
    assert!(true);
  }
  #[test]
  fn nine() {
    hr();
    n(9);
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
      d("\nThe concept of ownership will come up, throughout this course.\n"),
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
      code("\tlet example = String::from(\"Hello, Camper!\")\n"),
    ]);

    test(9);
    next(10);
    assert!(true);
  }
  #[test]
  fn ten() {
    hr();
    n(10);
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
      d(" call with your newly created variable.\n"),
    ]);

    test(10);
    next(11);
    assert!(true);
  }
  #[test]
  fn eleven() {
    hr();
    n(11);
    term(&[
      d("\n\n"),
      task(),
      d("Copy the current "),
      kw("println"),
      d(" call, and place it immediately after the first. Then, replace the second argument with "),
      kw("first_name\n"),
    ]);
    test(11);
    next(12);
    assert!(true);
  }
  #[test]
  fn twelve() {
    hr();
    n(12);
    term(&[
      d("\n\n"),
      task(),
      d("Run your code. You will see an error.\n"),
    ]);
    next(13);
    assert!(true);
  }
  #[test]
  fn thirteen() {
    hr();
    n(13);
    term(&[
      d("\n\nYour app errored out. "),
      d("The reason for this error is the last "),
      kw("println"),
      d(" call tries to use the "),
      kw("first_name"),
      d(" variable. However, this variable is no longer available, as it was "),
      emph("moved"),
      d(" into "),
      kw("name"),
      d(".\n"),
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
      d("Do this, by adding the reference symbol to the beginning of the "),
      kw("name"),
      d(" value. Here is an example:\n"),
    ]);
    term(&[
      code("\tlet value = String::from(\"\");\n"),
      code("\tlet referenced_value = &value;\n"),
    ]);
    term(&[
      d("This prevents "),
      kw("value"),
      d(" from being moved into "),
      kw("referenced_value"),
      d(", and, instead, uses a reference to the value of "),
      kw("value"),
      d(" in "),
      kw("referenced_value.\n\n"),
    ]);

    test(13);
    next(14);
    assert!(true);
  }
  #[test]
  fn fourteen() {
    hr();
    n(14);
    term(&[
      d("\n\n"),
      task(),
      d("Run your code. You should not see the error anymore.\n"),
    ]);

    next(15);
    assert!(true);
  }
  #[test]
  fn fifteen() {
    hr();
    n(15);
    term(&[
      d("\n\nYou want to add your surname (second name) to "),
      kw("name"),
      d(".\n"),
    ]);
    term(&[
      d("There are many ways to do this in Rust. If you try to just concatenate "),
      kw("\" Surname\""),
      d(" to "),
      kw("&first_name"),
      d(", Rust will error, because you cannot concatenate to a referenced value.\n\n"),
      d("You could remove the "),
      kw("&"),
      d(", but then the second "),
      kw("println"),
      d(" will cause the program to panic.\n"),
    ]);
    term(&[
      d("In order to concatenate a reference to a "),
      kw("str (&str)"),
      d(", the first argument needs to be "),
      emph("owned"),
      d(". A "),
      kw("String"),
      d(" can be used as an owned value with the "),
      kw("to_owned"),
      d(" method:\n\n"),
      code("\tlet owned_string = my_string.to_owned() + \" Surname\";\n"),
    ]);
    term(&[
      task(),
      d("Instead of moving "),
      kw("first_name"),
      d(", turn it into an owned value, and concatenate your surname to it - assigning the result to "),
      kw("name"), d(".\n")
    ]);
    term(&[d("Run your code. If it compiles and prints the two lines, you have completed the lesson correctly. If not, use the output to debug and fix your code.\n")]);

    next(16);
    assert!(true);
  }
  #[test]
  fn sixteen() {
    hr();
    n(16);
    term(&[
      d("\n\nA more idiomatic way to make use of the "),
      kw("String"),
      d(" type, is by using the "),
      kw("push_str"),
      d(" method:\n\n"),
      code("\tlet mut my_string = String::from(\"String\");\n"),
      code("\tmy_string.push_str(\"a str\");\n"),
    ]);
    term(&[
      task(),
      d("Delete "),
      kw("name"),
      d(" as well as the first "),
      kw("println"),
      d(" call. Then, use the "),
      kw("push_str"),
      d(" method on "),
      kw("first_name"),
      d(" to append your surname.\n"),
    ]);

    test(16);
    next(17);
    assert!(true);
  }
  #[test]
  fn seventeen() {
    hr();
    n(17);
    term(&[
      d("\n\n"),
      task(),
      d("Run your code. It should error out.\n"),
    ]);

    next(18);
    assert!(true);
  }
  #[test]
  fn eighteen() {
    hr();
    n(18);
    term(&[
      d("\n\nYour code errored out, because "),
      kw("first_name"),
      d(" is not "),
      emph("mutable"),
      d(".\n"),
    ]);
    term(&[
      task(),
      d("Use the hints from the compiler to make "),
      kw("first_name"),
      d(" mutable.\n"),
    ]);

    test(18);
    next(19);
    assert!(true);
  }
  #[test]
  fn nineteen() {
    hr();
    n(19);
    term(&[
      d("\n\n"),
      task(),
      d("Run your code again to make sure it compiles without error.\n"),
    ]);

    next(20);
    assert!(true);
  }

  #[test]
  fn twenty() {
    hr();
    n(20);
    term(&[
      d("\n\nSo far, you have learnt about the "),
      kw("str"),
      d(", and "),
      kw("String"),
      d(" types, as well as about references. If you have not accidentally used single quotes ('), you may not have noticed that, so far, everything to do with strings use double quotes (\").\n\n"),
      d("This is because there is a third standard type called "),
      kw("char"),
      d(".\n")
    ]);
    term(&[
      d("A "),
      kw("char"),
      d(" is a "),
      emph("USV (Unicode Scalar Value)"),
      d(", which is represented in unicode with values like "),
      kw("U+221E"),
      d(" - the unicode for '∞'.\n"),
    ]);
    term(&[
      d("Strings can be thought of as collections or arrays of "),
      kw("char"),
      d("s.\n\n"),
      task(),
      d("Remove all of your code from within your "),
      file("main"),
      d(" function. Then, declare a new variable "),
      kw("first"),
      d(", and assign it the first letter of your first name - "),
      kw("first"),
      d(" should be type "),
      kw("&str\n"),
    ]);

    test(20);
    next(21);
    assert!(true);
  }
  #[test]
  fn twentyone() {
    hr();
    n(21);
    term(&[
      d("\n\n"),
      task(),
      d("Print to the console the value of the "),
      kw(".len()"),
      d(" method on "),
      kw("first"),
      d(" and the value of "),
      kw("first.chars().count()\n\n"),
      d("Run your code to see the output. If it runs and prints "),
      cmd("'1 1'"),
      d(", you have correctly completed the lesson.\n"),
    ]);

    next(22);
    assert!(true);
  }
  #[test]
  fn twentytwo() {
    hr();
    n(22);
    term(&[
      d("\n\nYou should see "),
      Term::new("1 1", Red, Normal),
      d(" output in the console. The "),
      kw("len"),
      d(" method returns the length in bytes for the "),
      kw("str"),
      d(". The "),
      kw("chars"),
      d(" method returns an iterator over the "),
      kw("char"),
      d("s in the string slice, and the "),
      kw("count"),
      d(" method returns the number of elements in the iterator.\n"),
    ]);
    term(&[
      task(),
      d("Change the value of "),
      kw("first"),
      d(" to be a string slice of the infinity character: ∞\n\n"),
      emph("HINT: "),
      d("You can copy-paste the character from the above line\n\n"),
      d("Run your code to see the output. If it runs and prints "),
      cmd("'3 1'"),
      d(", you have correctly completed the lesson.\n"),
    ]);

    next(23);
    assert!(true);
  }
  #[test]
  fn twentythree() {
    hr();
    n(23);
    term(&[
      d("\n\nYou should see "),
      Term::new("3 1", Red, Normal),
      d(" output in the console.\n"),
      d("This is because the "),
      file("'∞'"),
      d(" char takes up 3 bytes in length.\n"),
    ]);
    term(&[
      task(),
      d("Feel free to play around with these new methods, as well as get an idea of what values different strings produce\n")
    ]);

    next(24);
    assert!(true);
  }
  #[test]
  fn twentyfour() {
    // 15
    hr();
    n(24);
    term(&[
      d("\n\nFrom this lesson on, you will be writing your code with TDD "),
      emph("Test Driven Development"),
      d(" in mind. That is, you will need to write your code to pass the existing tests, as well as write some tests to pass yourself.\n"),
    ]);
    term(&[
      task(),
      d("Run the following command to initialise your code with tests for the next lesson:\n"),
      cmd("\t$ fcc reset 24\n"),
    ]);

    test(24);
    next(25);
    assert!(true);
  }
  #[test]
  fn twentyfive() {
    // 16
    hr();
    n(25);
    term(&[
      d("\n\nAlready included is the basic setup for tests. The "),
      kw("#[]"),
      d(" syntax above a declaration is how "),
      emph("attributes"),
      d(" are added in Rust.\n\n"),
      kw("cfg(test)"),
      d(" configures the "),
      kw("test"),
      d(" trait to the below declaration, and the "),
      kw("#[test]"),
      d(" syntax declares which functions are to be run as tests.\n"),
    ]);
    term(&[
      task(),
      d("At the top of the script, add a function named "),
      kw("main"),
      d(". Then, at the top of the "),
      kw("tests"),
      d(" module, import the "),
      kw("main"),
      d(" function, using this syntax:\n\n"),
      code("\tuse crate::main;\n"),
    ]);
    term(&[
      d("The "),
      kw("use"),
      d(" keyword, in Rust, is similar to 'import', 'require', or 'include' as in other languages.\n")
    ]);

    test(25);
    next(26);
    assert!(true);
  }
  #[test]
  fn twentysix() {
    hr();
    n(26);
    term(&[
      d("\n\nAs you might notice from the tests, functions without explicit returns return an empty "),
      kw("tuple"),
      d(". Tuples are represented with parentheses () - why the test asserts the return of "),
      kw("main"),
      d(" is "),
      kw("()\n")
    ]);
    term(&[
      d("There are two ways to return. Using the "),
      kw("return"),
      d(" keyword, or by leaving off the semi-colon.\n\n"),
      d("Functions returning anything other than an empty tuple need to be explicitly typed:\n\n"),
      code("\tfn my_func() -> String {\n"),
      code("\t  let my_string: String = String::from(\"Nich\");\n"),
      code("\t  my_string\n"),
      code("\t}\n\n"),
      emph("Note: The above has been explicitly typed, for clarity.\n"),
    ]);
    term(&[
      task(),
      d("Pass the test, by returning 24 from "),
      kw("main"),
      d(", and type the return of the function with the type "),
      kw("usize\n\n"),
    ]);
    term(&[
      kw("usize"),
      d(" is the default type for a positive integer. The "),
      kw("u"),
      d(" stands for "),
      emph("unsigned"),
      d(", and "),
      kw("size"),
      d(" describes the bit-size of the system. This is commonly either 64- or 32- bit systems.\n"),
    ]);

    test(26);
    next(27);
    assert!(true);
  }
  #[test]
  fn twentyseven() {
    hr();
    n(27);
    term(&[
      d("\n\nThere are many types of number, in Rust:\n"),
      d("\t- Unsigned Integers: "),
      kw("u8, u16, u32, u64, usize, u128\n"),
      d("\t- Signed Integer: "),
      kw("i8, i16, i32, i64, isize, i128\n"),
      d("\t- Float: "),
      kw("f32, f64\n"),
    ]);
    term(&[
      d("Unsigned integers only represent positive whole numbers.\n"),
      d("Signed integers represent both positive and negative whole numbers.\n"),
      d("Floats only represent positive and negative fractions.\n"),
    ]);
    term(&[
      task(),
      d("Pass the tests, by changing the number and return type of the "),
      kw("main"),
      d(" function.\n\n"),
      emph("NOTE: "),
      d("The first test includes the "),
      kw("should_panic"),
      d(" trait. This means, the code should error out.\n"),
    ]);

    test(27);
    next(28);
    assert!(true);
  }
  #[test]
  fn twentyeight() {
    hr();
    n(28);
    term(&[
      d("\n\nYou want your calculator to be used on the command line like:\n"),
      cmd("\t$ calculator <first_number> <operator> <second_number>\n\n"),
      d("With an output like:\n"),
      cmd("\t$ <first_number> <operator> <second_number> = <result>\n\n"),
      d("Example:\n"),
      cmd("\t$ calcualtor 1 + 1\n"),
      cmd("\t$ 1 + 1 = 2\n"),
    ]);
    term(&[
      task(),
      d("Create a new function named "),
      kw("output"),
      d(" which accepts 4 arguments. The first, third, and fourth arguments should be signed integers, and the second argument should be a "),
      kw("char"),
      d(".\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Do not forget to import the new function into the "),
      kw("tests"),
      d(" module.\n"),
    ]);
    term(&[
      d("Here is an example function with typed arguments:\n\n"),
      code("\tfn example(first_arg: usize, second_arg: String) -> &str {\n"),
      code("\t  \"I return a reference to a string slice\"\n"),
      code("\t}\n"),
    ]);

    test(28);
    next(29);
    assert!(true);
  }
  #[test]
  fn twentynine() {
    hr();
    n(29);
    term(&[
      d("\n\nNow, to get "),
      kw("output"),
      d(" to return the correct output, you are going to use the "),
      kw("format"),
      d(" macro.\n"),
    ]);
    term(&[
      d("The "),
      kw("format"),
      d(" macro works almost identically to the "),
      kw("println"),
      d(" macro, you have been using. Except, instead of printing the output to the console, it returns the output as a "),
      kw("String\n"),
    ]);
    term(&[
      task(),
      d("Use the "),
      kw("format"),
      d(" macro to return an output following this format:\n"),
      cmd("\t<first_number> <operator> <second_number> = <result>\n"),
    ]);

    test(29);
    next(30);
    assert!(true);
  }
  #[test]
  fn thirty() {
    hr();
    n(30);
    term(&[
      d("\n\n"),
      task(),
      d("Within the "),
      kw("main"),
      d(" function, print to the console the result of calling "),
      kw("output"),
      d(" with any valid arguments.\n"),
    ]);

    test(30);
    next(31);
    assert!(true);
  }
  #[test]
  fn thirtyone() {
    hr();
    n(31);
    term(&[
      d("\n\n"),
      task(),
      d("Within the "),
      kw("main"),
      d(" function, declare three variables: "),
      kw("first_number"),
      d(", "),
      kw("operator"),
      d(", and "),
      kw("second_number\n\n"),
      d("Then, assign them valid values, and pass them as arguments within the "),
      kw("output"),
      d(" call.\n"),
    ]);

    test(31);
    next(32);
    assert!(true);
  }
  #[test]
  fn thirtytwo() {
    hr();
    n(32);
    term(&[
      d("\n\nYou may have noticed what is printed to the console is not correct. You need to perform an operation on the input numbers, to fix this.\n")
    ]);
    term(&[
      task(),
      d("Declare a new function named "),
      kw("operate"),
      d(" which accepts, in order, the "),
      kw("operator"),
      d(", "),
      kw("first_number"),
      d(", and "),
      kw("second_number\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Remember to import the function into the "),
      kw("tests"),
      d(" module.\n"),
    ]);

    test(32);
    next(33);
    assert!(true);
  }
  #[test]
  fn thirtythree() {
    hr();
    n(33);
    term(&[
      d("\n\nYou want to be able to perform the four basic operations: addition, subtraction, division, and multiplication.\n\n"),
      task(),
      d("Use multiple "),
      kw("if"),
      d(" statements to compare the cases where "),
      kw("operator"),
      d(" is one of: "),
      kw("'+' '-' '*' '/'\n\n"),
      d("An "), kw("if"), d(" statement follows this syntax:\n\n"),
      code("\tif my_var == \"my str\" {\n"),
      code("\t  // Do stuff\n"),
      code("\t} else if my_var == \"something else\" {\n"),
      code("\t  // Do more stuff\n"),
      code("\t} else {\n"),
      code("\t  // Finally...\n"),
      code("\t}\n")
    ]);
    term(&[
      task(),
      d("Return the result of the operation on "),
      kw("first_number"),
      d(" and "),
      kw("second_number"),
      d(", to pass the tests.\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Remember to include an "),
      kw("else"),
      d(" clause.\n"),
    ]);

    test(33);
    next(34);
    assert!(true);
  }
  #[test]
  fn thirtyfour() {
    hr();
    n(34);
    term(&[
      d("\n\nInstead of returning a result of '0', when an invalid operator is used, it might make more sense to panic the application.\n\n"),
      d("The "),
      kw("panic"),
      d(" macro does just that, and it accepts a reference to a string slice as an argument, which can contain a message to panic with.\n")
    ]);
    term(&[
      task(),
      d("Panic from your code, when an invalid operator is used.\n"),
    ]);

    test(34);
    next(35);
    assert!(true);
  }
  #[test]
  fn thirtyfive() {
    hr();
    n(35);
    term(&[
      d("\n\nInstead of many "),
      kw("if...else"),
      d(" statements, you can improve your code's readability and useability with Rust's "),
      kw("match"),
      d(" control flow. The "),
      kw("match"),
      d(" operator is similar to many languages' "),
      kw("switch"),
      d(" statement. However, it allows pattern matching.\n\n"),
    ]);
    term(&[
      d("A contrived example of an expression using the "),
      kw("match"),
      d(" operator:\n\n"),
      code("\tlet some_variable = 't';\n"),
      code("\tmatch some_variable {\n"),
      code("\t  'a' => 'A',\n"),
      code("\t  'b' => 'B',\n"),
      code("\t  _ => 'Z',\n"),
      code("\t}\n"),
      d("\nAs 't' does not match 'a' or 'b', the expression returns 'Z', following the base-case denoted by the underscore.\n")
    ]);
    term(&[
      task(),
      d("Convert the if/else logic within "),
      kw("operate"),
      d(" to use the "),
      kw("match"),
      d(" operator.\n"),
    ]);

    test(35);
    next(36);
    assert!(true);
  }
  #[test]
  fn thirtysix() {
    hr();
    n(36);
    term(&[
      d("\n\nYou should be able to use the calculator with an input like: "),
      cmd("calculator 3 x 3\n\n"),
      d("A "),
      kw("match"),
      d(" pattern can be extended using bit-wise logic like this:\n\n"),
      code("\tmatch name {\n"),
      code("\t  \"Quincy\" => \"Hello, Quincy\",\n"),
      code("\t  \"Tom\" | \"Nich\" => \"Hello, other\",\n"),
      code("\t  _ => panic!(\"Pattern not found\"),\n"),
      code("\t}\n\n"),
      d("With a "),
      kw("name"),
      d(" of \"Nich\", the second "),
      kw("match"),
      emph(" arm"),
      d(" would be matched.\n"),
    ]);
    term(&[
      task(),
      d("Extend the multiplication arm in the "),
      kw("match"),
      d(" operator to match on "),
      kw("operator"),
      d(" values of 'x' and 'X'.\n"),
    ]);

    test(36);
    next(37);
    assert!(true);
  }
  #[test]
  fn thirtyseven() {
    hr();
    n(37);
    term(&[
      d("\n\nCurrently, the "),
      kw("result"),
      d(" argument for "),
      kw("output"),
      d(" is hard-coded.\n"),
    ]);
    term(&[
      task(),
      d("Within "),
      kw("main"),
      d(", declare a new variable named "),
      kw("result"),
      d(", and assign it a value of calling "),
      kw("operate"),
      d(" with the first three variables. Then, pass "),
      kw("result"),
      d(" as the fourth argument to "),
      kw("output"),
      d(".\n"),
    ]);

    test(37);
    next(38);
    assert!(true);
  }
  #[test]
  fn thirtyeight() {
    hr();
    n(38);
    term(&[
      d("\n\nYou want this application to read values from command line arguments. Rust's standard library has an environment module which provides access to arguments passed through the CLI.\n")
    ]);
    term(&[
      d("Modules in the standard library are accessed using the following syntax:\n\n"),
      code("\tuse std::*;\n\n"),
      d("This imports all modules within the standard library. However, you only need one.\n"),
    ]);
    term(&[
      task(),
      d("At the root of the script, use the above syntax to import "),
      Term::new("only", White, Bold),
      d(" the "),
      kw("env"),
      d(" module from the standard library.\n"),
    ]);

    test(38);
    next(39);
    assert!(true);
  }
  #[test]
  fn thirtynine() {
    hr();
    n(39);
    term(&[
      d("\n\nNow that the "), kw("env"), d(" module has been brought into scope, you can reference its Structs, Enums, and Functions.\n")
    ]);
    term(&[
      task(),
      d("At the top of "),
      kw("main"),
      d(" declare a new variable named "),
      kw("args"),
      d(", and assign it the value of calling the "),
      kw("args"),
      d(" function, which exists within the "),
      kw("env"),
      d(" module.\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Remember, accessing a function within a module uses the '::' syntax.\n"),
    ]);

    test(39);
    next(40);
    assert!(true);
  }
  #[test]
  fn forty() {
    hr();
    n(40);
    term(&[
      d("\n\n"),
      task(),
      d("To get an idea of what "),
      kw("args"),
      d(" contains, print its value to the console.\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Remember, follow the compiler's helpful advice, if you are struggling to print the value.\n")
    ]);

    test(40);
    next(41);
    assert!(true);
  }
  #[test]
  fn fortyone() {
    hr();
    n(41);
    term(&[
      d("\n\nWithout passing any arguments when running the crate, the value of "),
      kw("args"),
      d(" still contains one argument - the relative path of the binary.\n"),
    ]);
    term(&[
      task(),
      d("See the different values of "),
      kw("args"),
      d(" by running commands like:\n"),
      cmd("\t$ cargo run --bin calculator -- 1 + 2\n"),
    ]);

    next(42);
    assert!(true);
  }
  #[test]
  fn fortytwo() {
    hr();
    n(42);
    term(&[
      d("\n\nIn order to access a specific argument in "),
      kw("args"),
      d(", you can use the "),
      kw("nth"),
      d(" method.\n"),
      d("The "), kw("nth"), d(" method accepts one numeric argument (n) to access the next 'nth' argument - using 0-based indexing.\n")
    ]);
    term(&[
      task(),
      d("Change the "),
      kw("args"),
      d(" println to print the first argument to the console.\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Remember to follow the compiler's helpful advice, if you get stuck.\n"),
    ]);

    test(42);
    next(43);
    assert!(true);
  }
  #[test]
  fn fortythree() {
    hr();
    n(43);
    term(&[
      d(
        "\n\nIf you followed the compiler's advice, in the previous lesson, you needed to delcare ",
      ),
      kw("args"),
      d(" as mutable. This is because the "),
      kw("nth"),
      d(" method mutably iterates over the elements\n"),
    ]);
    term(&[
      task(),
      d("Remove the println for 'args'. Then, change "),
      kw("first_number, operator"),
      d(", and "),
      kw("second_number"),
      d(" to be equal to the first, second, and third "),
      kw("args"),
      d(" respectfully.\n"),
    ]);

    test(43);
    next(44);
    assert!(true);
  }
  #[test]
  fn fortyfour() {
    hr();
    n(44);
    term(&[
      d("\n\nSome code has been commented out, so that the program compiles.\n\nIf you run the code now, you will see the output contains:\n"),
      cmd("\t$ Some(\"target/debug/calculator\"), None, None\n\n"),
      d("This is because "), kw("nth"), d(" does not return the value directly, but, instead, returns the value wrapped in an "), kw("Option"), d(".\n\n"),
      d("An "), kw("Option"), d(" is a type that includes either "), kw("Some"), d(" wrapped around a value, or "), kw("None"), d(" if the value does not exist.\n\n"),
      d("In order to use the value wrapped in "), kw("Some"), d(", the "), kw("Option"), d(" can be "), emph("unwrapped:\n\n"),
      code("\tlet my_option: Option<String> = env::args().nth(0);\n"),
      code("\tlet my_value: String = my_option.unwrap();\n")
    ]);
    term(&[
      task(),
      d("Unwrap the "),
      kw("first_number"),
      d(", "),
      kw("operator"),
      d(", and "),
      kw("second_number"),
      d(" variables at their declaration, and run your code to see what happens.\n"),
    ]);

    test(44);
    next(45);
    assert!(true);
  }
  #[test]
  fn fortyfive() {
    hr();
    n(45);
    term(&[
      d("\n\nCurrently, running the application with:\n"),
      cmd("\t$ cargo run --bin calculator\n"),
      d("Causes a panic. This is because trying to unwrapping a value where "), kw("None"), d(" exists is undefined behaviour.\n\n"),
      d("There are ways to handle errors more gracefully, but, for now, be sure to call your application with enough arguments:\n"),
      cmd("\t$ cargo run --bin calculator -- 1 + 2\n")
    ]);
    // term(&[
    //   task(),
    //   d("Seeing as the type of "), kw()
    // ]);
    term(&[
      task(),
      d(
        "Run your code again, but keep adding arguments after the '--', until there is no panic.\n",
      ),
    ]);

    next(46);
    assert!(true);
  }
  #[test]
  fn fortysix() {
    hr();
    n(46);
    term(&[
      d("\n\nCurrently, 5 arguments are needed, to prevent the application from panicking. It looks like you are only trying to unwrap the 3rd element, though?\n\n"),
      d("Actually, due to "),
      kw("nth"),
      d(" mutably iterating over "),
      kw("args"),
      d(", after accessing the first element, it is removed. So, trying to access the second element afterwards is equivalent to having originally tried to access the third.\n")
    ]);
    term(&[
      task(),
      d("Change the arguments passed to "),
      kw("nth"),
      d(" so that the correct elements are accessed. "),
      d("Running `"),
      cmd("cargo run --bin calculator -- 1 + 2`"),
      d(" should output: \"1\", \"+\", \"2\"\n"),
    ]);

    term(&[
      emph("HINT: "),
      d("Remember, the first element is the relative path to the binary - not the first_number."),
    ]);
    next(47);
    assert!(true);
  }
  #[test]
  fn fortyseven() {
    hr();
    n(47);
    term(&[
      d("\n\nIt can be useful to explicitly annotate your variables' types. You have already seen examples of this, but here is one more:\n\n"),
      code("\tlet my_var: &str = \"Mrugesh\";\n")
    ]);
    term(&[
      task(),
      d("Type-annotate your "),
      kw("args"),
      d(", "),
      kw("first_number"),
      d(", "),
      kw("operator"),
      d(", and "),
      kw("second_number"),
      d(" variables.\n"),
    ]);
    term(&[
      emph("HINT: "),
      d("Give something the incorrect type, and follow the compiler's advice to correct it. You will need to import a type from the "),
      kw("env"),
      d(" module.\n"),
    ]);

    test(47);
    next(48);
    assert!(true);
  }
  #[test]
  fn fortyeight() {
    hr();
    n(48);
    term(&[
      d("\n\nInstead of writing unecessary imports, you can combine them with the following syntax:\n\n"),
      code("\tuse std::env::{var, Vars};\n\n"),
      d("The above imports the "),
      kw("var"), d(" function, and the "),
      kw("Vars"), d(" struct"), d(" from the "),
      kw("env"), d(" module, in the standard library.\n")
    ]);
    term(&[
      task(),
      d("Use one import statement to import both the "),
      kw("args"),
      d(" function, as well as the "),
      kw("Args"),
      d(" struct.\n"),
    ]);

    test(48);
    next(49);
    assert!(true);
  }
  #[test]
  fn fortynine() {
    hr();
    n(49);
    term(&[
      d("\n\nNow, you need to fix the issue of "),
      kw("operate"),
      d(" and "),
      kw("output"),
      d(" expecting "),
      kw("i32"),
      d(" and "),
      kw("&str"),
      d(" type inputs.\n\n"),
      d("This can be acheived with the "),
      kw("parse"),
      d(" method, which exists on the "),
      kw("String"),
      d(" type.\n"),
    ]);
    term(&[
      d("The "),
      kw("parse"),
      d(" method converts a "),
      kw("String"),
      d(" into a given type. The type can be given using "),
      emph("turbofish"),
      d(" syntax:\n\n"),
      code("\tlet my_string_number: String = String::from(\"Kris\");\n"),
      code("\tlet my_number_option: Option<usize> = my_string_number.parse::<usize>();\n"),
      code("\tlet my_number: usize = my_number_option.unwrap();\n"),
    ]);
    term(&[
      task(),
      d("Within main, declare two new variables - "),
      kw("first"),
      d(" and "),
      kw("second"),
      d(" - and use turbofish syntax to assign the parsed and unwrapped values of "),
      kw("first_number"),
      d(" and "),
      kw("second_number"),
      d(" respectfully. Then, replace "),
      kw("first_number"),
      d(" and "),
      kw("second_number"),
      d(" with "),
      kw("first"),
      d(" and "),
      kw("second"),
      d(" in the println call.\n"),
    ]);

    test(49);
    next(50);
    assert!(true);
  }
  #[test]
  fn fifty() {
    hr();
    n(50);
    term(&[
      d("\n\nCurrently, "),
      kw("operator"),
      d(" is of type "),
      kw("String"),
      d(" when it needs to be "),
      kw("char"), d(". A "),
      kw("String"), d(" can be converted into a "),
      kw("char"), d(", using the "),
      kw("chars"), d(" method, and unwrapping the first "),
      kw("Option"), d(" returned by calling the "),
      kw("next"), d(" method.\n\n"),
      task(),
      d("Uncomment the commented-out code, and make the necessary adjustments to allow the code to compile.\n\n"),
      d("Be sure to follow the compiler's hints to get the code compiling. Then, remove the first "),
      kw("println"),
      d(" call so there is only one output.\n")
    ]);

    term(&[
      d("You can ensure the output is correct by running:\n"),
      cmd("\t$ cargo run --bin calculator -- 1 + -1\n"),
    ]);
    next(51);
    assert!(true);
  }
  #[test]
  fn fiftyone() {
    hr();
    n(51);
    term(&[
      d("\n\nCurrently, the calculator only accepts integers as inputs.\n\n"),
      task(),
      d("Change all the necessary types to allow the calculator to accept floating point numbers as well.\n")
    ]);

    test(51);
    next(52);
    assert!(true);
  }
  #[test]
  fn fiftytwo() {
    hr();
    n(52);
    term(&[
      d("\n\nYou have completed the code for your binary. Now, you need to compile and ship it to be used.\n")
    ]);
    term(&[
      task(),
      d("Run the following command to build your code into a binary:\n"),
      cmd("\t$ cargo build --bin calculator\n\n"),
      d("If you see no errors, you have successfully completed the lesson.\n"),
    ]);
    next(53);
    assert!(true);
  }
  #[test]
  fn fiftythree() {
    hr();
    n(53);
    term(&[
      d("\n\nCargo has just compiled your code into the "),
      file("target/debug"),
      d(" directory.\n\n"),
      task(),
      d("Run your application, using the following command:\n"),
      cmd("\t$ target/debug/calculator 1 + 2\n\n"),
      d("If you see the output "),
      cmd("'1 + 2 = 3'"),
      d(" you have successfully completed the lesson.\n"),
    ]);
    next(54);
    assert!(true);
  }
  #[test]
  fn fiftyfour() {
    hr();
    n(54);
    term(&[
      d("\n\nThe Rust compiler often compiles with incredible optimisations. However, you need to specify for Cargo to build a "), emph("release build"), d(" of your code, in order to get the most out of it.\n")
    ]);
    term(&[
      task(),
      d("Rebuild your application, this time using the "),
      kw("release "),
      d("flag:\n"),
      cmd("\t$ cargo build --release --bin calculator\n"),
    ]);
    term(&[
      d("You should be able to locate your optimised binary within the "),
      file("target/release"),
      d(" directory.\n"),
    ]);
    next(55);
    assert!(true);
  }
  #[test]
  fn fiftyfive() {
    hr();
    term(&[
      Term::new("Well Done!", Green, Underline),
      d("\n\nCongratulations. You have completed the "), code("freeCodeCamp - Rust in Replit - CLI Calculator"), d(" project.\n\n"),
      d("You are welcome to extend your current project - perhaps, to accept multiple operations...\n\n"),
      task(),
      d("Run the following command to begin the next project:\n"),
      cmd("\t$ fcc switch image-combiner\n\n")
    ]);
    hr();
  }

  fn d(text: &str) -> Term {
    Term::default(text)
  }
  fn task() -> Term<'static> {
    Term::new("Task: ", Pink, Bold)
  }
  fn next(num: usize) {
    term(&[
      d("When you are done, type the following for the next lesson:\n"),
      Term::new(format!("\t$ fcc {}\n", num).as_str(), Turquoise, Normal),
    ]);
    hr();
  }
  fn cmd(text: &str) -> Term {
    Term::new(text, Turquoise, Normal)
  }
  fn code(text: &str) -> Term {
    Term::new(text, Purple, Normal)
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
  fn n(num: usize) {
    term(&[Term::new(&format!("\nLESSON #{}", num), Green, Underline)]);
  }
  fn test(num: usize) {
    term(&[d(
      "You can see if you completed the lesson correctly by running:",
    )]);
    match num {
      1..=24 | 30 | 31 | 35 | 37 | 38 | 39 | 40 | 42 | 43 | 44 | 47 | 48 | 49 => {
        term(&[cmd(&format!("\t$ fcc test {}\n", num))]);
      }
      _ => {
        term(&[cmd("\t$ cargo test --bin calculator\n")]);
      }
    }
  }
}
