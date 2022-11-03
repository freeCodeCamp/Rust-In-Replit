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

const switchAlias = require("./switch");
const runLesson = require("./lesson");
const runSolution = require("./solution");
const runTests = require("./test");
const resetLesson = require("./reset");
const { t, getProjectMeta } = require("./t");

const { locales, translatedLocales } = require("./locales/conf");
const setLocale = require("./set-locale");

const ARGS = process.argv;
const CURRENT_PROJECT = getProjectMeta().CURRENT_PROJECT;
const LOCALE = getProjectMeta().LOCALE;

if (!locales.includes(LOCALE)) {
  console.error(t("call-to-translate", { locale: LOCALE }));
}

if (ARGS.length < 3) {
  console.log(`${t("not-enough-arguments")}\n`);
  console.log(help());
} else if (ARGS.length > 4) {
  console.log(`${t("too-many-arguments")}\n`);
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
        console.log(t("already-on-project", { project: CURRENT_PROJECT }));
      } else if (!["calculator", "combiner"].includes(ARGS[3])) {
        console.log(`${t("[project-not-exist", { project: ARGS[3] })}\n`);
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
    case "welcome":
      promptForLocale();
      break;
    case "locale":
      const lang = ARGS[3];
      const selected = Object.entries(translatedLocales).find(([k, v], i) => lang == i);
      if (selected) {
        setLocale(selected[0]);
      } else {
        console.log("ERR: INVALID INPUT")
        console.log(`\n\t${Object.values(translatedLocales).map((v, i) => `${i}) ${v}`).join("\n\t")}`);
      }
      break;
    default:
      console.log(ARGS, CURRENT_PROJECT, LOCALE);
      console.log(`${t("invalid-argument")}\n`);
      console.log(help());
      break;
  }
} else if (!isNaN(Number(ARGS[2]))) {
  if (CURRENT_PROJECT === "calculator") {
    if (Number(ARGS[2]) > 24) {
      resetLesson(CURRENT_PROJECT, Number(ARGS[2]));
    }
  } else {
    resetLesson(CURRENT_PROJECT, Number(ARGS[2]));
  }
  runLesson(CURRENT_PROJECT, Number(ARGS[2]));
} else {
  console.log(`${t("invalid-argument")}\n`);
  console.log(help());
}

function help() {
  return `
  chmod +x tooling/fcc - ${t("shell-permission")}

  ---

  fcc <n>              - ${t("fcc-n")}
  fcc reset <n>        - ${t("fcc-reset-n")}
  fcc solution <n>     - ${t("fcc-solution-n")}
  fcc help             - ${t("fcc-help")}
  fcc switch <project> - ${t("fcc-switch-project")}
  fcc test <n>         - ${t("fcc-test-n")}
  fcc locale <n>  - ${t("fcc-locale")}

  ---

  cargo run --bin <project> - ${t("cargo-run")}

  https://doc.rust-lang.org/std/index.html       - ${t("rust-docs")}
  https://doc.rust-lang.org/book/title-page.html - ${t("rust-book")}
  `;
}

function welcome() {
  return t("welcome");
}

function promptForLocale() {
  const greetings = locales.map((x) => t("greeting", {}, x));
  console.log("\n");
  greetings.forEach((x) => console.log(x));
  console.log(`
  
  \t${Object.values(translatedLocales).map((v, i) => `${i}) ${v}`).join("\n\t")}
  `);
  const readline = require("readline");
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  function getInput() {
    rl.question(">>: ", (lang = 0) => {
      const selected = Object.entries(translatedLocales).find(([k, v], i) => lang == i);
      if (!selected) {
        getInput();
      } else {
        setLocale(selected[0]);
        console.log(`Language set to ${selected[1]}`);
        rl.close();
      }
    });
  }
  getInput();

  rl.on("close", function () {
    console.log("\n\n");
    console.log(welcome());
    console.log("\n");
    process.exit(0);
  });
}
