<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\ContactList;
use App\Models\Contact;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;

/**
 * Contact List Controller (FluentCRM Mailing Lists)
 *
 * December 2025 Laravel 12 syntax.
 */
class ContactListController extends Controller
{
    public function index(Request $request): JsonResponse
    {
        $query = ContactList::query();

        if ($request->filled('search')) {
            $query->where('title', 'like', "%{$request->search}%");
        }

        if ($request->has('is_public')) {
            $query->where('is_public', $request->boolean('is_public'));
        }

        $lists = $query->with('creator')
            ->orderByDesc('created_at')
            ->paginate($request->integer('per_page', 25));

        return response()->json($lists);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'is_public' => 'nullable|boolean',
        ]);

        $list = ContactList::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json($list, 201);
    }

    public function show(string $id): JsonResponse
    {
        $list = ContactList::with('creator')->findOrFail($id);

        return response()->json([
            'list' => $list,
            'contacts_count' => $list->subscribedContacts()->count(),
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $list = ContactList::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'description' => 'nullable|string|max:1000',
            'is_public' => 'sometimes|boolean',
        ]);

        $list->update($validated);

        return response()->json($list->fresh());
    }

    public function destroy(string $id): JsonResponse
    {
        $list = ContactList::findOrFail($id);
        $list->delete();

        return response()->json(['message' => 'List deleted']);
    }

    public function getContacts(Request $request, string $id): JsonResponse
    {
        $list = ContactList::findOrFail($id);

        $contacts = $list->subscribedContacts()
            ->orderByPivot('subscribed_at', 'desc')
            ->paginate($request->integer('per_page', 25));

        return response()->json($contacts);
    }

    public function addContacts(Request $request, string $id): JsonResponse
    {
        $list = ContactList::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $added = 0;
        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            if ($contact && !$list->hasContact($contact)) {
                $list->addContact($contact);
                $added++;
            }
        }

        return response()->json([
            'message' => "Added {$added} contacts to list",
            'added_count' => $added,
        ]);
    }

    public function removeContacts(Request $request, string $id): JsonResponse
    {
        $list = ContactList::findOrFail($id);

        $validated = $request->validate([
            'contact_ids' => 'required|array',
            'contact_ids.*' => 'uuid|exists:contacts,id',
        ]);

        $removed = 0;
        foreach ($validated['contact_ids'] as $contactId) {
            $contact = Contact::find($contactId);
            if ($contact && $list->hasContact($contact)) {
                $list->removeContact($contact);
                $removed++;
            }
        }

        return response()->json([
            'message' => "Removed {$removed} contacts from list",
            'removed_count' => $removed,
        ]);
    }
}
