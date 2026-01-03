<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\Request;

class MeController extends Controller
{
    public function show(Request $request)
    {
        $user = $request->user();

        // Use Spatie HasRoles to expose roles and admin flag
        $roles = method_exists($user, 'getRoleNames') ? $user->getRoleNames()->toArray() : [];
        
        // Check if user is superadmin by email (from config)
        $superadminEmails = config('superadmin.emails', []);
        $isSuperadmin = in_array(strtolower($user->email), array_map('strtolower', $superadminEmails));
        
        // Add super-admin role if superadmin by email
        if ($isSuperadmin && !in_array('super-admin', $roles)) {
            $roles[] = 'super-admin';
        }

        // Check admin status
        $isAdmin = $isSuperadmin || (method_exists($user, 'hasAnyRole')
            ? $user->hasAnyRole(['admin', 'super-admin', 'super_admin', 'administrator'])
            : false);

        // Get all permissions for the user
        $permissions = [];
        if (method_exists($user, 'getAllPermissions')) {
            $permissions = $user->getAllPermissions()->pluck('name')->toArray();
        }
        
        // Superadmin gets all permissions
        if ($isSuperadmin) {
            $permissions = ['*']; // Wildcard for all permissions
        }

        return response()->json([
            'id' => $user->id,
            'name' => $user->name,
            'first_name' => $user->first_name ?? null,
            'last_name' => $user->last_name ?? null,
            'email' => $user->email,
            'email_verified_at' => $user->email_verified_at,
            'created_at' => $user->created_at,
            'updated_at' => $user->updated_at,
            'must_change_password' => (bool) ($user->must_change_password ?? false),
            'roles' => $roles,
            'permissions' => $permissions,
            'is_admin' => $isAdmin,
            'is_superadmin' => $isSuperadmin,
        ]);
    }

    public function memberships(Request $request)
    {
        $memberships = $request->user()
            ->memberships()
            ->with('plan')
            ->where('status', 'active')
            ->get();

        return response()->json($memberships);
    }

    public function products(Request $request)
    {
        $products = $request->user()
            ->products()
            ->get();

        return response()->json($products);
    }

    public function indicators(Request $request)
    {
        $indicators = $request->user()
            ->products()
            ->where('type', 'indicator')
            ->get();

        return response()->json($indicators);
    }

    /**
     * Get user's orders with formatted response
     * ICT 11 Protocol: Enterprise-grade order retrieval
     */
    public function orders(Request $request)
    {
        $orders = $request->user()
            ->orders()
            ->with(['items.product'])
            ->orderBy('created_at', 'desc')
            ->get()
            ->map(function ($order) {
                return [
                    'id' => $order->id,
                    'number' => $order->order_number ?? $order->id,
                    'date' => $order->created_at?->toIso8601String(),
                    'status' => $order->status,
                    'total' => number_format($order->total ?? 0, 2),
                    'currency' => $order->currency ?? 'USD',
                    'itemCount' => $order->items->count(),
                    'items' => $order->items->map(function ($item) {
                        return [
                            'id' => $item->id,
                            'name' => $item->product?->name ?? $item->name ?? 'Product',
                            'quantity' => $item->quantity ?? 1,
                            'price' => number_format($item->price ?? 0, 2),
                            'total' => number_format($item->total ?? 0, 2),
                        ];
                    }),
                    'billingAddress' => $order->billing_address ?? null,
                    'paymentMethod' => $order->payment_method ?? null,
                ];
            });

        return response()->json([
            'orders' => $orders
        ]);
    }

    /**
     * Get single order by ID
     * ICT 11 Protocol: Secure order detail retrieval with ownership verification
     */
    public function showOrder(Request $request, $id)
    {
        $order = $request->user()
            ->orders()
            ->with(['items.product'])
            ->where('id', $id)
            ->first();

        if (!$order) {
            return response()->json(['message' => 'Order not found'], 404);
        }

        return response()->json([
            'order' => [
                'id' => $order->id,
                'number' => $order->order_number ?? $order->id,
                'date' => $order->created_at?->toIso8601String(),
                'status' => $order->status,
                'total' => number_format($order->total ?? 0, 2),
                'subtotal' => number_format($order->subtotal ?? 0, 2),
                'tax' => number_format($order->tax ?? 0, 2),
                'discount' => number_format($order->discount ?? 0, 2),
                'currency' => $order->currency ?? 'USD',
                'paymentMethod' => $order->payment_method ?? null,
                'items' => $order->items->map(function ($item) {
                    return [
                        'id' => $item->id,
                        'name' => $item->product?->name ?? $item->name ?? 'Product',
                        'quantity' => $item->quantity ?? 1,
                        'price' => number_format($item->price ?? 0, 2),
                        'total' => number_format($item->total ?? 0, 2),
                    ];
                }),
                'billingAddress' => $order->billing_address ?? null,
                'shippingAddress' => $order->shipping_address ?? null,
                'notes' => $order->notes ?? null,
            ]
        ]);
    }
}
