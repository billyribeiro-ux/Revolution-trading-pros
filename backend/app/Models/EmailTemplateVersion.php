<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * EmailTemplateVersion Model
 * 
 * Tracks versions of email templates for history and rollback
 * 
 * @property int $id
 * @property int $template_id
 * @property int $version_number
 * @property array $snapshot
 * @property string|null $changes_summary
 * @property int $created_by
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read EmailTemplate $template
 * @property-read User $creator
 */
class EmailTemplateVersion extends Model
{
    use HasFactory;

    protected $fillable = [
        'template_id',
        'version_number',
        'snapshot',
        'changes_summary',
        'created_by',
    ];

    protected $casts = [
        'snapshot' => 'array',
    ];

    /**
     * Relationships
     */
    public function template(): BelongsTo
    {
        return $this->belongsTo(EmailTemplate::class);
    }

    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Scopes
     */
    public function scopeForTemplate(Builder $query, int $templateId): Builder
    {
        return $query->where('template_id', $templateId);
    }

    public function scopeLatestVersion(Builder $query): Builder
    {
        return $query->orderBy('version_number', 'desc');
    }

    /**
     * Get version label
     */
    public function getVersionLabelAttribute(): string
    {
        return "v{$this->version_number}";
    }

    /**
     * Check if this is the latest version
     */
    public function isLatest(): bool
    {
        $latestVersion = static::where('template_id', $this->template_id)
            ->max('version_number');
        
        return $this->version_number === $latestVersion;
    }

    /**
     * Restore template to this version
     */
    public function restore(): EmailTemplate
    {
        $template = $this->template;
        $template->fill($this->snapshot);
        $template->version = $this->version_number;
        $template->save();
        
        return $template;
    }

    /**
     * Create snapshot from template
     */
    public static function createFromTemplate(EmailTemplate $template, ?string $changesSummary = null): self
    {
        $latestVersion = static::where('template_id', $template->id)
            ->max('version_number') ?? 0;

        return static::create([
            'template_id' => $template->id,
            'version_number' => $latestVersion + 1,
            'snapshot' => $template->toArray(),
            'changes_summary' => $changesSummary,
            'created_by' => auth()->id() ?? 1,
        ]);
    }
}
