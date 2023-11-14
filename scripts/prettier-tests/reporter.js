import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export default class CustomReporter {
  constructor() {
    this._metrics = [];
  }

  onRunComplete(testContexts, results) {
    console.log(results);
    console.log("Custom reporter output:");
    let score_file_base = 0;
    let score_line_base_match = 0;
    let score_line_base_max = 0;
    const test_count = this._metrics.length;
    for (const metric of this._metrics) {
      score_file_base += metric[0] / metric[1];
      score_line_base_match += metric[0];
      score_line_base_max += metric[1];
    }
    const file_base = (score_file_base / test_count) * 100;
    const line_base = (score_line_base_match / score_line_base_max) * 100;
    console.log("File Based Average Prettier Similarity:", file_base);
    console.log("Line Based Average Prettier Similarity:", line_base);
    const snapshot_result = results.snapshot;
    const output = `---
# Prettier Similarity Test Results (${new Date().toISOString()})

- Snapshot Passing Rate: ${
      (snapshot_result.matched / snapshot_result.total) * 100
    } (${snapshot_result.matched} / ${snapshot_result.total})
- File Based Average Prettier Similarity: ${file_base}
- Line Based Average Prettier Similarity: ${line_base}

Formula for this score: https://github.com/rome/tools/issues/2555#issuecomment-1124787893

All test results: https://nissy-dev.github.io/biome/

`;

    const __filename = fileURLToPath(import.meta.url);
    const __dirname = path.dirname(__filename);
    const outputPath = path.resolve(__dirname, "output.md");
    fs.writeFileSync(outputPath, output, "utf8");
  }

  onTestResult(test, testResult) {
    for (const result of testResult.testResults) {
      if (result.fullName !== "format" || test.path === undefined) {
        continue;
      }
      const input_file_path = path.join(
        path.dirname(test.path),
        result.ancestorTitles[0],
      );
      if (!fs.existsSync(input_file_path)) {
        continue;
      }
      const prettier_line_length = fs
        .readFileSync(input_file_path, "utf8")
        .split("\n").length;
      if (result.failureMessages.length > 0) {
        const err_msg = result.failureMessages[0];
        const prettier_diff = Number(
          err_msg
            .match(/Snapshot\s*.\s*(\d+)/g)[0]
            .replace("Snapshot  ", "")
            .replaceAll(" ", ""),
        );
        const biome_diff = Number(
          err_msg
            .match(/Received\s*.\s*(\d+)/g)[0]
            .replace("Received  ", "")
            .replaceAll(" ", ""),
        );

        this._metrics.push([
          prettier_line_length - Math.abs(prettier_diff),
          Math.max(
            prettier_line_length,
            prettier_line_length + prettier_diff + biome_diff,
          ),
        ]);
      } else {
        this._metrics.push([prettier_line_length, prettier_line_length]);
      }
    }
  }
}
