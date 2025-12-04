<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Real-Time Collaboration Service
 *
 * Implements Sanity-style collaborative editing features:
 * - Document presence (who's viewing/editing)
 * - Cursor positions and selections
 * - Live mutations with operational transform
 * - Conflict resolution
 * - Session management
 *
 * Uses Redis for real-time state and WebSocket for broadcast.
 */
class RealTimeCollaborationService
{
    private const PRESENCE_TTL = 30; // seconds
    private const SESSION_TTL = 3600; // 1 hour
    private const MUTATION_BUFFER_SIZE = 100;

    /**
     * Join a document editing session
     */
    public function joinDocument(string $documentId, int $userId, array $metadata = []): array
    {
        $sessionId = Str::uuid()->toString();
        $now = now();

        $session = [
            'session_id' => $sessionId,
            'document_id' => $documentId,
            'user_id' => $userId,
            'user_name' => $metadata['name'] ?? "User {$userId}",
            'user_color' => $metadata['color'] ?? $this->generateUserColor($userId),
            'user_avatar' => $metadata['avatar'] ?? null,
            'cursor' => null,
            'selection' => null,
            'last_activity' => $now->toIso8601String(),
            'joined_at' => $now->toIso8601String(),
            'status' => 'active',
        ];

        // Store session
        Cache::put(
            $this->sessionKey($sessionId),
            $session,
            self::SESSION_TTL
        );

        // Add to document's active sessions
        $this->addToDocumentSessions($documentId, $sessionId);

        // Record presence
        $this->updatePresence($sessionId, $documentId);

        // Get current document state
        $documentState = $this->getDocumentState($documentId);

        return [
            'sessionId' => $sessionId,
            'documentId' => $documentId,
            'userId' => $userId,
            'color' => $session['user_color'],
            'collaborators' => $this->getActiveCollaborators($documentId, $sessionId),
            'documentState' => $documentState,
            'serverTime' => $now->toIso8601String(),
        ];
    }

    /**
     * Leave a document editing session
     */
    public function leaveDocument(string $sessionId): bool
    {
        $session = Cache::get($this->sessionKey($sessionId));

        if (!$session) {
            return false;
        }

        // Remove from document sessions
        $this->removeFromDocumentSessions($session['document_id'], $sessionId);

        // Delete session
        Cache::forget($this->sessionKey($sessionId));

        // Broadcast departure
        $this->broadcastPresenceChange($session['document_id'], [
            'type' => 'leave',
            'sessionId' => $sessionId,
            'userId' => $session['user_id'],
        ]);

        return true;
    }

    /**
     * Update cursor position
     */
    public function updateCursor(string $sessionId, array $cursor): array
    {
        $session = Cache::get($this->sessionKey($sessionId));

        if (!$session) {
            throw new \InvalidArgumentException('Session not found');
        }

        $session['cursor'] = [
            'path' => $cursor['path'] ?? [],
            'offset' => $cursor['offset'] ?? 0,
            'timestamp' => now()->toIso8601String(),
        ];
        $session['last_activity'] = now()->toIso8601String();

        Cache::put($this->sessionKey($sessionId), $session, self::SESSION_TTL);
        $this->updatePresence($sessionId, $session['document_id']);

        // Broadcast cursor update
        $this->broadcastCursorUpdate($session['document_id'], $sessionId, $session);

        return $session['cursor'];
    }

    /**
     * Update selection
     */
    public function updateSelection(string $sessionId, array $selection): array
    {
        $session = Cache::get($this->sessionKey($sessionId));

        if (!$session) {
            throw new \InvalidArgumentException('Session not found');
        }

        $session['selection'] = [
            'anchor' => $selection['anchor'] ?? null,
            'focus' => $selection['focus'] ?? null,
            'path' => $selection['path'] ?? [],
            'timestamp' => now()->toIso8601String(),
        ];
        $session['last_activity'] = now()->toIso8601String();

        Cache::put($this->sessionKey($sessionId), $session, self::SESSION_TTL);

        // Broadcast selection update
        $this->broadcastSelectionUpdate($session['document_id'], $sessionId, $session);

        return $session['selection'];
    }

    /**
     * Submit a mutation (operational transform)
     */
    public function submitMutation(string $sessionId, array $mutation): array
    {
        $session = Cache::get($this->sessionKey($sessionId));

        if (!$session) {
            throw new \InvalidArgumentException('Session not found');
        }

        $documentId = $session['document_id'];

        // Lock document for atomic operation
        $lock = Cache::lock("mutation_lock:{$documentId}", 5);

        try {
            $lock->block(5);

            // Get current document version
            $currentVersion = $this->getDocumentVersion($documentId);

            // Validate mutation base version
            $baseVersion = $mutation['baseVersion'] ?? 0;

            if ($baseVersion < $currentVersion) {
                // Need to transform against intervening operations
                $mutation = $this->transformMutation($documentId, $mutation, $baseVersion);
            }

            // Apply mutation
            $result = $this->applyMutation($documentId, $mutation, $session);

            // Increment version
            $newVersion = $currentVersion + 1;
            $this->setDocumentVersion($documentId, $newVersion);

            // Store mutation in history
            $this->recordMutation($documentId, $mutation, $session, $newVersion);

            // Broadcast mutation
            $this->broadcastMutation($documentId, $sessionId, $mutation, $newVersion);

            return [
                'success' => true,
                'version' => $newVersion,
                'mutationId' => $mutation['id'] ?? Str::uuid()->toString(),
                'timestamp' => now()->toIso8601String(),
                'result' => $result,
            ];
        } finally {
            $lock->release();
        }
    }

    /**
     * Get active collaborators on a document
     */
    public function getActiveCollaborators(string $documentId, ?string $excludeSessionId = null): array
    {
        $sessionIds = $this->getDocumentSessions($documentId);
        $collaborators = [];

        foreach ($sessionIds as $sessionId) {
            if ($sessionId === $excludeSessionId) {
                continue;
            }

            $session = Cache::get($this->sessionKey($sessionId));

            if ($session && $this->isSessionActive($session)) {
                $collaborators[] = [
                    'sessionId' => $sessionId,
                    'userId' => $session['user_id'],
                    'name' => $session['user_name'],
                    'color' => $session['user_color'],
                    'avatar' => $session['user_avatar'],
                    'cursor' => $session['cursor'],
                    'selection' => $session['selection'],
                    'lastActivity' => $session['last_activity'],
                ];
            }
        }

        return $collaborators;
    }

    /**
     * Heartbeat to maintain presence
     */
    public function heartbeat(string $sessionId): array
    {
        $session = Cache::get($this->sessionKey($sessionId));

        if (!$session) {
            throw new \InvalidArgumentException('Session not found or expired');
        }

        $session['last_activity'] = now()->toIso8601String();
        Cache::put($this->sessionKey($sessionId), $session, self::SESSION_TTL);
        $this->updatePresence($sessionId, $session['document_id']);

        return [
            'status' => 'alive',
            'serverTime' => now()->toIso8601String(),
            'collaborators' => count($this->getActiveCollaborators($session['document_id'])),
        ];
    }

    /**
     * Get document mutation history for sync
     */
    public function getMutationsSince(string $documentId, int $sinceVersion): array
    {
        $mutations = Cache::get($this->mutationHistoryKey($documentId), []);

        return array_filter($mutations, fn($m) => $m['version'] > $sinceVersion);
    }

    /**
     * Resolve conflicts between mutations
     */
    public function resolveConflict(string $documentId, array $localMutation, array $remoteMutation): array
    {
        // Operational Transform (OT) conflict resolution
        // Using Last-Writer-Wins with path-based conflict detection

        $localPath = $localMutation['path'] ?? [];
        $remotePath = $remoteMutation['path'] ?? [];

        // Check if paths conflict
        if ($this->pathsConflict($localPath, $remotePath)) {
            // Transform local mutation against remote
            return $this->transformAgainst($localMutation, $remoteMutation);
        }

        // No conflict, both can apply
        return $localMutation;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function sessionKey(string $sessionId): string
    {
        return "collab:session:{$sessionId}";
    }

    private function documentSessionsKey(string $documentId): string
    {
        return "collab:doc_sessions:{$documentId}";
    }

    private function presenceKey(string $documentId): string
    {
        return "collab:presence:{$documentId}";
    }

    private function documentVersionKey(string $documentId): string
    {
        return "collab:version:{$documentId}";
    }

    private function mutationHistoryKey(string $documentId): string
    {
        return "collab:mutations:{$documentId}";
    }

    private function generateUserColor(int $userId): string
    {
        $colors = [
            '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
            '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F',
            '#BB8FCE', '#85C1E9', '#F8B500', '#00CED1',
        ];

        return $colors[$userId % count($colors)];
    }

    private function addToDocumentSessions(string $documentId, string $sessionId): void
    {
        $sessions = Cache::get($this->documentSessionsKey($documentId), []);
        $sessions[] = $sessionId;
        Cache::put($this->documentSessionsKey($documentId), array_unique($sessions), self::SESSION_TTL);
    }

    private function removeFromDocumentSessions(string $documentId, string $sessionId): void
    {
        $sessions = Cache::get($this->documentSessionsKey($documentId), []);
        $sessions = array_filter($sessions, fn($s) => $s !== $sessionId);
        Cache::put($this->documentSessionsKey($documentId), array_values($sessions), self::SESSION_TTL);
    }

    private function getDocumentSessions(string $documentId): array
    {
        return Cache::get($this->documentSessionsKey($documentId), []);
    }

    private function updatePresence(string $sessionId, string $documentId): void
    {
        $presence = Cache::get($this->presenceKey($documentId), []);
        $presence[$sessionId] = now()->timestamp;
        Cache::put($this->presenceKey($documentId), $presence, self::PRESENCE_TTL);
    }

    private function isSessionActive(array $session): bool
    {
        $lastActivity = strtotime($session['last_activity']);
        return (time() - $lastActivity) < self::PRESENCE_TTL;
    }

    private function getDocumentState(string $documentId): array
    {
        // Get from document perspective service or database
        $perspective = DB::table('document_perspectives')
            ->where('document_id', $documentId)
            ->first();

        if (!$perspective) {
            return ['exists' => false];
        }

        return [
            'exists' => true,
            'version' => $this->getDocumentVersion($documentId),
            'draft' => $perspective->draft_content ? json_decode($perspective->draft_content, true) : null,
            'published' => $perspective->published_content ? json_decode($perspective->published_content, true) : null,
            'hasUnpublishedChanges' => (bool) $perspective->has_unpublished_changes,
        ];
    }

    private function getDocumentVersion(string $documentId): int
    {
        return (int) Cache::get($this->documentVersionKey($documentId), 0);
    }

    private function setDocumentVersion(string $documentId, int $version): void
    {
        Cache::put($this->documentVersionKey($documentId), $version, self::SESSION_TTL);
    }

    private function transformMutation(string $documentId, array $mutation, int $baseVersion): array
    {
        $currentVersion = $this->getDocumentVersion($documentId);
        $mutations = $this->getMutationsSince($documentId, $baseVersion);

        foreach ($mutations as $remoteMutation) {
            $mutation = $this->transformAgainst($mutation, $remoteMutation);
        }

        $mutation['baseVersion'] = $currentVersion;
        return $mutation;
    }

    private function transformAgainst(array $local, array $remote): array
    {
        $localOp = $local['operation'] ?? 'set';
        $remoteOp = $remote['operation'] ?? 'set';
        $localPath = $local['path'] ?? [];
        $remotePath = $remote['path'] ?? [];

        // If both are setting the same path, keep local but note conflict
        if ($localPath === $remotePath && $localOp === 'set' && $remoteOp === 'set') {
            $local['_conflict'] = [
                'type' => 'overwrite',
                'remoteValue' => $remote['value'] ?? null,
                'remoteUserId' => $remote['userId'] ?? null,
            ];
            return $local;
        }

        // If remote deleted a parent path, abort local
        if ($remoteOp === 'unset' && $this->isSubPath($localPath, $remotePath)) {
            $local['_aborted'] = true;
            $local['_reason'] = 'parent_deleted';
            return $local;
        }

        // Array index adjustments
        if ($this->isArrayOperation($remote) && $this->isArrayOperation($local)) {
            $local = $this->transformArrayIndices($local, $remote);
        }

        return $local;
    }

    private function pathsConflict(array $path1, array $path2): bool
    {
        $minLength = min(count($path1), count($path2));

        for ($i = 0; $i < $minLength; $i++) {
            if ($path1[$i] !== $path2[$i]) {
                return false;
            }
        }

        return true; // One is prefix of other
    }

    private function isSubPath(array $child, array $parent): bool
    {
        if (count($child) <= count($parent)) {
            return false;
        }

        for ($i = 0; $i < count($parent); $i++) {
            if ($child[$i] !== $parent[$i]) {
                return false;
            }
        }

        return true;
    }

    private function isArrayOperation(array $mutation): bool
    {
        $op = $mutation['operation'] ?? '';
        return in_array($op, ['insert', 'remove', 'move'], true);
    }

    private function transformArrayIndices(array $local, array $remote): array
    {
        // Adjust array indices based on remote insertions/deletions
        $remoteIndex = $remote['index'] ?? 0;
        $localIndex = $local['index'] ?? 0;

        if ($remote['operation'] === 'insert' && $localIndex >= $remoteIndex) {
            $local['index'] = $localIndex + 1;
        } elseif ($remote['operation'] === 'remove' && $localIndex > $remoteIndex) {
            $local['index'] = $localIndex - 1;
        }

        return $local;
    }

    private function applyMutation(string $documentId, array $mutation, array $session): array
    {
        // Store mutation result in document_mutations table
        DB::table('document_mutations')->insert([
            'id' => Str::uuid()->toString(),
            'document_id' => $documentId,
            'user_id' => $session['user_id'],
            'operation' => $mutation['operation'] ?? 'set',
            'path' => json_encode($mutation['path'] ?? []),
            'value' => isset($mutation['value']) ? json_encode($mutation['value']) : null,
            'previous_value' => isset($mutation['previousValue']) ? json_encode($mutation['previousValue']) : null,
            'created_at' => now(),
        ]);

        return [
            'applied' => true,
            'path' => $mutation['path'] ?? [],
            'operation' => $mutation['operation'] ?? 'set',
        ];
    }

    private function recordMutation(string $documentId, array $mutation, array $session, int $version): void
    {
        $mutations = Cache::get($this->mutationHistoryKey($documentId), []);

        $mutations[] = [
            'id' => $mutation['id'] ?? Str::uuid()->toString(),
            'version' => $version,
            'operation' => $mutation['operation'] ?? 'set',
            'path' => $mutation['path'] ?? [],
            'userId' => $session['user_id'],
            'timestamp' => now()->toIso8601String(),
        ];

        // Keep only last N mutations
        if (count($mutations) > self::MUTATION_BUFFER_SIZE) {
            $mutations = array_slice($mutations, -self::MUTATION_BUFFER_SIZE);
        }

        Cache::put($this->mutationHistoryKey($documentId), $mutations, self::SESSION_TTL);
    }

    private function broadcastPresenceChange(string $documentId, array $data): void
    {
        // Broadcast via Laravel Echo/Pusher/WebSocket
        // event(new CollaboratorPresenceChanged($documentId, $data));
    }

    private function broadcastCursorUpdate(string $documentId, string $sessionId, array $session): void
    {
        // event(new CursorMoved($documentId, $sessionId, $session['cursor']));
    }

    private function broadcastSelectionUpdate(string $documentId, string $sessionId, array $session): void
    {
        // event(new SelectionChanged($documentId, $sessionId, $session['selection']));
    }

    private function broadcastMutation(string $documentId, string $sessionId, array $mutation, int $version): void
    {
        // event(new DocumentMutated($documentId, $sessionId, $mutation, $version));
    }
}
