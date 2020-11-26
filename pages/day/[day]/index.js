import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import { findDay, getDays } from "../../../day";
import { Visual } from "../../../components/Visual";
import { LiveFooter } from "../../../components/LiveFooter";
import { SubTitle } from "../../../components/SubTitle";
import { Title } from "../../../components/Title";
import { SourceCodeFooter } from "../../../components/SourceCodeFooter";
import { Container } from "../../../components/Container";
import { Global } from "../../../components/Global";
import { Main } from "../../../components/Main";
import { Header } from "../../../components/Header";

export function getStaticPaths() {
  return {
    paths: getDays().map((Day) => {
      return {
        params: {
          day: String(Day.n),
        },
      };
    }),
    fallback: false,
  };
}

export function getStaticProps({ params }) {
  const day = parseInt(params.day, 10);
  return {
    props: { day },
  };
}

export default function Home({ day }) {
  const Day = findDay(parseInt(day, 10));
  if (!Day) return null;
  return (
    <Global>
      <Container>
        <Head>
          <title>
            One Day One Shader – Day {Day.n}. "{Day.title}"
          </title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <Title />
            <SubTitle Day={Day} />
          </Header>
          <Visual Day={Day} />
          <SourceCodeFooter Day={Day} />
          <LiveFooter />
        </Main>
      </Container>
    </Global>
  );
}
