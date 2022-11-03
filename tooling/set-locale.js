// This file sets the LOCALE of ./tooling/.meta

const util = require("util");
const execute = util.promisify(require("child_process").exec);
const { t } = require("./t");

// Set alias based on project argv
async function setLocale(locale) {
  const loc = locale === "undefined" ? "english" : locale;
  try {
    const { stdout, stderr } = await execute(
      `echo '\nLOCALE=${loc}' >> ./tooling/.meta`
    );
    if (stderr) {
      console.error(stderr);
    } else {
      console.log(
        `${t("set-locale-success", { loc })}\n\n${t(
          "access-lessons"
        )}\n\t$ fcc 1\n`
      );
    }
  } catch (error) {
    console.log(`\n${t("set-locale-error")}\n`);
    console.log(
      `${t("switch-navigate", {
        location: "./tooling/.meta",
      })}\n\tLOCALE=<${t("valid-locale")}>\n\n${t(
        "access-lessons"
      )}\n\t$ fcc 1\n`
    );
  }
}

module.exports = setLocale;
