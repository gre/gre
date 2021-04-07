import * as dayPage from "../shaderday/[day]";
import Home from "../shaderday/[day]";

// fallbacks to day for shaderday!

export function getStaticPaths() {
  const { paths, ...r } = dayPage.getStaticPaths();
  return {
    ...r,
    paths: paths.map(({ params: { day, ...p }, ...a }) => ({
      ...a,
      params: {
        ...p,
        year: day,
      },
    })),
  };
}

export function getStaticProps({ params }) {
  const day = parseInt(params.year, 10);
  return {
    props: { day },
  };
}

export default Home;
