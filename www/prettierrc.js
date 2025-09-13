/** @type {import("prettier").Config} */
module.exports = {
  arrowParens: "avoid",
  trailingComma: "es5",
  plugins: [
    require.resolve("@ianvs/prettier-plugin-sort-imports"),
    require.resolve("prettier-plugin-tailwindcss") // must come last to ensure compatibility with prettier-plugin-sort-imports
  ],
  // plugin sort imports
  importOrder: ["<THIRD_PARTY_MODULES>", "^[./]"],
  importOrderTypeScriptVersion: "5.0.2",
  // Enables prettier to work with typescript files that have decorator annotations
  importOrderParserPlugins: ["typescript", "jsx", "decorators-legacy"]
};
