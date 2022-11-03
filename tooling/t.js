const fs = require("fs");

const LOCALE = getProjectMeta().LOCALE;

function t(key, args = {}, forceLangToUse) {
  const loc = getProjectMeta().LOCALE;
  // Get key from ./locales/{locale}/comments.json
  // Read file and parse JSON
  const locale = forceLangToUse ?? (loc === "undefined" ? "en" : loc);
  const comments = require(`./locales/${locale}/comments.json`);

  // Get value from JSON
  const value = comments[key];
  // Replace placeholders in value with args
  const result =
    Object.values(args)?.length > 0
      ? value.replace(/\{\{(\w+)\}\}/g, (_, m) => args[m])
      : value;
  // Return value
  return result;
}

function getProjectMeta() {
  // Read .meta file for CURRENT_PROJECT variable
  const META_FILE = "tooling/.meta";
  let meta = {
    CURRENT_PROJECT: "calculator",
    LOCALE: "english",
  };
  try {
    const META = fs.readFileSync(META_FILE, "utf8");
    const metaArr = META.split("\n").filter(Boolean);
    const new_meta = metaArr.reduce((meta, line) => {
      const [key, value] = line.split("=");
      return { ...meta, [key]: value };
    }, "");
    meta = { ...meta, ...new_meta };
  } catch (err) {
    console.log(`${t("meta-file-error", { metaFile: META_FILE })}`);
    console.error(err);
  }
  return meta;
}

module.exports = { t, LOCALE, getProjectMeta };
