const path = require("path")
const findFiles = require("find")
const fs = require("fs")

// Helpers are used by `configs/postcss.config.js`.

module.exports = {
  /**
   * Search in Rust and Handlebars files for CSS classes.
   *
   * @returns Set of class names
   */
  getUsedCssClasses: function () {
    return new Set([...getUsedCssClassesInRust(), ...getUsedCustomCssClassesInRust()])
  },
}

// https://doc.rust-lang.org/book/appendix-01-keywords.html
function getKeywords() {
  return [
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "Self",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",

    "abstract",
    "async",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "try",
    "typeof",
    "unsized",
    "virtual",
    "yield",
  ]
}

/**
 * Search in Rust files for C.class_name.
 *
 * @returns usedCssClasses Set of class names
 */
function getUsedCssClassesInRust() {
  const usedCssClasses = new Set()
  // search in Rust files
  const files = findFiles.fileSync(/\.rs$/, "./src")
  files.forEach((filePath) => {
    const fileContent = fs.readFileSync(filePath, "utf8")
    // example of a used class in Rust code is `C.mb_16`
    const usedCssClassesInFile = fileContent.match(/C\.[a-z0-9_]+/g) || []
    usedCssClassesInFile
      // remove prefix `C.`
      .map((class_) => class_.substring(2))
      // add class to set
      .forEach((class_) => usedCssClasses.add(class_))
  })
  return usedCssClasses
}

/**
 * Search in Rust files for `C!["input-label"]`.
 *
 * @returns usedCssClasses Set of class names
 */
function getUsedCustomCssClassesInRust() {
  const usedCssClasses = new Set()
  // search in Rust files
  const files = findFiles.fileSync(/\.rs$/, "./src")
  files.forEach((filePath) => {
    const fileContent = fs.readFileSync(filePath, "utf8")
    // example of a used class in Rust code is `C!["input-label"]`
    // the unescaped regex `C\!\["[a-zA-Z0-9_-]+"]`
    const usedCssClassesInFile = fileContent.match(/C\!\["[a-zA-Z0-9_-]+"]/g) || []
    usedCssClassesInFile
      // remove prefix `C!["` and `"]` suffix
      .map((class_) => class_.slice(4, -2))
      // add class to set
      .forEach((class_) => usedCssClasses.add(class_))
  })
  return usedCssClasses
}
