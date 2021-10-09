// This file parses answer files for lesson content
import getLessonFromFile, { getLessonDescription } from "./parser";

export default function runLesson(project, LessonNumber) {
  const answerFile = `./answers-${project}.md`;
  const lesson = getLessonFromFile(answerFile, LessonNumber);
  const description = getLessonDescription(lesson);
  console.log(description);
}
