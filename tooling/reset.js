#!/usr/bin/env node
// Run the file with `node reset <argument>` in the terminal

const fs = require("fs");
const util = require("util");

const execute = util.promisify(require("child_process").exec);

const ERROR_MESSAGE = "An error occurred trying to reset your progress.";

export default async function reset(project, lessonNumber) {
  const rustFile =
    project === "cli-calculator"
      ? "../calculator/src/main.rs"
      : "../combiner/src/main.rs";
  const answerFile = `./answers-${project}.md`;
  try {
    if (lessonNumber === 1) {
      await execute(
        `rm -rf ${
          project === "cli-calculator" ? "../calculator" : "../combiner"
        }`,
        {
          cwd: ".",
          shell: "/bin/bash",
        }
      );
      return r(lessonNumber);
    }

    const answer = getAnswerForLesson(answerFile, lessonNumber);
    fs.writeFile(rustFile, answer, function (err) {
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

function getAnswerForLesson(answerFile, lessonNumber) {
  const answers = fs.readFileSync(answerFile, "utf8");
  const answer = answers.match(
    new RegExp(`## ${lessonNumber}\n\n\`\`\`rust\n(.*?)\n\`\`\``, "s")
  )[1];
  return answer;
}

function r(lessonNumber) {
  console.log(`Lesson #${lessonNumber} reset`);
}
