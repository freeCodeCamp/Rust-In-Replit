// This file displays the solution to the given lesson
const { getLessonFromFile, getLessonSeed } = require("./parser");

function runSolution(project, lessonNumber) {
  const answerFile = `./answers-${project}.md`;
  const lessonContent = getLessonFromFile(answerFile, lessonNumber + 1);
  const nextSeedAsSolution = getLessonSeed(lessonContent);
  console.log(nextSeedAsSolution);
}

module.exports = runSolution;
