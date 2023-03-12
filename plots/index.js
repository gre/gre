import matter from "gray-matter";
import marked from "marked";

export async function getPlots() {
  const context1 = require.context("./examples", true, /\/(\d{3})\/README.md$/);
  const context2 = require.context("./examples", true, /\/(\d{4})\/README.md$/);
  // ^ IDK why, but if we try to catch \d, we only get the {3} ones !!
  const plots = [];
  for (const key of context1.keys().concat(context2.keys())) {
    console.log(key)
    const folder = key.slice(2, key.length - 10);
    const m = folder.match(/^(\d+)$/);
    const content = await import(`./examples/${folder}/README.md`);
    const meta = matter(content.default);
    const rustFile = meta.data.noSource ? null :
      meta.data.sourceFolderURL
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
        rustFile &&
        `https://github.com/gre/gre/blob/master/plots/examples/${rustFile}`,
      content: marked(meta.content),
      data: meta.data,
    });
  }
  plots.reverse();
  return plots;
}
