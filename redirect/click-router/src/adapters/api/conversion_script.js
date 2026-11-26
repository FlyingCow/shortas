(function () {
  "use strict";
  var config = __CONFIG__;
  var PARAM = "shortas_cid";
  var MAX_HELD = 20;
  var RETRY_DELAYS = [1000, 5000];

  var script = document.currentScript ||
    document.querySelector('script[src*="/.well-known/shortas/c/' + config.key + '.js"]');
  if (!script) return;

  // `shortas` may be the snippet's queue, or the API of another conversion.
  var api = window.shortas || (window.shortas = {});
  var all = api.conversions || (api.conversions = {});
  if (all[config.key]) return;

  var endpoint = new URL("/.well-known/shortas/v1/events", script.src).href;
  var cookieName = "_shortas_" + config.key;
  var allowed = script.getAttribute("data-consent") !== "required";
  if (typeof api.consented === "boolean") allowed = api.consented;
  var held = [];

  function isClickId(value) {
    return typeof value === "string" && /^[0-9A-HJKMNP-TV-Z]{26}$/.test(value);
  }

  var landed = new URLSearchParams(location.search).get(PARAM);
  if (!isClickId(landed)) landed = null;

  function readCookie() {
    var parts = document.cookie.split("; ");
    for (var i = 0; i < parts.length; i++) {
      if (parts[i].indexOf(cookieName + "=") === 0) return parts[i].slice(cookieName.length + 1);
    }
    return null;
  }

  function writeCookie(value, maxAge, domain) {
    document.cookie = cookieName + "=" + value + "; path=/; max-age=" + maxAge + "; samesite=lax" +
      (domain ? "; domain=" + domain : "") + (location.protocol === "https:" ? "; secure" : "");
  }

  // Parent domains from the widest. Browsers refuse public suffixes.
  function parentDomains() {
    var labels = location.hostname.split(".");
    var domains = [];
    for (var i = labels.length - 2; i >= 0; i--) domains.push(labels.slice(i).join("."));
    return domains;
  }

  function storeClickId() {
    if (!landed || readCookie() === landed) return;
    var domains = parentDomains();
    var maxAge = config.windowDays * 86400;
    for (var i = 0; i < domains.length; i++) {
      writeCookie(landed, maxAge, domains[i]);
      if (readCookie() === landed) return;
    }
    writeCookie(landed, maxAge);
  }

  function deleteCookie() {
    writeCookie("", 0);
    parentDomains().forEach(function (domain) { writeCookie("", 0, domain); });
  }

  function clickId() {
    if (!allowed) return null;
    if (landed) return landed;
    var stored = readCookie();
    return isClickId(stored) ? stored : null;
  }

  function newEventId() {
    var bytes = crypto.getRandomValues(new Uint8Array(16));
    return Array.prototype.map.call(bytes, function (b) {
      return ("0" + b.toString(16)).slice(-2);
    }).join("");
  }

  function send(body, attempt) {
    function retry() {
      if (allowed && attempt < RETRY_DELAYS.length) {
        setTimeout(function () { if (allowed) send(body, attempt + 1); }, RETRY_DELAYS[attempt]);
      }
    }
    // Plain text needs no preflight, and keepalive survives leaving the page.
    fetch(endpoint, {
      method: "POST",
      body: body,
      headers: { "Content-Type": "text/plain" },
      credentials: "omit",
      keepalive: true
    }).then(function (response) {
      if (response.status === 429 || response.status >= 500) return retry();
      if (!response.ok && window.console) {
        response.text().then(function (text) { console.warn("shortas: event refused", text); });
      }
    }, retry);
  }

  function track(step, options) {
    options = options || {};
    if (!allowed) {
      if (held.length < MAX_HELD) held.push([step, options]);
      return;
    }
    var event = { key: config.key, step: step, eventId: options.eventId || newEventId() };
    var id = clickId();
    if (id) event.clickId = id;
    ["currency", "externalId", "test"].forEach(function (name) {
      if (options[name] !== undefined) event[name] = options[name];
    });
    // JSON turns NaN and Infinity into null, which would mean "no value".
    if (options.value !== undefined && options.value !== null) {
      event.value = typeof options.value === "number" && !isFinite(options.value)
        ? String(options.value) : options.value;
    }
    send(JSON.stringify(event), 0);
  }

  function setConsent(given) {
    allowed = given;
    if (!given) {
      held = [];
      deleteCookie();
      return;
    }
    storeClickId();
    held.splice(0).forEach(function (call) { track(call[0], call[1]); });
  }

  all[config.key] = { track: track, clickId: clickId, setConsent: setConsent };

  var queue = api.queue || [];
  api.queue = [];
  api.consent = function (given) {
    api.consented = given === true;
    Object.keys(all).forEach(function (key) { all[key].setConsent(api.consented); });
  };
  api.clickId = function (key) {
    return all[key] ? all[key].clickId() : null;
  };
  api.track = function (key, step, options) {
    if (all[key]) all[key].track(step, options);
    // Its script may still be loading.
    else if (api.queue.length < 100) api.queue.push(["track", key, step, options]);
  };
  queue.forEach(function (call) {
    api[call[0]].apply(api, Array.prototype.slice.call(call, 1));
  });

  if (allowed) storeClickId();
})();
