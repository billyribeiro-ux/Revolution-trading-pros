<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\CrmCompany;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * CRM Company Controller (B2B Account Management)
 *
 * December 2025 Laravel 12 syntax.
 */
class CrmCompanyController extends Controller
{
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'name', 'industry', 'contacts_count', 'total_deal_value'
    ];

    public function index(Request $request): JsonResponse
    {
        $query = CrmCompany::query();

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('website', 'like', "%{$search}%")
                  ->orWhere('email', 'like', "%{$search}%");
            });
        }

        if ($request->filled('industry')) {
            $query->where('industry', $request->industry);
        }

        if ($request->filled('size')) {
            $query->where('size', $request->size);
        }

        if ($request->filled('owner_id')) {
            $query->where('owner_id', $request->owner_id);
        }

        $sortBy = in_array($request->get('sort_by'), self::ALLOWED_SORT_COLUMNS, true)
            ? $request->get('sort_by')
            : 'created_at';
        $sortOrder = in_array(strtolower($request->get('sort_order', 'desc')), ['asc', 'desc'], true)
            ? $request->get('sort_order', 'desc')
            : 'desc';

        $companies = $query->with('owner')
            ->orderBy($sortBy, $sortOrder)
            ->paginate($request->integer('per_page', 25));

        return response()->json($companies);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'website' => 'nullable|url|max:500',
            'industry' => ['nullable', Rule::in(array_keys(CrmCompany::INDUSTRIES))],
            'size' => ['nullable', Rule::in(array_keys(CrmCompany::SIZES))],
            'annual_revenue' => 'nullable|numeric|min:0',
            'phone' => 'nullable|string|max:50',
            'email' => 'nullable|email|max:255',
            'description' => 'nullable|string|max:2000',
            'address_line1' => 'nullable|string|max:255',
            'address_line2' => 'nullable|string|max:255',
            'city' => 'nullable|string|max:100',
            'state' => 'nullable|string|max:100',
            'postal_code' => 'nullable|string|max:20',
            'country' => 'nullable|string|max:100',
            'linkedin_url' => 'nullable|url|max:500',
            'twitter_handle' => 'nullable|string|max:100',
            'logo_url' => 'nullable|url|max:500',
            'custom_fields' => 'nullable|array',
            'owner_id' => 'nullable|uuid|exists:users,id',
        ]);

        $company = CrmCompany::create([
            'id' => Str::uuid(),
            ...$validated,
        ]);

        return response()->json($company->load('owner'), 201);
    }

    public function show(string $id): JsonResponse
    {
        $company = CrmCompany::with([
            'owner',
            'contacts' => fn ($q) => $q->limit(10)->orderByDesc('lead_score'),
            'deals' => fn ($q) => $q->where('status', 'open')->orderByDesc('amount'),
        ])->findOrFail($id);

        $stats = $company->getStats();

        return response()->json([
            'company' => $company,
            'stats' => $stats,
            'industries' => CrmCompany::INDUSTRIES,
            'sizes' => CrmCompany::SIZES,
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $company = CrmCompany::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'website' => 'nullable|url|max:500',
            'industry' => ['nullable', Rule::in(array_keys(CrmCompany::INDUSTRIES))],
            'size' => ['nullable', Rule::in(array_keys(CrmCompany::SIZES))],
            'annual_revenue' => 'nullable|numeric|min:0',
            'phone' => 'nullable|string|max:50',
            'email' => 'nullable|email|max:255',
            'description' => 'nullable|string|max:2000',
            'address_line1' => 'nullable|string|max:255',
            'address_line2' => 'nullable|string|max:255',
            'city' => 'nullable|string|max:100',
            'state' => 'nullable|string|max:100',
            'postal_code' => 'nullable|string|max:20',
            'country' => 'nullable|string|max:100',
            'linkedin_url' => 'nullable|url|max:500',
            'twitter_handle' => 'nullable|string|max:100',
            'logo_url' => 'nullable|url|max:500',
            'custom_fields' => 'nullable|array',
            'owner_id' => 'nullable|uuid|exists:users,id',
        ]);

        $company->update($validated);

        return response()->json($company->fresh('owner'));
    }

    public function destroy(string $id): JsonResponse
    {
        $company = CrmCompany::findOrFail($id);
        $company->delete();

        return response()->json(['message' => 'Company deleted']);
    }

    public function getContacts(Request $request, string $id): JsonResponse
    {
        $company = CrmCompany::findOrFail($id);

        $contacts = $company->contacts()
            ->orderByDesc('lead_score')
            ->paginate($request->integer('per_page', 25));

        return response()->json($contacts);
    }

    public function getDeals(Request $request, string $id): JsonResponse
    {
        $company = CrmCompany::findOrFail($id);

        $query = $company->deals();

        if ($request->filled('status')) {
            $query->where('status', $request->status);
        }

        $deals = $query->with(['pipeline', 'stage'])
            ->orderByDesc('created_at')
            ->paginate($request->integer('per_page', 25));

        return response()->json($deals);
    }

    public function getIndustries(): JsonResponse
    {
        return response()->json(CrmCompany::INDUSTRIES);
    }

    public function getSizes(): JsonResponse
    {
        return response()->json(CrmCompany::SIZES);
    }
}
