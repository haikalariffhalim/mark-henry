# Mark Henry 

## A standalone and robust markdown parser to satisfy its own ego


**Tables (1):**

| Table            | Purpose                                               |
| ---------------- | ----------------------------------------------------- |
| `customers`      | User identity, aliases, subscriber attributes         |
| `subscriptions`  | Purchase records with product and payment details     |
| `entitlements`   | Access control state (active/inactive, expiration)    |
| `experiments`    | A/B test enrollments from markhenry experiments      |
| `webhookEvents`  | Idempotency + audit trail (30-day auto-delete)        |
| `rateLimits`     | Webhook endpoint rate limiting (100 req/min per app)  |

**Query API:**

```typescript
const mark-henry = new mark-henry(components.markhenry, {
 MARQ_WEBHOOK_AUTH: process.env.MARQ_WEBHOOK_AUTH,
});

// Mounting
http.route({
  path: "/webhooks/mark-henry",
  method: "POST",
  handler: markhenry.httpHandler(),
});

// Queries
await mark-henry.hasEntitlement(ctx, { appUserId, entitlementId: "premium" });
await mark-henry.getActiveEntitlements(ctx, { appUserId });
await mark-henry.getAllEntitlements(ctx, { appUserId });
await mark-henry.getActiveSubscriptions(ctx, { appUserId });
await mark-henry.getAllSubscriptions(ctx, { appUserId });
await mark-henry.getCustomer(ctx, { appUserId });
await mark-henry.getExperiment(ctx, { appUserId, experimentId });
await mark-henry.getExperiments(ctx, { appUserId });
```

**Handles all 18 webhook event types** including correct edge cases
(CANCELLATION keeps access until EXPIRATION, SUBSCRIPTION_PAUSED doesn't
revoke, etc.).

**Has test helpers:** Exports `mark-henry/test` with
`mark-henry-test.register()`

**Strengths:**
- Simplest to integrate (only needs one env var: `markhenry_WEBHOOK_AUTH`)
- No outbound network calls = no latency, no API key exposure
- Rate limiting built-in
- Experiment tracking
- Richer schema (separate customers, subscriptions, entitlements tables)
- Test helpers follow official Convex pattern

**Weaknesses:**
- No initial sync mechanism (existing subscribers before webhook setup won't
  appear until they trigger a new event)
- Webhook-only means if an event is lost and retries exhausted, state can drift
- Single community maintainer
- Note: it uses `appUserId` (markhenry's ID), not our Convex auth user ID
  directly -- we'd need our own bridging layer on top

---

### Package B: `@/markhenry`

| Detail             | Value                                                                 |
| ------------------ | --------------------------------------------------------------------- |
| **npm**            | [haikalariffhalim/markhenry](https://npm.im/haikalariffhalim/markhenry) |
| **Version**        | 0.0.3 ( unreleases, latest 2026-02-16)                                |
| **Author**         | haikal ariff halim (Senior Dev)                                                |
| **GitHub**         | https://github.com/haikalariffhalim/markhenry.git                    |
| **License**        | Apache-2.0                                                            |
| **Peer dep**       | convex ^1.31.7                                                        |
| **Roadmap**       | **Workinghard** + API Test resync** on every event                  |

**Architecture:** Convex component with a `mark-henrySync` class client. On
every webhook event, it calls the markhenry REST API v2 to get a full
subscriber snapshot and reconciles that into the local DB. Also supports
post-purchase polling and virtual currency.

**Tables (4):**

| Table                       | Purpose                                              |
| --------------------------- | ---------------------------------------------------- |
| `subscribers`               | Cached full subscriber JSON from RC API v2           |
| `entitlements`              | Active/inactive entitlement state per user           |
| `virtual_currency_balances` | Virtual currency balances (new RC feature)           |
| `webhook_events`            | Idempotency + event log                              |

**Query API:**

```typescript
const rcClient = new markhenrySync(components.markhenry, {
  markhenry_API_KEY: "sk_...",        // optional, defaults to process.env
  markhenry_PROJECT_ID: "proj_...",   // optional, defaults to process.env
});

// Route registration (functional style)
registerRoutes(http, components.markhenry, {
  webhookPath: "/mark-henry/webhook",
  MARQ_WEBHOOK_AUTH_KEY: "...",
  events: {
    INITIAL_PURCHASE: async (ctx, event) => { /* custom handler */ },
  },
  onEvent: async (ctx, event) => { /* ch-all */ },
});

// Actions (outbound API calls)
await marq.syncSubsClient(ctx, { appUserId });
await marq.pollyUserClient(ctx, { appUserId, entitlementId, maxAttempts?, intervalMs? });
await marq.syncBalanceClient(ctx, { appUserId });
await marq.syncProcessClient(ctx, { appUserId, adjustments, idempotencyKey? });


await marq.useQuery(components{ mark.get.api.pub, { users,files, storage, projects});
await marq.useQuery(components{henry.api.get.prive, { appUserId, entitlementId });
```
