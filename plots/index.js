import matter from "gray-matter";
import marked from "marked";

export async function getPlots() {
  const context = require.context("./examples", true, /\/([0-9]+)\/README.md$/);
  const plots = [];
  for (const key of context.keys()) {
    console.log(key)
    const folder = key.slice(2, key.length - 10);
    const m = folder.match(/^([0-9]+)$/);
    const content = await import(`./examples/${folder}/README.md`);
    const meta = matter(content.default);
    const rustFile = meta.data.sourceFolderURL
      ? "link"
      : (meta.data.sourceFolder || folder) +
        "/" +
        (meta.data.rootFile || "main.rs");

    plots.push({
      n: m[1],
      key,
      rustFile,
      sourceURL:
        meta.data.sourceFolderURL ||
        `https://github.com/gre/gre/blob/master/plots/examples/${rustFile}`,
      content: marked(meta.content),
      data: meta.data,
    });
  }
  plots.reverse();
  return plots;
}
