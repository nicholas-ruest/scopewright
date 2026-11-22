#!/usr/bin/env node
import {createHash} from "node:crypto"; import {readFileSync,readdirSync,writeFileSync} from "node:fs"; import {join} from "node:path"; import {execFileSync} from "node:child_process";
const evidence=process.argv[2]; if(!evidence)throw new Error("evidence directory required"); const sha=execFileSync("git",["rev-parse","HEAD"],{encoding:"utf8"}).trim(); const digest=p=>createHash("sha256").update(readFileSync(join(evidence,p))).digest("hex");
const backend=JSON.parse(readFileSync(join(evidence,readdirSync(evidence).find(n=>n.endsWith(".receipt.json")&&JSON.parse(readFileSync(join(evidence,n))).suite==="backend"))));
const artifacts=[
  ["desktop_screenshot","screenshots/primary-desktop.png"],["mobile_screenshot","screenshots/durable-mobile.png"],["browser_report","playwright-report.json"],["browser_trace","browser/trace.zip"],["correlation_trace","traces/primary.jsonl"],["semantic_review","semantic-review.md"],["composition_report","composition-report.json"],["selection_report","selection-report.json"],["backend_metadata",backend.log],
  ["integration_execution","traces/integrations.log"],["correlation_trace","traces/denial.jsonl"],["correlation_trace","traces/durable.jsonl"],["benchmark_result","benchmark.json"],["ablation_result","ablation.json"]
].map(([kind,path])=>({kind,path,sha256:digest(path)}));
writeFileSync(join(evidence,"manifest.json"),JSON.stringify({schema_version:2,source_sha:sha,artifacts},null,2)+"\n"); console.log(`MANIFEST_PASS artifacts=${artifacts.length}`);
