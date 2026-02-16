# Contributing to Revolution Trading Pros

**Apple Principal Engineer ICT Level 7+ Standards**

Thank you for your interest in contributing! This document outlines our development workflow, coding standards, and best practices.

---

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Development Setup](#development-setup)
3. [Coding Standards](#coding-standards)
4. [Git Workflow](#git-workflow)
5. [Testing Requirements](#testing-requirements)
6. [Pull Request Process](#pull-request-process)
7. [Architecture Decisions](#architecture-decisions)

---

## 🤝 Code of Conduct

- **Be respectful** - Treat all contributors with respect
- **Be constructive** - Provide helpful feedback
- **Be collaborative** - Work together to solve problems
- **Zero tolerance** - No harassment, discrimination, or unprofessional behavior

---

## 🛠️ Development Setup

### Prerequisites

- **Node.js** 20.x or later
- **Rust** 1.75 or later
- **PostgreSQL** 15+
- **Redis** 7+
- **Git** 2.40+

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/revolutiontradingpros/revolution-trading-pros.git
cd revolution-trading-pros

# Install frontend dependencies
cd frontend
npm install

# Install Rust dependencies
cd ../api
cargo build

# Set up environment variables
cp .env.example .env
# Edit .env with your local configuration
```

### Environment Variables

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for complete environment variable documentation.

---

## 📐 Coding Standards

### Frontend (TypeScript/Svelte)

#### Svelte 5 Runes (Required)

**✅ DO:**
```typescript
// Use Svelte 5 runes
let count = $state(0);
let doubled = $derived(count * 2);

$effect(() => {
  console.log('Count changed:', count);
});
```

**❌ DON'T:**
```typescript
// Deprecated Svelte 4 patterns
export let count = 0;  // Use $props() instead
$: doubled = count * 2;  // Use $derived() instead
```

#### TypeScript Standards

- **Strict mode enabled** - No `any` types without justification
- **Explicit return types** - All functions must declare return types
- **Named exports** - Prefer named exports over default exports
- **Type imports** - Use `import type` for type-only imports

```typescript
// ✅ Good
export function calculateProfit(entry: number, exit: number): number {
  return exit - entry;
}

// ❌ Bad
export default function(entry, exit) {
  return exit - entry;
}
```

#### Component Structure

```svelte
<script lang="ts">
  // 1. Imports
  import { onMount } from 'svelte';
  import type { ComponentProps } from './types';
  
  // 2. Props
  let { data, onUpdate }: ComponentProps = $props();
  
  // 3. State
  let isLoading = $state(false);
  
  // 4. Derived state
  let total = $derived(data.reduce((sum, item) => sum + item.value, 0));
  
  // 5. Effects
  $effect(() => {
    console.log('Data changed:', data);
  });
  
  // 6. Functions
  function handleClick() {
    onUpdate?.();
  }
</script>

<!-- 7. Template -->
<div>
  {#if isLoading}
    <p>Loading...</p>
  {:else}
    <p>Total: {total}</p>
  {/if}
</div>

<!-- 8. Styles -->
<style>
  div {
    padding: 1rem;
  }
</style>
```

### Backend (Rust)

#### Code Style

- **Follow Rust conventions** - Use `cargo fmt` and `cargo clippy`
- **Error handling** - Use `Result<T, E>` and `?` operator
- **Documentation** - All public functions must have doc comments
- **Type safety** - Leverage Rust's type system

```rust
/// Calculate profit/loss for a trade
///
/// # Arguments
/// * `entry_price` - Entry price in dollars
/// * `exit_price` - Exit price in dollars
/// * `quantity` - Number of shares/contracts
///
/// # Returns
/// Profit or loss in dollars
pub fn calculate_pnl(
    entry_price: f64,
    exit_price: f64,
    quantity: i32,
) -> Result<f64, TradeError> {
    if quantity <= 0 {
        return Err(TradeError::InvalidQuantity);
    }
    
    Ok((exit_price - entry_price) * quantity as f64)
}
```

#### Database Queries

- **Use SQLx** - Compile-time checked queries
- **Parameterized queries** - Never use string interpolation
- **Transactions** - Use transactions for multi-step operations

```rust
// ✅ Good - Compile-time checked
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = $1",
    email
)
.fetch_one(&pool)
.await?;

// ❌ Bad - SQL injection risk
let query = format!("SELECT * FROM users WHERE email = '{}'", email);
```

---

## 🔄 Git Workflow

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `refactor/description` - Code refactoring
- `docs/description` - Documentation updates
- `test/description` - Test additions/updates

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting
- `refactor` - Code restructuring
- `test` - Tests
- `chore` - Maintenance

**Examples:**
```
feat(explosive-swings): add trade plan modal
fix(auth): resolve OAuth redirect loop
docs(api): update Stripe webhook documentation
```

---

## 🧪 Testing Requirements

### All Pull Requests Must:

1. **Pass all existing tests**
2. **Add tests for new features**
3. **Maintain or improve coverage**
4. **Pass type checking** (`npm run check`)
5. **Pass linting** (`npm run lint`)

### Frontend Testing

```bash
# Unit tests
npm run test

# E2E tests
npm run test:e2e

# Type checking
npm run check

# Linting
npm run lint
```

### Backend Testing

```bash
# All tests
cargo test

# Linting
cargo clippy

# Format check
cargo fmt --check
```

---

## 🔍 Pull Request Process

1. **Create a branch** from `main`
2. **Make your changes** following coding standards
3. **Write tests** for new functionality
4. **Run all checks** locally before pushing
5. **Create PR** with clear description
6. **Address review feedback** promptly
7. **Squash commits** before merging

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] E2E tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings generated
```

---

## 🏗️ Architecture Decisions

### When to Create a New Component

- **Reusability** - Used in 2+ places
- **Complexity** - More than 100 lines
- **Separation of concerns** - Distinct responsibility

### When to Add a New API Endpoint

- **RESTful design** - Follows REST principles
- **Authentication** - Proper auth middleware
- **Documentation** - OpenAPI/Swagger docs
- **Testing** - Integration tests included

### Database Migrations

- **Never modify existing migrations** - Create new ones
- **Test rollback** - Ensure migrations are reversible
- **Data safety** - Never delete data without backup

---

## 📞 Getting Help

- **Questions?** Open a GitHub Discussion
- **Bug reports?** Create an issue with reproduction steps
- **Security issues?** Email security@revolutiontradingpros.com

---

**Thank you for contributing to Revolution Trading Pros!** 🚀

