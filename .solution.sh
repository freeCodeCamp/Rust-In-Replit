#!/usr/bin/env node
// Run the file with `node reset <argument>` in the terminal

const fs = require("fs");
const util = require("util");

const execute = util.promisify(require("child_process").exec);

const ARGS = process.argv;
const SOLUTION_TO_SHOW = ARGS[2];
const NUMBER_OF_STEPS = 41;
const INVALID_ARGUMENT_MESSAGE = `You should provide the lesson number you want the solution to as the only argument.\nExample: \`fcc solution 1\` will show a solution to lesson 1.\n\nThere are ${NUMBER_OF_STEPS} lesssons.`;
const NO_SOLUTION_MESSAGE = `There isn't an available solution for lesson ${SOLUTION_TO_SHOW}.`;
const ERROR_MESSAGE = "\nAn error occurred trying to show the solution.\n";

// Validate argument
if (
  ARGS.length != 3 ||
  isNaN(SOLUTION_TO_SHOW) ||
  SOLUTION_TO_SHOW <= 0 ||
  SOLUTION_TO_SHOW > NUMBER_OF_STEPS
) {
  console.log(INVALID_ARGUMENT_MESSAGE);
} else {
  solution(Number(SOLUTION_TO_SHOW) + 1);
}

async function solution(lessonNumber) {
  try {
    if (lessonNumber - 1 == 1) {
      return console.log(NO_SOLUTION_MESSAGE);
    }

    const answer = getAnswerForLesson(lessonNumber);
    console.log(`Solution for lesson #${lessonNumber - 1}\n`);
    console.log(answer);
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
