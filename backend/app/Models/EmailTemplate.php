<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Blade;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\View;
use Illuminate\Support\Str;
use Carbon\Carbon;
use League\CommonMark\CommonMarkConverter;
use TijsVerkoyen\CssToInlineStyles\CssToInlineStyles;
use Soundasleep\Html2Text;
use App\Traits\HasUuid;
use App\Traits\Versionable;
use App\Traits\Translatable;
use App\Traits\Searchable;
use App\Traits\HasMedia;
use App\Contracts\Renderable;
use App\Events\TemplateCreated;
use App\Events\TemplateUpdated;
use App\Events\TemplatePublished;
use App\Events\TemplateRendered;
use App\Enums\EmailType;
use App\Enums\TemplateStatus;
use App\Enums\TemplateEngine;
use App\Enums\ContentType;
use App\Services\Template\TemplateCompiler;
use App\Services\Template\TemplateValidator;
use App\Services\Template\TemplatePreviewGenerator;
use App\Services\Template\TemplateAnalyzer;
use App\Exceptions\TemplateException;

/**
 * EmailTemplate Model
 * 
 * Enterprise-grade email template system with versioning, personalization,
 * A/B testing, multi-language support, and comprehensive rendering capabilities.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $name
 * @property string $slug
 * @property string $description
 * @property string $category
 * @property string $email_type
 * @property string $subject
 * @property string $preheader
 * @property string $body_html
 * @property string $body_text
 * @property string $body_amp
 * @property string $body_markdown
 * @property string $body_mjml
 * @property string $template_engine
 * @property string $layout_id
 * @property array $blocks
 * @property array $variables
 * @property array $required_variables
 * @property array $optional_variables
 * @property array $default_values
 * @property array $variable_types
 * @property array $personalization_tokens
 * @property array $dynamic_content
 * @property array $conditional_blocks
 * @property string $css_framework
 * @property string $inline_css
 * @property string $external_css
 * @property bool $auto_inline_css
 * @property array $responsive_settings
 * @property array $dark_mode_settings
 * @property array $accessibility_settings
 * @property string $from_name
 * @property string $from_email
 * @property string $reply_to_name
 * @property string $reply_to_email
 * @property array $headers
 * @property array $attachments
 * @property array $embedded_images
 * @property bool $track_opens
 * @property bool $track_clicks
 * @property string $utm_source
 * @property string $utm_medium
 * @property string $utm_campaign
 * @property array $utm_params
 * @property string $unsubscribe_url
 * @property string $view_in_browser_url
 * @property array $social_links
 * @property array $footer_links
 * @property string $status
 * @property bool $is_active
 * @property bool $is_default
 * @property bool $is_system
 * @property bool $is_locked
 * @property int $version
 * @property string $version_notes
 * @property int $parent_version_id
 * @property Carbon $published_at
 * @property int $published_by
 * @property array $ab_test
 * @property string $ab_test_variant
 * @property float $performance_score
 * @property int $usage_count
 * @property int $open_count
 * @property int $click_count
 * @property float $open_rate
 * @property float $click_rate
 * @property float $conversion_rate
 * @property array $performance_metrics
 * @property array $engagement_metrics
 * @property int $spam_score
 * @property array $spam_report
 * @property array $compatibility
 * @property array $client_support
 * @property int $estimated_read_time
 * @property int $word_count
 * @property int $character_count
 * @property array $languages
 * @property string $primary_language
 * @property array $translations
 * @property array $localization_settings
 * @property array $tags
 * @property array $metadata
 * @property array $custom_fields
 * @property array $compliance
 * @property array $legal_footer
 * @property int $retention_days
 * @property Carbon $expires_at
 * @property bool $requires_approval
 * @property int $approved_by
 * @property Carbon $approved_at
 * @property string $approval_notes
 * @property array $restrictions
 * @property array $permissions
 * @property int $created_by
 * @property int $updated_by
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon $deleted_at
 * 
 * @property-read \App\Models\EmailTemplateLayout $layout
 * @property-read \App\Models\EmailTemplate $parentVersion
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplate[] $versions
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplateBlock[] $templateBlocks
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailLog[] $emailLogs
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailCampaign[] $campaigns
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplatePreview[] $previews
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplateTest[] $tests
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplateTranslation[] $translationRecords
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\EmailTemplateAnalytics[] $analytics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Tag[] $tagRecords
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Comment[] $comments
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Activity[] $activities
 * @property-read \App\Models\User $creator
 * @property-read \App\Models\User $updater
 * @property-read \App\Models\User $approver
 * @property-read \App\Models\User $publisher
 */
class EmailTemplate extends Model implements Renderable
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Versionable;
    use Translatable;
    use Searchable;
    use HasMedia;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'email_templates';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'name',
        'slug',
        'description',
        'category',
        'email_type',
        'subject',
        'preheader',
        'body_html',
        'body_text',
        'body_amp',
        'body_markdown',
        'body_mjml',
        'template_engine',
        'layout_id',
        'blocks',
        'variables',
        'required_variables',
        'optional_variables',
        'default_values',
        'variable_types',
        'personalization_tokens',
        'dynamic_content',
        'conditional_blocks',
        'css_framework',
        'inline_css',
        'external_css',
        'auto_inline_css',
        'responsive_settings',
        'dark_mode_settings',
        'accessibility_settings',
        'from_name',
        'from_email',
        'reply_to_name',
        'reply_to_email',
        'headers',
        'attachments',
        'embedded_images',
        'track_opens',
        'track_clicks',
        'utm_source',
        'utm_medium',
        'utm_campaign',
        'utm_params',
        'unsubscribe_url',
        'view_in_browser_url',
        'social_links',
        'footer_links',
        'status',
        'is_active',
        'is_default',
        'is_system',
        'is_locked',
        'version',
        'version_notes',
        'parent_version_id',
        'published_at',
        'published_by',
        'ab_test',
        'ab_test_variant',
        'performance_score',
        'spam_score',
        'spam_report',
        'compatibility',
        'client_support',
        'languages',
        'primary_language',
        'translations',
        'localization_settings',
        'tags',
        'metadata',
        'custom_fields',
        'compliance',
        'legal_footer',
        'retention_days',
        'expires_at',
        'requires_approval',
        'approved_by',
        'approved_at',
        'approval_notes',
        'restrictions',
        'permissions',
        'created_by',
        'updated_by',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'email_type' => EmailType::class,
        'template_engine' => TemplateEngine::class,
        'status' => TemplateStatus::class,
        'blocks' => 'array',
        'variables' => 'array',
        'required_variables' => 'array',
        'optional_variables' => 'array',
        'default_values' => 'array',
        'variable_types' => 'array',
        'personalization_tokens' => 'array',
        'dynamic_content' => 'array',
        'conditional_blocks' => 'array',
        'responsive_settings' => 'array',
        'dark_mode_settings' => 'array',
        'accessibility_settings' => 'array',
        'headers' => 'array',
        'attachments' => 'array',
        'embedded_images' => 'array',
        'utm_params' => 'array',
        'social_links' => 'array',
        'footer_links' => 'array',
        'ab_test' => 'array',
        'performance_metrics' => 'array',
        'engagement_metrics' => 'array',
        'spam_report' => 'array',
        'compatibility' => 'array',
        'client_support' => 'array',
        'languages' => 'array',
        'translations' => 'array',
        'localization_settings' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'custom_fields' => 'array',
        'compliance' => 'array',
        'legal_footer' => 'array',
        'restrictions' => 'array',
        'permissions' => 'array',
        'is_active' => 'boolean',
        'is_default' => 'boolean',
        'is_system' => 'boolean',
        'is_locked' => 'boolean',
        'track_opens' => 'boolean',
        'track_clicks' => 'boolean',
        'auto_inline_css' => 'boolean',
        'requires_approval' => 'boolean',
        'layout_id' => 'integer',
        'version' => 'integer',
        'parent_version_id' => 'integer',
        'published_by' => 'integer',
        'approved_by' => 'integer',
        'usage_count' => 'integer',
        'open_count' => 'integer',
        'click_count' => 'integer',
        'spam_score' => 'integer',
        'estimated_read_time' => 'integer',
        'word_count' => 'integer',
        'character_count' => 'integer',
        'retention_days' => 'integer',
        'performance_score' => 'float',
        'open_rate' => 'float',
        'click_rate' => 'float',
        'conversion_rate' => 'float',
        'published_at' => 'datetime',
        'approved_at' => 'datetime',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'deleted_at',
        'spam_report',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_published',
        'is_draft',
        'is_approved',
        'is_expired',
        'can_edit',
        'can_publish',
        'preview_url',
        'test_url',
        'variable_list',
        'estimated_size',
        'readability_score',
        'mobile_optimized',
    ];

    /**
     * The attributes that should be searchable.
     *
     * @var array<string>
     */
    protected $searchable = [
        'name',
        'slug',
        'description',
        'subject',
        'preheader',
        'category',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'email_type' => EmailType::TRANSACTIONAL,
        'template_engine' => TemplateEngine::BLADE,
        'status' => TemplateStatus::DRAFT,
        'version' => 1,
        'is_active' => true,
        'is_default' => false,
        'is_system' => false,
        'is_locked' => false,
        'track_opens' => false,
        'track_clicks' => false,
        'auto_inline_css' => true,
        'requires_approval' => false,
        'retention_days' => 90,
        'usage_count' => 0,
        'open_count' => 0,
        'click_count' => 0,
        'open_rate' => 0,
        'click_rate' => 0,
        'conversion_rate' => 0,
        'performance_score' => 0,
        'spam_score' => 0,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'tags' => ['email_templates'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $template) {
            $template->uuid = $template->uuid ?? (string) Str::uuid();
            $template->slug = $template->slug ?? Str::slug($template->name);
            $template->created_by = $template->created_by ?? auth()->id();
            
            $template->extractVariables();
            $template->analyzeContent();
            $template->generateTextVersion();
        });

        static::created(function (self $template) {
            event(new TemplateCreated($template));
            $template->createInitialPreview();
            $template->validateTemplate();
        });

        static::updating(function (self $template) {
            $template->updated_by = auth()->id();
            
            if ($template->isDirty(['body_html', 'body_mjml', 'body_markdown'])) {
                $template->extractVariables();
                $template->analyzeContent();
                $template->generateTextVersion();
                $template->incrementVersion();
            }
            
            if (empty($template->slug)) {
                $template->slug = Str::slug($template->name);
            }
        });

        static::updated(function (self $template) {
            $template->clearCache();
            event(new TemplateUpdated($template));
            
            if ($template->wasChanged(['body_html', 'subject'])) {
                $template->regeneratePreviews();
            }
        });

        static::deleting(function (self $template) {
            if ($template->is_system || $template->is_locked) {
                throw new TemplateException('Cannot delete system or locked templates');
            }
            
            if ($template->campaigns()->active()->exists()) {
                throw new TemplateException('Cannot delete template with active campaigns');
            }
        });
    }

    /**
     * Get the layout.
     */
    public function layout(): BelongsTo
    {
        return $this->belongsTo(EmailTemplateLayout::class, 'layout_id');
    }

    /**
     * Get the parent version.
     */
    public function parentVersion(): BelongsTo
    {
        return $this->belongsTo(self::class, 'parent_version_id');
    }

    /**
     * Get all versions.
     */
    public function versions(): HasMany
    {
        return $this->hasMany(self::class, 'parent_version_id')
            ->orderBy('version', 'desc');
    }

    /**
     * Get template blocks.
     */
    public function templateBlocks(): HasMany
    {
        return $this->hasMany(EmailTemplateBlock::class)
            ->orderBy('order');
    }

    /**
     * Get email logs.
     */
    public function emailLogs(): HasMany
    {
        return $this->hasMany(EmailLog::class, 'template_id');
    }

    /**
     * Get campaigns using this template.
     */
    public function campaigns(): BelongsToMany
    {
        return $this->belongsToMany(EmailCampaign::class, 'campaign_templates')
            ->withTimestamps()
            ->withPivot(['is_primary', 'variant']);
    }

    /**
     * Get previews.
     */
    public function previews(): HasMany
    {
        return $this->hasMany(EmailTemplatePreview::class)
            ->orderBy('created_at', 'desc');
    }

    /**
     * Get tests.
     */
    public function tests(): HasMany
    {
        return $this->hasMany(EmailTemplateTest::class)
            ->orderBy('tested_at', 'desc');
    }

    /**
     * Get translation records.
     */
    public function translationRecords(): HasMany
    {
        return $this->hasMany(EmailTemplateTranslation::class);
    }

    /**
     * Get analytics.
     */
    public function analytics(): HasMany
    {
        return $this->hasMany(EmailTemplateAnalytics::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get tags.
     */
    public function tagRecords(): BelongsToMany
    {
        return $this->belongsToMany(Tag::class, 'email_template_tags')
            ->withTimestamps();
    }

    /**
     * Get comments.
     */
    public function comments(): MorphMany
    {
        return $this->morphMany(Comment::class, 'commentable');
    }

    /**
     * Get activities.
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Get creator.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get updater.
     */
    public function updater(): BelongsTo
    {
        return $this->belongsTo(User::class, 'updated_by');
    }

    /**
     * Get approver.
     */
    public function approver(): BelongsTo
    {
        return $this->belongsTo(User::class, 'approved_by');
    }

    /**
     * Get publisher.
     */
    public function publisher(): BelongsTo
    {
        return $this->belongsTo(User::class, 'published_by');
    }

    /**
     * Scope for active templates.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where('status', TemplateStatus::PUBLISHED);
    }

    /**
     * Scope for published templates.
     */
    public function scopePublished(Builder $query): Builder
    {
        return $query->where('status', TemplateStatus::PUBLISHED)
            ->whereNotNull('published_at');
    }

    /**
     * Scope for draft templates.
     */
    public function scopeDraft(Builder $query): Builder
    {
        return $query->where('status', TemplateStatus::DRAFT);
    }

    /**
     * Scope by email type.
     */
    public function scopeByType(Builder $query, string $type): Builder
    {
        return $query->where('email_type', $type);
    }

    /**
     * Scope by category.
     */
    public function scopeByCategory(Builder $query, string $category): Builder
    {
        return $query->where('category', $category);
    }

    /**
     * Scope for templates that need approval.
     */
    public function scopeNeedsApproval(Builder $query): Builder
    {
        return $query->where('requires_approval', true)
            ->whereNull('approved_at');
    }

    /**
     * Get published status.
     */
    protected function isPublished(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->status === TemplateStatus::PUBLISHED
        );
    }

    /**
     * Get draft status.
     */
    protected function isDraft(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->status === TemplateStatus::DRAFT
        );
    }

    /**
     * Get approved status.
     */
    protected function isApproved(): Attribute
    {
        return Attribute::make(
            get: fn () => !$this->requires_approval || $this->approved_at !== null
        );
    }

    /**
     * Get expired status.
     */
    protected function isExpired(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->expires_at && $this->expires_at->isPast()
        );
    }

    /**
     * Get edit permission.
     */
    protected function canEdit(): Attribute
    {
        return Attribute::make(
            get: fn () => !$this->is_locked && 
                         !$this->is_system &&
                         auth()->user()?->can('edit', $this)
        );
    }

    /**
     * Get publish permission.
     */
    protected function canPublish(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_approved && 
                         auth()->user()?->can('publish', $this)
        );
    }

    /**
     * Get preview URL.
     */
    protected function previewUrl(): Attribute
    {
        return Attribute::make(
            get: fn () => route('email-templates.preview', $this->uuid)
        );
    }

    /**
     * Get test URL.
     */
    protected function testUrl(): Attribute
    {
        return Attribute::make(
            get: fn () => route('email-templates.test', $this->uuid)
        );
    }

    /**
     * Get variable list.
     */
    protected function variableList(): Attribute
    {
        return Attribute::make(
            get: fn () => array_merge(
                $this->required_variables ?? [],
                $this->optional_variables ?? []
            )
        );
    }

    /**
     * Get estimated size in KB.
     */
    protected function estimatedSize(): Attribute
    {
        return Attribute::make(
            get: function () {
                $size = strlen($this->body_html ?? '') + 
                       strlen($this->body_text ?? '') + 
                       strlen($this->inline_css ?? '');
                return round($size / 1024, 2);
            }
        );
    }

    /**
     * Get readability score.
     */
    protected function readabilityScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateReadabilityScore()
        );
    }

    /**
     * Get mobile optimization status.
     */
    protected function mobileOptimized(): Attribute
    {
        return Attribute::make(
            get: fn () => !empty($this->responsive_settings) || 
                         str_contains($this->body_html ?? '', 'media query')
        );
    }

    /**
     * Extract variables from template.
     */
    protected function extractVariables(): void
    {
        $compiler = app(TemplateCompiler::class);
        $variables = $compiler->extractVariables($this->body_html ?? '');
        
        $this->variables = $variables['all'];
        $this->required_variables = $variables['required'];
        $this->optional_variables = $variables['optional'];
        $this->personalization_tokens = $variables['personalization'];
    }

    /**
     * Analyze content for metrics and quality.
     */
    protected function analyzeContent(): void
    {
        $analyzer = app(TemplateAnalyzer::class);
        $analysis = $analyzer->analyze($this);
        
        $this->word_count = $analysis['word_count'];
        $this->character_count = $analysis['character_count'];
        $this->estimated_read_time = $analysis['estimated_read_time'];
        $this->spam_score = $analysis['spam_score'];
        $this->spam_report = $analysis['spam_report'];
        $this->compatibility = $analysis['compatibility'];
        $this->client_support = $analysis['client_support'];
    }

    /**
     * Generate text version from HTML.
     */
    protected function generateTextVersion(): void
    {
        if (!$this->body_text && $this->body_html) {
            $converter = new Html2Text($this->body_html);
            $this->body_text = $converter->getText();
        }
    }

    /**
     * Calculate readability score.
     */
    protected function calculateReadabilityScore(): float
    {
        if (!$this->body_text) {
            return 0;
        }
        
        // Flesch Reading Ease calculation
        $words = str_word_count($this->body_text);
        $sentences = preg_match_all('/[.!?]+/', $this->body_text, $matches);
        $syllables = $this->countSyllables($this->body_text);
        
        if ($words === 0 || $sentences === 0) {
            return 0;
        }
        
        $score = 206.835 - 1.015 * ($words / $sentences) - 84.6 * ($syllables / $words);
        return max(0, min(100, $score));
    }

    /**
     * Count syllables in text.
     */
    protected function countSyllables(string $text): int
    {
        $text = strtolower($text);
        $count = 0;
        $words = str_word_count($text, 1);
        
        foreach ($words as $word) {
            $count += max(1, preg_match_all('/[aeiou]/', $word, $matches));
        }
        
        return $count;
    }

    /**
     * Render the template with data.
     */
    public function render(array $data = [], string $language = null): array
    {
        $cacheKey = $this->getCacheKey($data, $language);
        
        return Cache::tags(self::$cacheConfig['tags'])
            ->remember($cacheKey, 300, function () use ($data, $language) {
                return $this->performRender($data, $language);
            });
    }

    /**
     * Perform template rendering.
     */
    protected function performRender(array $data, ?string $language): array
    {
        $compiler = app(TemplateCompiler::class);
        
        // Get translated content if needed
        $content = $this->getContentForLanguage($language);
        
        // Merge default values with provided data
        $data = array_merge($this->default_values ?? [], $data);
        
        // Compile the template
        $compiled = $compiler->compile($content, $data, $this->template_engine);
        
        // Inline CSS if needed
        if ($this->auto_inline_css && $compiled['html']) {
            $inliner = new CssToInlineStyles();
            $compiled['html'] = $inliner->convert(
                $compiled['html'],
                $this->inline_css ?? ''
            );
        }
        
        // Track rendering
        $this->recordRender($data, $language);
        
        event(new TemplateRendered($this, $data));
        
        return [
            'subject' => $compiled['subject'],
            'html' => $compiled['html'],
            'text' => $compiled['text'],
            'amp' => $compiled['amp'] ?? null,
            'headers' => $this->buildHeaders($data),
            'attachments' => $this->processAttachments($data),
        ];
    }

    /**
     * Get content for specific language.
     */
    protected function getContentForLanguage(?string $language): array
    {
        if (!$language || $language === $this->primary_language) {
            return [
                'subject' => $this->subject,
                'body_html' => $this->body_html,
                'body_text' => $this->body_text,
                'body_amp' => $this->body_amp,
            ];
        }
        
        $translation = $this->translationRecords()
            ->where('language', $language)
            ->first();
        
        if ($translation) {
            return [
                'subject' => $translation->subject,
                'body_html' => $translation->body_html,
                'body_text' => $translation->body_text,
                'body_amp' => $translation->body_amp,
            ];
        }
        
        // Fallback to primary language
        return $this->getContentForLanguage(null);
    }

    /**
     * Build email headers.
     */
    protected function buildHeaders(array $data): array
    {
        $headers = $this->headers ?? [];
        
        if ($this->track_opens) {
            $headers['X-Track-Opens'] = 'true';
        }
        
        if ($this->track_clicks) {
            $headers['X-Track-Clicks'] = 'true';
        }
        
        if ($this->unsubscribe_url) {
            $url = $this->processUrl($this->unsubscribe_url, $data);
            $headers['List-Unsubscribe'] = "<{$url}>";
            $headers['List-Unsubscribe-Post'] = 'List-Unsubscribe=One-Click';
        }
        
        return $headers;
    }

    /**
     * Process attachments.
     */
    protected function processAttachments(array $data): array
    {
        if (empty($this->attachments)) {
            return [];
        }
        
        return collect($this->attachments)->map(function ($attachment) use ($data) {
            if (isset($attachment['condition']) && !$this->evaluateCondition($attachment['condition'], $data)) {
                return null;
            }
            
            return [
                'path' => $attachment['path'],
                'name' => $attachment['name'] ?? basename($attachment['path']),
                'mime' => $attachment['mime'] ?? 'application/octet-stream',
            ];
        })->filter()->values()->toArray();
    }

    /**
     * Process URL with data.
     */
    protected function processUrl(string $url, array $data): string
    {
        foreach ($data as $key => $value) {
            if (is_scalar($value)) {
                $url = str_replace('{' . $key . '}', $value, $url);
            }
        }
        
        // Add UTM parameters
        if ($this->utm_params) {
            $separator = str_contains($url, '?') ? '&' : '?';
            $url .= $separator . http_build_query($this->utm_params);
        }
        
        return $url;
    }

    /**
     * Evaluate condition.
     */
    protected function evaluateCondition(string $condition, array $data): bool
    {
        // Simple condition evaluation
        // In production, use a proper expression evaluator
        return eval("return {$condition};");
    }

    /**
     * Validate template.
     */
    public function validateTemplate(): array
    {
        $validator = app(TemplateValidator::class);
        $results = $validator->validate($this);
        
        $this->update([
            'spam_score' => $results['spam_score'],
            'compatibility' => $results['compatibility'],
        ]);
        
        return $results;
    }

    /**
     * Create initial preview.
     */
    protected function createInitialPreview(): void
    {
        $generator = app(TemplatePreviewGenerator::class);
        $preview = $generator->generate($this);
        
        $this->previews()->create([
            'html' => $preview['html'],
            'text' => $preview['text'],
            'subject' => $preview['subject'],
            'device' => 'desktop',
            'sample_data' => $preview['data'],
        ]);
    }

    /**
     * Regenerate all previews.
     */
    public function regeneratePreviews(): void
    {
        $generator = app(TemplatePreviewGenerator::class);
        
        foreach (['desktop', 'mobile', 'tablet'] as $device) {
            $preview = $generator->generate($this, ['device' => $device]);
            
            $this->previews()->updateOrCreate(
                ['device' => $device],
                [
                    'html' => $preview['html'],
                    'text' => $preview['text'],
                    'subject' => $preview['subject'],
                    'sample_data' => $preview['data'],
                ]
            );
        }
    }

    /**
     * Publish template.
     */
    public function publish(): void
    {
        if (!$this->is_approved) {
            throw new TemplateException('Template must be approved before publishing');
        }
        
        $this->update([
            'status' => TemplateStatus::PUBLISHED,
            'published_at' => now(),
            'published_by' => auth()->id(),
        ]);
        
        event(new TemplatePublished($this));
    }

    /**
     * Create a new version.
     */
    public function createVersion(string $notes = null): self
    {
        $newVersion = $this->replicate();
        $newVersion->parent_version_id = $this->id;
        $newVersion->version = $this->version + 1;
        $newVersion->version_notes = $notes;
        $newVersion->status = TemplateStatus::DRAFT;
        $newVersion->published_at = null;
        $newVersion->approved_at = null;
        $newVersion->save();
        
        return $newVersion;
    }

    /**
     * Clone for A/B testing.
     */
    public function cloneForABTest(string $variant, array $changes = []): self
    {
        $clone = $this->replicate();
        $clone->fill($changes);
        $clone->name = $this->name . ' - ' . $variant;
        $clone->slug = $this->slug . '-' . strtolower($variant);
        $clone->ab_test_variant = $variant;
        $clone->ab_test = [
            'original_id' => $this->id,
            'variant' => $variant,
            'changes' => array_keys($changes),
            'started_at' => now(),
        ];
        $clone->save();
        
        return $clone;
    }

    /**
     * Test send the template.
     */
    public function testSend(string $email, array $data = []): bool
    {
        $rendered = $this->render($data);
        
        // Send test email
        $result = Mail::html($rendered['html'], function ($message) use ($email, $rendered) {
            $message->to($email)
                ->subject('[TEST] ' . $rendered['subject']);
            
            foreach ($rendered['headers'] as $key => $value) {
                $message->getHeaders()->addTextHeader($key, $value);
            }
        });
        
        // Record test
        $this->tests()->create([
            'recipient' => $email,
            'data' => $data,
            'success' => $result !== null,
            'tested_at' => now(),
        ]);
        
        return $result !== null;
    }

    /**
     * Record template render.
     */
    protected function recordRender(array $data, ?string $language): void
    {
        $this->increment('usage_count');
        
        $this->analytics()->create([
            'action' => 'render',
            'data' => [
                'language' => $language,
                'variables_used' => array_keys($data),
            ],
            'recorded_at' => now(),
        ]);
    }

    /**
     * Update performance metrics.
     */
    public function updatePerformanceMetrics(): void
    {
        $logs = $this->emailLogs()
            ->where('created_at', '>=', now()->subDays(30))
            ->get();
        
        if ($logs->count() > 0) {
            $this->open_count = $logs->sum('open_count');
            $this->click_count = $logs->sum('click_count');
            $this->open_rate = ($logs->where('open_count', '>', 0)->count() / $logs->count()) * 100;
            $this->click_rate = ($logs->where('click_count', '>', 0)->count() / $logs->count()) * 100;
            
            // Calculate performance score
            $score = 0;
            $score += min(50, $this->open_rate);
            $score += min(30, $this->click_rate * 2);
            $score += max(0, 20 - $this->spam_score * 2);
            
            $this->performance_score = $score;
            $this->save();
        }
    }

    /**
     * Get cache key for rendered template.
     */
    protected function getCacheKey(array $data, ?string $language): string
    {
        return sprintf(
            'template_%d_%s_%s',
            $this->id,
            md5(serialize($data)),
            $language ?? 'default'
        );
    }

    /**
     * Clear cache.
     */
    protected function clearCache(): void
    {
        Cache::tags(self::$cacheConfig['tags'])->flush();
    }

    /**
     * Get template categories.
     */
    public static function getCategories(): array
    {
        return Cache::tags(self::$cacheConfig['tags'])
            ->remember('template_categories', self::$cacheConfig['ttl'], function () {
                return self::distinct('category')
                    ->whereNotNull('category')
                    ->pluck('category')
                    ->toArray();
            });
    }
}