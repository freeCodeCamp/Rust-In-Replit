// This file displays the solution to the given lesson
const { getLessonFromFile, getLessonSeed } = require("./parser");
const { LOCALE } = require("./t");

function runSolution(project, lessonNumber) {
  const locale = LOCALE === "undefined" ? "english" : LOCALE;
  const answerFile = `./tooling/locales/${locale}/answers-${project}.md`;
  const lessonContent = getLessonFromFile(answerFile, lessonNumber + 1);
  const nextSeedAsSolution = getLessonSeed(lessonContent);
  console.log(nextSeedAsSolution);
}

module.exports = runSolution;
