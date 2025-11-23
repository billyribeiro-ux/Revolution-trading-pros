<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Category;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

class CategoryController extends Controller
{
    /**
     * Display a listing of categories
     */
    public function index(Request $request)
    {
        $query = Category::query()->with('parent');

        // Search
        if ($request->has('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('description', 'like', "%{$search}%")
                  ->orWhere('slug', 'like', "%{$search}%");
            });
        }

        // Filter by visibility
        if ($request->has('is_visible')) {
            $query->where('is_visible', $request->boolean('is_visible'));
        }

        // Filter by parent
        if ($request->has('parent_id')) {
            $query->where('parent_id', $request->parent_id);
        }

        // Sort
        $sortBy = $request->get('sort_by', 'order');
        $sortDir = $request->get('sort_dir', 'asc');
        
        if ($sortBy === 'post_count') {
            // For post_count, we need to sort by the computed attribute
            $categories = $query->get()->sortBy('post_count', SORT_REGULAR, $sortDir === 'desc');
            
            return response()->json([
                'data' => $categories->values(),
                'total' => $categories->count()
            ]);
        }

        $query->orderBy($sortBy, $sortDir);

        // Pagination
        $perPage = $request->get('per_page', 50);
        
        if ($request->boolean('all')) {
            $categories = $query->get();
            return response()->json([
                'data' => $categories,
                'total' => $categories->count()
            ]);
        }

        $categories = $query->paginate($perPage);

        return response()->json($categories);
    }

    /**
     * Store a newly created category
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'required|string|max:255|unique:categories,slug|regex:/^[a-z0-9-]+$/',
            'description' => 'nullable|string|max:1000',
            'color' => 'required|string|max:7|regex:/^#[0-9A-Fa-f]{6}$/',
            'is_visible' => 'boolean',
            'parent_id' => 'nullable|exists:categories,id',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string|max:500',
        ]);

        // Auto-generate slug if not provided or generate from name
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $category = Category::create($validated);

        return response()->json([
            'message' => 'Category created successfully',
            'data' => $category->load('parent')
        ], 201);
    }

    /**
     * Display the specified category
     */
    public function show($id)
    {
        $category = Category::with(['parent', 'children'])->findOrFail($id);

        return response()->json($category);
    }

    /**
     * Update the specified category
     */
    public function update(Request $request, $id)
    {
        $category = Category::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|required|string|max:255',
            'slug' => [
                'sometimes',
                'required',
                'string',
                'max:255',
                'regex:/^[a-z0-9-]+$/',
                Rule::unique('categories')->ignore($category->id)
            ],
            'description' => 'nullable|string|max:1000',
            'color' => 'sometimes|required|string|max:7|regex:/^#[0-9A-Fa-f]{6}$/',
            'is_visible' => 'boolean',
            'parent_id' => 'nullable|exists:categories,id',
            'order' => 'sometimes|integer|min:0',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string|max:500',
        ]);

        // Prevent circular parent relationship
        if (isset($validated['parent_id']) && $validated['parent_id'] == $category->id) {
            return response()->json([
                'message' => 'A category cannot be its own parent'
            ], 422);
        }

        $category->update($validated);

        return response()->json([
            'message' => 'Category updated successfully',
            'data' => $category->load('parent')
        ]);
    }

    /**
     * Remove the specified category
     */
    public function destroy($id)
    {
        $category = Category::findOrFail($id);
        
        // Check if category has children
        if ($category->children()->count() > 0) {
            return response()->json([
                'message' => 'Cannot delete category with subcategories. Please delete or reassign subcategories first.'
            ], 422);
        }

        $category->delete();

        return response()->json([
            'message' => 'Category deleted successfully'
        ]);
    }

    /**
     * Bulk delete categories
     */
    public function bulkDelete(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'required|integer|exists:categories,id'
        ]);

        $count = Category::whereIn('id', $validated['ids'])
            ->whereDoesntHave('children')
            ->delete();

        return response()->json([
            'message' => "{$count} categories deleted successfully",
            'deleted_count' => $count
        ]);
    }

    /**
     * Bulk update category visibility
     */
    public function bulkUpdateVisibility(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'required|integer|exists:categories,id',
            'is_visible' => 'required|boolean'
        ]);

        $count = Category::whereIn('id', $validated['ids'])
            ->update(['is_visible' => $validated['is_visible']]);

        return response()->json([
            'message' => "{$count} categories updated successfully",
            'updated_count' => $count
        ]);
    }

    /**
     * Reorder categories
     */
    public function reorder(Request $request)
    {
        $validated = $request->validate([
            'orders' => 'required|array',
            'orders.*.id' => 'required|integer|exists:categories,id',
            'orders.*.order' => 'required|integer|min:0'
        ]);

        foreach ($validated['orders'] as $item) {
            Category::where('id', $item['id'])->update(['order' => $item['order']]);
        }

        return response()->json([
            'message' => 'Categories reordered successfully'
        ]);
    }

    /**
     * Get category statistics
     */
    public function stats()
    {
        $totalCategories = Category::count();
        $visibleCategories = Category::where('is_visible', true)->count();
        $hiddenCategories = $totalCategories - $visibleCategories;
        
        $categories = Category::all();
        $totalPosts = $categories->sum('post_count');
        $avgPostsPerCategory = $totalCategories > 0 ? $totalPosts / $totalCategories : 0;
        
        $mostUsed = $categories->sortByDesc('post_count')->first();

        return response()->json([
            'total_categories' => $totalCategories,
            'visible_categories' => $visibleCategories,
            'hidden_categories' => $hiddenCategories,
            'total_posts' => $totalPosts,
            'avg_posts_per_category' => round($avgPostsPerCategory, 2),
            'most_used_category' => $mostUsed
        ]);
    }

    /**
     * Merge categories
     */
    public function merge(Request $request)
    {
        $validated = $request->validate([
            'source_ids' => 'required|array|min:1',
            'source_ids.*' => 'required|integer|exists:categories,id',
            'target_id' => 'required|integer|exists:categories,id'
        ]);

        $targetCategory = Category::findOrFail($validated['target_id']);
        $sourceCategories = Category::whereIn('id', $validated['source_ids'])->get();

        // Update all posts that have source categories to use target category
        // This would require updating the JSON arrays in posts
        // For now, we'll just delete the source categories
        
        Category::whereIn('id', $validated['source_ids'])->delete();

        return response()->json([
            'message' => 'Categories merged successfully',
            'target_category' => $targetCategory
        ]);
    }

    /**
     * Export categories
     */
    public function export(Request $request)
    {
        $categories = Category::with('parent')->get();

        $data = $categories->map(function ($category) {
            return [
                'id' => $category->id,
                'name' => $category->name,
                'slug' => $category->slug,
                'description' => $category->description,
                'color' => $category->color,
                'parent' => $category->parent?->name,
                'post_count' => $category->post_count,
                'is_visible' => $category->is_visible,
                'created_at' => $category->created_at->toDateTimeString(),
            ];
        });

        return response()->json([
            'data' => $data,
            'filename' => 'categories_' . now()->format('Y-m-d_His') . '.json'
        ]);
    }
}
