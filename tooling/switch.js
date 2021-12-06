// This file switches the alias of `fcc='node ./tooling/fcc.js <project>'`

const util = require("util");
const execute = util.promisify(require("child_process").exec);
const { t } = require("./t");

// Set alias based on project argv
async function switchAlias(project) {
  try {
    const { stdout, stderr } = await execute(
      `echo '\nCURRENT_PROJECT=${project}' >> ./tooling/.meta`
    );
    if (stderr) {
      console.error(stderr);
    } else {
      console.log(
        `${t("switch-success", { project })}\n\n${t(
          "access-lessons"
        )}\n\t$ fcc 1\n`
      );
    }
  } catch (error) {
    console.log(`\n${t("switch-error")}\n`);
    console.log(
      `${t("switch-navigate", {
        location: "./tooling/.meta",
      })}\n\tCURRENT_PROJECT=<${t("valid-project")}>\n\n${t(
        "access-lessons"
      )}\n\t$ fcc 1\n`
    );
  }
}

module.exports = switchAlias;
