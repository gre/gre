import { useRouter } from "next/router";
import { Page } from "../../../components/Page";
import { findDay, getDays } from "../../../day";

export function getStaticPaths() {
  return {
    paths: getDays().map((n) => {
      return {
        params: {
          day: String(n),
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
  return <Page Day={Day} />;
}
