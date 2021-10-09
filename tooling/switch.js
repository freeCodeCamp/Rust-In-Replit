// This file switches the alias of `fcc='node ./tooling/fcc.js <project>'`

const fs = require("fs");
const util = require("util");
const execute = util.promisify(require("child_process").exec);

// Set alias based on project argv
async function switchAlias(project) {
  try {
    const { stdout, stderr } = await execute(
      `echo 'alias fcc="node ./tooling/fcc.js ${project}"' >> ~/.bashrc`
    );
    if (stderr) {
      console.error(stderr);
    } else {
      const { stderr } = await execute(`source ~/.bashrc`);
      if (stderr) {
        console.error(stderr);
        console.log("\nYou will need to manually source the `bashrc` file\n");
      }
      console.log(`Successfully switched to project: ${project}\n`);
    }
  } catch (error) {
    console.log(
      "\nAn error has occured trying to switch the chosen project:\n\n"
    );
    console.error(error);
  }
}

module.exports = switchAlias;
