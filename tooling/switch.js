// This file switches the alias of `fcc='node ./tooling/fcc.js <project>'`

const util = require("util");
const execute = util.promisify(require("child_process").exec);

// Set alias based on project argv
async function switchAlias(project) {
  try {
    const { stdout, stderr } = await execute(
      `echo 'CURRENT_PROJECT=${project}' >> ./tooling/.meta`
    );
    if (stderr) {
      console.error(stderr);
    } else {
      console.log(`Successfully switched to project: ${project}\n`);
    }
  } catch (error) {
    console.log(
      "\nAn error has occured trying to switch to the chosen project:\n"
    );
    console.log(
      "Please navigate to the `./tooling/.meta` file, and add the following line:\n\tCURRENT_PROJECT=<a valid project>\n\nThen, you should be able to access the lessons with:\n\t$ fcc 1\n"
    );
  }
}

module.exports = switchAlias;
