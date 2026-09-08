import { spawn } from "node:child_process";
import { resolve } from "node:path";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import pkg from "./package.json" with { type: "json" };

export default defineConfig({
  optimizeDeps: { entries: ["index.html"] },
  define: { __APP_VERSION__: JSON.stringify(pkg.version) },
  plugins: [
    react(),
    {
      name: "boxmaker-rust-dev",
      configureServer(server) {
        server.middlewares.use("/api/engine", (req, res) => {
          if (
            req.method !== "POST" ||
            (req.headers.origin &&
              req.headers.origin !==
                `http://127.0.0.1:${server.config.server.port}`)
          ) {
            res.writeHead(403).end();
            return;
          }
          let body = "";
          req.on("data", (chunk) => {
            body += chunk;
            if (body.length > 32_768) req.destroy();
          });
          req.on("end", () => {
            const child = spawn(resolve("target/debug/boxmaker-cli.exe"), [], {
              windowsHide: true,
            });
            let output = "",
              error = "";
            child.stdout.on("data", (chunk) => {
              output += chunk;
            });
            child.stderr.on("data", (chunk) => {
              error += chunk;
            });
            child.on("error", () => {
              if (!res.writableEnded)
                res.writeHead(503).end(
                  JSON.stringify({
                    error: "Moteur Rust indisponible. Relancez bun run dev.",
                  }),
                );
            });
            child.on("close", (code) => {
              if (!res.writableEnded)
                res
                  .writeHead(code === 0 ? 200 : 400, {
                    "Content-Type": "application/json",
                  })
                  .end(code === 0 ? output : JSON.stringify({ error }));
            });
            child.stdin.end(body);
          });
        });
      },
    },
  ],
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
    watch: { ignored: ["**/target/**", "**/src-tauri/**"] },
  },
  clearScreen: false,
});
