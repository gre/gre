import React from "react";
import Head from "next/head";
import Link from "next/link";
import { Container } from "../../../../components/Container";
import { Global } from "../../../../components/Global";
import { getPost, getAllPosts } from "../../../../posts";

export async function getStaticPaths() {
  const posts = await getAllPosts();
  const paths = posts.map(({ year, month, slug }) => ({
    params: {
      year,
      month,
      slug,
    },
  }));
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  const post = await getPost(params.year, params.month, params.slug);
  return {
    props: post,
  };
}

const siteURL = "https://greweb.me";

export default function Home({
  year,
  month,
  day,
  data: { title, description, tags, thumbnail },
  content,
}) {
  return (
    <Global>
      <Container>
        <Head>
          <title>{title}</title>
          <link rel="icon" href="/favicon.ico" />
          <meta name="author" content="Gaëtan Renaudeau" />
          <meta name="description" content={description} />
          <meta name="keywords" content={tags?.join(", ")} />

          <meta name="HandheldFriendly" content="True" />
          <meta name="MobileOptimized" content="320" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />

          <meta name="twitter:card" content="summary" />
          <meta name="twitter:site" content="@greweb" />
          <meta name="twitter:title" content={title} />
          {description ? (
            <meta name="twitter:description" content={description} />
          ) : null}
          <meta name="twitter:creator" content="@greweb" />
          {thumbnail ? (
            <>
              <meta
                name="twitter:image:src"
                content={`${siteURL}/${thumbnail}`}
              />
              <link rel="image_src" href={`${siteURL}/${thumbnail}`} />
            </>
          ) : null}

          <title>@greweb - {title}</title>
          <link
            href="http://fonts.googleapis.com/css?family=Fredericka+the+Great|Arapey|Roboto:400,700,400italic"
            rel="stylesheet"
            type="text/css"
          />
          <link
            rel="stylesheet"
            href="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/styles/default.min.css"
          />
          <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/highlight.min.js"></script>
          <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/javascript.min.js"></script>
          <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/cpp.min.js"></script>
          <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/glsl.min.js"></script>
          <link rel="stylesheet" href="/style/main.css" />
        </Head>
        <div id="container">
          <div id="main">
            <div id="content">
              <article>
                <header>
                  <h1>
                    <Link href="/">
                      <a>{title}</a>
                    </Link>
                  </h1>

                  <time className="date" dateTime={`${year}-${month}-${day}`}>
                    {`${year}-${month}-${day}`}
                  </time>
                  <span className="tags">
                    {tags.map((t) => (
                      <a key={t} className="tag">
                        {t}
                      </a>
                    ))}
                  </span>
                </header>

                <div
                  className="entry-content"
                  dangerouslySetInnerHTML={{ __html: content }}
                />

                {/*
                <footer className="comments">
                  <div id="disqus_thread"></div>
                </footer>
                <script src="//greweb.disqus.com/embed.js" />
*/}
              </article>
            </div>
          </div>
        </div>
        <script>hljs.highlightAll();</script>
      </Container>
    </Global>
  );
}
