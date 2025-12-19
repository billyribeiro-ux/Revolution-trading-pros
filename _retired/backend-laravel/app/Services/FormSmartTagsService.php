<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\User;
use Illuminate\Support\Str;
use Illuminate\Support\Carbon;

/**
 * Form Smart Tags Service - Dynamic merge fields and personalization
 *
 * Supports categories:
 * - User tags ({user.name}, {user.email}, etc.)
 * - Form tags ({form.title}, {form.id}, etc.)
 * - Submission tags ({submission.id}, {submission.date}, etc.)
 * - Field tags ({field.fieldname}, etc.)
 * - Date/Time tags ({date.today}, {date.year}, etc.)
 * - System tags ({site.name}, {site.url}, etc.)
 * - Custom tags (user-defined)
 * - Conditional tags ({if:condition}content{/if})
 * - Loop tags ({foreach:items}content{/foreach})
 *
 * @version 2.0.0
 */
class FormSmartTagsService
{
    /**
     * Available tag categories and their tags
     */
    private const TAG_CATEGORIES = [
        'user' => [
            'name' => 'User full name',
            'first_name' => 'User first name',
            'last_name' => 'User last name',
            'email' => 'User email address',
            'id' => 'User ID',
            'avatar' => 'User avatar URL',
            'role' => 'User role',
            'created_at' => 'User registration date',
        ],
        'form' => [
            'id' => 'Form ID',
            'title' => 'Form title',
            'description' => 'Form description',
            'slug' => 'Form URL slug',
            'url' => 'Full form URL',
            'submission_count' => 'Total submissions',
        ],
        'submission' => [
            'id' => 'Submission ID',
            'uuid' => 'Submission UUID',
            'date' => 'Submission date',
            'time' => 'Submission time',
            'datetime' => 'Full date and time',
            'status' => 'Submission status',
            'ip' => 'Submitter IP address',
            'browser' => 'Submitter browser',
            'device' => 'Submitter device type',
            'referrer' => 'Referrer URL',
        ],
        'date' => [
            'today' => 'Current date',
            'now' => 'Current date and time',
            'year' => 'Current year',
            'month' => 'Current month',
            'day' => 'Current day',
            'weekday' => 'Current weekday name',
            'timestamp' => 'Unix timestamp',
        ],
        'site' => [
            'name' => 'Site name',
            'url' => 'Site URL',
            'admin_email' => 'Admin email',
            'logo' => 'Site logo URL',
        ],
        'random' => [
            'number' => 'Random number',
            'string' => 'Random string',
            'uuid' => 'Random UUID',
        ],
    ];

    /**
     * Context data
     */
    private ?User $user = null;
    private ?Form $form = null;
    private ?FormSubmission $submission = null;
    private array $formData = [];
    private array $customTags = [];

    /**
     * Process text and replace all smart tags
     *
     * @param string $text Text containing smart tags
     * @param array $context Context data [user, form, submission, formData, customTags]
     * @return string Processed text
     */
    public function process(string $text, array $context = []): string
    {
        // Set context
        $this->user = $context['user'] ?? auth()->user();
        $this->form = $context['form'] ?? null;
        $this->submission = $context['submission'] ?? null;
        $this->formData = $context['formData'] ?? [];
        $this->customTags = $context['customTags'] ?? [];

        // Process in order: conditionals, loops, then simple tags
        $text = $this->processConditionals($text);
        $text = $this->processLoops($text);
        $text = $this->processSimpleTags($text);
        $text = $this->processFieldTags($text);
        $text = $this->processFormatters($text);

        return $text;
    }

    /**
     * Get all available tags with descriptions
     *
     * @return array Categories with tags
     */
    public function getAvailableTags(): array
    {
        $tags = [];

        foreach (self::TAG_CATEGORIES as $category => $categoryTags) {
            $tags[$category] = [];
            foreach ($categoryTags as $tag => $description) {
                $tags[$category][] = [
                    'tag' => "{{$category}.{$tag}}",
                    'description' => $description,
                ];
            }
        }

        // Add field tags if form is set
        if ($this->form) {
            $tags['fields'] = [];
            foreach ($this->form->fields as $field) {
                $tags['fields'][] = [
                    'tag' => "{field.{$field->name}}",
                    'description' => $field->label,
                ];
            }
        }

        return $tags;
    }

    /**
     * Validate tags in text
     *
     * @param string $text Text to validate
     * @return array ['valid' => bool, 'invalid_tags' => array]
     */
    public function validateTags(string $text): array
    {
        $invalidTags = [];

        // Find all tags
        preg_match_all('/\{([a-zA-Z_][a-zA-Z0-9_.]*)\}/', $text, $matches);

        foreach ($matches[1] as $tag) {
            if (!$this->isValidTag($tag)) {
                $invalidTags[] = "{{$tag}}";
            }
        }

        return [
            'valid' => empty($invalidTags),
            'invalid_tags' => $invalidTags,
        ];
    }

    /**
     * Process conditional tags
     * Syntax: {if:condition}content{/if} or {if:condition}content{else}other{/if}
     */
    private function processConditionals(string $text): string
    {
        // Process if/else conditionals
        $pattern = '/\{if:([^}]+)\}(.*?)(?:\{else\}(.*?))?\{\/if\}/s';

        return preg_replace_callback($pattern, function ($matches) {
            $condition = $matches[1];
            $trueContent = $matches[2];
            $falseContent = $matches[3] ?? '';

            $result = $this->evaluateCondition($condition);

            return $result ? $trueContent : $falseContent;
        }, $text);
    }

    /**
     * Process loop tags
     * Syntax: {foreach:array_field}content with {item} and {index}{/foreach}
     */
    private function processLoops(string $text): string
    {
        $pattern = '/\{foreach:([^}]+)\}(.*?)\{\/foreach\}/s';

        return preg_replace_callback($pattern, function ($matches) {
            $arrayField = $matches[1];
            $template = $matches[2];

            $items = $this->getFieldValue($arrayField);

            if (!is_array($items)) {
                return '';
            }

            $output = '';
            foreach ($items as $index => $item) {
                $itemContent = $template;
                $itemContent = str_replace('{item}', is_string($item) ? $item : json_encode($item), $itemContent);
                $itemContent = str_replace('{index}', (string) $index, $itemContent);
                $itemContent = str_replace('{count}', (string) ($index + 1), $itemContent);

                // Handle item properties for objects
                if (is_array($item)) {
                    foreach ($item as $key => $value) {
                        $itemContent = str_replace("{item.{$key}}", (string) $value, $itemContent);
                    }
                }

                $output .= $itemContent;
            }

            return $output;
        }, $text);
    }

    /**
     * Process simple tags {category.tag}
     */
    private function processSimpleTags(string $text): string
    {
        return preg_replace_callback('/\{([a-zA-Z]+)\.([a-zA-Z_]+)\}/', function ($matches) {
            $category = $matches[1];
            $tag = $matches[2];

            return $this->getTagValue($category, $tag);
        }, $text);
    }

    /**
     * Process field tags {field.fieldname}
     */
    private function processFieldTags(string $text): string
    {
        return preg_replace_callback('/\{field\.([a-zA-Z_][a-zA-Z0-9_]*)\}/', function ($matches) {
            $fieldName = $matches[1];
            return $this->getFieldValue($fieldName);
        }, $text);
    }

    /**
     * Process formatters {tag|formatter:options}
     */
    private function processFormatters(string $text): string
    {
        return preg_replace_callback('/\{([^}|]+)\|([a-z]+)(?::([^}]*))?\}/', function ($matches) {
            $value = $matches[1];
            $formatter = $matches[2];
            $options = $matches[3] ?? '';

            // Get the value first if it's a tag reference
            if (preg_match('/^[a-zA-Z]+\.[a-zA-Z_]+$/', $value)) {
                $parts = explode('.', $value);
                $value = $this->getTagValue($parts[0], $parts[1]);
            }

            return $this->applyFormatter($value, $formatter, $options);
        }, $text);
    }

    /**
     * Get tag value by category and tag name
     */
    private function getTagValue(string $category, string $tag): string
    {
        return match ($category) {
            'user' => $this->getUserTag($tag),
            'form' => $this->getFormTag($tag),
            'submission' => $this->getSubmissionTag($tag),
            'date' => $this->getDateTag($tag),
            'site' => $this->getSiteTag($tag),
            'random' => $this->getRandomTag($tag),
            'custom' => $this->customTags[$tag] ?? '',
            default => '',
        };
    }

    /**
     * Get user tag value
     */
    private function getUserTag(string $tag): string
    {
        if (!$this->user) {
            return '';
        }

        return match ($tag) {
            'name' => $this->user->name ?? '',
            'first_name' => Str::before($this->user->name ?? '', ' '),
            'last_name' => Str::after($this->user->name ?? '', ' '),
            'email' => $this->user->email ?? '',
            'id' => (string) ($this->user->id ?? ''),
            'avatar' => $this->user->avatar ?? '',
            'role' => $this->user->getRoleNames()->first() ?? '',
            'created_at' => $this->user->created_at?->format('Y-m-d') ?? '',
            default => '',
        };
    }

    /**
     * Get form tag value
     */
    private function getFormTag(string $tag): string
    {
        if (!$this->form) {
            return '';
        }

        return match ($tag) {
            'id' => (string) $this->form->id,
            'title' => $this->form->title ?? '',
            'description' => $this->form->description ?? '',
            'slug' => $this->form->slug ?? '',
            'url' => url("/forms/{$this->form->slug}"),
            'submission_count' => (string) ($this->form->submission_count ?? 0),
            default => '',
        };
    }

    /**
     * Get submission tag value
     */
    private function getSubmissionTag(string $tag): string
    {
        if (!$this->submission) {
            return '';
        }

        return match ($tag) {
            'id' => (string) $this->submission->id,
            'uuid' => $this->submission->submission_id ?? '',
            'date' => $this->submission->created_at?->format('Y-m-d') ?? '',
            'time' => $this->submission->created_at?->format('H:i:s') ?? '',
            'datetime' => $this->submission->created_at?->format('Y-m-d H:i:s') ?? '',
            'status' => $this->submission->status ?? '',
            'ip' => $this->submission->ip_address ?? '',
            'browser' => $this->extractBrowser($this->submission->user_agent ?? ''),
            'device' => $this->extractDevice($this->submission->user_agent ?? ''),
            'referrer' => $this->submission->referrer ?? '',
            default => '',
        };
    }

    /**
     * Get date tag value
     */
    private function getDateTag(string $tag): string
    {
        $now = Carbon::now();

        return match ($tag) {
            'today' => $now->format('Y-m-d'),
            'now' => $now->format('Y-m-d H:i:s'),
            'year' => $now->format('Y'),
            'month' => $now->format('F'),
            'day' => $now->format('d'),
            'weekday' => $now->format('l'),
            'timestamp' => (string) $now->timestamp,
            default => '',
        };
    }

    /**
     * Get site tag value
     */
    private function getSiteTag(string $tag): string
    {
        return match ($tag) {
            'name' => config('app.name', 'Revolution Trading Pros'),
            'url' => config('app.url', ''),
            'admin_email' => config('mail.from.address', ''),
            'logo' => config('app.logo', ''),
            default => '',
        };
    }

    /**
     * Get random tag value
     */
    private function getRandomTag(string $tag): string
    {
        return match ($tag) {
            'number' => (string) random_int(100000, 999999),
            'string' => Str::random(16),
            'uuid' => Str::uuid()->toString(),
            default => '',
        };
    }

    /**
     * Get field value from form data
     */
    private function getFieldValue(string $fieldName): mixed
    {
        // Check form data first
        if (isset($this->formData[$fieldName])) {
            $value = $this->formData[$fieldName];

            if (is_array($value)) {
                return implode(', ', $value);
            }

            return (string) $value;
        }

        // Check submission data
        if ($this->submission) {
            $field = $this->submission->data->firstWhere('field_name', $fieldName);
            if ($field) {
                return $field->value;
            }
        }

        return '';
    }

    /**
     * Apply formatter to value
     */
    private function applyFormatter(string $value, string $formatter, string $options): string
    {
        return match ($formatter) {
            'upper' => strtoupper($value),
            'lower' => strtolower($value),
            'title' => Str::title($value),
            'slug' => Str::slug($value),
            'trim' => trim($value),
            'truncate' => Str::limit($value, intval($options) ?: 100),
            'date' => $this->formatDate($value, $options ?: 'Y-m-d'),
            'number' => number_format(floatval($value), intval($options) ?: 2),
            'currency' => $this->formatCurrency($value, $options ?: 'USD'),
            'mask' => $this->maskValue($value, $options),
            'default' => $value ?: $options,
            'nl2br' => nl2br($value),
            'strip' => strip_tags($value),
            'escape' => htmlspecialchars($value, ENT_QUOTES, 'UTF-8'),
            'md5' => md5($value),
            'base64' => base64_encode($value),
            default => $value,
        };
    }

    /**
     * Format date value
     */
    private function formatDate(string $value, string $format): string
    {
        try {
            return Carbon::parse($value)->format($format);
        } catch (\Exception $e) {
            return $value;
        }
    }

    /**
     * Format currency value
     */
    private function formatCurrency(string $value, string $currency): string
    {
        $amount = floatval($value);
        $symbols = ['USD' => '$', 'EUR' => '€', 'GBP' => '£', 'JPY' => '¥', 'BRL' => 'R$'];
        $symbol = $symbols[strtoupper($currency)] ?? $currency . ' ';

        return $symbol . number_format($amount, 2);
    }

    /**
     * Mask value for privacy
     */
    private function maskValue(string $value, string $options): string
    {
        $showChars = intval($options) ?: 4;
        $length = strlen($value);

        if ($length <= $showChars) {
            return str_repeat('*', $length);
        }

        return str_repeat('*', $length - $showChars) . substr($value, -$showChars);
    }

    /**
     * Evaluate condition for {if:condition}
     */
    private function evaluateCondition(string $condition): bool
    {
        // Replace field references
        $condition = preg_replace_callback('/field\.([a-zA-Z_][a-zA-Z0-9_]*)/', function ($matches) {
            $value = $this->getFieldValue($matches[1]);
            return is_numeric($value) ? $value : "'{$value}'";
        }, $condition);

        // Simple comparisons
        if (preg_match('/^(.+?)\s*(==|!=|>|<|>=|<=)\s*(.+)$/', $condition, $matches)) {
            $left = $this->resolveValue(trim($matches[1]));
            $operator = $matches[2];
            $right = $this->resolveValue(trim($matches[3]));

            return match ($operator) {
                '==' => $left == $right,
                '!=' => $left != $right,
                '>' => $left > $right,
                '<' => $left < $right,
                '>=' => $left >= $right,
                '<=' => $left <= $right,
                default => false,
            };
        }

        // Check for empty/not empty
        if (Str::startsWith($condition, 'empty:')) {
            $field = Str::after($condition, 'empty:');
            return empty($this->getFieldValue($field));
        }

        if (Str::startsWith($condition, 'not_empty:')) {
            $field = Str::after($condition, 'not_empty:');
            return !empty($this->getFieldValue($field));
        }

        // Check for contains
        if (preg_match('/^contains:(.+?):(.+)$/', $condition, $matches)) {
            $field = trim($matches[1]);
            $needle = trim($matches[2], '"\'');
            return str_contains($this->getFieldValue($field), $needle);
        }

        // Default: check if value is truthy
        return !empty($this->resolveValue($condition));
    }

    /**
     * Resolve a value (could be a tag or literal)
     */
    private function resolveValue(string $value): mixed
    {
        // Remove quotes
        $value = trim($value, '"\'');

        // Check if it's a tag reference
        if (preg_match('/^([a-zA-Z]+)\.([a-zA-Z_]+)$/', $value, $matches)) {
            return $this->getTagValue($matches[1], $matches[2]);
        }

        return $value;
    }

    /**
     * Check if tag is valid
     */
    private function isValidTag(string $tag): bool
    {
        $parts = explode('.', $tag);

        if (count($parts) !== 2) {
            return false;
        }

        [$category, $tagName] = $parts;

        // Check built-in categories
        if (isset(self::TAG_CATEGORIES[$category])) {
            return isset(self::TAG_CATEGORIES[$category][$tagName]);
        }

        // Check field tags
        if ($category === 'field' && $this->form) {
            return $this->form->fields->contains('name', $tagName);
        }

        // Check custom tags
        if ($category === 'custom') {
            return isset($this->customTags[$tagName]);
        }

        return false;
    }

    /**
     * Extract browser from user agent
     */
    private function extractBrowser(string $userAgent): string
    {
        if (str_contains($userAgent, 'Chrome')) return 'Chrome';
        if (str_contains($userAgent, 'Firefox')) return 'Firefox';
        if (str_contains($userAgent, 'Safari')) return 'Safari';
        if (str_contains($userAgent, 'Edge')) return 'Edge';
        if (str_contains($userAgent, 'Opera')) return 'Opera';
        if (str_contains($userAgent, 'MSIE') || str_contains($userAgent, 'Trident')) return 'Internet Explorer';

        return 'Unknown';
    }

    /**
     * Extract device type from user agent
     */
    private function extractDevice(string $userAgent): string
    {
        if (preg_match('/Mobile|Android|iPhone|iPad/', $userAgent)) {
            return str_contains($userAgent, 'iPad') ? 'Tablet' : 'Mobile';
        }

        return 'Desktop';
    }
}
