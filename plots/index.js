import matter from "gray-matter";
import marked from "marked";

export async function getPlots() {
  const context = require.context("./examples", true, /\/([0-9]+)\/README.md$/);
  const plots = [];
  for (const key of context.keys()) {
    const folder = key.slice(2, key.length - 10);
    const m = folder.match(/^([0-9]+)$/);
    const content = await import(`./examples/${folder}/README.md`);
    const meta = matter(content.default);
    plots.push({
      n: m[1],
      key,
      rustFile: (meta.data.sourceFolder || folder) + "/main.rs",
      content: marked(meta.content),
      data: meta.data,
    });
  }
  plots.reverse();
  return plots;
}
