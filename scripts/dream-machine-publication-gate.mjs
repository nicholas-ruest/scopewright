#!/usr/bin/env node
// One ordered execution path for product verification followed by publication.
// This is not a credential boundary: commands outside this entrypoint remain possible.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, lstatSync, mkdirSync, readFileSync, realpathSync, writeFileSync } from 'node:fs';
import { delimiter, dirname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const PINS = Object.freeze({
  'scripts/dream-machine-product-gate.mjs': '193079f8e70b16f84197e8a308bf41b773d25ca539f9ebf5afbddd0ed6f98a0e',
  'scripts/dream-machine-selection-gate.mjs': '27e99d82a90012110e097e9d94d0cfd2f9577c4ef12686fc589d68674e26ef49',
  'scripts/dream-machine-backend-gate.mjs': '82fa38c26a5ed091372de192cf5fedabc5cdbc63896ffff1503ab0fdafbde81e',
  'scripts/dream-machine-gate-bundle.mjs': '99bb5a8fdfd8c5fc1ed4ea83a7b8ca38d3a04452a5d8ed720280e1d4d729a8c6',
  'scripts/verify-prototype.sh': 'f4d28a4cab92f6a1081ac48aa7bb5e0711bd9626e3979c48a5cc7b81d28dc28f',
  'scripts/dream-machine-rust-share.mjs': 'ba06d9af532e1ea5ebe02ae77b4f0d2e9bfa1f243e81b99bdc4977a28e8ed801',
});
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (root, ...args) => execFileSync('git', ['-C', root, ...args], { encoding: 'utf8' }).trim();

export function orderedPublication(checks, publish) {
  for (const check of checks) check();
  return publish();
}

function pinnedFiles(root) {
  for (const [path, expected] of Object.entries(PINS)) {
    const full = resolve(root, path);
    if (!existsSync(full) || !lstatSync(full).isFile() || digest(readFileSync(full)) !== expected) {
      throw new Error(`Missing or changed required verifier: ${path}`);
    }
  }
}

function cleanHead(root, expected) {
  const sha = git(root, 'rev-parse', 'HEAD');
  if (!/^[a-f0-9]{40}$/.test(sha) || (expected && sha !== expected)) throw new Error('Candidate source SHA changed');
  if (git(root, 'status', '--porcelain=v1', '--untracked-files=all')) throw new Error('Candidate source is dirty');
  return sha;
}

function externalDirectory(root, requested) {
  mkdirSync(requested, { recursive: true });
  const actual = realpathSync(requested);
  const rel = relative(root, actual);
  if (rel === '' || (!isAbsolute(rel) && rel !== '..' && !rel.startsWith(`..${sep}`))) {
    throw new Error('Publication evidence must be outside the candidate checkout');
  }
  return actual;
}

export function verifyAndPublish(requestedRoot, requestedEvidence, command = []) {
  const root = realpathSync(git(resolve(requestedRoot), 'rev-parse', '--show-toplevel'));
  const evidence = externalDirectory(root, resolve(requestedEvidence));
  const receiptPath = resolve(evidence, 'publication-gate.json');
  const producer = process.env.DREAM_MACHINE_PRODUCER_RUN_ID;
  const receipt = { schema_version: 1, status: 'PUBLISH_DENIED', source_sha: null,
    producer_binding: producer ? digest(producer) : null, stages: [], publication_attempted: false };
  const save = () => writeFileSync(receiptPath, JSON.stringify(receipt, null, 2) + '\n');
  // Invalidate any previous PASS before reading source or executing candidate code.
  save();
  const env = { ...process.env,
    PATH: dirname(process.execPath) + delimiter + (process.env.PATH ?? ''),
    DREAM_MACHINE_EVIDENCE_DIR: evidence };
  const stage = (name, fn) => () => {
    const result = { name, exit_code: null };
    receipt.stages.push(result);
    try { fn(); result.exit_code = 0; }
    catch (error) { result.exit_code = Number.isInteger(error.status) ? error.status : 1; throw error; }
    finally { save(); }
  };
  const run = (name, executable, args) => {
    const out = resolve(evidence, `publication-${name}.stdout.log`);
    const err = resolve(evidence, `publication-${name}.stderr.log`);
    try {
      const stdout = execFileSync(executable, args, { cwd: root, env, encoding: 'utf8',
        maxBuffer: 32 * 1024 * 1024 });
      writeFileSync(out, stdout ?? '');
      return stdout;
    } catch (error) {
      writeFileSync(out, error.stdout ?? '');
      writeFileSync(err, error.stderr ?? String(error));
      throw new Error(`Verification failed: ${name}; inspect external execution logs`, { cause: error });
    }
  };
  try {
    return orderedPublication([
      stage('producer-and-clean-source', () => {
        if (!producer || producer.length < 8) throw new Error('Missing actual producer run identity');
        receipt.source_sha = cleanHead(root);
        env.DREAM_MACHINE_SOURCE_SHA = receipt.source_sha;
      }),
      stage('immutable-verifier-integrity', () => pinnedFiles(root)),
      stage('canonical-bundle', () => run('bundle', process.execPath, ['scripts/dream-machine-gate-bundle.mjs', '.'])),
      stage('rust-source-floor', () => {
        const raw = run('rust-share', process.execPath, ['scripts/dream-machine-rust-share.mjs', '.']);
        writeFileSync(resolve(evidence, 'rust-share.json'), raw);
        const report = JSON.parse(raw);
        if (!report.pass || report.source_sha !== receipt.source_sha || report.rust_bytes * 2 < report.source_bytes) {
          throw new Error('Rust-share report failed or is not bound to this source');
        }
      }),
      stage('complete-product-execution', () => run('full-product', 'bash', ['scripts/verify-prototype.sh'])),
      stage('final-source-and-verifier-integrity', () => {
        cleanHead(root, receipt.source_sha);
        pinnedFiles(root);
      }),
    ], () => {
      receipt.status = 'PUBLISH_GATE_PASS';
      save();
      if (command.length) {
        receipt.publication_attempted = true;
        receipt.status = 'PUBLICATION_IN_PROGRESS';
        save();
        execFileSync(command[0], command.slice(1), { cwd: root, env, stdio: 'inherit' });
        receipt.status = 'PUBLICATION_COMMAND_SUCCEEDED';
        save();
      }
      return receipt;
    });
  } catch (error) {
    receipt.status = receipt.publication_attempted ? 'PUBLICATION_FAILED' : 'PUBLISH_DENIED';
    receipt.error = error.message;
    save();
    throw error;
  }
}

function selfTest() {
  const names = ['source', 'pins', 'bundle', 'rust', 'full-product', 'final-source'];
  for (let failed = 0; failed < names.length; failed += 1) {
    const calls = [];
    let publications = 0;
    assert.throws(() => orderedPublication(names.map((name, i) => () => {
      calls.push(name);
      if (i === failed) throw new Error('injected failure');
    }), () => { publications += 1; }));
    assert.deepEqual(calls, names.slice(0, failed + 1));
    assert.equal(publications, 0);
  }
  let publications = 0;
  const calls = [];
  orderedPublication(names.map(name => () => calls.push(name)), () => { publications += 1; });
  assert.deepEqual(calls, names);
  assert.equal(publications, 1);
  // A Rust PASS followed by a full-product failure must never publish.
  let attempted = false;
  assert.throws(() => orderedPublication([() => true, () => { throw new Error('missing frontend'); }],
    () => { attempted = true; }));
  assert.equal(attempted, false);
  console.log('PUBLICATION_ORDER_SELF_TEST_PASS: all six failure boundaries block publication');
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const args = process.argv.slice(2);
    if (args.length === 1 && args[0] === '--self-test') selfTest();
    else {
      const [mode, root, evidence, separator, ...command] = args;
      if (!root || !evidence || !['--check', '--publish'].includes(mode) ||
          (mode === '--check' && args.length !== 3) ||
          (mode === '--publish' && (separator !== '--' || command.length === 0))) {
        throw new Error('Usage: gate --check ROOT EVIDENCE | --publish ROOT EVIDENCE -- COMMAND [ARGS]');
      }
      console.log(JSON.stringify(verifyAndPublish(root, evidence, mode === '--publish' ? command : []), null, 2));
    }
  } catch (error) {
    console.error(JSON.stringify({ status: 'PUBLISH_DENIED_OR_FAILED', error: error.message }));
    process.exitCode = 1;
  }
}
