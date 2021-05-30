import React, { useState, useEffect, useMemo } from "react";
import Head from "next/head";
import Link from "next/link";
import { Leva } from "leva";
import { Title } from "../components/Title";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { Visual } from "../components/Visual";
import { getDays } from "../shaderdays";
import { getAllPosts } from "../posts";
import { getPlots } from "../plots";
import me from "../me";

const ITEMS_SIZE = 4;
const GRID = 2;

const COLLECTION = "/candidates/{index}.gif";
const candidates = Array(76)
  .fill(null)
  .map((_o, i) => i);

export default function Home() {
  const key = "elector_" + COLLECTION;
  const initialScores = useMemo(() => {
    try {
      const value = localStorage[key];
      if (value) {
        const arr = JSON.parse(value);
        if (arr.length === candidates.length) {
          return arr;
        }
      }
    } catch (e) {
      console.error(e);
    }
  }, []);

  const [scores, setScores] = useState(
    initialScores || candidates.map(() => null)
  );
  const [items, setItems] = useState(() => sampleItems());
  const [choices, setChoices] = useState(() => Array(ITEMS_SIZE).fill(0));

  // save & restore localstorage
  // read
  useEffect(() => {
    if (initialScores) {
      setScores(initialScores);
    }
  }, [initialScores]);
  // write
  useEffect(() => {
    localStorage[key] = JSON.stringify(scores);
  }, [key, scores]);

  function sampleItems() {
    const unsorted = scores
      .map((s, i) => [s, i])
      .filter((e) => e[0] === null)
      .map((e) => e[1])
      .sort(() => Math.random() - 0.5);

    console.log("unsorted", unsorted);

    // as soon as a we have a high score group of X candidates, we will use it as discriminator to remove out the others.
    const LEVEL_GROUP = ITEMS_SIZE - 1;
    let level = 0;
    if (unsorted.length === 0) {
      const sorted = scores.map((s, i) => [s, i]).sort((a, b) => b[0] - a[0]);
      let s = sorted[0][0];
      let count = 0;
      for (let i = 0; i < sorted.length; i++) {
        const curS = sorted[i][0];
        if (curS === s) {
          count++;
        } else {
          count = 0;
          s = curS;
        }
        if (count >= LEVEL_GROUP) {
          level = s;
          break;
        }
      }
    }
    const s = unsorted.concat(
      candidates
        .filter((c) => (scores[c] || 0) >= level)
        .sort((a, b) => {
          // TODO allow some random candidtate to still be selected
          const rand = 1.1 * (Math.random() - 0.5);
          const lvlSort =
            Number((scores[a] || 0) >= level) -
            Number((scores[b] || 0) >= level);
          return rand + lvlSort;
        })
    );
    return s.slice(0, ITEMS_SIZE);
  }

  const next = () => {
    setScores((scores) =>
      scores.map((old, i) => {
        const index = items.findIndex((c) => c === i);
        return index === -1 ? old : old + choices[index];
      })
    );
    setChoices(Array(ITEMS_SIZE).fill(0));
    setTimeout(() => setItems(sampleItems()), 50);
  };

  // UX needs to be more efficient
  return (
    <Global>
      <Container>
        <Head>
          <title withBreadcrumb>Elector</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(" + GRID + ", 1fr)",
            marginBottom: 20,
          }}
        >
          {items.map((index, i) => (
            <div key={index}>
              <img width="100%" src={COLLECTION.replace("{index}", index)} />
              <div style={{ textAlign: "center" }}>
                <strong>
                  #{index} ({scores[index] ?? "?"})
                </strong>
                {["+1", "0", "-1"].map((text, j) => (
                  <label key={j}>
                    <input
                      onChange={() => {
                        setChoices((c) =>
                          c.map((old, k) => (k === i ? 1 - j : old))
                        );
                      }}
                      checked={choices[i] === 1 - j}
                      type="radio"
                      name={i}
                    />{" "}
                    {text}
                  </label>
                ))}
              </div>
            </div>
          ))}
        </div>
        <button onClick={next}>NEXT</button>
        <div>
          <h2>scoreboard</h2>
          <ul>
            {scores
              .map((s, i) => [s, i])
              .filter((e) => e[0] !== null)
              .sort((a, b) => b[0] - a[0])
              .map(([score, i]) => (
                <li key={i}>
                  #{i}: {String(score)}
                </li>
              ))}
          </ul>
        </div>
      </Container>
    </Global>
  );
}
