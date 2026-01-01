<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Enums\ConversationPriority;
use App\Enums\ConversationStatus;
use App\Http\Controllers\Controller;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use App\Services\Email\EmailAttachmentService;
use App\Services\Email\EmailThreadingService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Validation\Rule;

/**
 * EmailConversationController
 *
 * Admin controller for managing inbound email conversations.
 * Provides full CRUD and conversation management operations.
 *
 * @version 1.0.0
 */
class EmailConversationController extends Controller
{
    public function __construct(
        private readonly EmailThreadingService $threadingService,
        private readonly EmailAttachmentService $attachmentService,
    ) {}

    /**
     * List all conversations with filtering and pagination.
     *
     * @route GET /api/admin/email/conversations
     */
    public function index(Request $request): JsonResponse
    {
        $query = EmailConversation::with(['contact:id,email,first_name,last_name', 'assignee:id,name,email'])
            ->withCount(['messages', 'messages as unread_messages_count' => function ($q) {
                $q->where('is_read', false);
            }]);

        // Filter by status
        if ($request->has('status')) {
            $status = $request->input('status');
            if ($status === 'active') {
                $query->active();
            } elseif ($status === 'spam') {
                $query->spam();
            } else {
                $query->where('status', $status);
            }
        }

        // Filter by priority
        if ($request->has('priority')) {
            $query->priority(ConversationPriority::from($request->input('priority')));
        }

        // Filter by contact
        if ($request->has('contact_id')) {
            $query->forContact($request->integer('contact_id'));
        }

        // Filter by assignee
        if ($request->has('assigned_to')) {
            if ($request->input('assigned_to') === 'unassigned') {
                $query->unassigned();
            } else {
                $query->assignedTo($request->integer('assigned_to'));
            }
        }

        // Filter by starred
        if ($request->boolean('starred')) {
            $query->starred();
        }

        // Filter by unread
        if ($request->boolean('unread')) {
            $query->withUnread();
        }

        // Filter by tags
        if ($request->has('tags')) {
            $tags = is_array($request->input('tags'))
                ? $request->input('tags')
                : explode(',', $request->input('tags'));

            $query->where(function ($q) use ($tags) {
                foreach ($tags as $tag) {
                    $q->orWhereJsonContains('tags', $tag);
                }
            });
        }

        // Search by subject or contact email
        if ($request->has('search')) {
            $search = $request->input('search');
            $query->where(function ($q) use ($search) {
                $q->where('subject', 'ilike', "%{$search}%")
                    ->orWhereHas('contact', function ($q2) use ($search) {
                        $q2->where('email', 'ilike', "%{$search}%")
                            ->orWhere('first_name', 'ilike', "%{$search}%")
                            ->orWhere('last_name', 'ilike', "%{$search}%");
                    });
            });
        }

        // Date range filter
        if ($request->has('from_date')) {
            $query->where('created_at', '>=', $request->input('from_date'));
        }
        if ($request->has('to_date')) {
            $query->where('created_at', '<=', $request->input('to_date'));
        }

        // Sorting
        $sortBy = $request->input('sort_by', 'latest_message_at');
        $sortDir = $request->input('sort_dir', 'desc');
        $query->orderBy($sortBy, $sortDir);

        // Pagination
        $perPage = min($request->integer('per_page', 20), 100);
        $conversations = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $conversations,
        ]);
    }

    /**
     * Get conversation statistics.
     *
     * @route GET /api/admin/email/conversations/stats
     */
    public function stats(): JsonResponse
    {
        $stats = [
            'total' => EmailConversation::count(),
            'by_status' => [
                'open' => EmailConversation::open()->count(),
                'pending' => EmailConversation::pending()->count(),
                'resolved' => EmailConversation::resolved()->count(),
                'archived' => EmailConversation::where('status', ConversationStatus::ARCHIVED)->count(),
                'spam' => EmailConversation::spam()->count(),
            ],
            'by_priority' => [
                'urgent' => EmailConversation::priority(ConversationPriority::URGENT)->count(),
                'high' => EmailConversation::priority(ConversationPriority::HIGH)->count(),
                'normal' => EmailConversation::priority(ConversationPriority::NORMAL)->count(),
                'low' => EmailConversation::priority(ConversationPriority::LOW)->count(),
            ],
            'unassigned' => EmailConversation::unassigned()->active()->count(),
            'with_unread' => EmailConversation::withUnread()->count(),
            'starred' => EmailConversation::starred()->count(),
            'avg_response_time_hours' => round(
                EmailConversation::whereNotNull('response_time_seconds')
                    ->avg('response_time_seconds') / 3600,
                2
            ),
            'messages_today' => EmailMessage::whereDate('created_at', today())->count(),
            'messages_this_week' => EmailMessage::where('created_at', '>=', now()->startOfWeek())->count(),
        ];

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get a single conversation with all messages.
     *
     * @route GET /api/admin/email/conversations/{id}
     */
    public function show(string $id): JsonResponse
    {
        $conversation = EmailConversation::with([
            'contact',
            'assignee:id,name,email',
            'creator:id,name,email',
            'messages' => function ($q) {
                $q->with('attachments')
                    ->orderBy('created_at', 'asc');
            },
        ])->findOrFail($id);

        return response()->json([
            'success' => true,
            'data' => $conversation,
        ]);
    }

    /**
     * Update conversation details.
     *
     * @route PUT /api/admin/email/conversations/{id}
     */
    public function update(Request $request, string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);

        $validated = $request->validate([
            'status' => ['sometimes', Rule::enum(ConversationStatus::class)],
            'priority' => ['sometimes', Rule::enum(ConversationPriority::class)],
            'assigned_to' => ['sometimes', 'nullable', 'exists:users,id'],
            'tags' => ['sometimes', 'array'],
            'tags.*' => ['string', 'max:50'],
            'is_starred' => ['sometimes', 'boolean'],
            'is_muted' => ['sometimes', 'boolean'],
        ]);

        $conversation->update($validated);

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(['contact', 'assignee']),
            'message' => 'Conversation updated successfully',
        ]);
    }

    /**
     * Delete a conversation (soft delete).
     *
     * @route DELETE /api/admin/email/conversations/{id}
     */
    public function destroy(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->delete();

        return response()->json([
            'success' => true,
            'message' => 'Conversation deleted successfully',
        ]);
    }

    /**
     * Assign conversation to a user.
     *
     * @route POST /api/admin/email/conversations/{id}/assign
     */
    public function assign(Request $request, string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);

        $validated = $request->validate([
            'user_id' => ['nullable', 'exists:users,id'],
        ]);

        $conversation->assignTo($validated['user_id'] ?? null);

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(['assignee']),
            'message' => $validated['user_id']
                ? 'Conversation assigned successfully'
                : 'Conversation unassigned successfully',
        ]);
    }

    /**
     * Mark conversation as resolved.
     *
     * @route POST /api/admin/email/conversations/{id}/resolve
     */
    public function resolve(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->resolve();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Conversation marked as resolved',
        ]);
    }

    /**
     * Reopen a resolved conversation.
     *
     * @route POST /api/admin/email/conversations/{id}/reopen
     */
    public function reopen(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->reopen();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Conversation reopened',
        ]);
    }

    /**
     * Mark conversation as spam.
     *
     * @route POST /api/admin/email/conversations/{id}/spam
     */
    public function markAsSpam(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->markAsSpam();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Conversation marked as spam',
        ]);
    }

    /**
     * Mark conversation as not spam.
     *
     * @route POST /api/admin/email/conversations/{id}/not-spam
     */
    public function markAsNotSpam(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->reopen();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Conversation marked as not spam',
        ]);
    }

    /**
     * Archive conversation.
     *
     * @route POST /api/admin/email/conversations/{id}/archive
     */
    public function archive(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->archive();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Conversation archived',
        ]);
    }

    /**
     * Toggle starred status.
     *
     * @route POST /api/admin/email/conversations/{id}/star
     */
    public function toggleStar(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->toggleStar();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => $conversation->is_starred
                ? 'Conversation starred'
                : 'Conversation unstarred',
        ]);
    }

    /**
     * Mark all messages in conversation as read.
     *
     * @route POST /api/admin/email/conversations/{id}/read
     */
    public function markAsRead(string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->markAsRead();

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'All messages marked as read',
        ]);
    }

    /**
     * Add tags to conversation.
     *
     * @route POST /api/admin/email/conversations/{id}/tags
     */
    public function addTags(Request $request, string $id): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);

        $validated = $request->validate([
            'tags' => ['required', 'array'],
            'tags.*' => ['string', 'max:50'],
        ]);

        $conversation->addTags($validated['tags']);

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Tags added successfully',
        ]);
    }

    /**
     * Remove a tag from conversation.
     *
     * @route DELETE /api/admin/email/conversations/{id}/tags/{tag}
     */
    public function removeTag(string $id, string $tag): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($id);
        $conversation->removeTag($tag);

        return response()->json([
            'success' => true,
            'data' => $conversation->fresh(),
            'message' => 'Tag removed successfully',
        ]);
    }

    /**
     * Get all unique tags used in conversations.
     *
     * @route GET /api/admin/email/conversations/tags
     */
    public function allTags(): JsonResponse
    {
        $tags = DB::table('email_conversations')
            ->whereNotNull('tags')
            ->pluck('tags')
            ->flatten()
            ->unique()
            ->values();

        return response()->json([
            'success' => true,
            'data' => $tags,
        ]);
    }

    /**
     * Merge two conversations.
     *
     * @route POST /api/admin/email/conversations/{id}/merge
     */
    public function merge(Request $request, string $id): JsonResponse
    {
        $validated = $request->validate([
            'source_id' => ['required', 'uuid', 'exists:email_conversations,id'],
        ]);

        $target = EmailConversation::findOrFail($id);
        $source = EmailConversation::findOrFail($validated['source_id']);

        if ($target->id === $source->id) {
            return response()->json([
                'success' => false,
                'error' => 'Cannot merge conversation with itself',
            ], 422);
        }

        $merged = $this->threadingService->mergeConversations($target, $source);

        return response()->json([
            'success' => true,
            'data' => $merged->fresh(['contact', 'messages']),
            'message' => 'Conversations merged successfully',
        ]);
    }

    /**
     * Bulk update conversations.
     *
     * @route POST /api/admin/email/conversations/bulk-update
     */
    public function bulkUpdate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'ids' => ['required', 'array'],
            'ids.*' => ['uuid'],
            'action' => ['required', Rule::in([
                'resolve', 'archive', 'spam', 'delete',
                'assign', 'unassign', 'star', 'unstar',
                'mark_read', 'set_priority',
            ])],
            'assigned_to' => ['required_if:action,assign', 'exists:users,id'],
            'priority' => ['required_if:action,set_priority', Rule::enum(ConversationPriority::class)],
        ]);

        $count = 0;

        DB::transaction(function () use ($validated, &$count) {
            $conversations = EmailConversation::whereIn('id', $validated['ids'])->get();

            foreach ($conversations as $conversation) {
                switch ($validated['action']) {
                    case 'resolve':
                        $conversation->resolve();
                        break;
                    case 'archive':
                        $conversation->archive();
                        break;
                    case 'spam':
                        $conversation->markAsSpam();
                        break;
                    case 'delete':
                        $conversation->delete();
                        break;
                    case 'assign':
                        $conversation->assignTo($validated['assigned_to']);
                        break;
                    case 'unassign':
                        $conversation->assignTo(null);
                        break;
                    case 'star':
                        $conversation->update(['is_starred' => true]);
                        break;
                    case 'unstar':
                        $conversation->update(['is_starred' => false]);
                        break;
                    case 'mark_read':
                        $conversation->markAsRead();
                        break;
                    case 'set_priority':
                        $conversation->setPriority(ConversationPriority::from($validated['priority']));
                        break;
                }
                $count++;
            }
        });

        return response()->json([
            'success' => true,
            'message' => "{$count} conversations updated successfully",
        ]);
    }

    /**
     * Get attachment download URL.
     *
     * @route GET /api/admin/email/conversations/{conversationId}/attachments/{attachmentId}/download
     */
    public function downloadAttachment(string $conversationId, string $attachmentId): JsonResponse
    {
        $conversation = EmailConversation::findOrFail($conversationId);

        $attachment = $conversation->messages()
            ->join('email_attachments', 'email_messages.id', '=', 'email_attachments.message_id')
            ->where('email_attachments.id', $attachmentId)
            ->select('email_attachments.*')
            ->firstOrFail();

        $url = $this->attachmentService->getDownloadUrl($attachment);

        if (!$url) {
            return response()->json([
                'success' => false,
                'error' => 'Attachment not available for download',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'url' => $url,
                'filename' => $attachment->original_filename,
                'expires_in' => 3600, // 1 hour
            ],
        ]);
    }
}
