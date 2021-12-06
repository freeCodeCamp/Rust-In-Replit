const { getProjectMeta } = require("./fcc");

export const LOCALE = getProjectMeta().LOCALE;

export default function t(key, args) {
  // Get key from ./locales/{locale}/comments.json
  // Read file and parse JSON
  const locale = LOCALE;
  const comments = require(`./locales/${locale}/comments.json`);

  // Get value from JSON
  const value = comments[key];
  // Replace placeholders in value with args
  const result =
    Object.values(args)?.length > 0
      ? value.replace(/\{\{\w+\}\}/g, (_, index) => args[index])
      : value;
  // Return value
  return result;
}
