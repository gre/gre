import matter from "gray-matter";
import marked from "marked";

export async function getTagsContent() {
    const context = require.context(".", true, /.\/(.*)\/README.md$/);
    const tags = [];
    for (const key of context.keys()) {
        const folder = key.slice(2, key.length - 10);
        tags.push(await getTagContent(folder));
    }
    return tags;
}

export async function getTagContent(name) {
    const content = await import(`./${name}/README.md`);
    const meta = matter(content.default);
    return {
        name,
        content: marked(meta.content),
        data: meta.data,
    }
}