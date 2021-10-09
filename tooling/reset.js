const { getLessonFromFile, getLessonSeed } = require("./parser");

const fs = require("fs");
const util = require("util");

const execute = util.promisify(require("child_process").exec);

const ERROR_MESSAGE = "An error occurred trying to reset your progress.";

async function reset(project, lessonNumber) {
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

    const lesson = getLessonFromFile(answerFile, lessonNumber);
    const seed = getLessonSeed(lesson);
    fs.writeFile(rustFile, seed, function (err) {
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

function r(lessonNumber) {
  console.log(`Lesson #${lessonNumber} reset`);
}

module.exports = reset;
