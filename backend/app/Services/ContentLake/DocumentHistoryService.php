<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Document History Service
 *
 * Implements complete document revision history with diffs.
 * Similar to Sanity's History API.
 *
 * Features:
 * - Full revision history
 * - JSON Patch operations
 * - Human-readable diffs
 * - Point-in-time restoration
 * - Change summaries
 * - Author tracking
 */
class DocumentHistoryService
{
    /**
     * Record a change to document history
     */
    public function recordChange(
        string $documentId,
        string $documentType,
        array $content,
        string $action,
        ?int $userId = null,
        ?string $changeSummary = null
    ): string {
        $historyId = Str::uuid()->toString();

        // Get previous revision
        $previousRevision = DB::table('document_history')
            ->where('document_id', $documentId)
            ->orderBy('revision_number', 'desc')
            ->first();

        $revisionNumber = ($previousRevision->revision_number ?? 0) + 1;

        // Calculate patches and diff from previous
        $patches = null;
        $diff = null;

        if ($previousRevision) {
            $previousContent = json_decode($previousRevision->snapshot, true);
            $patches = $this->calculateJsonPatches($previousContent, $content);
            $diff = $this->calculateHumanReadableDiff($previousContent, $content);
        }

        // Auto-generate change summary if not provided
        if (!$changeSummary) {
            $changeSummary = $this->generateChangeSummary($action, $patches ?? [], $diff ?? []);
        }

        // Get user info
        $user = $userId ? DB::table('users')->find($userId) : null;

        DB::table('document_history')->insert([
            'history_id' => $historyId,
            'document_id' => $documentId,
            'document_type' => $documentType,
            'revision_number' => $revisionNumber,
            'snapshot' => json_encode($content),
            'patches' => $patches ? json_encode($patches) : null,
            'diff' => $diff ? json_encode($diff) : null,
            'change_summary' => $changeSummary,
            'action' => $action,
            'user_id' => $userId,
            'user_name' => $user?->name,
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
            'created_at' => now(),
        ]);

        return $historyId;
    }

    /**
     * Get document history
     */
    public function getHistory(
        string $documentId,
        int $limit = 50,
        int $offset = 0
    ): array {
        $history = DB::table('document_history')
            ->where('document_id', $documentId)
            ->orderBy('revision_number', 'desc')
            ->skip($offset)
            ->take($limit)
            ->get();

        $total = DB::table('document_history')
            ->where('document_id', $documentId)
            ->count();

        return [
            'items' => $history->map(fn($item) => [
                'id' => $item->history_id,
                'revision' => $item->revision_number,
                'action' => $item->action,
                'summary' => $item->change_summary,
                'author' => [
                    'id' => $item->user_id,
                    'name' => $item->user_name,
                ],
                'timestamp' => $item->created_at,
                'diff' => $item->diff ? json_decode($item->diff, true) : null,
            ])->toArray(),
            'total' => $total,
            'hasMore' => ($offset + $limit) < $total,
        ];
    }

    /**
     * Get specific revision
     */
    public function getRevision(string $documentId, int $revisionNumber): ?array
    {
        $revision = DB::table('document_history')
            ->where('document_id', $documentId)
            ->where('revision_number', $revisionNumber)
            ->first();

        if (!$revision) {
            return null;
        }

        return [
            'id' => $revision->history_id,
            'documentId' => $revision->document_id,
            'documentType' => $revision->document_type,
            'revision' => $revision->revision_number,
            'content' => json_decode($revision->snapshot, true),
            'action' => $revision->action,
            'summary' => $revision->change_summary,
            'author' => [
                'id' => $revision->user_id,
                'name' => $revision->user_name,
            ],
            'timestamp' => $revision->created_at,
            'patches' => $revision->patches ? json_decode($revision->patches, true) : null,
            'diff' => $revision->diff ? json_decode($revision->diff, true) : null,
        ];
    }

    /**
     * Restore document to specific revision
     */
    public function restore(
        string $documentId,
        int $revisionNumber,
        ?int $userId = null
    ): array {
        $revision = $this->getRevision($documentId, $revisionNumber);

        if (!$revision) {
            throw new \InvalidArgumentException("Revision {$revisionNumber} not found");
        }

        // Record the restore action
        $this->recordChange(
            $documentId,
            $revision['documentType'],
            $revision['content'],
            'restore',
            $userId,
            "Restored to revision #{$revisionNumber}"
        );

        // Update the perspective
        app(DocumentPerspectiveService::class)->updateDraft(
            $documentId,
            $revision['content'],
            $userId
        );

        return $revision;
    }

    /**
     * Compare two revisions
     */
    public function compareRevisions(
        string $documentId,
        int $fromRevision,
        int $toRevision
    ): array {
        $from = $this->getRevision($documentId, $fromRevision);
        $to = $this->getRevision($documentId, $toRevision);

        if (!$from || !$to) {
            throw new \InvalidArgumentException("One or both revisions not found");
        }

        return [
            'from' => [
                'revision' => $fromRevision,
                'timestamp' => $from['timestamp'],
                'author' => $from['author'],
            ],
            'to' => [
                'revision' => $toRevision,
                'timestamp' => $to['timestamp'],
                'author' => $to['author'],
            ],
            'patches' => $this->calculateJsonPatches($from['content'], $to['content']),
            'diff' => $this->calculateHumanReadableDiff($from['content'], $to['content']),
        ];
    }

    /**
     * Get document at specific point in time
     */
    public function getAtTime(string $documentId, string $timestamp): ?array
    {
        $revision = DB::table('document_history')
            ->where('document_id', $documentId)
            ->where('created_at', '<=', $timestamp)
            ->orderBy('revision_number', 'desc')
            ->first();

        if (!$revision) {
            return null;
        }

        return [
            'revision' => $revision->revision_number,
            'timestamp' => $revision->created_at,
            'content' => json_decode($revision->snapshot, true),
        ];
    }

    /**
     * Calculate JSON Patch operations (RFC 6902)
     */
    private function calculateJsonPatches(array $old, array $new, string $path = ''): array
    {
        $patches = [];

        // Find changed and added fields
        foreach ($new as $key => $value) {
            $fieldPath = $path ? "{$path}/{$key}" : "/{$key}";

            if (!array_key_exists($key, $old)) {
                $patches[] = [
                    'op' => 'add',
                    'path' => $fieldPath,
                    'value' => $value,
                ];
            } elseif (is_array($value) && is_array($old[$key])) {
                // Check if both are associative arrays
                if ($this->isAssociativeArray($value) && $this->isAssociativeArray($old[$key])) {
                    $nestedPatches = $this->calculateJsonPatches($old[$key], $value, $fieldPath);
                    $patches = array_merge($patches, $nestedPatches);
                } elseif ($old[$key] !== $value) {
                    $patches[] = [
                        'op' => 'replace',
                        'path' => $fieldPath,
                        'value' => $value,
                    ];
                }
            } elseif ($old[$key] !== $value) {
                $patches[] = [
                    'op' => 'replace',
                    'path' => $fieldPath,
                    'value' => $value,
                ];
            }
        }

        // Find removed fields
        foreach ($old as $key => $value) {
            if (!array_key_exists($key, $new)) {
                $fieldPath = $path ? "{$path}/{$key}" : "/{$key}";
                $patches[] = [
                    'op' => 'remove',
                    'path' => $fieldPath,
                ];
            }
        }

        return $patches;
    }

    /**
     * Calculate human-readable diff
     */
    private function calculateHumanReadableDiff(array $old, array $new, string $path = ''): array
    {
        $diff = [];

        // Find changed and added fields
        foreach ($new as $key => $value) {
            $fieldPath = $path ? "{$path}.{$key}" : $key;

            if (!array_key_exists($key, $old)) {
                $diff[] = [
                    'type' => 'added',
                    'field' => $fieldPath,
                    'value' => $this->formatValue($value),
                ];
            } elseif (is_array($value) && is_array($old[$key])) {
                if ($this->isAssociativeArray($value) && $this->isAssociativeArray($old[$key])) {
                    $nestedDiff = $this->calculateHumanReadableDiff($old[$key], $value, $fieldPath);
                    $diff = array_merge($diff, $nestedDiff);
                } elseif ($old[$key] !== $value) {
                    $diff[] = [
                        'type' => 'changed',
                        'field' => $fieldPath,
                        'from' => $this->formatValue($old[$key]),
                        'to' => $this->formatValue($value),
                    ];
                }
            } elseif ($old[$key] !== $value) {
                // String diff for text fields
                if (is_string($old[$key]) && is_string($value) && strlen($old[$key]) > 50) {
                    $diff[] = [
                        'type' => 'text_changed',
                        'field' => $fieldPath,
                        'changes' => $this->calculateTextDiff($old[$key], $value),
                    ];
                } else {
                    $diff[] = [
                        'type' => 'changed',
                        'field' => $fieldPath,
                        'from' => $this->formatValue($old[$key]),
                        'to' => $this->formatValue($value),
                    ];
                }
            }
        }

        // Find removed fields
        foreach ($old as $key => $value) {
            if (!array_key_exists($key, $new)) {
                $fieldPath = $path ? "{$path}.{$key}" : $key;
                $diff[] = [
                    'type' => 'removed',
                    'field' => $fieldPath,
                    'value' => $this->formatValue($value),
                ];
            }
        }

        return $diff;
    }

    /**
     * Calculate text diff (word-level)
     */
    private function calculateTextDiff(string $old, string $new): array
    {
        $oldWords = preg_split('/\s+/', $old);
        $newWords = preg_split('/\s+/', $new);

        $changes = [];
        $oldLen = count($oldWords);
        $newLen = count($newWords);

        // Simple LCS-based diff
        $lcs = $this->longestCommonSubsequence($oldWords, $newWords);

        $oldIndex = 0;
        $newIndex = 0;
        $lcsIndex = 0;

        while ($oldIndex < $oldLen || $newIndex < $newLen) {
            if ($lcsIndex < count($lcs) &&
                $oldIndex < $oldLen &&
                $newIndex < $newLen &&
                $oldWords[$oldIndex] === $lcs[$lcsIndex] &&
                $newWords[$newIndex] === $lcs[$lcsIndex]) {
                // Unchanged
                $changes[] = ['type' => 'unchanged', 'text' => $oldWords[$oldIndex]];
                $oldIndex++;
                $newIndex++;
                $lcsIndex++;
            } elseif ($newIndex < $newLen &&
                ($lcsIndex >= count($lcs) || $newWords[$newIndex] !== $lcs[$lcsIndex])) {
                // Added
                $changes[] = ['type' => 'added', 'text' => $newWords[$newIndex]];
                $newIndex++;
            } elseif ($oldIndex < $oldLen &&
                ($lcsIndex >= count($lcs) || $oldWords[$oldIndex] !== $lcs[$lcsIndex])) {
                // Removed
                $changes[] = ['type' => 'removed', 'text' => $oldWords[$oldIndex]];
                $oldIndex++;
            }
        }

        return $changes;
    }

    /**
     * Calculate LCS for diff
     */
    private function longestCommonSubsequence(array $a, array $b): array
    {
        $m = count($a);
        $n = count($b);
        $dp = array_fill(0, $m + 1, array_fill(0, $n + 1, 0));

        for ($i = 1; $i <= $m; $i++) {
            for ($j = 1; $j <= $n; $j++) {
                if ($a[$i - 1] === $b[$j - 1]) {
                    $dp[$i][$j] = $dp[$i - 1][$j - 1] + 1;
                } else {
                    $dp[$i][$j] = max($dp[$i - 1][$j], $dp[$i][$j - 1]);
                }
            }
        }

        // Backtrack to find LCS
        $lcs = [];
        $i = $m;
        $j = $n;

        while ($i > 0 && $j > 0) {
            if ($a[$i - 1] === $b[$j - 1]) {
                array_unshift($lcs, $a[$i - 1]);
                $i--;
                $j--;
            } elseif ($dp[$i - 1][$j] > $dp[$i][$j - 1]) {
                $i--;
            } else {
                $j--;
            }
        }

        return $lcs;
    }

    /**
     * Generate automatic change summary
     */
    private function generateChangeSummary(string $action, array $patches, array $diff): string
    {
        $changeCount = count($patches);

        if ($changeCount === 0) {
            return match ($action) {
                'create' => 'Created document',
                'publish' => 'Published document',
                'unpublish' => 'Unpublished document',
                'restore' => 'Restored document',
                default => 'Updated document',
            };
        }

        $summary = [];

        foreach ($diff as $change) {
            $field = $change['field'];
            $type = $change['type'];

            // Skip internal fields
            if (str_starts_with($field, '_')) {
                continue;
            }

            $fieldName = ucfirst(str_replace('_', ' ', $field));

            $summary[] = match ($type) {
                'added' => "Added {$fieldName}",
                'removed' => "Removed {$fieldName}",
                'changed', 'text_changed' => "Changed {$fieldName}",
                default => "Modified {$fieldName}",
            };
        }

        // Limit to 3 changes in summary
        if (count($summary) > 3) {
            $remaining = count($summary) - 3;
            $summary = array_slice($summary, 0, 3);
            $summary[] = "and {$remaining} more changes";
        }

        return implode(', ', $summary) ?: 'Updated document';
    }

    /**
     * Format value for display
     */
    private function formatValue(mixed $value): string
    {
        if (is_null($value)) {
            return 'null';
        }

        if (is_bool($value)) {
            return $value ? 'true' : 'false';
        }

        if (is_array($value)) {
            return '[array]';
        }

        if (is_string($value) && strlen($value) > 100) {
            return substr($value, 0, 100) . '...';
        }

        return (string) $value;
    }

    /**
     * Check if array is associative
     */
    private function isAssociativeArray(array $arr): bool
    {
        if (empty($arr)) {
            return false;
        }
        return array_keys($arr) !== range(0, count($arr) - 1);
    }

    /**
     * Prune old history entries
     */
    public function pruneHistory(string $documentId, int $keepRevisions = 100): int
    {
        $total = DB::table('document_history')
            ->where('document_id', $documentId)
            ->count();

        if ($total <= $keepRevisions) {
            return 0;
        }

        $toDelete = $total - $keepRevisions;

        $deleted = DB::table('document_history')
            ->where('document_id', $documentId)
            ->orderBy('revision_number', 'asc')
            ->limit($toDelete)
            ->delete();

        return $deleted;
    }

    /**
     * Get revision statistics
     */
    public function getStats(string $documentId): array
    {
        $stats = DB::table('document_history')
            ->where('document_id', $documentId)
            ->selectRaw('
                COUNT(*) as total_revisions,
                COUNT(DISTINCT user_id) as unique_authors,
                MIN(created_at) as first_revision_at,
                MAX(created_at) as last_revision_at
            ')
            ->first();

        $actionCounts = DB::table('document_history')
            ->where('document_id', $documentId)
            ->selectRaw('action, COUNT(*) as count')
            ->groupBy('action')
            ->pluck('count', 'action')
            ->toArray();

        return [
            'totalRevisions' => $stats->total_revisions,
            'uniqueAuthors' => $stats->unique_authors,
            'firstRevisionAt' => $stats->first_revision_at,
            'lastRevisionAt' => $stats->last_revision_at,
            'actionCounts' => $actionCounts,
        ];
    }
}
