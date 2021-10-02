#!/usr/bin/env node
// Run the file with `node reset <argument>` in the terminal

const fs = require("fs");
const util = require("util");

const execute = util.promisify(require("child_process").exec);

const ARGS = process.argv;
const STEP_TO_RESET = ARGS[2];
const NUMBER_OF_STEPS = 30;
const INVALID_ARGUMENT_MESSAGE = `You should provide the lesson number you want to reset to as the only argument.\nExample \`fcc reset 1\` will reset your progress to the beginning of step 1.\n\nThere are ${NUMBER_OF_STEPS} steps.`;
const NO_RESET_MESSAGE = `There isn't an available reset for lesson ${STEP_TO_RESET}.`;
const ERROR_MESSAGE = "An error occurred trying to reset your progress.";

// Validate argument
if (
  ARGS.length != 3 ||
  isNaN(STEP_TO_RESET) ||
  STEP_TO_RESET <= 0 ||
  STEP_TO_RESET > NUMBER_OF_STEPS
) {
  console.log(INVALID_ARGUMENT_MESSAGE);
} else {
  reset(STEP_TO_RESET);
}

async function reset(lessonNumber) {
  try {
    if (lessonNumber === 1) {
      await execute("rm -rf ./calculator", {
        cwd: ".",
        shell: "/bin/bash",
      });
      return r(lessonNumber);
    }

    const answer = getAnswerForLesson(lessonNumber);
    fs.writeFile("./calculator/src/main.rs", answer, function (err) {
      if (err) {
        console.log(ERROR_MESSAGE);
      } else {
        r(lessonNumber);
      }
    });
  } catch (err) {
    console.error(err);
    console.log(ERROR_MESSAGE);
  }
}

function getAnswerForLesson(lessonNumber) {
  const answers = fs.readFileSync("./answers.md", "utf8");
  const answer = answers.match(
    new RegExp(`## ${lessonNumber}\n\n\`\`\`rust\n(.*?)\n\`\`\``, "s")
  )[1];
  return answer;
}

function r(lessonNumber) {
  console.log(`Lesson #${lessonNumber} reset`);
}
