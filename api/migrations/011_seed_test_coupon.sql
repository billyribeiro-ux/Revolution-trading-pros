-- Seed 100% off test coupon
-- ICT 11+ Principal Engineer Grade - January 2026

-- Create a 100% discount test coupon for checkout testing
INSERT INTO coupons (
    code,
    description,
    discount_type,
    discount_value,
    min_purchase,
    max_discount,
    usage_limit,
    usage_count,
    is_active,
    starts_at,
    expires_at,
    created_at,
    updated_at
) VALUES (
    'FREE100',
    'Test coupon - 100% off for checkout testing',
    'percent',
    100.00,
    0.00,
    NULL,
    1000,
    0,
    true,
    NOW(),
    NOW() + INTERVAL '1 year',
    NOW(),
    NOW()
) ON CONFLICT (code) DO UPDATE SET
    discount_value = 100.00,
    is_active = true,
    usage_count = 0,
    expires_at = NOW() + INTERVAL '1 year',
    updated_at = NOW();
