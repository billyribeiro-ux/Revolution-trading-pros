<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Product;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class ProductController extends Controller
{
    /**
     * Display a listing of products.
     */
    public function index(Request $request)
    {
        $query = Product::query();

        // Filter by type (course, indicator, membership, bundle)
        if ($request->has('type')) {
            $query->where('type', $request->type);
        }

        // Search
        if ($request->has('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('description', 'like', "%{$search}%");
            });
        }

        // Filter by active status
        if ($request->has('is_active')) {
            $query->where('is_active', $request->boolean('is_active'));
        }

        $products = $query->orderBy('created_at', 'desc')
            ->paginate($request->get('per_page', 20));

        return response()->json($products);
    }

    /**
     * Store a newly created product.
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:products,slug',
            'type' => 'required|in:course,indicator,membership,bundle',
            'description' => 'nullable|string',
            'long_description' => 'nullable|string',
            'price' => 'required|numeric|min:0',
            'is_active' => 'boolean',
            'metadata' => 'nullable|array',
            'thumbnail' => 'nullable|string',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string',
            'indexable' => 'boolean',
            'canonical_url' => 'nullable|string|max:255',
        ]);

        // Auto-generate slug if not provided
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $product = Product::create($validated);

        return response()->json([
            'message' => ucfirst($product->type) . ' created successfully',
            'product' => $product,
        ], 201);
    }

    /**
     * Display the specified product.
     */
    public function show(string $id)
    {
        $product = Product::with('users')->findOrFail($id);

        return response()->json($product);
    }

    /**
     * Update the specified product.
     */
    public function update(Request $request, string $id)
    {
        $product = Product::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'slug' => 'sometimes|string|max:255|unique:products,slug,' . $product->id,
            'type' => 'sometimes|in:course,indicator,membership,bundle',
            'description' => 'nullable|string',
            'long_description' => 'nullable|string',
            'price' => 'sometimes|numeric|min:0',
            'is_active' => 'boolean',
            'metadata' => 'nullable|array',
            'thumbnail' => 'nullable|string',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string',
            'indexable' => 'boolean',
            'canonical_url' => 'nullable|string|max:255',
        ]);

        $product->update($validated);

        return response()->json([
            'message' => ucfirst($product->type) . ' updated successfully',
            'product' => $product->fresh(),
        ]);
    }

    /**
     * Remove the specified product.
     */
    public function destroy(string $id)
    {
        $product = Product::findOrFail($id);
        
        // Check if product has users
        $userCount = $product->users()->count();
        if ($userCount > 0) {
            return response()->json([
                'message' => "Cannot delete {$product->type} with {$userCount} user(s). Please remove user associations first.",
            ], 422);
        }

        $type = $product->type;
        $product->delete();

        return response()->json([
            'message' => ucfirst($type) . ' deleted successfully',
        ]);
    }

    /**
     * Get products by type (courses, indicators, memberships).
     */
    public function byType(string $type)
    {
        $validTypes = ['course', 'indicator', 'membership', 'bundle'];
        
        if (!in_array($type, $validTypes)) {
            return response()->json(['message' => 'Invalid product type'], 400);
        }

        $products = Product::where('type', $type)
            ->orderBy('created_at', 'desc')
            ->get();

        return response()->json($products);
    }

    /**
     * Get product statistics.
     */
    public function stats()
    {
        $totalProducts = Product::count();
        $activeProducts = Product::where('is_active', true)->count();
        $courseCount = Product::where('type', 'course')->count();
        $indicatorCount = Product::where('type', 'indicator')->count();
        $membershipCount = Product::where('type', 'membership')->count();
        $bundleCount = Product::where('type', 'bundle')->count();
        $totalRevenue = Product::where('is_active', true)->sum('price');

        return response()->json([
            'total_products' => $totalProducts,
            'active_products' => $activeProducts,
            'courses' => $courseCount,
            'indicators' => $indicatorCount,
            'memberships' => $membershipCount,
            'bundles' => $bundleCount,
            'total_revenue_potential' => $totalRevenue,
        ]);
    }

    /**
     * Assign product to user.
     */
    public function assignToUser(Request $request, string $productId)
    {
        $validated = $request->validate([
            'user_id' => 'required|exists:users,id',
            'order_id' => 'nullable|string',
        ]);

        $product = Product::findOrFail($productId);
        $userId = $validated['user_id'];

        // Check if already assigned
        if ($product->users()->where('user_id', $userId)->exists()) {
            return response()->json([
                'message' => 'User already has access to this ' . $product->type,
            ], 422);
        }

        $product->users()->attach($userId, [
            'purchased_at' => now(),
            'order_id' => $validated['order_id'] ?? null,
        ]);

        return response()->json([
            'message' => ucfirst($product->type) . ' assigned to user successfully',
        ]);
    }

    /**
     * Remove product from user.
     */
    public function removeFromUser(Request $request, string $productId)
    {
        $validated = $request->validate([
            'user_id' => 'required|exists:users,id',
        ]);

        $product = Product::findOrFail($productId);
        $product->users()->detach($validated['user_id']);

        return response()->json([
            'message' => ucfirst($product->type) . ' removed from user successfully',
        ]);
    }

    /**
     * Get users who have access to a product.
     */
    public function productUsers(string $productId)
    {
        $product = Product::with('users')->findOrFail($productId);

        return response()->json([
            'product' => $product->only(['id', 'name', 'type']),
            'users' => $product->users,
        ]);
    }
}
