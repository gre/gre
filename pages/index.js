import Head from "next/head";
import { Visual } from "../components/Visual";
import { LiveFooter } from "../components/LiveFooter";
import { SubTitle } from "../components/SubTitle";
import { Title } from "../components/Title";
import { SourceCodeFooter } from "../components/SourceCodeFooter";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { getToday } from "../shaderdays";

export default function Home() {
  const Day = getToday();
  return (
    <Global>
      <Container>
        <Head>
          <title>One Day One Shader</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <Title />
            <SubTitle Day={Day} />
          </Header>
          <Visual Day={Day} />
          <SourceCodeFooter Day={Day} />
          <LiveFooter Day={Day} />
        </Main>
      </Container>
    </Global>
  );
}
