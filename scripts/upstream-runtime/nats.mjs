import { spawn } from "node:child_process";
import { once } from "node:events";
import { connect } from "node:net";

const binary = process.argv[2];
const port = 24321;
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const server = spawn(binary, ["-a", "127.0.0.1", "-p", String(port)], {
  stdio: ["ignore", "ignore", "pipe"],
});
let startupError = "";
server.stderr.setEncoding("utf8");
server.stderr.on("data", (chunk) => (startupError += chunk));
const open = () =>
  new Promise((resolve, reject) => {
    const candidate = connect({ host: "127.0.0.1", port });
    candidate.once("connect", () => resolve(candidate));
    candidate.once("error", (error) => {
      candidate.destroy();
      reject(error);
    });
  });

let socket;
try {
  for (let attempt = 0; attempt < 50 && !socket; attempt += 1) {
    if (server.exitCode !== null)
      throw new Error(`NATS startup failed: ${startupError.trim()}`);
    try {
      socket = await open();
    } catch {
      await wait(100);
    }
  }
  if (!socket) throw new Error(`NATS readiness timeout: ${startupError.trim()}`);
  socket.setEncoding("utf8");
  let data = "";
  socket.on("data", (chunk) => (data += chunk));
  const until = async (needle) => {
    for (let attempt = 0; attempt < 40; attempt += 1) {
      if (data.includes(needle)) return;
      await wait(50);
    }
    throw new Error(`NATS protocol timeout waiting for ${needle}`);
  };
  socket.write('CONNECT {"verbose":true}\r\nSUB scopewright.events 1\r\nPING\r\n');
  await until("PONG");
  socket.write("PUB scopewright.events 5\r\nready\r\n");
  await until("MSG scopewright.events");
  console.log("nats_ack");
} finally {
  socket?.destroy();
  server.kill("SIGTERM");
  await Promise.race([once(server, "exit"), wait(2000)]);
}
