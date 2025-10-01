module.exports = {
  src: "./",
  schema: "../../../schema.graphql",
  excludes: ["**/node_modules/**", "**/__generated__/**"],
  artifactDirectory: "./__generated__",
  language: "typescript",
  typescriptExcludeUndefinedFromNullableUnion: true,
  eagerEsModules: true,
};
