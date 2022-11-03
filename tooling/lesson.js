// This file parses answer files for lesson content
const { getLessonFromFile, getLessonDescription } = require("./parser");
const { t, LOCALE } = require("./t");

function runLesson(project, lessonNumber) {
  const locale = LOCALE === "undefined" ? "english" : LOCALE ?? "english";
  const answerFile = `./tooling/locales/${locale}/answers-${project}.md`;
  const lesson = getLessonFromFile(answerFile, lessonNumber);
  const nextLesson = getLessonFromFile(answerFile, lessonNumber + 1);
  const description = getLessonDescription(lesson)
    .replace(
      new RegExp(`${t("task")}:`, "g"),
      `${Colour.FgMagenta}${t("task")}:${Colour.Reset}`
    )
    .replace(
      /```(rust|bash)\n(.+?)```\n/gs,
      `${Colour.FgCyan}$2${Colour.Reset}`
    )
    .replace(/`([^`]+)`/g, `${Colour.FgBlue}$1${Colour.Reset}`)
    .replace(/\*\*([^\*]+)\*\*/g, `${Colour.Bright}$1${Colour.Reset}`)
    .replace(/(\s)_([^_]+)_(\s)/g, `$1${Colour.Italic}$2${Colour.Reset}$3`);
  console.log(
    `\n${Colour.Underscore + Colour.FgGreen}${t("lesson")} #${lessonNumber}${
      Colour.Reset
    }\n`
  );
  console.log(description);
  if (!!nextLesson) {
    console.log(
      `${t("next-lesson")}\n\t${Colour.FgCyan}$ fcc ${lessonNumber + 1}${
        Colour.Reset
      }\n`
    );
  }
}

module.exports = runLesson;

const Colour = {
  Reset: "\x1b[0m",
  Bright: "\x1b[1m",
  Dim: "\x1b[2m",
  Italic: "\x1b[3m",
  Underscore: "\x1b[4m",
  Blink: "\x1b[5m",
  Reverse: "\x1b[7m",
  Hidden: "\x1b[8m",

  FgBlack: "\x1b[30m",
  FgRed: "\x1b[31m",
  FgGreen: "\x1b[32m",
  FgYellow: "\x1b[33m",
  FgBlue: "\x1b[34m",
  FgMagenta: "\x1b[35m",
  FgCyan: "\x1b[36m",
  FgWhite: "\x1b[37m",

  BgBlack: "\x1b[40m",
  BgRed: "\x1b[41m",
  BgGreen: "\x1b[42m",
  BgYellow: "\x1b[43m",
  BgBlue: "\x1b[44m",
  BgMagenta: "\x1b[45m",
  BgCyan: "\x1b[46m",
  BgWhite: "\x1b[47m",
};
