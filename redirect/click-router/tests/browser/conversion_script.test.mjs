// Runs the conversion script on pages of another site.
//
//   PLAYWRIGHT_CORE=/path/to/node_modules/playwright-core/index.mjs \
//   CHROME=/usr/bin/chromium node --test conversion_script.test.mjs
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import http from "node:http";
import { after, before, beforeEach, test } from "node:test";

const { chromium } = await import(process.env.PLAYWRIGHT_CORE ?? "playwright-core");
const source = readFileSync(
  new URL("../../src/adapters/api/conversion_script.js", import.meta.url), "utf8");

const CID = "01M8A0TWHKY97G6TJFAFJVJZV0";
const CID2 = "01M8A0TWJBXKW545S4TATH20GA";
let server, port, browser, page, context, events, preflights, failures, page_html;

const STUB = `<script>(function(w){var s=w.shortas=w.shortas||{queue:[]};["track","consent"].forEach(function(m){
s[m]=s[m]||function(){s.queue.push([m].concat([].slice.call(arguments)))}})})(window)</script>`;
const tag = (key, attributes = "") =>
  `<script async src="http://go.short.test:${port}/.well-known/shortas/c/${key}.js" ${attributes}></script>`;

before(async () => {
  server = http.createServer((req, res) => {
    const url = new URL(req.url, "http://x");
    const script = url.pathname.match(/\/c\/(\w+)\.js$/);
    if (script) {
      const config = { key: script[1], windowDays: script[1] === "pk_b" ? 7 : 30 };
      res.writeHead(200, { "content-type": "text/javascript" });
      return setTimeout(() => res.end(source.replace("__CONFIG__", JSON.stringify(config))), 50);
    }
    if (url.pathname === "/.well-known/shortas/v1/events") {
      if (req.method === "OPTIONS") preflights++;
      let body = "";
      req.on("data", (d) => (body += d));
      req.on("end", () => {
        const status = failures.length ? failures.shift() : 202;
        events.push({ ...JSON.parse(body || "{}"), headers: req.headers, status });
        res.writeHead(status, { "access-control-allow-origin": "*" });
        res.end("{}");
      });
      return;
    }
    res.writeHead(200, { "content-type": "text/html" });
    res.end(url.pathname === "/blank" ? "<p>blank" : page_html);
  });
  await new Promise((r) => server.listen(0, "127.0.0.1", r));
  port = server.address().port;
  browser = await chromium.launch({
    executablePath: process.env.CHROME ?? "/usr/bin/chromium",
    args: ["--host-resolver-rules=MAP *.test 127.0.0.1", "--no-sandbox"],
  });
});
after(async () => { await browser.close(); server.close(); });

beforeEach(async () => {
  await context?.close();
  context = await browser.newContext();
  page = await context.newPage();
  events = []; preflights = 0; failures = [];
  page_html = STUB + tag("pk_a");
});

const shop = (path = "/", host = "www.shop.test") => `http://${host}:${port}${path}`;
const ready = (key = "pk_a") =>
  page.waitForFunction((k) => window.shortas?.conversions?.[k], key);
const track = (...args) => page.evaluate((a) => window.shortas.track(...a), args);
async function received(n) {
  for (let i = 0; i < 100 && events.length < n; i++) await new Promise((r) => setTimeout(r, 50));
  await new Promise((r) => setTimeout(r, 150));
  assert.equal(events.length, n);
  return events;
}
const cookies = async () => (await context.cookies()).filter((c) => c.name.startsWith("_shortas"));

test("sends an event without preflight or credentials", async () => {
  await context.addCookies([{ name: "session", value: "1", domain: "go.short.test", path: "/" }]);
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready();
  await track("pk_a", "purchase", { value: 49.5, currency: "EUR", externalId: "o1" });
  const [event] = await received(1);
  assert.equal(event.key, "pk_a");
  assert.equal(event.step, "purchase");
  assert.equal(event.clickId, CID);
  assert.equal(event.value, 49.5);
  assert.equal(event.currency, "EUR");
  assert.equal(event.externalId, "o1");
  assert.match(event.eventId, /^[0-9a-f]{32}$/);
  assert.equal(event.headers.origin, shop("").replace(/\/$/, ""));
  assert.equal(event.headers.cookie, undefined);
  assert.equal(preflights, 0);
});

test("later pages and subdomains use the stored click ID", async () => {
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready();
  const [cookie] = await cookies();
  assert.equal(cookie.domain, ".shop.test");
  assert.equal(cookie.sameSite, "Lax");
  const days = (cookie.expires - Date.now() / 1000) / 86400;
  assert.ok(days > 29.9 && days <= 30, `${days}`);

  await page.goto(shop("/thanks", "checkout.shop.test"));
  await ready();
  await track("pk_a", "purchase");
  assert.equal((await received(1))[0].clickId, CID);
});

test("the last click wins and reloading does not extend it", async () => {
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready();
  const first = (await cookies())[0].expires;
  await new Promise((r) => setTimeout(r, 1100));
  await page.reload();
  await ready();
  assert.equal((await cookies())[0].expires, first);
  await page.goto(shop(`/?shortas_cid=${CID2}`));
  await ready();
  const all = await cookies();
  assert.equal(all.length, 1);
  assert.equal(all[0].value, CID2);
});

test("ignores malformed click IDs and works without one", async () => {
  await page.goto(shop("/?shortas_cid=x;%20domain=evil"));
  await ready();
  await track("pk_a", "cart");
  assert.equal((await received(1))[0].clickId, undefined);
  assert.deepEqual(await cookies(), []);
});

test("required consent: nothing until given, then held events are sent", async () => {
  page_html = STUB + tag("pk_a", 'data-consent="required"');
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready();
  await track("pk_a", "cart");
  await received(0);
  assert.deepEqual(await cookies(), []);
  assert.equal(await page.evaluate(() => window.shortas.clickId("pk_a")), null);

  await page.evaluate(() => window.shortas.consent(true));
  assert.equal((await received(1))[0].clickId, CID);
  assert.equal((await cookies()).length, 1);

  await page.evaluate(() => window.shortas.consent(false));
  assert.deepEqual(await cookies(), []);
  await track("pk_a", "purchase");
  await received(1);
});

test("consent refused before the script loads", async () => {
  page_html = STUB + "<script>shortas.consent(false)</script>" + tag("pk_a");
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready();
  await page.waitForTimeout(100);
  await track("pk_a", "cart");
  await received(0);
  assert.deepEqual(await cookies(), []);
});

test("an event survives leaving the page", async () => {
  await page.goto(shop());
  await ready();
  await page.evaluate((to) => {
    addEventListener("pagehide", () => window.shortas.track("pk_a", "leave"));
    location.href = to;
  }, shop("/blank", "elsewhere.test"));
  await page.waitForURL("**/blank");
  assert.equal((await received(1))[0].step, "leave");
});

test("several conversions on one page, called before they load", async () => {
  page_html = STUB + tag("pk_a") + tag("pk_b") + tag("pk_a") +
    `<script>shortas.track("pk_b","b1");shortas.track("pk_a","a1",{value:1})</script>`;
  await page.goto(shop(`/?shortas_cid=${CID}`));
  await ready("pk_a"); await ready("pk_b");
  await track("pk_b", "b2");
  const all = (await received(3)).map((e) => `${e.key}:${e.step}`).sort();
  assert.deepEqual(all, ["pk_a:a1", "pk_b:b1", "pk_b:b2"]);
  assert.ok(events.every((e) => e.clickId === CID));
  assert.equal((await cookies()).length, 2);
});

test("temporary failures are retried with the same event ID", async () => {
  failures = [503, 429];
  await page.goto(shop());
  await ready();
  await track("pk_a", "cart");
  await page.waitForTimeout(6500);
  const all = await received(3);
  assert.equal(new Set(all.map((e) => e.eventId)).size, 1);
});

test("refused events are not retried", async () => {
  failures = [403];
  await page.goto(shop());
  await ready();
  await track("pk_a", "cart");
  await page.waitForTimeout(1500);
  await received(1);
});

test("numbers that are not finite are sent as such", async () => {
  await page.goto(shop());
  await ready();
  await page.evaluate(() => window.shortas.track("pk_a", "cart", { value: NaN }));
  assert.equal((await received(1))[0].value, "NaN");
});
