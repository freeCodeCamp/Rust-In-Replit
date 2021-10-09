// This file displays the solution to the given lesson
import getLessonFromFile, { getLessonSeed } from "./parser";

export default function runSolution(project, lessonNumber) {
  const answerFile = `./answers-${project}.md`;
  const lessonContent = getLessonFromFile(answerFile, lessonNumber + 1);
  const nextSeedAsSolution = getLessonSeed(lessonContent);
  console.log(nextSeedAsSolution);
}
