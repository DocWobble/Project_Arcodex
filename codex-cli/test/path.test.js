import test from "node:test";
import assert from "node:assert/strict";
import path from "node:path";
import { getUpdatedPath } from "../bin/codex.js";

test("getUpdatedPath avoids duplicate PATH entries on repeated invocation", () => {
  const pathSep = process.platform === "win32" ? ";" : ":";
  const originalPath = process.env.PATH;
  const dir = path.join(process.cwd(), "dummy-dir");
  try {
    process.env.PATH = "";
    const first = getUpdatedPath([dir]);
    process.env.PATH = first;
    const second = getUpdatedPath([dir]);
    const occurrences = second.split(pathSep).filter((p) => p === dir).length;
    assert.equal(occurrences, 1);
  } finally {
    process.env.PATH = originalPath;
  }
});
