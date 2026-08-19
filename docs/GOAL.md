# Grocery Optimizer

## Problem

Groceries are bought on a recurring rhythm, but that rhythm is invisible. A
shopper forgets staple items until they run out, buys duplicates of things
already in the pantry, and never has a complete list when walking into the
store. Receipts hold the data needed to predict the next purchase, but that
data stays locked inside a store's website and out of reach.

## Product

Grocery Optimizer turns receipt history into a predictive shopping list. The
shopper shops as usual. The system scrapes past receipts, learns which items
recur and on what cadence, and produces a list of items likely needed soon -
before the shopper notices they are out.

The output is a shopping list, not a dashboard. The value is anticipation: the
right item surfaces at the right time because the pattern says so.

## How It Works

The pipeline has three stages, built in order:

1. **Ingest** - A Rust backend logs into the shopper's store account (using
   their own credentials) and scrapes past receipts via the store's website.
   Receipts are parsed into a normalized record: store, date, line items,
   quantities, prices.
2. **Model** - Receipt history is analyzed to identify recurring items and
   their purchase cadence. An item bought every 12 days, last bought 11 days
   ago, is a candidate for the next list. Frequency, recency, and variance
   feed the prediction.
3. **Predict** - The system generates a shopping list of items the shopper is
   likely to need now, ranked by confidence and urgency. The list is the
   product; the shopper acts on it.

## Phase One Scope

Phase one proves the hardest, riskiest part: ingestion. A CLI tool (not the
API server, not the frontend) logs into one store and scrapes receipts
successfully. The CLI exists to validate that the scraping approach works
against a real, login-gated, possibly JavaScript-heavy store site before any
other layer is built.

- One store. One adapter. No abstraction for multiple stores yet.
- CLI only - the API server and SPA come later, once ingestion is proven.
- Scraping uses a system-installed Chrome instance driven via CDP, not a
  bundled headless browser. The store's real login flow must work.
- Credentials live in a local TOML file supplied via a CLI flag. The file is
  a development artifact, not a shipped config surface.
- Receipts are parsed into a normalized structure and printed, not yet
  persisted. Persistence lands with the API server.

## Open Questions

- **Prediction model.** Cadence-only (item bought every N days) is the
  starting point. Whether seasonality, basket co-occurrence, or quantity
  drift improve predictions is undecided until receipt data exists to test
  against.
- **Store name.** Which specific store phase one targets - left as a
  placeholder until the scraper is pointed at a real site.
- **Persistence.** Where receipts live long-term (Postgres, SQLite, files)
  is deferred until the API server phase.

## Notes

Scraping a store site with the user's own login may violate that store's
Terms of Service and can trigger bot detection. This is a personal-use
project acting on the owner's own account and data. Proceed with awareness.