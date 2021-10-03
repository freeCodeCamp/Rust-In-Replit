#!/usr/bin/env node
// Run the file with `node test <argument>` in the terminal

// IMPORTS
const fs = require("fs");
const util = require("util");

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

// CONST
const ARGS = process.argv;
const STEP_TO_TEST = ARGS[2];
const NUMBER_OF_STEPS = 41;
const INVALID_ARGUMENT_MESSAGE = `You should provide the step number you want to test as the only argument. Example \`node test 1\`. There are ${NUMBER_OF_STEPS} steps.`;
const NO_TEST_MESSAGE = `There isn't an available test for step ${STEP_TO_TEST}.`;

// Validate argument
if (ARGS.length != 3 || STEP_TO_TEST <= 0 || STEP_TO_TEST > NUMBER_OF_STEPS) {
  console.log(INVALID_ARGUMENT_MESSAGE);
} else {
  runSwitch();
  // runTests(STEP_TO_TEST);
}

// SWITCH
function runSwitch() {
  switch (STEP_TO_TEST) {
    case "1":
    case "2":
      test1();
      break;
    default:
      runTests(STEP_TO_TEST);
    // console.log(STEP_TO_TEST);
  }
}

// TESTS
async function test1() {
  try {
    const fileContents = await getFileContents("./calculator/src/main.rs");
    if (fileContents) {
      console.log("Lesson #1 is correct.");
    } else {
      console.log("Lesson #1 is not correct.");
    }
  } catch (err) {
    console.log("Lesson #1 is not correct.");
  }
}

async function runTests(lessonNumber) {
  try {
    const camperCode = await getFileContents(`./calculator/src/main.rs`);
    // const camperCode = await getFileContents(`./answers.md`);
    const answers = fs.readFileSync("./answers.md", "utf-8");
    let testTexts = answers.match(
      new RegExp(`## ${lessonNumber}\n+\`\`\`rust[^\`]+\`\`\`\n+([^#]+)`)
    );

    testTexts = testTexts[1]
      .split("-")
      .filter((x) => x.length > 1)
      .map((x) => x.trim());

    for (let i = 0; i < testTexts.length / 2; i += 2) {
      const text = testTexts[i];
      const test = new RegExp(testTexts[i + 1].replace(/[`]/g, ""));
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
          console.log(`Lesson #${lessonNumber} is correct.`);
        } else {
          console.log(`\n${text}\n`);
        }
      } else if (test.test(camperCode)) {
        // TODO
      } else {
        console.log(`\n${text}\n`);
      }
    }
  } catch (e) {
    console.log("An error has occurred");
    console.error(e);
  }
}
