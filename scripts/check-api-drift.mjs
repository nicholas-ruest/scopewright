#!/usr/bin/env node
import {readFileSync} from "node:fs";
const schema=JSON.parse(readFileSync(new URL("../api/openapi.json",import.meta.url)));
const client=readFileSync(new URL("../frontend/src/api/client.ts",import.meta.url),"utf8");
if(schema.info["x-schema-fingerprint"]!=="scopewright-api-v1-2026-11-20")throw new Error("schema fingerprint drift");
for(const name of ["CreateFactoryRun","FactoryRunResponse","ErrorResponse"]){if(!client.includes(`type ${name}`))throw new Error(`missing TypeScript type ${name}`)}
for(const path of ["/api/v1/factory_runs","/api/v1/audit"]){if(!client.includes(path))throw new Error(`missing client path ${path}`)}
console.log("API_DRIFT_PASS fingerprint=scopewright-api-v1-2026-11-20");
