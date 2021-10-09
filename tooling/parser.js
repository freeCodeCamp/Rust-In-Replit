// This file contains the parser for the markdown lessons
import fs from "fs";

const DESCRIPTION_MARKER = "### --description--";
const SEED_MARKER = "### --seed--";
const TEST_MARKER = "### --tests--";

/**
 * Gets all content within a lesson
 * @param {string} file - The relative path to the
 * @param {number} lessonNumber - The author of the book.
 */
export default function getLessonFromFile(file, lessonNumber) {
  const fileContent = fs.readFileSync(file, "utf8");
  const lesson = fileContent.match(
    new RegExp(`## ${lessonNumber}\n(.*?)\n## ${lessonNumber + 1}`, "s")
  )?.[1];
  return lesson;
}

/**
 * Gets the description of the lesson
 * @param {string} lesson - The lesson content
 */
export function getLessonDescription(lesson) {
  const description = lesson.match(
    new RegExp(`${DESCRIPTION_MARKER}\n(.*)\n${SEED_MARKER}`, "s")
  )?.[1];
  return description;
}

/**
 * Gets the seed of the lesson
 * @param {string} lesson - The lesson content
 */
export function getLessonSeed(lesson) {
  const seed = lesson.match(
    new RegExp(`${SEED_MARKER}\n(.*)\n${TEST_MARKER}`, "s")
  )?.[1];
  return seed;
}

/**
 * Gets the tests of the lesson
 * @param {string} lesson - The lesson content
 */
export function getLessonTests(lesson) {
  const tests = lesson.match(new RegExp(`${TEST_MARKER}\n(.*)`, "s"))?.[1];
  return tests;
}
