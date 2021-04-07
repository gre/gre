import React from "react";
import Head from "next/head";
import { Container } from "../../../../components/Container";
import { Global } from "../../../../components/Global";
import { getPostBySlug, getAllPosts } from "../../../../posts";

export async function getStaticPaths() {
  const posts = await getAllPosts();
  return {
    paths: posts
      .map((p) => p.slug.split("/"))
      .filter((p) => p.length === 3)
      .map(([year, month, slug]) => ({
        params: {
          year,
          month,
          slug,
        },
      })),
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  const post = await getPostBySlug(
    params.year + "/" + params.month + "/" + params.slug
  );
  return {
    props: post,
  };
}

const siteURL = "https://shaderday.com";

export default function Home({
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
          <link rel="stylesheet" href="/style/main.css" />
        </Head>
        <div id="container">
          <div id="main">
            <div id="content">
              <article>
                <header>
                  <h1>{title}</h1>
                  {/*
                <time class="date" datetime="">
                  ...
                </time>
                */}
                  {/*
   <span class="tags">
     <a class="tag" href="{{ tag | tag_url }}">{{tag}}</a>
     {% endfor %}
   </span>
    */}
                </header>

                <div
                  className="entry-content"
                  dangerouslySetInnerHTML={{ __html: content }}
                />
              </article>
            </div>
          </div>
        </div>
      </Container>
    </Global>
  );
}
