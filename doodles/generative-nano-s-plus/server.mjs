import { generate } from "./features.mjs";
import express from "express";
import _ from "lodash";
import fs from "fs";
import { diff } from "jest-diff";
import rimraf from "rimraf";
import busboy from "connect-busboy";
import mkdirp from "mkdirp";
import morgan from "morgan";

const port = 3000;
const TOTAL = 2048;

const app = express();
app.use(morgan("tiny"));
app.use(express.static("."));
app.use(busboy());

async function tasks(debug) {
  const files = await fs.promises.readdir("files");
  const indexes = files.map((d) => parseInt(d, 10)).filter((d) => !isNaN(d));
  const statuses = await Promise.all(
    indexes.map(async (index) => {
      const files = await fs.promises.readdir("files/" + index);
      const hasMetadata = files.includes("metadata.json");
      const hasVideo = files.includes("video.mp4");
      const hasImage = files.includes("image.png");
      const hasImageAlt = files.includes("image-alt.png");
      let divergedMetadata = false;
      const { metadata } = generate(index);
      if (hasMetadata) {
        const content = await fs.promises.readFile(
          "files/" + index + "/metadata.json",
          "utf-8"
        );
        const parsed = JSON.parse(content);
        if (!_.isEqual(parsed, metadata)) {
          if (debug) {
            const d = diff(parsed, metadata);
            console.log("DIFF FOR " + index);
            console.log(d);
          }
          divergedMetadata = true;
        }
      }
      const hasImages = Array(120)
        .fill(null)
        .map((_, i) => "frame" + (i + "").padStart(8, "0") + ".png")
        .every((f) => files.includes(f));
      let status = divergedMetadata
        ? "invalid-metadata"
        : files.length === 0
        ? "todo"
        : hasMetadata && hasImages
        ? "complete"
        : "pending";
      if (hasVideo && hasImage && hasImageAlt) {
        status = "complete";
      } else if (status === "complete") {
        status = "video-todo";
      }
      return {
        index,
        status,
      };
    })
  );
  for (let i = 0; i < TOTAL; i++) {
    if (!indexes.includes(i)) {
      statuses.push({
        index: i,
        status: "todo",
      });
    }
  }
  return statuses;
}

function cleanupCandidates(tasksR) {
  return tasksR
    .filter(
      (s) =>
        s.status !== "todo" &&
        s.status !== "complete" &&
        s.status !== "video-todo"
    )
    .filter((s) => {
      const folder = "files/" + s.index;
      const r = fs.statSync(folder);
      return new Date() - r.mtime > 60000;
    });
}

function cleanup(tasksR) {
  return cleanupCandidates(tasksR).map((s) => {
    const folder = "files/" + s.index;
    console.log("CLEAN " + folder);
    rimraf.sync(folder);
  }).length;
}

app.get("/task/status", function (req, res) {
  tasks().then(
    (r) => {
      const stats = {};
      r.forEach((r) => {
        stats[r.status] = (stats[r.status] || 0) + 1;
      });
      res.send(stats);
    },
    (e) => {
      console.error(e);
      res.sendStatus(500);
    }
  );
});

app.get("/task/status/full", function (req, res) {
  tasks(true).then(
    (r) => res.send(r),
    (e) => {
      console.error(e);
      res.sendStatus(500);
    }
  );
});

app.get("/task/status/cleanup", function (req, res) {
  tasks()
    .then(cleanup)
    .then((count) => {
      res.send(String(count));
    })
    .catch((e) => {
      console.error(e);
      res.sendStatus(500);
    });
});

app.get("/task/ffmpeg", function (req, res) {
  tasks().then(
    (all) => {
      let todo = all.filter((s) => s.status === "video-todo");
      res.send(
        todo.length === 0
          ? ""
          : String(todo[Math.floor(Math.random() * todo.length)].index)
      );
    },
    (e) => {
      console.error(e);
      res.sendStatus(500);
    }
  );
});

app.get("/task/client", function (req, res) {
  const mode = req.query?.mode || "full";
  if (mode !== "full" && mode !== "noemojis" && mode !== "emojis") {
    res.send("");
    return;
  }
  tasks().then(
    (all) => {
      let todo = all.filter((s) => s.status === "todo");
      if (mode === "noemojis") {
        todo = todo.filter((s) => {
          const { opts } = generate(s.index);
          return (
            !opts.hasEmojiSticker &&
            !(opts.sentence || "").includes("🤌") &&
            !(opts.sentence || "").includes("🧑‍🌾")
          );
        });
      } else if (mode === "emojis") {
        todo = todo.filter((s) => {
          const { opts } = generate(s.index);
          return opts.hasEmojiSticker;
        });
      }
      console.log(mode + " = " + todo.map((t) => t.index).join(" "));
      res.send(
        todo.length === 0
          ? ""
          : String(
              todo[Math.floor(Math.random() * Math.random() * todo.length)]
                .index
            )
      );
    },
    (e) => {
      console.error(e);
      res.sendStatus(500);
    }
  );
});

app.delete("/task/client/:index", function (req, res) {
  const { index } = req.params;
  console.log("cleanup request", index);
  res.send();
});

app.post("/task/client/:index/:filename", function (req, res) {
  const { index, filename } = req.params;
  let fstream;
  req.pipe(req.busboy);
  const folder = "files/" + index;
  mkdirp.sync(folder);
  req.busboy.on("file", function (fieldname, file) {
    fstream = fs.createWriteStream(folder + "/" + filename);
    file.pipe(fstream);
  });
  req.busboy.on("finish", function () {
    res.writeHead(200, { Connection: "close", Location: "/" });
    res.end();
  });
});

mkdirp.sync("files");
tasks(true).then((tasksR) => {
  const list = cleanupCandidates(tasksR);
  if (list.length) {
    console.log(
      list.length +
        ` files to cleanup. http://localhost:${port}/task/status/cleanup`
    );
  }
  app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`);
  });
});
