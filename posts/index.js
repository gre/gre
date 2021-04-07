import matter from "gray-matter";
import marked from "marked";

export async function getAllPosts() {
  const context = require.context("../posts", true, /\.md$/);
  const posts = [];
  for (const key of context.keys()) {
    const post = key.slice(2);
    const content = await import(`../posts/${post}`);
    const meta = matter(content.default);
    posts.push({
      slug: post.replace(".md", ""),
      data: meta.data,
    });
  }
  return posts;
}

export async function getPostBySlug(slug) {
  const fileContent = await import(`../posts/${slug}.md`);
  const meta = matter(fileContent.default);
  const content = marked(meta.content);
  return {
    data: meta.data,
    content,
  };
}
