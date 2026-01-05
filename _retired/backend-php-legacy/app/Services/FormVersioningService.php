<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormField;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Form Versioning Service - Full version control for forms
 *
 * Features:
 * - Full form versioning with fields
 * - Version comparison (diff)
 * - Rollback to any version
 * - Branch/fork support
 * - Auto-save drafts
 * - Conflict resolution
 * - Change history
 *
 * @version 2.0.0
 */
class FormVersioningService
{
    /**
     * Create a new version of a form
     *
     * @param Form $form The form to version
     * @param string $description Version description/commit message
     * @param bool $isPublished Whether this version is being published
     * @return array Version data
     */
    public function createVersion(Form $form, string $description = '', bool $isPublished = false): array
    {
        return DB::transaction(function () use ($form, $description, $isPublished) {
            // Get current max version number
            $latestVersion = DB::table('form_versions')
                ->where('form_id', $form->id)
                ->max('version_number') ?? 0;

            $versionNumber = $latestVersion + 1;

            // Snapshot the form
            $snapshot = $this->createSnapshot($form);

            // Store the version
            $versionId = DB::table('form_versions')->insertGetId([
                'form_id' => $form->id,
                'version_number' => $versionNumber,
                'description' => $description,
                'snapshot' => json_encode($snapshot),
                'checksum' => $this->calculateChecksum($snapshot),
                'is_published' => $isPublished,
                'published_at' => $isPublished ? now() : null,
                'created_by' => auth()->id(),
                'created_at' => now(),
                'updated_at' => now(),
            ]);

            // Update form's current version
            $form->update(['version' => $versionNumber]);

            return [
                'id' => $versionId,
                'version_number' => $versionNumber,
                'checksum' => $this->calculateChecksum($snapshot),
                'is_published' => $isPublished,
            ];
        });
    }

    /**
     * Get version history for a form
     *
     * @param Form $form
     * @param int $limit
     * @return array
     */
    public function getVersionHistory(Form $form, int $limit = 50): array
    {
        return DB::table('form_versions')
            ->where('form_id', $form->id)
            ->leftJoin('users', 'form_versions.created_by', '=', 'users.id')
            ->select([
                'form_versions.id',
                'form_versions.version_number',
                'form_versions.description',
                'form_versions.is_published',
                'form_versions.published_at',
                'form_versions.checksum',
                'form_versions.created_at',
                'users.name as created_by_name',
                'users.email as created_by_email',
            ])
            ->orderBy('form_versions.version_number', 'desc')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get a specific version
     *
     * @param Form $form
     * @param int $versionNumber
     * @return array|null
     */
    public function getVersion(Form $form, int $versionNumber): ?array
    {
        $version = DB::table('form_versions')
            ->where('form_id', $form->id)
            ->where('version_number', $versionNumber)
            ->first();

        if (!$version) {
            return null;
        }

        return [
            'id' => $version->id,
            'version_number' => $version->version_number,
            'description' => $version->description,
            'snapshot' => json_decode($version->snapshot, true),
            'checksum' => $version->checksum,
            'is_published' => $version->is_published,
            'published_at' => $version->published_at,
            'created_at' => $version->created_at,
        ];
    }

    /**
     * Rollback form to a specific version
     *
     * @param Form $form
     * @param int $versionNumber
     * @param string $reason Rollback reason
     * @return Form
     */
    public function rollback(Form $form, int $versionNumber, string $reason = ''): Form
    {
        $version = $this->getVersion($form, $versionNumber);

        if (!$version) {
            throw new \InvalidArgumentException("Version {$versionNumber} not found");
        }

        return DB::transaction(function () use ($form, $version, $reason) {
            $snapshot = $version['snapshot'];

            // Create a new version before rollback (for history)
            $this->createVersion($form, "Pre-rollback snapshot (rolling back to v{$version['version_number']})");

            // Restore form data
            $form->update([
                'title' => $snapshot['form']['title'],
                'description' => $snapshot['form']['description'],
                'settings' => $snapshot['form']['settings'],
                'styles' => $snapshot['form']['styles'],
            ]);

            // Delete current fields
            $form->fields()->delete();

            // Restore fields from snapshot
            foreach ($snapshot['fields'] as $fieldData) {
                unset($fieldData['id'], $fieldData['form_id'], $fieldData['created_at'], $fieldData['updated_at']);
                $form->fields()->create($fieldData);
            }

            // Create version record for rollback
            $this->createVersion($form, "Rolled back to v{$version['version_number']}: {$reason}");

            return $form->fresh(['fields']);
        });
    }

    /**
     * Compare two versions
     *
     * @param Form $form
     * @param int $fromVersion
     * @param int $toVersion
     * @return array Diff details
     */
    public function compareVersions(Form $form, int $fromVersion, int $toVersion): array
    {
        $from = $this->getVersion($form, $fromVersion);
        $to = $this->getVersion($form, $toVersion);

        if (!$from || !$to) {
            throw new \InvalidArgumentException('One or both versions not found');
        }

        $diff = [
            'from_version' => $fromVersion,
            'to_version' => $toVersion,
            'form_changes' => $this->diffArrays($from['snapshot']['form'], $to['snapshot']['form']),
            'field_changes' => $this->diffFields($from['snapshot']['fields'], $to['snapshot']['fields']),
            'summary' => [
                'fields_added' => 0,
                'fields_removed' => 0,
                'fields_modified' => 0,
                'form_settings_changed' => false,
            ],
        ];

        // Calculate summary
        foreach ($diff['field_changes'] as $change) {
            switch ($change['type']) {
                case 'added':
                    $diff['summary']['fields_added']++;
                    break;
                case 'removed':
                    $diff['summary']['fields_removed']++;
                    break;
                case 'modified':
                    $diff['summary']['fields_modified']++;
                    break;
            }
        }

        $diff['summary']['form_settings_changed'] = !empty($diff['form_changes']);

        return $diff;
    }

    /**
     * Create a branch (fork) of a form
     *
     * @param Form $form Original form
     * @param string $branchName Branch name
     * @return Form New forked form
     */
    public function createBranch(Form $form, string $branchName): Form
    {
        return DB::transaction(function () use ($form, $branchName) {
            // Create new form as copy
            $newForm = $form->replicate();
            $newForm->title = $form->title . ' (' . $branchName . ')';
            $newForm->slug = Str::slug($newForm->title) . '-' . Str::random(6);
            $newForm->status = 'draft';
            $newForm->parent_id = $form->id;
            $newForm->version = 1;
            $newForm->submission_count = 0;
            $newForm->save();

            // Copy fields
            foreach ($form->fields as $field) {
                $newField = $field->replicate();
                $newField->form_id = $newForm->id;
                $newField->save();
            }

            // Create initial version
            $this->createVersion($newForm, "Branched from '{$form->title}' v{$form->version}");

            return $newForm->fresh(['fields']);
        });
    }

    /**
     * Merge changes from a branch back to parent
     *
     * @param Form $branch Branch form
     * @param Form $parent Parent form
     * @param array $options Merge options
     * @return Form Updated parent
     */
    public function mergeBranch(Form $branch, Form $parent, array $options = []): Form
    {
        $strategy = $options['strategy'] ?? 'theirs'; // 'theirs', 'ours', 'manual'

        return DB::transaction(function () use ($branch, $parent, $strategy) {
            // Create backup version of parent
            $this->createVersion($parent, 'Pre-merge snapshot');

            // Get conflicts
            $conflicts = $this->detectConflicts($branch, $parent);

            if (!empty($conflicts) && $strategy === 'manual') {
                throw new \Exception('Conflicts detected, manual resolution required');
            }

            // Apply changes based on strategy
            if ($strategy === 'theirs') {
                // Use branch values for everything
                $parent->update([
                    'settings' => $branch->settings,
                    'styles' => $branch->styles,
                ]);

                // Replace fields
                $parent->fields()->delete();
                foreach ($branch->fields as $field) {
                    $newField = $field->replicate();
                    $newField->form_id = $parent->id;
                    $newField->save();
                }
            }

            // Create merge version
            $this->createVersion($parent, "Merged from '{$branch->title}'");

            return $parent->fresh(['fields']);
        });
    }

    /**
     * Detect conflicts between branch and parent
     */
    public function detectConflicts(Form $branch, Form $parent): array
    {
        $conflicts = [];

        // Check if parent was modified after branch was created
        if ($parent->updated_at > $branch->created_at) {
            // Compare fields by name
            $parentFields = $parent->fields->keyBy('name');
            $branchFields = $branch->fields->keyBy('name');

            foreach ($branchFields as $name => $branchField) {
                if (isset($parentFields[$name])) {
                    $parentField = $parentFields[$name];

                    // Check if both changed from original
                    if ($branchField->updated_at > $branch->created_at &&
                        $parentField->updated_at > $branch->created_at) {
                        $conflicts[] = [
                            'type' => 'field',
                            'field_name' => $name,
                            'branch_value' => $branchField->toArray(),
                            'parent_value' => $parentField->toArray(),
                        ];
                    }
                }
            }
        }

        return $conflicts;
    }

    /**
     * Auto-save draft version
     *
     * @param Form $form
     * @param array $changes Unsaved changes
     * @return array Draft data
     */
    public function autosaveDraft(Form $form, array $changes): array
    {
        $draftKey = "form_draft:{$form->id}:" . (auth()->id() ?? 'anonymous');

        $draft = [
            'form_id' => $form->id,
            'user_id' => auth()->id(),
            'changes' => $changes,
            'saved_at' => now()->toIso8601String(),
        ];

        // Store in cache (expires in 24 hours)
        cache()->put($draftKey, $draft, now()->addDay());

        return $draft;
    }

    /**
     * Get autosaved draft
     */
    public function getAutosavedDraft(Form $form): ?array
    {
        $draftKey = "form_draft:{$form->id}:" . (auth()->id() ?? 'anonymous');
        return cache()->get($draftKey);
    }

    /**
     * Clear autosaved draft
     */
    public function clearAutosavedDraft(Form $form): void
    {
        $draftKey = "form_draft:{$form->id}:" . (auth()->id() ?? 'anonymous');
        cache()->forget($draftKey);
    }

    /**
     * Create a snapshot of the form
     */
    private function createSnapshot(Form $form): array
    {
        return [
            'form' => [
                'title' => $form->title,
                'description' => $form->description,
                'settings' => $form->settings,
                'styles' => $form->styles,
                'status' => $form->status,
            ],
            'fields' => $form->fields->map(function ($field) {
                return [
                    'field_type' => $field->field_type,
                    'label' => $field->label,
                    'name' => $field->name,
                    'placeholder' => $field->placeholder,
                    'help_text' => $field->help_text,
                    'default_value' => $field->default_value,
                    'options' => $field->options,
                    'validation' => $field->validation,
                    'conditional_logic' => $field->conditional_logic,
                    'attributes' => $field->attributes,
                    'required' => $field->required,
                    'order' => $field->order,
                    'width' => $field->width,
                ];
            })->toArray(),
            'metadata' => [
                'field_count' => $form->fields->count(),
                'snapshot_at' => now()->toIso8601String(),
            ],
        ];
    }

    /**
     * Calculate checksum for snapshot
     */
    private function calculateChecksum(array $snapshot): string
    {
        return hash('sha256', json_encode($snapshot));
    }

    /**
     * Diff two arrays
     */
    private function diffArrays(array $from, array $to): array
    {
        $diff = [];

        foreach ($to as $key => $value) {
            if (!isset($from[$key])) {
                $diff[$key] = ['type' => 'added', 'value' => $value];
            } elseif ($from[$key] !== $value) {
                $diff[$key] = [
                    'type' => 'modified',
                    'from' => $from[$key],
                    'to' => $value,
                ];
            }
        }

        foreach ($from as $key => $value) {
            if (!isset($to[$key])) {
                $diff[$key] = ['type' => 'removed', 'value' => $value];
            }
        }

        return $diff;
    }

    /**
     * Diff fields between versions
     */
    private function diffFields(array $fromFields, array $toFields): array
    {
        $changes = [];

        // Index by name for comparison
        $fromByName = collect($fromFields)->keyBy('name');
        $toByName = collect($toFields)->keyBy('name');

        // Find added and modified
        foreach ($toByName as $name => $toField) {
            if (!isset($fromByName[$name])) {
                $changes[] = [
                    'type' => 'added',
                    'field_name' => $name,
                    'field' => $toField,
                ];
            } else {
                $fromField = $fromByName[$name];
                $fieldDiff = $this->diffArrays($fromField, $toField);

                if (!empty($fieldDiff)) {
                    $changes[] = [
                        'type' => 'modified',
                        'field_name' => $name,
                        'changes' => $fieldDiff,
                    ];
                }
            }
        }

        // Find removed
        foreach ($fromByName as $name => $fromField) {
            if (!isset($toByName[$name])) {
                $changes[] = [
                    'type' => 'removed',
                    'field_name' => $name,
                    'field' => $fromField,
                ];
            }
        }

        return $changes;
    }
}
