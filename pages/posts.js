import React from "react";
import Head from "next/head";
import Link from "next/link";
import { Title } from "../components/Title";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { getAllPosts } from "../posts";

export async function getStaticProps() {
  const posts = await getAllPosts();
  return {
    props: { posts },
  };
}

export default function Home({ posts }) {
  return (
    <Global>
      <Container>
        <Head>
          <title>greweb.me</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <img width="200" src="/profile.jpg" />
            <Title text="greweb.me" />
            <p className="social">
              <a href="https//twitter.com/greweb">Twitter</a>
              {" – "}
              <a href="https//github.com/gre">Github</a>
            </p>
          </Header>
          <h2>Blog Posts</h2>
          <ul>
            {posts.map((p, i) => (
              <li key={i}>
                <Link href={`/${p.year}/${p.month}/${p.slug}`}>
                  <a>{p.data.title}</a>
                </Link>
              </li>
            ))}
          </ul>
        </Main>
      </Container>
    </Global>
  );
}
