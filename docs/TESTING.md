# Testing Guide

**Revolution Trading Pros - Apple Principal Engineer ICT Level 7+**

---

## 📋 Overview

Our testing strategy ensures **zero errors/warnings** in production through comprehensive test coverage across unit, integration, and end-to-end tests.

### Testing Philosophy

1. **Test behavior, not implementation** - Focus on user-facing functionality
2. **Fast feedback loops** - Unit tests run in milliseconds
3. **Realistic scenarios** - E2E tests mirror real user workflows
4. **Continuous integration** - All tests run on every commit
5. **Zero tolerance** - Failing tests block deployment

---

## 🎯 Test Coverage

### Current Status

| Category | Tests | Coverage | Status |
|----------|-------|----------|--------|
| **E2E Tests** | 363+ | Critical paths | ✅ Passing |
| **Unit Tests** | 150+ | Business logic | ✅ Passing |
| **Integration Tests** | 50+ | API endpoints | ✅ Passing |
| **Type Checking** | All files | 100% | ✅ Passing |

---

## 🧪 Frontend Testing

### Technology Stack

- **E2E:** Playwright
- **Unit:** Vitest
- **Component:** Testing Library
- **Type Checking:** TypeScript + svelte-check

### Running Tests

```bash
cd frontend

# Unit tests
npm run test              # Run all unit tests
npm run test:watch        # Watch mode
npm run test:coverage     # With coverage report

# E2E tests
npm run test:e2e          # All E2E tests
npm run test:e2e:ui       # Interactive UI mode
npm run test:e2e:debug    # Debug mode

# Type checking
npm run check             # Full type check
npm run check:watch       # Watch mode

# Linting
npm run lint              # ESLint
npm run lint:fix          # Auto-fix issues
```

### E2E Test Structure

```typescript
// e2e/blog-editor.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Blog Editor', () => {
  test.beforeEach(async ({ page }) => {
    // Login and navigate to editor
    await page.goto('/admin/posts/new');
  });

  test('should create a new blog post', async ({ page }) => {
    // Type title
    await page.fill('[data-testid="post-title"]', 'Test Post');
    
    // Add content
    await page.click('[data-testid="add-paragraph"]');
    await page.fill('[data-testid="block-0"]', 'Test content');
    
    // Publish
    await page.click('[data-testid="publish-button"]');
    
    // Verify success
    await expect(page.locator('.success-message')).toBeVisible();
  });
});
```

### Unit Test Structure

```typescript
// src/lib/utils/calculations.test.ts
import { describe, it, expect } from 'vitest';
import { calculatePnL } from './calculations';

describe('calculatePnL', () => {
  it('should calculate profit correctly', () => {
    const result = calculatePnL({
      entryPrice: 100,
      exitPrice: 110,
      quantity: 10
    });
    
    expect(result).toBe(100); // $10 profit × 10 shares
  });

  it('should calculate loss correctly', () => {
    const result = calculatePnL({
      entryPrice: 100,
      exitPrice: 90,
      quantity: 10
    });
    
    expect(result).toBe(-100); // $10 loss × 10 shares
  });
});
```

### Component Testing

```typescript
// src/lib/components/TradeCard.test.ts
import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import TradeCard from './TradeCard.svelte';

describe('TradeCard', () => {
  it('should render trade details', () => {
    render(TradeCard, {
      props: {
        trade: {
          symbol: 'SPY',
          entryPrice: 450,
          exitPrice: 455,
          pnl: 500
        }
      }
    });
    
    expect(screen.getByText('SPY')).toBeInTheDocument();
    expect(screen.getByText('$500')).toBeInTheDocument();
  });
});
```

---

## ⚙️ Backend Testing

### Technology Stack

- **Unit/Integration:** Rust built-in test framework
- **HTTP Testing:** axum-test
- **Database:** SQLx with test database

### Running Tests

```bash
cd api

# All tests
cargo test

# Specific test
cargo test test_create_user

# With output
cargo test -- --nocapture

# Linting
cargo clippy

# Format check
cargo fmt --check
```

### Unit Test Structure

```rust
// src/services/trade_calculator.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pnl() {
        let result = calculate_pnl(100.0, 110.0, 10);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_calculate_pnl_loss() {
        let result = calculate_pnl(100.0, 90.0, 10);
        assert_eq!(result, -100.0);
    }
}
```

### Integration Test Structure

```rust
// tests/api/users.rs
use axum_test::TestServer;

#[tokio::test]
async fn test_create_user() {
    let app = create_test_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/users")
        .json(&json!({
            "email": "test@example.com",
            "password": "SecurePass123!"
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "email": "test@example.com"
    }));
}
```

---

## 🎭 E2E Test Scenarios

### Critical User Flows

#### 1. Authentication Flow
```typescript
test('complete authentication flow', async ({ page }) => {
  // 1. Visit homepage
  await page.goto('/');
  
  // 2. Click login
  await page.click('[data-testid="login-button"]');
  
  // 3. Enter credentials
  await page.fill('[name="email"]', 'test@example.com');
  await page.fill('[name="password"]', 'password123');
  
  // 4. Submit
  await page.click('[type="submit"]');
  
  // 5. Verify redirect to dashboard
  await expect(page).toHaveURL('/dashboard');
});
```

#### 2. Blog Post Creation
```typescript
test('create and publish blog post', async ({ page }) => {
  // Login as admin
  await loginAsAdmin(page);
  
  // Navigate to editor
  await page.goto('/admin/posts/new');
  
  // Create post
  await page.fill('[data-testid="title"]', 'Test Post');
  await page.click('[data-testid="add-paragraph"]');
  await page.fill('[data-testid="block-0"]', 'Content');
  
  // Publish
  await page.click('[data-testid="publish"]');
  
  // Verify on public site
  await page.goto('/blog/test-post');
  await expect(page.locator('h1')).toHaveText('Test Post');
});
```

#### 3. Course Enrollment
```typescript
test('enroll in course', async ({ page }) => {
  // Login
  await loginAsUser(page);
  
  // Browse courses
  await page.goto('/courses');
  
  // Select course
  await page.click('[data-testid="course-card-1"]');
  
  // Enroll
  await page.click('[data-testid="enroll-button"]');
  
  // Verify access
  await expect(page.locator('.lesson-list')).toBeVisible();
});
```

---

## 📊 Test Reports

### Coverage Reports

```bash
# Frontend coverage
cd frontend
npm run test:coverage
# → Open coverage/index.html

# Backend coverage
cd api
cargo tarpaulin --out Html
# → Open tarpaulin-report.html
```

### CI/CD Integration

All tests run automatically on:
- **Pull requests** - Blocks merge if failing
- **Main branch commits** - Deployment gate
- **Nightly builds** - Full test suite

---

## 🐛 Debugging Tests

### Frontend

```bash
# Debug E2E tests
npm run test:e2e:debug

# Debug specific test
npx playwright test blog-editor.spec.ts --debug

# View test report
npx playwright show-report
```

### Backend

```bash
# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture

# Debug with lldb
rust-lldb target/debug/deps/test_binary
```

---

## ✅ Best Practices

### DO:
- ✅ Write tests before fixing bugs
- ✅ Test edge cases and error conditions
- ✅ Use descriptive test names
- ✅ Keep tests independent
- ✅ Mock external dependencies

### DON'T:
- ❌ Test implementation details
- ❌ Write flaky tests
- ❌ Skip tests to make CI pass
- ❌ Commit commented-out tests
- ❌ Test third-party libraries

---

## 🚀 Continuous Improvement

- **Weekly:** Review test coverage reports
- **Monthly:** Audit flaky tests
- **Quarterly:** Update testing dependencies
- **Annually:** Review testing strategy

