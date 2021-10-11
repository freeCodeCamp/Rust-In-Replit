// IMPORTS
const fs = require("fs");
const util = require("util");

const { getLessonFromFile, getLessonTests } = require("./parser.js");

const execute = util.promisify(require("child_process").exec);
const readFile = util.promisify(fs.readFile);

// HELPER FUNCTIONS
const getCommandOutput = async function (command) {
  const { stdout } = await execute(command, { cwd: ".", shell: "/bin/bash" });
  return stdout;
};

const getFileContents = async (file) => {
  const fileContents = await readFile(file);
  return fileContents.toString();
};

async function runTests(project, lessonNumber) {
  try {
    const camperCodeFile =
      project === "cli-calculator"
        ? "./calculator/src/main.rs"
        : "./combiner/src/main.rs";
    const camperCode = await getFileContents(camperCodeFile);
    const answerFile = `./tooling/answers-${project}.md`;
    const lesson = getLessonFromFile(answerFile, lessonNumber);
    let testTexts = getLessonTests(lesson);

    testTexts = testTexts[1]
      .split(/\n-/)
      .filter((x) => x.length > 1)
      .map((x) => x.trim().replace(/^- /, ""));

    const numTests = testTexts.length / 2;
    let c = 0;
    for (let i = 0; i < numTests * 2; i += 2) {
      const text = testTexts[i];
      if (testTexts[i + 1].includes("getCommandOutput")) {
        const commandOutput = await getCommandOutput(
          "cargo run --bin calculator"
        );
        const re = new RegExp(
          testTexts[i + 1]
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
      } else if (testTexts[i + 1].includes("getTestOutput")) {
        const commandOutput = await getCommandOutput(
          "cargo test --bin calculator"
        );
        const re = new RegExp(
          testTexts[i + 1]
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
        new RegExp(testTexts[i + 1].replace(/[`]/g, "")).test(camperCode)
      ) {
        c++;
        // console.log("\nAll tests pass!\n");
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
