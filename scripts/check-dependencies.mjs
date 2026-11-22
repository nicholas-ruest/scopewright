#!/usr/bin/env node
import {execFileSync} from "node:child_process";
const metadata=JSON.parse(execFileSync("cargo",["metadata","--locked","--offline","--format-version","1"],{encoding:"utf8"}));
const external=metadata.packages.filter(p=>!p.source?.startsWith("path+")&&!metadata.workspace_members.includes(p.id));
const missing=external.filter(p=>!p.license&&!p.license_file); if(missing.length)throw new Error(`dependencies without license metadata: ${missing.map(x=>x.name).join(",")}`);
const git=external.filter(p=>p.source?.startsWith("git+")&&!/#[a-f0-9]{40}$/.test(p.source)); if(git.length)throw new Error("unpinned git dependency");
console.log(`DEPENDENCY_POLICY_PASS packages=${external.length} licensed=${external.length-missing.length}`);
