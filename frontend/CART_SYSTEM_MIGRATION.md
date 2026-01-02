# WooCommerce to Custom Cart System Migration

## Overview
This document outlines the complete migration from WooCommerce class names to a custom Revolution Trading cart system while maintaining **pixel-perfect identical appearance**.

## WooCommerce Classes Found

### Page-Level Classes
| WooCommerce Class | Custom Cart Class | Purpose |
|------------------|-------------------|---------|
| `woocommerce-cart` | `rtp-cart` | Main cart page wrapper |
| `woocommerce-page` | `rtp-page` | General page wrapper |
| `woocommerce-checkout` | `rtp-checkout` | Main checkout page wrapper |
| `woocommerce-cart-form` | `rtp-cart-form` | Cart form container |
| `woocommerce-cart-form__contents` | `rtp-cart-form__contents` | Cart form contents |
| `woocommerce-checkout-review-order` | `rtp-checkout-review-order` | Checkout order review section |
| `woocommerce-checkout-review-order-table` | `rtp-checkout-review-order-table` | Checkout order table |

### Price Display Classes
| WooCommerce Class | Custom Cart Class | Purpose |
|------------------|-------------------|---------|
| `woocommerce-Price-amount` | `rtp-price-amount` | Price amount wrapper |
| `woocommerce-Price-amount-wrap` | `rtp-price-amount-wrap` | Price amount outer wrapper |
| `woocommerce-Price-currencySymbol` | `rtp-price-currency` | Currency symbol ($) |

### Form Classes
| WooCommerce Class | Custom Cart Class | Purpose |
|------------------|-------------------|---------|
| `woocommerce-form-coupon-toggle` | `rtp-form-coupon-toggle` | Coupon toggle button wrapper |
| `woocommerce-form-coupon` | `rtp-form-coupon` | Coupon form |
| `woocommerce-info` | `rtp-info` | Info message box |

### Other Classes
| WooCommerce Class | Custom Cart Class | Purpose |
|------------------|-------------------|---------|
| `checkout_coupon` | `rtp-checkout-coupon` | Checkout coupon form |

## Migration Strategy

### Phase 1: CSS Class Replacement
Replace all WooCommerce classes with custom RTP (Revolution Trading Pros) classes:
- Maintain exact same CSS rules
- Keep all styling identical
- Ensure no visual changes

### Phase 2: Component Updates
Update all Svelte components:
1. Cart page (`/routes/cart/+page.svelte`)
2. Checkout page (`/routes/checkout/+page.svelte`)
3. NonMemberCheckout component (`/lib/components/cart/NonMemberCheckout.svelte`)

### Phase 3: Testing
- Visual regression testing
- Cart functionality testing
- Checkout flow testing
- Mobile responsiveness testing

## Implementation Details

### Custom Cart System Naming Convention

**Prefix:** `rtp-` (Revolution Trading Pros)

**Structure:**
- `rtp-cart` - Cart-related classes
- `rtp-checkout` - Checkout-related classes
- `rtp-price` - Price display classes
- `rtp-form` - Form-related classes
- `rtp-info` - Information/message classes

### CSS Compatibility

All CSS rules will be duplicated to support both old and new class names during transition:

```css
/* Old WooCommerce class */
.woocommerce-cart { ... }

/* New custom cart class */
.rtp-cart { ... }

/* Combined selector for transition period */
.woocommerce-cart,
.rtp-cart { ... }
```

### WordPress Nonce Removal

The WordPress nonce field will be replaced with custom CSRF protection:

**Old:**
```html
<input type="hidden" name="_wpnonce" value={cartNonce} />
```

**New:**
```html
<input type="hidden" name="_csrf_token" value={csrfToken} />
```

## Files to Update

### 1. Cart Page
**File:** `/routes/cart/+page.svelte`
- Replace `woocommerce-cart` → `rtp-cart`
- Replace `woocommerce-page` → `rtp-page`
- Replace `woocommerce-cart-form` → `rtp-cart-form`
- Replace `woocommerce-cart-form__contents` → `rtp-cart-form__contents`
- Replace `woocommerce-Price-amount` → `rtp-price-amount`
- Replace `woocommerce-Price-amount-wrap` → `rtp-price-amount-wrap`
- Replace `woocommerce-Price-currencySymbol` → `rtp-price-currency`
- Remove WordPress nonce, add CSRF token

### 2. Checkout Page
**File:** `/routes/checkout/+page.svelte`
- Replace `woocommerce-checkout` → `rtp-checkout`
- Replace `woocommerce-page` → `rtp-page`
- Replace price-related classes
- Update form classes

### 3. NonMemberCheckout Component
**File:** `/lib/components/cart/NonMemberCheckout.svelte`
- Replace `woocommerce-checkout` → `rtp-checkout`
- Replace `woocommerce-page` → `rtp-page`
- Replace `woocommerce-checkout-review-order` → `rtp-checkout-review-order`
- Replace `woocommerce-checkout-review-order-table` → `rtp-checkout-review-order-table`
- Replace `woocommerce-form-coupon-toggle` → `rtp-form-coupon-toggle`
- Replace `woocommerce-form-coupon` → `rtp-form-coupon`
- Replace `woocommerce-info` → `rtp-info`
- Replace `checkout_coupon` → `rtp-checkout-coupon`
- Replace price-related classes

## CSS Updates

All CSS will be updated to use the new class names while maintaining identical styling:

```css
/* Cart Page */
.rtp-cart { /* same as .woocommerce-cart */ }
.rtp-cart-form { /* same as .woocommerce-cart-form */ }
.rtp-cart-form__contents { /* same as .woocommerce-cart-form__contents */ }

/* Checkout Page */
.rtp-checkout { /* same as .woocommerce-checkout */ }

/* Price Display */
.rtp-price-amount { /* same as .woocommerce-Price-amount */ }
.rtp-price-currency { /* same as .woocommerce-Price-currencySymbol */ }

/* Forms */
.rtp-form-coupon-toggle { /* same as .woocommerce-form-coupon-toggle */ }
.rtp-form-coupon { /* same as .woocommerce-form-coupon */ }
.rtp-info { /* same as .woocommerce-info */ }
```

## Security Enhancements

### CSRF Protection
Replace WordPress nonce with custom CSRF token:

```typescript
// Generate CSRF token
import { generateCSRFToken } from '$lib/utils/security';

let csrfToken = $state(generateCSRFToken());
```

### Session Management
Implement custom session management:
- JWT-based authentication
- Secure cookie handling
- Token refresh mechanism

## Testing Checklist

- [ ] Cart page loads with identical appearance
- [ ] Checkout page loads with identical appearance
- [ ] NonMemberCheckout component renders correctly
- [ ] Add to cart functionality works
- [ ] Remove from cart functionality works
- [ ] Cart quantity updates work
- [ ] Coupon application works
- [ ] Checkout flow completes successfully
- [ ] Price calculations are accurate
- [ ] Subscription handling works
- [ ] Mobile responsive design maintained
- [ ] All CSS classes render correctly
- [ ] No console errors
- [ ] CSRF protection functional

## Rollback Plan

If issues arise:
1. Revert to WooCommerce classes
2. Keep custom cart logic
3. Gradual migration approach

## Benefits of Custom Cart System

1. **No WordPress Dependency** - Fully decoupled from WordPress
2. **Better Performance** - Optimized for SvelteKit
3. **Custom Features** - Easier to add Revolution Trading-specific features
4. **Modern Stack** - Built with modern web technologies
5. **Maintainability** - Easier to maintain and update
6. **Security** - Custom security implementation
7. **Scalability** - Better scaling capabilities

## Timeline

- **Phase 1 (CSS):** 1 hour
- **Phase 2 (Components):** 2 hours
- **Phase 3 (Testing):** 1 hour
- **Total:** 4 hours

## Notes

- All visual appearance must remain **identical**
- No functionality changes during migration
- Maintain backward compatibility during transition
- Document all changes thoroughly
- Test thoroughly before deployment

## Related Documentation

- See `CLASSES_IMPLEMENTATION.md` for similar migration patterns
- See `INDICATORS_IMPLEMENTATION.md` for component structure
- See cart store implementation in `/lib/stores/cart.ts`
