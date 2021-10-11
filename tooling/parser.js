// This file contains the parser for the markdown lessons
const fs = require("fs");

const DESCRIPTION_MARKER = "### --description--";
const SEED_MARKER = "### --seed--";
const TEST_MARKER = "### --tests--";

/**
 * Gets all content within a lesson
 * @param {string} file - The relative path to the
 * @param {number} lessonNumber - The author of the book.
 * @returns {string} The content of the lesson
 */
function getLessonFromFile(file, lessonNumber) {
  const fileContent = fs.readFileSync(file, "utf8");
  const lesson = fileContent.match(
    new RegExp(`## ${lessonNumber}\n(.*?)\n## ${lessonNumber + 1}`, "s")
  )?.[1];
  return lesson;
}

/**
 * Gets the description of the lesson
 * @param {string} lesson - The lesson content
 * @returns {string} The description of the lesson
 */
function getLessonDescription(lesson) {
  const description = lesson.match(
    new RegExp(`${DESCRIPTION_MARKER}\n(.*)\n${SEED_MARKER}`, "s")
  )?.[1];
  return description;
}

/**
 * Gets the seed of the lesson
 * @param {string} lesson - The lesson content
 * @returns {string} The seed of the lesson
 */
function getLessonSeed(lesson) {
  const seed = lesson.match(
    new RegExp(`${SEED_MARKER}\n(.*?)\n${TEST_MARKER}`, "s")
  )?.[1];
  return seed;
}

/**
 * Removes the Markdown from the seed
 * @param {string} seed - The seed content
 * @returns {string} The seed without the markdown
 */
function removeMarkdownFromSeed(seed) {
  const seedWithoutMarkdown = seed.replace(/\n```rust\n/g, "");
  return seedWithoutMarkdown.replace(/```/g, "");
}

/**
 * Gets the tests of the lesson
 * @param {string} lesson - The lesson content
 * @returns {string} The tests of the lesson
 */
function getLessonTests(lesson) {
  const tests = lesson.match(new RegExp(`${TEST_MARKER}\n(.*)`, "s"))?.[1];
  return tests;
}

module.exports = {
  getLessonFromFile,
  getLessonDescription,
  getLessonSeed,
  removeMarkdownFromSeed,
  getLessonTests,
};
