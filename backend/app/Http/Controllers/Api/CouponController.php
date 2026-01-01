<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Coupon;
use App\Events\CouponCreated;
use App\Events\CouponUpdated;
use App\Events\CouponDeleted;
use App\Events\CouponRedeemed;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

class CouponController extends Controller
{
    /**
     * Display a listing of coupons.
     */
    public function index(Request $request)
    {
        $query = Coupon::query();

        // Filter by status
        if ($request->has('is_active')) {
            $query->where('is_active', $request->boolean('is_active'));
        }

        // Search by code
        if ($request->has('search')) {
            $query->where('code', 'like', '%' . $request->search . '%');
        }

        // Pagination
        $perPage = $request->get('per_page', 20);
        $coupons = $query->orderBy('created_at', 'desc')->paginate($perPage);

        return response()->json([
            'coupons' => $coupons->items(),
            'total' => $coupons->total(),
            'current_page' => $coupons->currentPage(),
            'last_page' => $coupons->lastPage(),
        ]);
    }

    /**
     * Store a newly created coupon.
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'code' => 'required|string|unique:coupons,code|max:50',
            'type' => ['required', Rule::in(['percentage', 'fixed', 'bogo', 'free_shipping', 'tiered', 'bundle', 'cashback', 'points'])],
            'value' => 'required|numeric|min:0',
            'display_name' => 'nullable|string|max:255',
            'description' => 'nullable|string',
            'internal_notes' => 'nullable|string',
            'max_uses' => 'nullable|integer|min:0',
            'expiry_date' => 'nullable|date',
            'start_date' => 'nullable|date',
            'max_discount_amount' => 'nullable|numeric|min:0',
            'applicable_products' => 'nullable|array',
            'applicable_categories' => 'nullable|array',
            'min_purchase_amount' => 'nullable|numeric|min:0',
            'is_active' => 'boolean',
            'restrictions' => 'nullable|array',
            'campaign_id' => 'nullable|string|max:255',
            'segments' => 'nullable|array',
            'rules' => 'nullable|array',
            'stackable' => 'nullable|boolean',
            'priority' => 'nullable|integer',
            'referral_source' => 'nullable|string|max:255',
            'affiliate_id' => 'nullable|string|max:255',
            'influencer_id' => 'nullable|string|max:255',
            'tags' => 'nullable|array',
            'meta' => 'nullable|array',
            'ab_test' => 'nullable|array',
            'tiers' => 'nullable|array',
        ]);

        $validated['code'] = strtoupper($validated['code']);
        $validated['max_uses'] = $validated['max_uses'] ?? 0;
        $validated['current_uses'] = 0;

        $coupon = Coupon::create($validated);

        // Broadcast coupon creation via WebSocket
        broadcast(new CouponCreated($coupon))->toOthers();

        return response()->json([
            'message' => 'Coupon created successfully',
            'coupon' => $coupon
        ], 201);
    }

    /**
     * Display the specified coupon.
     */
    public function show(string $id)
    {
        $coupon = Coupon::findOrFail($id);
        return response()->json(['coupon' => $coupon]);
    }

    /**
     * Update the specified coupon.
     */
    public function update(Request $request, string $id)
    {
        $coupon = Coupon::findOrFail($id);

        $validated = $request->validate([
            'code' => ['sometimes', 'string', 'max:50', Rule::unique('coupons')->ignore($coupon->id)],
            'type' => ['sometimes', Rule::in(['percentage', 'fixed', 'bogo', 'free_shipping', 'tiered', 'bundle', 'cashback', 'points'])],
            'value' => 'sometimes|numeric|min:0',
            'display_name' => 'nullable|string|max:255',
            'description' => 'nullable|string',
            'internal_notes' => 'nullable|string',
            'max_uses' => 'sometimes|integer|min:0',
            'expiry_date' => 'nullable|date',
            'start_date' => 'nullable|date',
            'max_discount_amount' => 'nullable|numeric|min:0',
            'applicable_products' => 'nullable|array',
            'applicable_categories' => 'nullable|array',
            'min_purchase_amount' => 'nullable|numeric|min:0',
            'is_active' => 'sometimes|boolean',
            'restrictions' => 'nullable|array',
            'campaign_id' => 'nullable|string|max:255',
            'segments' => 'nullable|array',
            'rules' => 'nullable|array',
            'stackable' => 'nullable|boolean',
            'priority' => 'nullable|integer',
            'referral_source' => 'nullable|string|max:255',
            'affiliate_id' => 'nullable|string|max:255',
            'influencer_id' => 'nullable|string|max:255',
            'tags' => 'nullable|array',
            'meta' => 'nullable|array',
            'ab_test' => 'nullable|array',
            'tiers' => 'nullable|array',
        ]);

        if (isset($validated['code'])) {
            $validated['code'] = strtoupper($validated['code']);
        }

        $changedAttributes = array_keys($coupon->getDirty());
        $coupon->update($validated);

        // Broadcast coupon update via WebSocket
        broadcast(new CouponUpdated($coupon, $changedAttributes))->toOthers();

        return response()->json([
            'message' => 'Coupon updated successfully',
            'coupon' => $coupon
        ]);
    }

    /**
     * Remove the specified coupon.
     */
    public function destroy(string $id)
    {
        $coupon = Coupon::findOrFail($id);
        $couponId = $coupon->id;
        $couponCode = $coupon->code;
        $coupon->delete();

        // Broadcast coupon deletion via WebSocket
        broadcast(new CouponDeleted($couponId, $couponCode))->toOthers();

        return response()->json([
            'message' => 'Coupon deleted successfully'
        ]);
    }

    /**
     * Validate a coupon code (public endpoint).
     */
    public function validate(Request $request)
    {
        $request->validate([
            'code' => 'required|string',
            'cartTotal' => 'required|numeric|min:0',
        ]);

        $code = strtoupper($request->code);
        $cartTotal = $request->cartTotal;

        $coupon = Coupon::where('code', $code)
            ->where('is_active', true)
            ->first();

        if (!$coupon) {
            return response()->json([
                'valid' => false,
                'message' => 'Invalid coupon code',
                'discount' => 0
            ], 404);
        }

        // Check expiry
        if ($coupon->expiry_date && now()->isAfter($coupon->expiry_date)) {
            return response()->json([
                'valid' => false,
                'message' => 'This coupon has expired',
                'discount' => 0
            ], 400);
        }

        // Check usage limit
        if ($coupon->max_uses > 0 && $coupon->current_uses >= $coupon->max_uses) {
            return response()->json([
                'valid' => false,
                'message' => 'This coupon has reached its usage limit',
                'discount' => 0
            ], 400);
        }

        // Check minimum purchase
        if ($coupon->min_purchase_amount > 0 && $cartTotal < $coupon->min_purchase_amount) {
            return response()->json([
                'valid' => false,
                'message' => "Minimum purchase amount of $" . number_format($coupon->min_purchase_amount, 2) . " required",
                'discount' => 0
            ], 400);
        }

        // Calculate discount
        $discountAmount = 0;
        if ($coupon->type === 'percentage') {
            $discountAmount = ($cartTotal * $coupon->value) / 100;
        } else {
            $discountAmount = min($coupon->value, $cartTotal);
        }

        return response()->json([
            'valid' => true,
            'coupon' => $coupon,
            'discount' => $coupon->value,
            'discountAmount' => round($discountAmount, 2),
            'type' => $coupon->type,
            'message' => 'Coupon applied successfully'
        ]);
    }
}
