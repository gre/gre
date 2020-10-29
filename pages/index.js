import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

const shaders = Shaders.create({
  day001: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float blue;
void main() {
  gl_FragColor = vec4(uv.x, uv.y, blue, 1.0);
}`,
  },
});

class Day001 extends React.Component {
  static n = 1;
  static title = "Hello World";
  render() {
    const { blue } = this.props;
    return <Node shader={shaders.day001} uniforms={{ blue }} />;
  }
}

export default function Home() {
  const Current = Day001;
  const Day = Day001;
  return (
    <div className="container">
      <Head>
        <title>One Day One Shader</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <header>
          <h1 className="title">One Day, One Shader</h1>

          <p className="description">
            Day {Day.n}. <span className="daily">"{Day.title}"</span> by{" "}
            <a href="https://twitter.com/greweb" target="_blank">
              @greweb
            </a>
          </p>
        </header>

        <div className="day">
          <Surface width={400} height={400}>
            <Day blue={0.5} />
          </Surface>
        </div>

        <footer>France confinement #2 Day {Current.n} – Be strong 💪</footer>
      </main>

      <style jsx>{`
        .container {
          min-height: 100vh;
          padding: 0 0.5rem;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }

        main {
          flex: 1;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }

        a {
          color: inherit;
          text-decoration: none;
        }

        a:hover,
        a:focus,
        a:active {
          text-decoration: underline;
        }

        .title,
        .description {
          text-align: center;
        }

        .title {
          margin: 0;
          line-height: 1.15;
          font-size: 2rem;
        }

        .description {
          line-height: 1rem;
          font-size: 1.5rem;
          font-weight: 300;
        }

        .description .daily {
          font-weight: 600;
        }

        header {
          padding: 1rem 0;
        }
        footer {
          padding: 1rem 0;
          font-size: 0.8rem;
          font-style: italic;
        }
      `}</style>

      <style jsx global>{`
        html,
        body {
          padding: 0;
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
            Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue,
            sans-serif;
        }

        * {
          box-sizing: border-box;
        }
      `}</style>
    </div>
  );
}
