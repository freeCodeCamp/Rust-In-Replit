#!/usr/bin/env node
// Run the file with `node reset <argument>` in the terminal

const fs = require("fs");
const util = require("util");

const execute = util.promisify(require("child_process").exec);

const ARGS = process.argv;
const STEP_TO_RESET = ARGS[2];
const NUMBER_OF_STEPS = 41;
const INVALID_ARGUMENT_MESSAGE = `You should provide the lesson number you want to reset to as the only argument.\nExample \`fcc reset 1\` will reset your progress to the beginning of step 1.\n\nThere are ${NUMBER_OF_STEPS} steps.`;
const ERROR_MESSAGE = "An error occurred adding the lesson title to the file.";

// Validate argument
if (
  ARGS.length != 3 ||
  isNaN(STEP_TO_RESET) ||
  STEP_TO_RESET <= 0 ||
  STEP_TO_RESET > NUMBER_OF_STEPS
) {
  console.log(INVALID_ARGUMENT_MESSAGE);
} else {
  addLessonNumber(STEP_TO_RESET);
}

async function addLessonNumber(lessonNumber) {
  try {
    // This adds `// Lesson ${lessonNumber}` to the top of the file 'calculator/src/main.rs'
    const { stdout, stderr } = await execute(
      `sed -i '1 i\// Lesson #${lessonNumber}\n' calculator/src/main.rs`
    );
    if (stderr) {
      console.error(ERROR_MESSAGE);
    }
  } catch (err) {
    console.error(err);
    console.log(ERROR_MESSAGE);
  }
}
