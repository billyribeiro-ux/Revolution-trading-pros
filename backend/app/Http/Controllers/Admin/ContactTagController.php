<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\ContactTag;
use App\Models\Contact;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;

/**
 * Contact Tag Controller (FluentCRM Tags)
 *
 * December 2025 Laravel 12 syntax.
 */
class ContactTagController extends Controller
{
    public function index(Request $request): JsonResponse
    {
        $query = ContactTag::query();

        if ($request->filled('search')) {
            $query->where('title', 'like', "%{$request->search}%");
        }

        $tags = $query->with('creator')
            ->orderBy('title')
            ->paginate($request->integer('per_page', 50));

        return response()->json($tags);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'color' => 'nullable|string|regex:/^#[0-9A-Fa-f]{6}$/',
        ]);

        $tag = ContactTag::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json($tag, 201);
    }

    public function show(string $id): JsonResponse
    {
        $tag = ContactTag::with('creator')->findOrFail($id);

        return response()->json([
            'tag' => $tag,
            'contacts_count' => $tag->contacts()->count(),
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $tag = ContactTag::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'description' => 'nullable|string|max:1000',
            'color' => 'nullable|string|regex:/^#[0-9A-Fa-f]{6}$/',
        ]);

        $tag->update($validated);

        return response()->json($tag->fresh());
    }

    public function destroy(string $id): JsonResponse
    {
        $tag = ContactTag::findOrFail($id);
        $tag->delete();

        return response()->json(['message' => 'Tag deleted']);
    }

    public function getContacts(Request $request, string $id): JsonResponse
    {
        $tag = ContactTag::findOrFail($id);

        $contacts = $tag->contacts()
            ->orderByPivot('applied_at', 'desc')
            ->paginate($request->integer('per_page', 25));

        return response()->json($contacts);
    }

    public function applyToContacts(Request $request, string $id): JsonResponse
    {
        $tag = ContactTag::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $applied = 0;
        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            if ($contact && $tag->applyToContact($contact, auth()->id())) {
                $applied++;
            }
        }

        return response()->json([
            'message' => "Applied tag to {$applied} contacts",
            'applied_count' => $applied,
        ]);
    }

    public function removeFromContacts(Request $request, string $id): JsonResponse
    {
        $tag = ContactTag::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $removed = 0;
        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            if ($contact && $tag->removeFromContact($contact)) {
                $removed++;
            }
        }

        return response()->json([
            'message' => "Removed tag from {$removed} contacts",
            'removed_count' => $removed,
        ]);
    }

    public function bulkApply(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'tag_ids' => 'required|array',
            'tag_ids.*' => 'uuid|exists:contact_tags,id',
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $applied = 0;
        foreach ($validated['tag_ids'] as $tagId) {
            $tag = ContactTag::find($tagId);
            foreach ($validated['contact_ids'] as $contactId) {
                $contact = Contact::find($contactId);
                if ($tag && $contact && $tag->applyToContact($contact, auth()->id())) {
                    $applied++;
                }
            }
        }

        return response()->json([
            'message' => "Applied {$applied} tag-contact combinations",
            'applied_count' => $applied,
        ]);
    }
}
