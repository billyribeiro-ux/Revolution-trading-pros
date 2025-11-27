<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\MembershipPlan;
use App\Models\Order;
use App\Models\Product;
use App\Models\Coupon;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

class CartController extends Controller
{
    /**
     * Tax rates by region (ISO 3166-1 alpha-2 country codes)
     * Digital products/services typically have different tax rules
     */
    private const TAX_RATES = [
        'US' => [
            'default' => 0.0,  // No federal sales tax on digital goods (varies by state)
            'CA' => 0.0725,    // California
            'NY' => 0.08,      // New York
            'TX' => 0.0625,    // Texas
            'FL' => 0.06,      // Florida
            'WA' => 0.065,     // Washington
        ],
        'GB' => 0.20,  // UK VAT
        'DE' => 0.19,  // Germany VAT
        'FR' => 0.20,  // France VAT
        'AU' => 0.10,  // Australia GST
        'CA' => 0.05,  // Canada GST (provinces have additional PST/HST)
        'default' => 0.0,
    ];

    /**
     * Process checkout and create order
     */
    public function checkout(Request $request)
    {
        $request->validate([
            'items' => 'required|array|min:1',
            'items.*.type' => 'required|in:product,membership',
            'items.*.id' => 'required|integer',
            'items.*.quantity' => 'integer|min:1|max:100',
            'coupon_code' => 'nullable|string|max:50',
            'billing_country' => 'nullable|string|size:2',
            'billing_state' => 'nullable|string|max:10',
        ]);

        $user = $request->user();

        return DB::transaction(function () use ($request, $user) {
            $items = $request->items;
            $subtotal = 0;
            $orderItems = [];

            // Calculate subtotal and validate items
            foreach ($items as $item) {
                if ($item['type'] === 'product') {
                    $product = Product::findOrFail($item['id']);

                    // Check if product is available
                    if (!$product->is_active) {
                        return response()->json([
                            'error' => 'Product is no longer available',
                            'product_id' => $item['id'],
                        ], 400);
                    }

                    $price = $product->price;
                    $name = $product->name;
                    $isTaxable = $product->is_taxable ?? true;
                } else {
                    $plan = MembershipPlan::findOrFail($item['id']);

                    if (!$plan->is_active) {
                        return response()->json([
                            'error' => 'Membership plan is no longer available',
                            'plan_id' => $item['id'],
                        ], 400);
                    }

                    $price = $plan->price;
                    $name = $plan->name;
                    $isTaxable = $plan->is_taxable ?? true;
                }

                $quantity = $item['quantity'] ?? 1;
                $lineTotal = $price * $quantity;
                $subtotal += $lineTotal;

                $orderItems[] = [
                    'item_type' => $item['type'],
                    'item_id' => $item['id'],
                    'name' => $name,
                    'price' => $price,
                    'quantity' => $quantity,
                    'line_total' => $lineTotal,
                    'is_taxable' => $isTaxable,
                ];
            }

            // Apply coupon if provided
            $discount = 0;
            $coupon = null;
            $couponCode = $request->coupon_code;

            if ($couponCode) {
                $coupon = Coupon::where('code', strtoupper($couponCode))
                    ->where('is_active', true)
                    ->where(function ($q) {
                        $q->whereNull('starts_at')
                          ->orWhere('starts_at', '<=', now());
                    })
                    ->where(function ($q) {
                        $q->whereNull('expires_at')
                          ->orWhere('expires_at', '>', now());
                    })
                    ->first();

                if (!$coupon) {
                    return response()->json([
                        'error' => 'Invalid or expired coupon code',
                    ], 400);
                }

                // Check usage limit
                if ($coupon->max_uses && $coupon->uses_count >= $coupon->max_uses) {
                    return response()->json([
                        'error' => 'Coupon has reached its usage limit',
                    ], 400);
                }

                // Calculate discount
                if ($coupon->type === 'percentage') {
                    $discount = $subtotal * ($coupon->value / 100);
                    if ($coupon->max_discount) {
                        $discount = min($discount, $coupon->max_discount);
                    }
                } else {
                    $discount = min($coupon->value, $subtotal);
                }

                // Increment coupon usage
                $coupon->increment('uses_count');
            }

            // Calculate taxable amount (subtotal minus discount on taxable items)
            $taxableAmount = 0;
            foreach ($orderItems as $item) {
                if ($item['is_taxable']) {
                    $taxableAmount += $item['line_total'];
                }
            }

            // Apply discount proportionally to taxable amount
            if ($discount > 0 && $subtotal > 0) {
                $discountRatio = $discount / $subtotal;
                $taxableAmount = $taxableAmount * (1 - $discountRatio);
            }

            // Calculate tax based on billing location
            $country = $request->billing_country ?? 'US';
            $state = $request->billing_state;
            $taxRate = $this->getTaxRate($country, $state);
            $tax = round($taxableAmount * $taxRate, 2);

            // Calculate total
            $total = $subtotal - $discount + $tax;

            // Create order
            $order = Order::create([
                'user_id' => $user->id,
                'order_number' => 'ORD-' . strtoupper(Str::random(10)),
                'subtotal' => round($subtotal, 2),
                'discount' => round($discount, 2),
                'coupon_id' => $coupon?->id,
                'coupon_code' => $coupon?->code,
                'tax' => $tax,
                'tax_rate' => $taxRate,
                'billing_country' => $country,
                'billing_state' => $state,
                'total' => round($total, 2),
                'status' => 'pending',
                'currency' => 'USD',
            ]);

            // Create order items
            foreach ($orderItems as $item) {
                $order->items()->create($item);
            }

            Log::info('Order created', [
                'order_id' => $order->id,
                'user_id' => $user->id,
                'total' => $total,
                'tax' => $tax,
            ]);

            // TODO: Integrate payment provider (Stripe, PayPal, etc.)
            // For now, return order for client to complete payment

            return response()->json([
                'order' => $order->load('items'),
                'payment_required' => true,
                'message' => 'Order created. Proceed with payment.',
            ], 201);
        });
    }

    /**
     * Get tax rate based on country and state
     */
    private function getTaxRate(string $country, ?string $state = null): float
    {
        // Check if country has state-specific rates
        if (isset(self::TAX_RATES[$country]) && is_array(self::TAX_RATES[$country])) {
            if ($state && isset(self::TAX_RATES[$country][$state])) {
                return self::TAX_RATES[$country][$state];
            }
            return self::TAX_RATES[$country]['default'] ?? 0.0;
        }

        // Check for country-level rate
        if (isset(self::TAX_RATES[$country])) {
            return self::TAX_RATES[$country];
        }

        // Default rate
        return self::TAX_RATES['default'];
    }

    /**
     * Calculate tax preview for cart (without creating order)
     */
    public function calculateTax(Request $request)
    {
        $request->validate([
            'subtotal' => 'required|numeric|min:0',
            'billing_country' => 'required|string|size:2',
            'billing_state' => 'nullable|string|max:10',
        ]);

        $taxRate = $this->getTaxRate($request->billing_country, $request->billing_state);
        $tax = round($request->subtotal * $taxRate, 2);

        return response()->json([
            'subtotal' => $request->subtotal,
            'tax_rate' => $taxRate,
            'tax_rate_percentage' => $taxRate * 100,
            'tax' => $tax,
            'total' => $request->subtotal + $tax,
        ]);
    }
}
