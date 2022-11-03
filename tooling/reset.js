const {
  getLessonFromFile,
  getLessonSeed,
  removeMarkdownFromSeed,
} = require("./parser");

const fs = require("fs");
const util = require("util");

const { t, LOCALE } = require("./t");

const execute = util.promisify(require("child_process").exec);

const ERROR_MESSAGE = t("reset-error");

async function resetLesson(project, lessonNumber) {
  const rustFile = `./${project}/src/main.rs`;
  const locale = LOCALE === "undefined" ? "english" : LOCALE;
  const answerFile = `./tooling/locales/${locale}/answers-${project}.md`;
  try {
    if (lessonNumber === 1) {
      await execute(`rm -rf ../${project}`, {
        cwd: ".",
        shell: "/bin/bash",
      });
      return r(lessonNumber);
    }

    const lesson = getLessonFromFile(answerFile, lessonNumber);
    const seed = getLessonSeed(lesson);
    const seedWithoutMarkdown = removeMarkdownFromSeed(seed);
    fs.writeFile(rustFile, seedWithoutMarkdown, function (err) {
      if (err) {
        console.log(ERROR_MESSAGE);
      } else {
        // r(lessonNumber);
      }
    });
  } catch (err) {
    console.error(err);
    console.log(ERROR_MESSAGE);
  }
}

function r(lessonNumber) {
  console.log(`${t("lesson-reset", { lessonNumber })}`);
}

module.exports = resetLesson;
