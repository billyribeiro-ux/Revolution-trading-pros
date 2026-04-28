#!/usr/bin/env node
/**
 * seed_stripe_test_products.js
 *
 * Creates Stripe test-mode products + prices for all 20 membership_plans rows,
 * then UPDATEs membership_plans with the resulting stripe_product_id + stripe_price_id.
 *
 * Prerequisites:
 *   - STRIPE_SECRET env var (sk_test_...) or reads from api/.env
 *   - psql reachable at the DATABASE_URL in api/.env (or localhost:5432 default)
 *   - npm install stripe pg  (or: node --require ... if already installed globally)
 *
 * Usage:
 *   node scripts/seed_stripe_test_products.js
 *   node scripts/seed_stripe_test_products.js --dry-run   # prints plan without touching Stripe/DB
 *
 * Idempotent: re-running looks up existing products by metadata.plan_id and skips
 * creation if one already exists with that marker. Safe to run multiple times.
 */

'use strict';

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

// ── Load env from api/.env ───────────────────────────────────────────────────
function loadEnvFile(filePath) {
  if (!fs.existsSync(filePath)) return {};
  const lines = fs.readFileSync(filePath, 'utf8').split('\n');
  const env = {};
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const eq = trimmed.indexOf('=');
    if (eq === -1) continue;
    const key = trimmed.slice(0, eq).trim();
    const val = trimmed.slice(eq + 1).trim();
    if (val) env[key] = val;
  }
  return env;
}

const repoRoot = path.resolve(__dirname, '..');
const envFile = path.join(repoRoot, 'api', '.env');
const fileEnv = loadEnvFile(envFile);

const STRIPE_SECRET = process.env.STRIPE_SECRET || fileEnv.STRIPE_SECRET;
const DATABASE_URL = process.env.DATABASE_URL || fileEnv.DATABASE_URL ||
  'postgres://rtp:rtp_password@localhost:5432/revolution_trading_pros';

if (!STRIPE_SECRET || !STRIPE_SECRET.startsWith('sk_test_')) {
  console.error('ERROR: STRIPE_SECRET must be a sk_test_... key. Set it in api/.env or as env var.');
  process.exit(1);
}

// ── Plan definitions (mirrors membership_plans rows exactly) ─────────────────
// billing_cycle → Stripe interval
const INTERVAL_MAP = { monthly: 'month', quarterly: 'month', annual: 'year' };
const INTERVAL_COUNT_MAP = { monthly: 1, quarterly: 3, annual: 1 };

const PLANS = [
  { id: 1,  name: 'Day Trading Room',                    price_cents: 19700, billing_cycle: 'monthly'  },
  { id: 2,  name: 'Swing Trading Room',                  price_cents: 14700, billing_cycle: 'monthly'  },
  { id: 3,  name: 'Small Account Mentorship',            price_cents:  9700, billing_cycle: 'monthly'  },
  { id: 4,  name: 'Alerts Only',                         price_cents:  9700, billing_cycle: 'monthly'  },
  { id: 5,  name: 'Explosive Swing',                     price_cents: 14700, billing_cycle: 'monthly'  },
  { id: 6,  name: 'SPX Profit Pulse',                    price_cents: 19700, billing_cycle: 'monthly'  },
  { id: 7,  name: 'Weekly Watchlist',                    price_cents:  4700, billing_cycle: 'monthly'  },
  { id: 8,  name: 'Day Trading Room - Quarterly',        price_cents: 52900, billing_cycle: 'quarterly' },
  { id: 9,  name: 'Day Trading Room - Annual',           price_cents: 172700, billing_cycle: 'annual'  },
  { id: 10, name: 'Swing Trading Room - Quarterly',      price_cents: 39700, billing_cycle: 'quarterly' },
  { id: 11, name: 'Swing Trading Room - Annual',         price_cents: 129700, billing_cycle: 'annual'  },
  { id: 12, name: 'Small Account Mentorship - Quarterly',price_cents: 26200, billing_cycle: 'quarterly' },
  { id: 13, name: 'Small Account Mentorship - Annual',   price_cents: 85700, billing_cycle: 'annual'   },
  { id: 14, name: 'Explosive Swings - Quarterly',        price_cents: 39700, billing_cycle: 'quarterly' },
  { id: 15, name: 'Explosive Swings - Annual',           price_cents: 129700, billing_cycle: 'annual'  },
  { id: 16, name: 'SPX Profit Pulse - Quarterly',        price_cents: 52900, billing_cycle: 'quarterly' },
  { id: 17, name: 'SPX Profit Pulse - Annual',           price_cents: 172700, billing_cycle: 'annual'  },
  { id: 18, name: 'High Octane Scanner - Monthly',       price_cents:  9700, billing_cycle: 'monthly'  },
  { id: 19, name: 'High Octane Scanner - Quarterly',     price_cents: 26200, billing_cycle: 'quarterly' },
  { id: 20, name: 'High Octane Scanner - Annual',        price_cents: 85700, billing_cycle: 'annual'   },
];

// ── Stripe API helper (raw fetch, no npm dependency) ─────────────────────────
async function stripePost(path, params) {
  const body = new URLSearchParams(params).toString();
  const resp = await fetch(`https://api.stripe.com/v1${path}`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${STRIPE_SECRET}`,
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body,
  });
  const data = await resp.json();
  if (data.error) throw new Error(`Stripe error on POST ${path}: ${JSON.stringify(data.error)}`);
  return data;
}

async function stripeGet(path, query = {}) {
  const qs = new URLSearchParams(query).toString();
  const url = `https://api.stripe.com/v1${path}${qs ? '?' + qs : ''}`;
  const resp = await fetch(url, {
    headers: { 'Authorization': `Bearer ${STRIPE_SECRET}` },
  });
  const data = await resp.json();
  if (data.error) throw new Error(`Stripe error on GET ${path}: ${JSON.stringify(data.error)}`);
  return data;
}

// ── DB helper via psql ────────────────────────────────────────────────────────
function psql(sql) {
  try {
    // Connect to host postgres (not inside Docker) — use localhost URL
    const hostDb = DATABASE_URL.replace('@db:', '@localhost:').replace('@db/', '@localhost/');
    const result = execSync(
      `psql "${hostDb}" -t -A -c ${JSON.stringify(sql)}`,
      { encoding: 'utf8', stdio: ['pipe', 'pipe', 'pipe'] }
    );
    return result.trim();
  } catch (e) {
    throw new Error(`psql error: ${e.stderr || e.message}`);
  }
}

// ── Main ──────────────────────────────────────────────────────────────────────
const DRY_RUN = process.argv.includes('--dry-run');

async function main() {
  console.log(`\n=== seed_stripe_test_products.js ===`);
  console.log(`Mode: ${DRY_RUN ? 'DRY RUN' : 'LIVE'}`);
  console.log(`Stripe account: ${STRIPE_SECRET.slice(0, 20)}...`);
  console.log(`Database: ${DATABASE_URL.replace(/:\/\/[^@]+@/, '://<redacted>@')}\n`);

  // Fetch existing products tagged with our plan_id metadata to support idempotency
  const existingProducts = {};
  let hasMore = true;
  let startingAfter = undefined;
  while (hasMore) {
    const q = { limit: '100', 'metadata[rtp_plan_id]': '' };
    if (startingAfter) q.starting_after = startingAfter;
    // Can't filter by metadata key presence in list API — fetch all and filter locally
    const page = await stripeGet('/products', { limit: '100', ...(startingAfter ? { starting_after: startingAfter } : {}) });
    for (const p of page.data) {
      if (p.metadata && p.metadata.rtp_plan_id) {
        existingProducts[p.metadata.rtp_plan_id] = p;
      }
    }
    hasMore = page.has_more;
    if (page.data.length > 0) startingAfter = page.data[page.data.length - 1].id;
    else hasMore = false;
  }
  console.log(`Found ${Object.keys(existingProducts).length} existing RTP products in Stripe.\n`);

  const results = [];

  for (const plan of PLANS) {
    const planIdStr = String(plan.id);
    const interval = INTERVAL_MAP[plan.billing_cycle];
    const interval_count = INTERVAL_COUNT_MAP[plan.billing_cycle];

    let product;
    if (existingProducts[planIdStr]) {
      product = existingProducts[planIdStr];
      console.log(`[${plan.id}] SKIP product (exists): ${product.id}  "${plan.name}"`);
    } else {
      if (DRY_RUN) {
        console.log(`[${plan.id}] DRY: would create product "${plan.name}"`);
        results.push({ plan_id: plan.id, stripe_product_id: 'prod_DRY', stripe_price_id: 'price_DRY' });
        continue;
      }
      product = await stripePost('/products', {
        name: plan.name,
        'metadata[rtp_plan_id]': planIdStr,
        'metadata[rtp_billing_cycle]': plan.billing_cycle,
      });
      console.log(`[${plan.id}] CREATED product: ${product.id}  "${plan.name}"`);
    }

    // Check if a price already exists for this product with the right amount
    const existingPrices = await stripeGet('/prices', {
      product: product.id,
      limit: '10',
      active: 'true',
      'recurring[interval]': interval,
    });

    let price;
    const matchingPrice = existingPrices.data.find(p =>
      p.unit_amount === plan.price_cents &&
      p.recurring &&
      p.recurring.interval === interval &&
      p.recurring.interval_count === interval_count
    );

    if (matchingPrice) {
      price = matchingPrice;
      console.log(`[${plan.id}] SKIP price (exists): ${price.id}  $${(price.unit_amount/100).toFixed(2)}/${plan.billing_cycle}`);
    } else {
      if (DRY_RUN) {
        console.log(`[${plan.id}] DRY: would create price $${(plan.price_cents/100).toFixed(2)}/${plan.billing_cycle}`);
        results.push({ plan_id: plan.id, stripe_product_id: product.id, stripe_price_id: 'price_DRY' });
        continue;
      }
      price = await stripePost('/prices', {
        product: product.id,
        unit_amount: String(plan.price_cents),
        currency: 'usd',
        'recurring[interval]': interval,
        'recurring[interval_count]': String(interval_count),
        'metadata[rtp_plan_id]': planIdStr,
        'metadata[rtp_billing_cycle]': plan.billing_cycle,
      });
      console.log(`[${plan.id}] CREATED price:   ${price.id}  $${(price.unit_amount/100).toFixed(2)}/${plan.billing_cycle}`);
    }

    results.push({
      plan_id: plan.id,
      stripe_product_id: product.id,
      stripe_price_id: price.id,
    });
  }

  if (DRY_RUN) {
    console.log('\nDRY RUN complete — no Stripe or DB changes made.');
    return;
  }

  // ── Update membership_plans in DB ──────────────────────────────────────────
  console.log('\n── Updating membership_plans in DB ──');
  for (const r of results) {
    if (r.stripe_price_id === 'price_DRY') continue;
    const sql = `UPDATE membership_plans SET stripe_product_id = '${r.stripe_product_id}', stripe_price_id = '${r.stripe_price_id}', updated_at = NOW() WHERE id = ${r.plan_id};`;
    const out = psql(sql);
    console.log(`  [${r.plan_id}] UPDATE → ${r.stripe_product_id} / ${r.stripe_price_id}  (psql: "${out}")`);
  }

  // ── Verify ─────────────────────────────────────────────────────────────────
  console.log('\n── Verification: SELECT from membership_plans ──');
  const rows = psql("SELECT id, name, stripe_product_id, stripe_price_id FROM membership_plans ORDER BY id;");
  console.log(rows);

  // ── Output mapping JSON ────────────────────────────────────────────────────
  const mappingPath = path.join(repoRoot, 'scripts', 'stripe_test_product_mapping.json');
  fs.writeFileSync(mappingPath, JSON.stringify(results, null, 2));
  console.log(`\nMapping written to: ${mappingPath}`);
  console.log('\n=== seed_stripe_test_products.js COMPLETE ===\n');
}

main().catch(e => {
  console.error('FATAL:', e.message);
  process.exit(1);
});
