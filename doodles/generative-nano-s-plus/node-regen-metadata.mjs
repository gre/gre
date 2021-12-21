import fs from "fs";
import { generate } from "./features.mjs";

for (let i = 0; i < 2048; i++) {
  const { metadata } = generate(i);
  fs.writeFileSync(
    `files/${i}/metadata.json`,
    JSON.stringify(metadata, null, 2),
    "utf-8"
  );
}
