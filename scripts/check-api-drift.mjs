#!/usr/bin/env node
import {readFileSync} from "node:fs"; const schema=JSON.parse(readFileSync(new URL("../api/openapi.json",import.meta.url))); const client=readFileSync(new URL("../frontend/src/api.ts",import.meta.url),"utf8");
if(schema.info["x-schema-fingerprint"]!=="scopewright-api-v1-2026-11-21")throw new Error("schema fingerprint drift");
for(const n of ["RunInput","RunResponse"]){if(!client.includes(`type ${n}`))throw new Error(`missing generated type ${n}`)} for(const p of ["/api/v1/factory_runs","/api/v1/audit"]){if(!client.includes(p))throw new Error(`missing client path ${p}`)} if(!client.includes("decision:string"))throw new Error("decision field drift"); console.log("API_DRIFT_PASS fingerprint=scopewright-api-v1-2026-11-21");
