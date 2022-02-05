import matter from "gray-matter";
import marked from "marked";

export async function getAllPosts(includesFuture = false) {
  const context = require.context("../posts", false, /\.md$/);
  const posts = [];
  for (const key of context.keys()) {
    const post = key.slice(2);
    const m = post.match(/^(\d+)-(\d+)-(\d+)-(.*).md$/);
    if (!m) continue;
    const [, year, month, day, slug] = m;
    if (!includesFuture) {
      const date = new Date(year, month - 1, day);
      if (date > new Date()) continue;
    }
    const content = await import(`../posts/${post}`);
    const meta = matter(content.default);
    posts.push({
      id: post.replace(".md", ""),
      year,
      month,
      day,
      slug,
      content: meta.content,
      data: meta.data,
    });
  }
  posts.reverse();
  return posts;
}

export async function getPost(year, month, slug) {
  const all = await getAllPosts(true);
  const m = all.find(
    (p) => p.year === year && p.month === month && p.slug === slug
  );
  if (!m) throw new Error("not found");
  const content = marked(m.content);
  return {
    ...m,
    content,
  };
}
