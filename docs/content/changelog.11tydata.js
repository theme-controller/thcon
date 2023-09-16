module.exports = function() {
  const fs = require("fs");
  const path = require("path");

  return {
    changelogMd: fs.readFileSync(path.join(__dirname, "../../CHANGELOG.md")),
  };
}