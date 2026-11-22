// Module: web/apps/signal-web/server/edge-server.mjs — Tier-1 Node.js 26 LTS Route-Partitioned COOP/COEP & API Edge Server
// Invariants: Sets COOP: same-origin + COEP: require-corp on /org/* (F-127); keeps /auth/* unsafe-none; proxies /api/* to FastAPI.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import { existsSync, readFileSync, statSync } from "node:fs";
import { createServer, request as httpRequest } from "node:http";
import { extname, join, normalize, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = resolve(fileURLToPath(import.meta.url), "..");
const browserDist = resolve(here, "../../../dist/signal-web/browser");
const port = Number(process.env.SIGNAL_WEB_PORT ?? "4200");
const apiHost = process.env.SIGNAL_API_UPSTREAM_HOST ?? "127.0.0.1";
const apiPort = Number(process.env.SIGNAL_API_UPSTREAM_PORT ?? "8080");

const MIME_MAP = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
};

function applySecurityHeaders(reqUrl, res) {
  res.setHeader("X-Content-Type-Options", "nosniff");
  res.setHeader("X-Frame-Options", "DENY");
  res.setHeader("Referrer-Policy", "no-referrer");
  res.setHeader("Cache-Control", "no-store");
  if (reqUrl.startsWith("/auth/")) {
    res.setHeader("Cross-Origin-Opener-Policy", "unsafe-none");
  } else {
    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
    res.setHeader("Cross-Origin-Resource-Policy", "same-origin");
  }
}

const server = createServer((req, res) => {
  const urlPath = (req.url ?? "/").split("?")[0];
  applySecurityHeaders(urlPath, res);

  if (urlPath.startsWith("/api/")) {
    const proxyReq = httpRequest(
      {
        hostname: apiHost,
        port: apiPort,
        path: req.url,
        method: req.method,
        headers: { ...req.headers, "x-signal-guest-user": "local-guest-0001" },
      },
      (proxyRes) => {
        res.writeHead(proxyRes.statusCode ?? 502, proxyRes.headers);
        proxyRes.pipe(res);
      },
    );
    proxyReq.on("error", () => {
      res.writeHead(502, { "content-type": "application/json" });
      res.end(JSON.stringify({ code: "UPSTREAM_UNAVAILABLE", message: "FastAPI Control Plane unreachable" }));
    });
    req.pipe(proxyReq);
    return;
  }

  const safeRel = normalize(urlPath).replace(/^(\.\.(\/|\\|$))+/, "");
  const candidate = join(browserDist, safeRel);
  if (existsSync(candidate) && statSync(candidate).isFile()) {
    const ext = extname(candidate);
    res.writeHead(200, { "content-type": MIME_MAP[ext] ?? "application/octet-stream" });
    res.end(readFileSync(candidate));
    return;
  }

  const indexHtml = join(browserDist, "index.html");
  res.writeHead(200, { "content-type": "text/html; charset=utf-8" });
  res.end(readFileSync(indexHtml));
});

server.listen(port, "0.0.0.0", () => {
  console.log(`[signal-web] Tier-1 Angular v22.3 + Node ${process.version} edge listening on http://127.0.0.1:${port}`);
});
