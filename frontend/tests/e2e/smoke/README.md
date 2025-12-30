# E2E Smoke Tests

Quick validation tests that run on every PR to ensure critical functionality works.

## Test Files

- **homepage.spec.ts** - Homepage loads, navigation visible, no console errors, responsive
- **dashboard.spec.ts** - Dashboard route exists, handles unauthenticated access
- **api-health.spec.ts** - App loads without critical network errors

## Running Locally

```bash
# Run all smoke tests
npm run test:e2e:smoke

# Run specific test
npx playwright test tests/e2e/smoke/homepage.spec.ts

# Run with UI
npx playwright test tests/e2e/smoke --ui
```

## CI/CD

These tests run automatically on:
- Pull requests to main
- Push to main branch
- Manual workflow dispatch

They provide fast feedback (< 2 minutes) before running the full E2E suite.
