// This file acts as the environment communication layer between commands
// Execution: Bash alias
// fcc='node tooling/fcc.js <project> [<n>] [reset <n>] [solution <n>] [help] [switch <project>]'
// Overloads:
// fcc <n>              - Runs the nth lesson
// fcc reset <n>        - Resets the nth lesson
// fcc solution <n>     - Prints the solution for the nth lesson
// fcc help             - Prints this help message
// fcc switch <project> - Switches between the lessons for <project>
// *fcc test <n>        - Runs the regex tests for the nth lesson

const fs = require("fs");
const util = require("util");
const execute = util.promisify(require("child_process").exec);

const switchAlias = require("./switch");
const runLesson = require("./lesson");
const runSolution = require("./solution");
const runTests = require("./test");
const resetLesson = require("./reset");

const ARGS = process.argv;
const CURRENT_PROJECT = getProjectMeta().CURRENT_PROJECT;

if (ARGS.length < 3) {
  console.log("Not enough arguments given\n");
  console.log(help());
} else if (ARGS.length > 4) {
  console.log("Too many arguments given\n");
  console.log(help());
}

if (isNaN(Number(ARGS[2]))) {
  switch (ARGS[2]) {
    case "help":
    case "--help":
    case "-h":
      console.log(help());
      break;
    case "switch":
      if (CURRENT_PROJECT === ARGS[3]) {
        console.log("Already on project " + CURRENT_PROJECT);
      } else if (!["calculator", "combiner"].includes(ARGS[3])) {
        console.log(
          `Project '${ARGS[3]}' does not exist. Here are the available projects:\n`
        );
        console.log("\tcalculator\n\tcombiner\n");
      } else {
        switchAlias(ARGS[3]);
      }
      break;
    case "reset":
      resetLesson(CURRENT_PROJECT, Number(ARGS[3]));
      break;
    case "solution":
      runSolution(CURRENT_PROJECT, Number(ARGS[3]));
      break;
    case "test":
      runTests(CURRENT_PROJECT, Number(ARGS[3]));
      break;
    default:
      console.log("Invalid argument\n");
      console.log(help());
  }
} else if (!isNaN(Number(ARGS[2]))) {
  if (CURRENT_PROJECT === "calculator") {
    (async () => {
      const cmdToExec = getCmd(ARGS[2]);
      const { stdout, stderr } = await execute(cmdToExec);
      if (stderr) {
        console.log(stderr);
      } else {
        console.log(stdout);
      }
    })();
  } else {
    resetLesson(CURRENT_PROJECT, Number(ARGS[2]));
  }
  runLesson(CURRENT_PROJECT, Number(ARGS[2]));
} else {
  console.log("Invalid arguments\n");
  console.log(help());
}

function getProjectMeta() {
  // Read .meta file for CURRENT_PROJECT variable
  const META_FILE = "tooling/.meta";
  let meta = {
    CURRENT_PROJECT: "calculator",
  };
  try {
    const META = fs.readFileSync(META_FILE, "utf8");
    const metaArr = META.split("\n").filter(Boolean);
    const new_meta = metaArr.reduce((meta, line) => {
      const [key, value] = line.split("=");
      return { ...meta, [key]: value };
    }, "");
    meta = { ...meta, ...new_meta };
  } catch (err) {
    console.log(
      `Defaulting to 'calculator' project.\n\nError reading ${META_FILE}: ${err}`
    );
  }
  return meta;
}

function getCmd(lessonNumber) {
  const numberAsString = numberToName(lessonNumber);
  let t = `tests::${numberAsString}`;
  return `cargo test --bin fcc -q ${t} -- --exact --show-output | sed -n '/--fcc--/, /--fcc--/{ /--fcc--/! p }'`;
}

function numberToName(number) {
  switch (Number(number)) {
    case 0:
      return "";
    case 1:
      return "one";
    case 2:
      return "two";
    case 3:
      return "three";
    case 4:
      return "four";
    case 5:
      return "five";
    case 6:
      return "six";
    case 7:
      return "seven";
    case 8:
      return "eight";
    case 9:
      return "nine";
    case 10:
      return "ten";
    case 11:
      return "eleven";
    case 12:
      return "twelve";
    case 13:
      return "thirteen";
    case 14:
      return "fourteen";
    case 15:
      return "fifteen";
    case 16:
      return "sixteen";
    case 17:
      return "seventeen";
    case 18:
      return "eighteen";
    case 19:
      return "nineteen";
    default:
      return worderiseNumber(number);
  }
}

function worderiseNumber(number) {
  const prefi = [
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
  ];

  let stringRep = String(number);
  let first = stringRep[0];
  let second = stringRep[1];
  let pre = prefi[first - 2];
  console.log(pre, first, second);
  let suf = numberToName(Number(second));
  return `${pre}${suf}`;
}

function help() {
  return `
  chmod +x tooling/fcc - Gives the shell permission to run fcc

  ---

  fcc <n>              - Runs the nth lesson
  fcc reset <n>        - Resets the nth lesson
  fcc solution <n>     - Prints the solution for the nth lesson
  fcc help             - Prints this help message
  fcc switch <project> - Switches between the lessons for <project>
  fcc test <n>         - Runs the regex tests for the nth lesson

  ---

  cargo run --bin <project> - Runs the <project>/src/main.rs binary

  https://doc.rust-lang.org/std/index.html       - Rust documentation
  https://doc.rust-lang.org/book/title-page.html - Rust book
  `;
}
