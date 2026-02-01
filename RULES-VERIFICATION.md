# RULES-VERIFICATION.md
## Zero Errors Verification Protocol

## 🚨 ABSOLUTE REQUIREMENT
Every change = 0 errors, 0 warnings, 0 regressions. No exceptions.

## 🔒 BEFORE ANY CHANGE
```
□ READ the ENTIRE file (not just the area)
□ TRACE every import - where does it come from?
□ TRACE every export - what files depend on this?
□ IDENTIFY all type definitions involved
□ MAP component/function hierarchy
□ CHECK for tests that might break
□ RUN: pnpm run check (MUST pass before starting)
```

## ✅ AFTER EVERY CHANGE
```bash
pnpm run check    # MUST show 0 errors
pnpm run lint     # MUST show 0 errors, 0 warnings
pnpm run build    # MUST complete successfully
pnpm run test     # ALL tests MUST pass
```
**If ANY fails → FIX IMMEDIATELY. Do NOT proceed.**

## 🔴 DEPENDENCY TRACING
Before modifying ANY file, create this map:
```
═══════════════════════════════════════════════════════
DEPENDENCY MAP FOR: src/lib/components/Button.svelte
═══════════════════════════════════════════════════════
IMPORTS FROM:
├── svelte (Snippet type)
├── $lib/types/components.ts (ButtonProps)
└── $lib/utils/cn.ts (cn function)

IMPORTED BY:
├── src/lib/components/ui/index.ts
├── src/routes/+page.svelte
└── 12 other files...
═══════════════════════════════════════════════════════
```
**Can't produce this? You don't understand the file enough.**

## 🔴 RIPPLE EFFECT PREVENTION
When Changing Component Props:
```
Adding required prop to Button.svelte
MUST UPDATE:
├── Button.svelte          ← Add prop
├── Button.test.ts         ← Update tests
├── Every file using Button ← Add prop
└── types/components.ts    ← Update types
```

## 🔴 TYPE SAFETY
### ❌ NEVER DO:
```typescript
let data: any = fetchData();           // BANNED
const user = data as User;              // BANNED
// @ts-ignore                           // BANNED
```

### ✅ ALWAYS DO:
```typescript
let data: ApiResponse<User> = await fetchData();

function isUser(data: unknown): data is User {
  return typeof data === 'object' && data !== null && 'id' in data;
}

const value = obj?.prop ?? defaultValue;
```

## 🚨 THE MANTRA
"I will not introduce errors. I will trace all dependencies. I will update all affected files. I will verify with evidence."
