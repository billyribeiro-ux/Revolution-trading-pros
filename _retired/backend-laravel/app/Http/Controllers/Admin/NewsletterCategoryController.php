<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\NewsletterCategory;
use App\Services\Newsletter\FormNewsletterBridgeService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * NewsletterCategoryController
 *
 * Admin controller for managing newsletter categories/topics.
 * Enables segmented email campaigns and subscriber preference management.
 *
 * @level ICT11 Principal Engineer
 */
class NewsletterCategoryController extends Controller
{
    public function __construct(
        private readonly FormNewsletterBridgeService $bridgeService
    ) {}

    /**
     * List all newsletter categories
     */
    public function index(Request $request): JsonResponse
    {
        $query = NewsletterCategory::query();

        // Filter by active status
        if ($request->has('active')) {
            $query->where('is_active', $request->boolean('active'));
        }

        // Filter by default status
        if ($request->has('default')) {
            $query->where('is_default', $request->boolean('default'));
        }

        // Search by name
        if ($request->filled('search')) {
            $search = $request->string('search')->value();
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                    ->orWhere('description', 'like', "%{$search}%");
            });
        }

        // Sorting
        $sortBy = $request->get('sort_by', 'sort_order');
        $sortDir = $request->get('sort_dir', 'asc');
        $allowedSorts = ['name', 'sort_order', 'subscriber_count', 'created_at', 'engagement_score'];

        if (in_array($sortBy, $allowedSorts)) {
            $query->orderBy($sortBy, $sortDir === 'desc' ? 'desc' : 'asc');
        }

        $categories = $request->boolean('paginate', true)
            ? $query->paginate($request->integer('per_page', 20))
            : $query->get();

        return response()->json([
            'success' => true,
            'data' => $categories,
        ]);
    }

    /**
     * Create a new category
     */
    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:newsletter_categories',
            'description' => 'nullable|string|max:1000',
            'icon' => 'nullable|string|max:50',
            'color' => ['nullable', 'string', 'regex:/^#[0-9A-Fa-f]{6}$/'],
            'sort_order' => 'nullable|integer|min:0',
            'is_active' => 'nullable|boolean',
            'is_default' => 'nullable|boolean',
            'settings' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        try {
            $category = NewsletterCategory::create([
                'name' => $validated['name'],
                'slug' => $validated['slug'] ?? Str::slug($validated['name']),
                'description' => $validated['description'] ?? null,
                'icon' => $validated['icon'] ?? null,
                'color' => $validated['color'] ?? null,
                'sort_order' => $validated['sort_order'] ?? 0,
                'is_active' => $validated['is_active'] ?? true,
                'is_default' => $validated['is_default'] ?? false,
                'settings' => $validated['settings'] ?? null,
                'metadata' => $validated['metadata'] ?? null,
            ]);

            Log::info('Newsletter category created', [
                'category_id' => $category->id,
                'name' => $category->name,
                'admin_id' => auth()->id(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Category created successfully',
                'data' => $category,
            ], 201);
        } catch (\Exception $e) {
            Log::error('Failed to create newsletter category', [
                'error' => $e->getMessage(),
                'data' => $validated,
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to create category',
            ], 500);
        }
    }

    /**
     * Show a specific category
     */
    public function show(int $id): JsonResponse
    {
        $category = NewsletterCategory::find($id);

        if (!$category) {
            return response()->json([
                'success' => false,
                'message' => 'Category not found',
            ], 404);
        }

        // Include statistics
        $category->load_count = $category->subscribers()->count();

        return response()->json([
            'success' => true,
            'data' => $category,
            'statistics' => $category->getStatistics(),
        ]);
    }

    /**
     * Update a category
     */
    public function update(Request $request, int $id): JsonResponse
    {
        $category = NewsletterCategory::find($id);

        if (!$category) {
            return response()->json([
                'success' => false,
                'message' => 'Category not found',
            ], 404);
        }

        $validated = $request->validate([
            'name' => 'sometimes|required|string|max:255',
            'slug' => [
                'sometimes',
                'required',
                'string',
                'max:255',
                Rule::unique('newsletter_categories')->ignore($id),
            ],
            'description' => 'nullable|string|max:1000',
            'icon' => 'nullable|string|max:50',
            'color' => ['nullable', 'string', 'regex:/^#[0-9A-Fa-f]{6}$/'],
            'sort_order' => 'nullable|integer|min:0',
            'is_active' => 'nullable|boolean',
            'is_default' => 'nullable|boolean',
            'settings' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        try {
            $category->update($validated);

            Log::info('Newsletter category updated', [
                'category_id' => $category->id,
                'changes' => $category->getChanges(),
                'admin_id' => auth()->id(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Category updated successfully',
                'data' => $category->fresh(),
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to update newsletter category', [
                'category_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to update category',
            ], 500);
        }
    }

    /**
     * Delete a category
     */
    public function destroy(int $id): JsonResponse
    {
        $category = NewsletterCategory::find($id);

        if (!$category) {
            return response()->json([
                'success' => false,
                'message' => 'Category not found',
            ], 404);
        }

        // Check if category has subscribers
        $subscriberCount = $category->subscribers()->count();
        if ($subscriberCount > 0) {
            return response()->json([
                'success' => false,
                'message' => "Cannot delete category with {$subscriberCount} active subscribers",
                'subscriber_count' => $subscriberCount,
            ], 422);
        }

        try {
            $category->delete();

            Log::info('Newsletter category deleted', [
                'category_id' => $id,
                'name' => $category->name,
                'admin_id' => auth()->id(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Category deleted successfully',
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to delete newsletter category', [
                'category_id' => $id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to delete category',
            ], 500);
        }
    }

    /**
     * Reorder categories
     */
    public function reorder(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'order' => 'required|array',
            'order.*.id' => 'required|integer|exists:newsletter_categories,id',
            'order.*.sort_order' => 'required|integer|min:0',
        ]);

        try {
            DB::transaction(function () use ($validated) {
                foreach ($validated['order'] as $item) {
                    NewsletterCategory::where('id', $item['id'])
                        ->update(['sort_order' => $item['sort_order']]);
                }
            });

            return response()->json([
                'success' => true,
                'message' => 'Categories reordered successfully',
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to reorder newsletter categories', [
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to reorder categories',
            ], 500);
        }
    }

    /**
     * Get categories for form select/dropdown
     */
    public function forSelect(): JsonResponse
    {
        $categories = NewsletterCategory::getForSelect();

        return response()->json([
            'success' => true,
            'data' => $categories,
        ]);
    }

    /**
     * Seed default categories
     */
    public function seedDefaults(): JsonResponse
    {
        try {
            NewsletterCategory::seedDefaults();

            return response()->json([
                'success' => true,
                'message' => 'Default categories seeded successfully',
                'data' => NewsletterCategory::ordered()->get(),
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to seed default categories', [
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to seed default categories',
            ], 500);
        }
    }

    /**
     * Get category analytics
     */
    public function analytics(int $id): JsonResponse
    {
        $category = NewsletterCategory::find($id);

        if (!$category) {
            return response()->json([
                'success' => false,
                'message' => 'Category not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'category' => $category,
                'statistics' => $category->getStatistics(),
                'engagement_score' => $category->calculateEngagementScore(),
            ],
        ]);
    }

    /**
     * Refresh subscriber count for all categories
     */
    public function refreshCounts(): JsonResponse
    {
        try {
            NewsletterCategory::all()->each(function (NewsletterCategory $category) {
                $category->updateSubscriberCount();
            });

            return response()->json([
                'success' => true,
                'message' => 'Subscriber counts refreshed',
            ]);
        } catch (\Exception $e) {
            Log::error('Failed to refresh category counts', [
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Failed to refresh counts',
            ], 500);
        }
    }

    /**
     * Get form-newsletter integration analytics
     */
    public function formSignupAnalytics(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'form_id' => 'nullable|integer|exists:forms,id',
            'period' => ['nullable', Rule::in(['day', 'week', 'month', 'year'])],
        ]);

        $analytics = $this->bridgeService->getFormSignupAnalytics(
            $validated['form_id'] ?? null,
            $validated['period'] ?? 'month'
        );

        return response()->json([
            'success' => true,
            'data' => $analytics,
        ]);
    }
}
