---
layout: professional-theme
title: Conversion tracking
---

# Conversion tracking

Report signups, orders and other goals reached by visitors of your short links. Replace `go.example.com` with the conversion's short domain, `pk_abc` with its public key, and `sk_abc_secret` with its secret key.

Covered links add `shortas_cid` to their destination. It identifies the click and must be returned with each event for the event to be attributed. Nothing is inferred from IP addresses.

## Browser script

```html
<script>(function(w){var s=w.shortas=w.shortas||{queue:[]};["track","consent"].forEach(function(m){
s[m]=s[m]||function(){s.queue.push([m].concat([].slice.call(arguments)))}})})(window)</script>
<script async src="https://go.example.com/.well-known/shortas/c/pk_abc.js"></script>
```

Put it on the landing page and on every page that reports a step. The first part lets you call `track` before the script has loaded.

```html
<script>
shortas.track("pk_abc", "cart");
shortas.track("pk_abc", "purchase", { value: 49.90, currency: "EUR", externalId: "order-1042" });
</script>
```

| Option | Meaning |
|---|---|
| `value` | Number or decimal string, at most 4 decimals. Numbers are rounded. Without it, the step's default applies. |
| `currency` | ISO 4217 code. Defaults to the conversion's currency. Applies only with a `value`. |
| `externalId` | Your ID, such as an order number. Reporting it twice for a step counts once when every conversion counts. |
| `eventId` | Identifies a retry of the same event. Generated when left out. |
| `test` | `true` shows the event in the log without counting it. Works before activation. |

Several conversions can share a page: add one script element for each.

### Consent

Add `data-consent="required"` to the script element to write no cookie and send nothing until:

```js
shortas.consent(true);
```

Up to 20 events are held in memory meanwhile. `shortas.consent(false)` deletes the cookies and drops held events. You are responsible for deciding whether consent is needed, and for asking.

### Cookie limits

The click ID is kept in a first-party cookie, `_shortas_pk_abc`, for the attribution window. Safari limits cookies set by scripts to 7 days, and sometimes less, whatever the window is. Some ad blockers stop the script altogether. When that matters, pass the click ID to your server with the form or order and report the step from there:

```js
document.querySelector("#cid").value = shortas.clickId("pk_abc") || "";
```

## Server events

Never put the secret key in a page or app. It is shown once, when created or rotated.

```bash
curl -X POST https://go.example.com/.well-known/shortas/v1/conversions \
  -H "Authorization: Bearer sk_abc_secret" \
  -H "Content-Type: application/json" \
  -d '{"step":"purchase","clickId":"01M8A0TWHKY97G6TJFAFJVJZV0","value":"49.90","currency":"EUR","externalId":"order-1042"}'
```

The response is `202` with `{"eventId": "..."}`. Besides the options above, `occurredAt` accepts an RFC 3339 time at most 90 days old. Up to a minute in the future is accepted as now, because clocks differ.

## Refunds

```bash
curl -X POST https://go.example.com/.well-known/shortas/v1/refunds \
  -H "Authorization: Bearer sk_abc_secret" \
  -H "Content-Type: application/json" \
  -d '{"refundId":"refund-7","externalId":"order-1042","value":"20","currency":"EUR"}'
```

`externalId` names the refunded event and the currency must be the same. A repeated `refundId` applies once. Refunds need an active conversion.

## Responses

Errors are JSON, such as `{"error":"invalid","field":"step"}`.

| Status | Meaning |
|---|---|
| 202 | Stored durably. |
| 400 | The named `field` is invalid. |
| 401 | The secret key is missing or wrong. |
| 403 | The page's origin is not allowed. Test events are accepted from any origin. |
| 408, 413 | The body was too slow, or larger than 16 KB. |
| 409 | The conversion is not active. |
| 429, 503 | Retry after `Retry-After` seconds. |

When you retry, send the same `eventId`, `externalId` or `refundId`: an event answered with 503 may still have been stored. The script retries twice by itself.

Anyone can read the public key from a page, so browser events can be forged. Allowed origins only deter casual misuse. Accept values only from servers when they matter.
