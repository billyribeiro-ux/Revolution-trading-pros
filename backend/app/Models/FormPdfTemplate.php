<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Carbon;

/**
 * FormPdfTemplate Model - PDF Generation for Form Submissions
 *
 * Manages PDF templates for converting form submissions to downloadable/emailable PDFs.
 *
 * @property int $id
 * @property int $form_id
 * @property string $name
 * @property string $template_type
 * @property bool $active
 * @property string $paper_size
 * @property string $orientation
 * @property string|null $header_html
 * @property string|null $body_html
 * @property string|null $footer_html
 * @property array|null $field_settings
 * @property array|null $style_settings
 * @property array|null $conditional_logic
 * @property bool $attach_to_email
 * @property bool $attach_to_confirmation
 * @property string|null $filename_pattern
 * @property string|null $password
 * @property bool $flatten_form
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormPdfTemplate extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'form_pdf_templates';

    protected $fillable = [
        'form_id',
        'name',
        'template_type',
        'active',
        'paper_size',
        'orientation',
        'header_html',
        'body_html',
        'footer_html',
        'field_settings',
        'style_settings',
        'conditional_logic',
        'attach_to_email',
        'attach_to_confirmation',
        'filename_pattern',
        'password',
        'flatten_form',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'active' => 'boolean',
        'field_settings' => 'array',
        'style_settings' => 'array',
        'conditional_logic' => 'array',
        'attach_to_email' => 'boolean',
        'attach_to_confirmation' => 'boolean',
        'flatten_form' => 'boolean',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'active' => true,
        'template_type' => 'custom',
        'paper_size' => 'letter',
        'orientation' => 'portrait',
        'attach_to_email' => false,
        'attach_to_confirmation' => false,
        'flatten_form' => false,
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    // Template Types
    public const TYPE_CUSTOM = 'custom';
    public const TYPE_RECEIPT = 'receipt';
    public const TYPE_INVOICE = 'invoice';
    public const TYPE_CERTIFICATE = 'certificate';
    public const TYPE_CONFIRMATION = 'confirmation';
    public const TYPE_SUMMARY = 'summary';

    public const TEMPLATE_TYPES = [
        self::TYPE_CUSTOM => 'Custom Template',
        self::TYPE_RECEIPT => 'Payment Receipt',
        self::TYPE_INVOICE => 'Invoice',
        self::TYPE_CERTIFICATE => 'Certificate',
        self::TYPE_CONFIRMATION => 'Confirmation',
        self::TYPE_SUMMARY => 'Submission Summary',
    ];

    // Paper Sizes
    public const SIZE_LETTER = 'letter';
    public const SIZE_LEGAL = 'legal';
    public const SIZE_A4 = 'a4';
    public const SIZE_A5 = 'a5';
    public const SIZE_CUSTOM = 'custom';

    public const PAPER_SIZES = [
        self::SIZE_LETTER => ['width' => 612, 'height' => 792, 'label' => 'Letter (8.5" x 11")'],
        self::SIZE_LEGAL => ['width' => 612, 'height' => 1008, 'label' => 'Legal (8.5" x 14")'],
        self::SIZE_A4 => ['width' => 595, 'height' => 842, 'label' => 'A4 (210mm x 297mm)'],
        self::SIZE_A5 => ['width' => 420, 'height' => 595, 'label' => 'A5 (148mm x 210mm)'],
    ];

    // Orientations
    public const ORIENTATION_PORTRAIT = 'portrait';
    public const ORIENTATION_LANDSCAPE = 'landscape';

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeActive($query)
    {
        return $query->where('active', true);
    }

    public function scopeForForm($query, int $formId)
    {
        return $query->where('form_id', $formId);
    }

    public function scopeOfType($query, string $type)
    {
        return $query->where('template_type', $type);
    }

    public function scopeAttachToEmail($query)
    {
        return $query->where('attach_to_email', true);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getTypeLabelAttribute(): string
    {
        return self::TEMPLATE_TYPES[$this->template_type] ?? ucfirst($this->template_type);
    }

    public function getPaperDimensionsAttribute(): array
    {
        $dimensions = self::PAPER_SIZES[$this->paper_size] ?? self::PAPER_SIZES[self::SIZE_LETTER];

        if ($this->orientation === self::ORIENTATION_LANDSCAPE) {
            return [
                'width' => $dimensions['height'],
                'height' => $dimensions['width'],
            ];
        }

        return $dimensions;
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Generate filename for PDF.
     */
    public function generateFilename(FormSubmission $submission): string
    {
        $pattern = $this->filename_pattern ?? '{form_name}_{submission_id}_{date}';

        $replacements = [
            '{form_name}' => $this->sanitizeFilename($this->form->title ?? 'form'),
            '{submission_id}' => $submission->id,
            '{date}' => now()->format('Y-m-d'),
            '{datetime}' => now()->format('Y-m-d_H-i-s'),
            '{user_name}' => $this->sanitizeFilename($submission->user?->name ?? 'guest'),
            '{user_email}' => $this->sanitizeFilename($submission->email ?? ''),
        ];

        $filename = str_replace(array_keys($replacements), array_values($replacements), $pattern);

        return $filename . '.pdf';
    }

    /**
     * Sanitize string for use in filename.
     */
    protected function sanitizeFilename(string $name): string
    {
        $name = preg_replace('/[^a-zA-Z0-9_\-]/', '_', $name);
        $name = preg_replace('/_+/', '_', $name);
        return trim($name, '_');
    }

    /**
     * Check if template should be used for submission.
     */
    public function shouldApplyTo(FormSubmission $submission): bool
    {
        if (!$this->active) {
            return false;
        }

        if (empty($this->conditional_logic)) {
            return true;
        }

        return $this->evaluateConditionalLogic($submission);
    }

    /**
     * Evaluate conditional logic.
     */
    protected function evaluateConditionalLogic(FormSubmission $submission): bool
    {
        $logic = $this->conditional_logic;

        if (!isset($logic['rules']) || empty($logic['rules'])) {
            return true;
        }

        $results = [];
        $data = $submission->getFormattedData();

        foreach ($logic['rules'] as $rule) {
            $fieldValue = $data[$rule['field']] ?? null;
            $ruleValue = $rule['value'];

            $result = match($rule['operator']) {
                'equals' => $fieldValue == $ruleValue,
                'not_equals' => $fieldValue != $ruleValue,
                'contains' => str_contains((string)$fieldValue, (string)$ruleValue),
                'not_contains' => !str_contains((string)$fieldValue, (string)$ruleValue),
                'greater_than' => (float)$fieldValue > (float)$ruleValue,
                'less_than' => (float)$fieldValue < (float)$ruleValue,
                'is_empty' => empty($fieldValue),
                'is_not_empty' => !empty($fieldValue),
                default => true,
            };

            $results[] = $result;
        }

        $matchType = $logic['match'] ?? 'all';

        return $matchType === 'all'
            ? !in_array(false, $results, true)
            : in_array(true, $results, true);
    }

    /**
     * Get default header HTML.
     */
    public static function getDefaultHeader(): string
    {
        return <<<'HTML'
<div style="text-align: center; padding: 20px 0; border-bottom: 2px solid #1e3a8a;">
    <h1 style="margin: 0; color: #1e3a8a; font-size: 24px;">{{form_title}}</h1>
    <p style="margin: 5px 0 0; color: #6b7280; font-size: 12px;">Submission #{{submission_id}} | {{submission_date}}</p>
</div>
HTML;
    }

    /**
     * Get default body HTML.
     */
    public static function getDefaultBody(): string
    {
        return <<<'HTML'
<div style="padding: 20px 0;">
    {{#each fields}}
    <div style="margin-bottom: 15px; padding-bottom: 15px; border-bottom: 1px solid #e5e7eb;">
        <label style="display: block; font-weight: 600; color: #374151; margin-bottom: 5px;">{{label}}</label>
        <div style="color: #111827;">{{value}}</div>
    </div>
    {{/each}}
</div>
HTML;
    }

    /**
     * Get default footer HTML.
     */
    public static function getDefaultFooter(): string
    {
        return <<<'HTML'
<div style="text-align: center; padding: 15px 0; border-top: 1px solid #e5e7eb; font-size: 11px; color: #6b7280;">
    <p style="margin: 0;">Generated on {{current_date}} | Page {{page_number}} of {{total_pages}}</p>
    <p style="margin: 5px 0 0;">© {{year}} Revolution Trading Pros. All rights reserved.</p>
</div>
HTML;
    }

    /**
     * Get default style settings.
     */
    public static function getDefaultStyles(): array
    {
        return [
            'font_family' => 'DejaVu Sans, Arial, sans-serif',
            'font_size' => 12,
            'line_height' => 1.5,
            'primary_color' => '#1e3a8a',
            'secondary_color' => '#6b7280',
            'background_color' => '#ffffff',
            'text_color' => '#111827',
            'border_color' => '#e5e7eb',
            'margin_top' => 20,
            'margin_right' => 20,
            'margin_bottom' => 20,
            'margin_left' => 20,
        ];
    }
}
