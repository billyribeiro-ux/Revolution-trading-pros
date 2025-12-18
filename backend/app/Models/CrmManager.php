<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * CRM Manager Model (Team permissions for FluentCRM Pro)
 *
 * @property string $id
 * @property string $user_id
 * @property array|null $permissions
 * @property array|null $accessible_tags
 * @property array|null $accessible_lists
 * @property bool $can_manage_contacts
 * @property bool $can_manage_campaigns
 * @property bool $can_manage_sequences
 * @property bool $can_manage_automations
 * @property bool $can_view_reports
 * @property bool $can_export_data
 * @property bool $is_super_admin
 * @property string|null $created_by
 */
class CrmManager extends Model
{
    use HasUuids;

    protected $fillable = [
        'user_id',
        'permissions',
        'accessible_tags',
        'accessible_lists',
        'can_manage_contacts',
        'can_manage_campaigns',
        'can_manage_sequences',
        'can_manage_automations',
        'can_view_reports',
        'can_export_data',
        'is_super_admin',
        'created_by',
    ];

    protected function casts(): array
    {
        return [
            'permissions' => 'array',
            'accessible_tags' => 'array',
            'accessible_lists' => 'array',
            'can_manage_contacts' => 'boolean',
            'can_manage_campaigns' => 'boolean',
            'can_manage_sequences' => 'boolean',
            'can_manage_automations' => 'boolean',
            'can_view_reports' => 'boolean',
            'can_export_data' => 'boolean',
            'is_super_admin' => 'boolean',
        ];
    }

    // Available granular permissions
    public const PERMISSIONS = [
        'contacts' => [
            'view' => 'View contacts',
            'create' => 'Create contacts',
            'edit' => 'Edit contacts',
            'delete' => 'Delete contacts',
            'import' => 'Import contacts',
            'export' => 'Export contacts',
        ],
        'campaigns' => [
            'view' => 'View campaigns',
            'create' => 'Create campaigns',
            'edit' => 'Edit campaigns',
            'delete' => 'Delete campaigns',
            'send' => 'Send campaigns',
        ],
        'sequences' => [
            'view' => 'View sequences',
            'create' => 'Create sequences',
            'edit' => 'Edit sequences',
            'delete' => 'Delete sequences',
        ],
        'automations' => [
            'view' => 'View automations',
            'create' => 'Create automations',
            'edit' => 'Edit automations',
            'delete' => 'Delete automations',
        ],
        'reports' => [
            'view' => 'View reports',
            'export' => 'Export reports',
        ],
        'settings' => [
            'view' => 'View settings',
            'edit' => 'Edit settings',
        ],
    ];

    protected static function booted(): void
    {
        static::creating(function (self $manager): void {
            $manager->permissions ??= [];
            $manager->accessible_tags ??= [];
            $manager->accessible_lists ??= [];
        });
    }

    // Relationships
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    // Permission Checks
    public function hasPermission(string $category, string $action): bool
    {
        if ($this->is_super_admin) {
            return true;
        }

        return in_array("{$category}.{$action}", $this->permissions ?? [], true);
    }

    public function canAccessTag(string $tagId): bool
    {
        if ($this->is_super_admin || empty($this->accessible_tags)) {
            return true;
        }

        return in_array($tagId, $this->accessible_tags ?? [], true);
    }

    public function canAccessList(string $listId): bool
    {
        if ($this->is_super_admin || empty($this->accessible_lists)) {
            return true;
        }

        return in_array($listId, $this->accessible_lists ?? [], true);
    }

    public function canManageContact(Contact $contact): bool
    {
        if (!$this->can_manage_contacts) {
            return false;
        }

        if ($this->is_super_admin || empty($this->accessible_lists)) {
            return true;
        }

        // Check if contact is in any of the accessible lists
        // This would require the contact-list relationship to be set up
        return true;
    }

    // Grant/Revoke Permissions
    public function grantPermission(string $category, string $action): void
    {
        $permission = "{$category}.{$action}";
        $permissions = $this->permissions ?? [];

        if (!in_array($permission, $permissions, true)) {
            $permissions[] = $permission;
            $this->update(['permissions' => $permissions]);
        }
    }

    public function revokePermission(string $category, string $action): void
    {
        $permission = "{$category}.{$action}";
        $permissions = array_filter(
            $this->permissions ?? [],
            fn ($p) => $p !== $permission
        );

        $this->update(['permissions' => array_values($permissions)]);
    }

    public function grantCategoryPermissions(string $category): void
    {
        $categoryPermissions = self::PERMISSIONS[$category] ?? [];
        $permissions = $this->permissions ?? [];

        foreach (array_keys($categoryPermissions) as $action) {
            $permission = "{$category}.{$action}";
            if (!in_array($permission, $permissions, true)) {
                $permissions[] = $permission;
            }
        }

        $this->update(['permissions' => $permissions]);
    }

    public function revokeCategoryPermissions(string $category): void
    {
        $permissions = array_filter(
            $this->permissions ?? [],
            fn ($p) => !str_starts_with($p, "{$category}.")
        );

        $this->update(['permissions' => array_values($permissions)]);
    }

    // Tag/List Access Management
    public function grantTagAccess(string $tagId): void
    {
        $tags = $this->accessible_tags ?? [];
        if (!in_array($tagId, $tags, true)) {
            $tags[] = $tagId;
            $this->update(['accessible_tags' => $tags]);
        }
    }

    public function revokeTagAccess(string $tagId): void
    {
        $tags = array_filter(
            $this->accessible_tags ?? [],
            fn ($t) => $t !== $tagId
        );
        $this->update(['accessible_tags' => array_values($tags)]);
    }

    public function grantListAccess(string $listId): void
    {
        $lists = $this->accessible_lists ?? [];
        if (!in_array($listId, $lists, true)) {
            $lists[] = $listId;
            $this->update(['accessible_lists' => $lists]);
        }
    }

    public function revokeListAccess(string $listId): void
    {
        $lists = array_filter(
            $this->accessible_lists ?? [],
            fn ($l) => $l !== $listId
        );
        $this->update(['accessible_lists' => array_values($lists)]);
    }

    // Static helpers
    public static function findByUser(User $user): ?self
    {
        return self::where('user_id', $user->id)->first();
    }

    public static function createForUser(User $user, array $attributes = []): self
    {
        return self::create([
            'user_id' => $user->id,
            ...$attributes,
        ]);
    }
}
