// This file parses answer files for lesson content
const { getLessonFromFile, getLessonDescription } = require("./parser");

function runLesson(project, LessonNumber) {
  const answerFile = `./tooling/answers-${project}.md`;
  const lesson = getLessonFromFile(answerFile, LessonNumber);
  const description = getLessonDescription(lesson);
  console.log(description);
}

module.exports = runLesson;
