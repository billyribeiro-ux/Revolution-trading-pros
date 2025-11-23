# Enterprise Subscription System Upgrade Plan

## Goal
Upgrade the subscription system to "Google Principal Engineer L7" standards. This means robust architecture, type safety, transactional integrity, event-driven architecture, and a premium user experience.

## User Review Required
> [!IMPORTANT]
> This refactor introduces a new `SubscriptionService` and moves logic out of Controllers and Models. This is a significant architectural improvement but requires careful testing.

## Proposed Changes

### Backend (Laravel)

#### [NEW] Enums
- Create `App\Enums\SubscriptionStatus` to replace hardcoded strings.
- Create `App\Enums\SubscriptionInterval` for billing periods.

#### [NEW] Service Layer
- Create `App\Services\SubscriptionService`.
    - Handle all business logic (create, cancel, pause, resume, renew).
    - Wrap operations in Database Transactions.
    - Fire Events.

#### [NEW] Events
- `SubscriptionCreated`
- `SubscriptionCancelled`
- `SubscriptionPaused`
- `SubscriptionResumed`
- `SubscriptionRenewed`

#### [MODIFY] Models
- `App\Models\UserSubscription`: Update to use Enums. Remove business logic (move to Service).
- `App\Models\SubscriptionPlan`: Update to use Enums.

#### [MODIFY] Controllers
- `App\Http\Controllers\Admin\UserSubscriptionController`: Refactor to use `SubscriptionService`. Use `FormRequest` for validation. Use `JsonResource` for responses.

#### [NEW] API Resources
- `App\Http\Resources\SubscriptionResource`: Standardize JSON output.

### Frontend (SvelteKit)

#### [MODIFY] `src/routes/admin/subscriptions/+page.svelte`
- Update types to match new Backend Enums.
- Implement optimistic UI updates.
- Replace simple alerts with Toast notifications.
- Improve loading states (skeleton loaders).

#### [MODIFY] `src/lib/api/subscriptions.ts`
- Ensure strict typing for responses.

## Verification Plan

### Automated Tests
- **Unit Tests**: Test `SubscriptionService` methods in isolation.
- **Feature Tests**: Test API endpoints (`UserSubscriptionController`) to ensure correct JSON structure and status codes.
    - `php artisan test --filter Subscription`

### Manual Verification
1.  **Create Subscription**: Use Admin UI to create a new subscription for a user.
2.  **Pause/Resume**: Test pause and resume flows, verifying DB state and UI updates.
3.  **Cancel**: Test cancellation (immediate vs end of period).
4.  **Renew**: Trigger renewal and verify dates update correctly.
