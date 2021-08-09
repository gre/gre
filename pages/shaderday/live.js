import React from "react";
import Head from "next/head";
import { Visual } from "../../components/Visual";
import { Title } from "../../components/Title";
import { Container } from "../../components/Container";
import { GlobalLive } from "../../components/Global";
import { Main } from "../../components/Main";
import { Header } from "../../components/Header";
import { getToday } from "../../shaderdays";

export default function Home() {
  const Day = getToday();
  return (
    <GlobalLive>
      <Container>
        <Head>
          <title>greweb.me</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <Title withBreadcrumb text="live" />
          </Header>
          <Visual Day={Day} />
        </Main>
      </Container>
    </GlobalLive>
  );
}
