// IMPORTS
const fs = require("fs");
const util = require("util");

const { getLessonFromFile, getLessonTests } = require("./parser.js");

const execute = util.promisify(require("child_process").exec);
const readFile = util.promisify(fs.readFile);

// HELPER FUNCTIONS
const getCommandOutput = async function (command) {
  let output = "";
  try {
    const { stdout } = await execute(command, {
      cwd: ".",
      shell: "/bin/bash",
    });
    output = stdout;
  } catch (err) {
    console.log(output);
  }
  return output;
};

const getFileContents = async (file) => {
  const fileContents = await readFile(file);
  return fileContents.toString();
};

async function runTests(project, lessonNumber) {
  try {
    const camperCodeFile = `./${project}/src/main.rs`;
    let camperCode = "";
    try {
      camperCode = await getFileContents(camperCodeFile);
    } catch (err) {
      return console.log(
        "\nIt looks like you have not created a new crate with:\n\t$ cargo new <crate_name>\n"
      );
    }
    const answerFile = `./tooling/answers-${project}.md`;
    const lesson = getLessonFromFile(answerFile, lessonNumber);
    const testTexts = getLessonTests(lesson);

    const testTextsArr = testTexts
      .split(/\n-/)
      .filter((x) => x.length > 1)
      .map((x) => x.trim().replace(/^- /, ""));

    const numTests = testTextsArr.length / 2;
    let c = 0;
    for (let i = 0; i < numTests * 2; i += 2) {
      const text = testTextsArr[i];
      if (testTextsArr[i + 1].includes("getCommandOutput")) {
        const commandOutput = await getCommandOutput(
          `cargo run --bin ${project}`
        );
        const re = new RegExp(
          testTextsArr[i + 1]
            .replace(/[`]/g, "")
            .replace(/getCommandOutput\(/, "")
            .replace(/\)$/, "")
        );
        if (re.test(commandOutput)) {
          c++;
        } else {
          console.log(`\n${text}\n`);
        }
        // Feature for seeing if all Cargo tests pass
      } else if (testTextsArr[i + 1].includes("getTestOutput")) {
        const commandOutput = await getCommandOutput(
          `cargo test --bin ${project}`
        );
        const re = new RegExp(
          testTextsArr[i + 1]
            .replace(/[`]/g, "")
            .replace(/getTestOutput\(/, "")
            .replace(/\)$/, "")
        );
        if (re.test(commandOutput)) {
          c++;
        } else {
          console.log(`\n${text}\n`);
        }
      } else if (
        new RegExp(testTextsArr[i + 1].replace(/[`]/g, "")).test(camperCode)
      ) {
        c++;
      } else {
        console.log(`\n${text}\n`);
      }
    }
    if (c === numTests) {
      console.log(`Lesson #${lessonNumber} is correct.`);
    }
  } catch (e) {
    console.log("An error has occurred");
    console.error(e);
  }
}

module.exports = runTests;
