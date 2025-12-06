<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Carbon;

/**
 * FormIntegration Model - FluentForm Pro Integrations
 *
 * Manages third-party integrations for forms including email marketing,
 * CRMs, webhooks, messaging platforms, and payment processors.
 *
 * @property int $id
 * @property int $form_id
 * @property string $integration_type Integration category
 * @property string $provider Integration provider name
 * @property string $name User-defined integration name
 * @property array $settings Provider-specific settings
 * @property array $field_mapping Form field to provider field mapping
 * @property array $conditional_logic Conditional execution rules
 * @property bool $active Active status
 * @property int $priority Execution order
 * @property Carbon|null $last_run_at Last successful execution
 * @property string|null $last_error Last error message
 * @property int $success_count Successful executions
 * @property int $failure_count Failed executions
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 *
 * @property-read Form $form
 */
class FormIntegration extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'form_integrations';

    protected $fillable = [
        'form_id',
        'integration_type',
        'provider',
        'name',
        'settings',
        'field_mapping',
        'conditional_logic',
        'active',
        'priority',
        'last_run_at',
        'last_error',
        'success_count',
        'failure_count',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'settings' => 'encrypted:array',
        'field_mapping' => 'array',
        'conditional_logic' => 'array',
        'active' => 'boolean',
        'priority' => 'integer',
        'last_run_at' => 'datetime',
        'success_count' => 'integer',
        'failure_count' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'active' => true,
        'priority' => 10,
        'success_count' => 0,
        'failure_count' => 0,
    ];

    // =========================================================================
    // INTEGRATION TYPE CONSTANTS
    // =========================================================================

    public const TYPE_WEBHOOK = 'webhook';
    public const TYPE_EMAIL_MARKETING = 'email_marketing';
    public const TYPE_CRM = 'crm';
    public const TYPE_MESSAGING = 'messaging';
    public const TYPE_PAYMENT = 'payment';
    public const TYPE_AUTOMATION = 'automation';
    public const TYPE_STORAGE = 'storage';
    public const TYPE_NOTIFICATION = 'notification';
    public const TYPE_USER = 'user';

    public const INTEGRATION_TYPES = [
        self::TYPE_WEBHOOK,
        self::TYPE_EMAIL_MARKETING,
        self::TYPE_CRM,
        self::TYPE_MESSAGING,
        self::TYPE_PAYMENT,
        self::TYPE_AUTOMATION,
        self::TYPE_STORAGE,
        self::TYPE_NOTIFICATION,
        self::TYPE_USER,
    ];

    // =========================================================================
    // PROVIDER CONSTANTS - FluentForm Pro v6.1.8 Integrations
    // =========================================================================

    // Email Marketing Providers
    public const PROVIDER_ACTIVECAMPAIGN = 'activecampaign';
    public const PROVIDER_MAILCHIMP = 'mailchimp';
    public const PROVIDER_CONVERTKIT = 'convertkit';
    public const PROVIDER_MAILERLITE = 'mailerlite';
    public const PROVIDER_GETRESPONSE = 'getresponse';
    public const PROVIDER_DRIP = 'drip';
    public const PROVIDER_SENDINBLUE = 'sendinblue';
    public const PROVIDER_CAMPAIGNMONITOR = 'campaignmonitor';
    public const PROVIDER_CONSTANTCONTACT = 'constantcontact';
    public const PROVIDER_MOOSEND = 'moosend';
    public const PROVIDER_SENDFOX = 'sendfox';
    public const PROVIDER_AUTOMIZY = 'automizy';
    public const PROVIDER_CLEVERREACH = 'cleverreach';
    public const PROVIDER_MAILJET = 'mailjet';
    public const PROVIDER_MAILSTER = 'mailster';
    public const PROVIDER_GETGIST = 'getgist';
    public const PROVIDER_PLATFORMLY = 'platformly';
    public const PROVIDER_ICONTACT = 'icontact';

    // CRM Providers
    public const PROVIDER_HUBSPOT = 'hubspot';
    public const PROVIDER_SALESFORCE = 'salesforce';
    public const PROVIDER_PIPEDRIVE = 'pipedrive';
    public const PROVIDER_ZOHOCRM = 'zohocrm';
    public const PROVIDER_AMOCRM = 'amocrm';
    public const PROVIDER_ONEPAGECRM = 'onepagecrm';
    public const PROVIDER_SALESFLARE = 'salesflare';
    public const PROVIDER_INSIGHTLY = 'insightly';

    // Messaging Providers
    public const PROVIDER_SLACK = 'slack';
    public const PROVIDER_DISCORD = 'discord';
    public const PROVIDER_TELEGRAM = 'telegram';
    public const PROVIDER_SMS = 'sms';
    public const PROVIDER_CLICKSEND = 'clicksend';

    // Automation Providers
    public const PROVIDER_ZAPIER = 'zapier';
    public const PROVIDER_WEBHOOK = 'webhook';
    public const PROVIDER_MAKE = 'make';
    public const PROVIDER_PABBLY = 'pabbly';

    // Storage/Spreadsheet Providers
    public const PROVIDER_GOOGLESHEETS = 'googlesheets';
    public const PROVIDER_AIRTABLE = 'airtable';
    public const PROVIDER_NOTION = 'notion';
    public const PROVIDER_TRELLO = 'trello';

    // Payment Providers
    public const PROVIDER_STRIPE = 'stripe';
    public const PROVIDER_PAYPAL = 'paypal';
    public const PROVIDER_SQUARE = 'square';
    public const PROVIDER_RAZORPAY = 'razorpay';
    public const PROVIDER_MOLLIE = 'mollie';
    public const PROVIDER_PAYSTACK = 'paystack';
    public const PROVIDER_PADDLE = 'paddle';
    public const PROVIDER_AUTHORIZENET = 'authorizenet';
    public const PROVIDER_OFFLINE = 'offline';

    // User/Auth Providers
    public const PROVIDER_USER_REGISTRATION = 'user_registration';
    public const PROVIDER_USER_UPDATE = 'user_update';

    /**
     * All available providers grouped by type.
     */
    public const PROVIDERS = [
        self::TYPE_EMAIL_MARKETING => [
            self::PROVIDER_ACTIVECAMPAIGN => 'ActiveCampaign',
            self::PROVIDER_MAILCHIMP => 'Mailchimp',
            self::PROVIDER_CONVERTKIT => 'ConvertKit',
            self::PROVIDER_MAILERLITE => 'MailerLite',
            self::PROVIDER_GETRESPONSE => 'GetResponse',
            self::PROVIDER_DRIP => 'Drip',
            self::PROVIDER_SENDINBLUE => 'Brevo (Sendinblue)',
            self::PROVIDER_CAMPAIGNMONITOR => 'Campaign Monitor',
            self::PROVIDER_CONSTANTCONTACT => 'Constant Contact',
            self::PROVIDER_MOOSEND => 'Moosend',
            self::PROVIDER_SENDFOX => 'SendFox',
            self::PROVIDER_AUTOMIZY => 'Automizy',
            self::PROVIDER_CLEVERREACH => 'CleverReach',
            self::PROVIDER_MAILJET => 'Mailjet',
            self::PROVIDER_MAILSTER => 'Mailster',
            self::PROVIDER_GETGIST => 'GetGist',
            self::PROVIDER_PLATFORMLY => 'Platformly',
            self::PROVIDER_ICONTACT => 'iContact',
        ],
        self::TYPE_CRM => [
            self::PROVIDER_HUBSPOT => 'HubSpot',
            self::PROVIDER_SALESFORCE => 'Salesforce',
            self::PROVIDER_PIPEDRIVE => 'Pipedrive',
            self::PROVIDER_ZOHOCRM => 'Zoho CRM',
            self::PROVIDER_AMOCRM => 'amoCRM',
            self::PROVIDER_ONEPAGECRM => 'OnePageCRM',
            self::PROVIDER_SALESFLARE => 'Salesflare',
            self::PROVIDER_INSIGHTLY => 'Insightly',
        ],
        self::TYPE_MESSAGING => [
            self::PROVIDER_SLACK => 'Slack',
            self::PROVIDER_DISCORD => 'Discord',
            self::PROVIDER_TELEGRAM => 'Telegram',
            self::PROVIDER_SMS => 'SMS Notification',
            self::PROVIDER_CLICKSEND => 'ClickSend',
        ],
        self::TYPE_AUTOMATION => [
            self::PROVIDER_ZAPIER => 'Zapier',
            self::PROVIDER_WEBHOOK => 'Custom Webhook',
            self::PROVIDER_MAKE => 'Make (Integromat)',
            self::PROVIDER_PABBLY => 'Pabbly Connect',
        ],
        self::TYPE_STORAGE => [
            self::PROVIDER_GOOGLESHEETS => 'Google Sheets',
            self::PROVIDER_AIRTABLE => 'Airtable',
            self::PROVIDER_NOTION => 'Notion',
            self::PROVIDER_TRELLO => 'Trello',
        ],
        self::TYPE_PAYMENT => [
            self::PROVIDER_STRIPE => 'Stripe',
            self::PROVIDER_PAYPAL => 'PayPal',
            self::PROVIDER_SQUARE => 'Square',
            self::PROVIDER_RAZORPAY => 'Razorpay',
            self::PROVIDER_MOLLIE => 'Mollie',
            self::PROVIDER_PAYSTACK => 'Paystack',
            self::PROVIDER_PADDLE => 'Paddle',
            self::PROVIDER_AUTHORIZENET => 'Authorize.Net',
            self::PROVIDER_OFFLINE => 'Offline Payment',
        ],
        self::TYPE_USER => [
            self::PROVIDER_USER_REGISTRATION => 'User Registration',
            self::PROVIDER_USER_UPDATE => 'User Update',
        ],
    ];

    /**
     * Provider-specific required settings.
     */
    public const PROVIDER_SETTINGS = [
        self::PROVIDER_ACTIVECAMPAIGN => ['api_url', 'api_key', 'list_id'],
        self::PROVIDER_MAILCHIMP => ['api_key', 'list_id', 'double_optin'],
        self::PROVIDER_CONVERTKIT => ['api_key', 'api_secret', 'form_id'],
        self::PROVIDER_MAILERLITE => ['api_key', 'group_id'],
        self::PROVIDER_HUBSPOT => ['api_key', 'portal_id'],
        self::PROVIDER_SALESFORCE => ['client_id', 'client_secret', 'refresh_token'],
        self::PROVIDER_PIPEDRIVE => ['api_token', 'domain'],
        self::PROVIDER_SLACK => ['webhook_url', 'channel'],
        self::PROVIDER_DISCORD => ['webhook_url'],
        self::PROVIDER_TELEGRAM => ['bot_token', 'chat_id'],
        self::PROVIDER_ZAPIER => ['webhook_url'],
        self::PROVIDER_WEBHOOK => ['url', 'method', 'headers', 'auth_type'],
        self::PROVIDER_GOOGLESHEETS => ['spreadsheet_id', 'sheet_name', 'credentials'],
        self::PROVIDER_AIRTABLE => ['api_key', 'base_id', 'table_name'],
        self::PROVIDER_NOTION => ['api_key', 'database_id'],
        self::PROVIDER_STRIPE => ['secret_key', 'publishable_key', 'webhook_secret'],
        self::PROVIDER_PAYPAL => ['client_id', 'client_secret', 'mode'],
        self::PROVIDER_SQUARE => ['access_token', 'application_id', 'location_id'],
        self::PROVIDER_RAZORPAY => ['key_id', 'key_secret', 'webhook_secret'],
        self::PROVIDER_MOLLIE => ['api_key', 'webhook_url'],
        self::PROVIDER_PAYSTACK => ['public_key', 'secret_key', 'webhook_secret'],
        self::PROVIDER_PADDLE => ['vendor_id', 'vendor_auth_code', 'public_key'],
        self::PROVIDER_AUTHORIZENET => ['api_login_id', 'transaction_key', 'client_key'],
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the form this integration belongs to.
     */
    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to active integrations.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('active', true);
    }

    /**
     * Scope to integrations by type.
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('integration_type', $type);
    }

    /**
     * Scope to integrations by provider.
     */
    public function scopeForProvider(Builder $query, string $provider): Builder
    {
        return $query->where('provider', $provider);
    }

    /**
     * Scope to order by priority.
     */
    public function scopeOrdered(Builder $query): Builder
    {
        return $query->orderBy('priority', 'asc');
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    /**
     * Get provider display name.
     */
    public function getProviderLabelAttribute(): string
    {
        foreach (self::PROVIDERS as $type => $providers) {
            if (isset($providers[$this->provider])) {
                return $providers[$this->provider];
            }
        }
        return ucfirst($this->provider);
    }

    /**
     * Get integration type label.
     */
    public function getTypeLabelAttribute(): string
    {
        return match ($this->integration_type) {
            self::TYPE_WEBHOOK => 'Webhook',
            self::TYPE_EMAIL_MARKETING => 'Email Marketing',
            self::TYPE_CRM => 'CRM',
            self::TYPE_MESSAGING => 'Messaging',
            self::TYPE_PAYMENT => 'Payment',
            self::TYPE_AUTOMATION => 'Automation',
            self::TYPE_STORAGE => 'Storage',
            self::TYPE_NOTIFICATION => 'Notification',
            self::TYPE_USER => 'User Management',
            default => ucfirst($this->integration_type),
        };
    }

    /**
     * Get success rate as percentage.
     */
    public function getSuccessRateAttribute(): float
    {
        $total = $this->success_count + $this->failure_count;
        if ($total === 0) {
            return 0;
        }
        return round(($this->success_count / $total) * 100, 2);
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Record a successful execution.
     */
    public function recordSuccess(): void
    {
        $this->increment('success_count');
        $this->update([
            'last_run_at' => now(),
            'last_error' => null,
        ]);
    }

    /**
     * Record a failed execution.
     */
    public function recordFailure(string $error): void
    {
        $this->increment('failure_count');
        $this->update([
            'last_run_at' => now(),
            'last_error' => $error,
        ]);
    }

    /**
     * Check if integration should execute based on conditional logic.
     */
    public function shouldExecute(array $submissionData): bool
    {
        if (empty($this->conditional_logic)) {
            return true;
        }

        // Evaluate conditional logic
        return $this->evaluateConditions($submissionData);
    }

    /**
     * Evaluate conditional logic rules.
     */
    protected function evaluateConditions(array $data): bool
    {
        $logic = $this->conditional_logic;
        $conditions = $logic['conditions'] ?? [];
        $operator = $logic['operator'] ?? 'and';

        if (empty($conditions)) {
            return true;
        }

        $results = [];
        foreach ($conditions as $condition) {
            $fieldValue = $data[$condition['field']] ?? null;
            $compareValue = $condition['value'] ?? null;
            $compareOperator = $condition['operator'] ?? 'equals';

            $results[] = $this->evaluateCondition($fieldValue, $compareOperator, $compareValue);
        }

        if ($operator === 'and') {
            return !in_array(false, $results, true);
        }

        return in_array(true, $results, true);
    }

    /**
     * Evaluate a single condition.
     */
    protected function evaluateCondition(mixed $fieldValue, string $operator, mixed $compareValue): bool
    {
        return match ($operator) {
            'equals', '==' => $fieldValue == $compareValue,
            'not_equals', '!=' => $fieldValue != $compareValue,
            'contains' => str_contains((string) $fieldValue, (string) $compareValue),
            'not_contains' => !str_contains((string) $fieldValue, (string) $compareValue),
            'starts_with' => str_starts_with((string) $fieldValue, (string) $compareValue),
            'ends_with' => str_ends_with((string) $fieldValue, (string) $compareValue),
            'greater_than', '>' => (float) $fieldValue > (float) $compareValue,
            'less_than', '<' => (float) $fieldValue < (float) $compareValue,
            'greater_or_equal', '>=' => (float) $fieldValue >= (float) $compareValue,
            'less_or_equal', '<=' => (float) $fieldValue <= (float) $compareValue,
            'is_empty' => empty($fieldValue),
            'is_not_empty' => !empty($fieldValue),
            'in' => in_array($fieldValue, (array) $compareValue),
            'not_in' => !in_array($fieldValue, (array) $compareValue),
            default => true,
        };
    }

    /**
     * Map form submission data to provider fields.
     */
    public function mapFields(array $submissionData): array
    {
        $mapped = [];
        foreach ($this->field_mapping as $providerField => $formField) {
            if (is_array($formField)) {
                // Complex mapping with transformations
                $value = $submissionData[$formField['field']] ?? null;
                if (isset($formField['transform'])) {
                    $value = $this->transformValue($value, $formField['transform']);
                }
                $mapped[$providerField] = $value;
            } else {
                // Simple field mapping
                $mapped[$providerField] = $submissionData[$formField] ?? null;
            }
        }
        return $mapped;
    }

    /**
     * Transform a value based on transformation rules.
     */
    protected function transformValue(mixed $value, string $transform): mixed
    {
        return match ($transform) {
            'uppercase' => strtoupper((string) $value),
            'lowercase' => strtolower((string) $value),
            'trim' => trim((string) $value),
            'int' => (int) $value,
            'float' => (float) $value,
            'bool' => (bool) $value,
            'json' => json_encode($value),
            'date' => date('Y-m-d', strtotime((string) $value)),
            'datetime' => date('Y-m-d H:i:s', strtotime((string) $value)),
            default => $value,
        };
    }

    /**
     * Get all available providers as flat array.
     */
    public static function getAllProviders(): array
    {
        $all = [];
        foreach (self::PROVIDERS as $type => $providers) {
            foreach ($providers as $key => $label) {
                $all[$key] = [
                    'label' => $label,
                    'type' => $type,
                ];
            }
        }
        return $all;
    }

    /**
     * Get required settings for a provider.
     */
    public static function getRequiredSettings(string $provider): array
    {
        return self::PROVIDER_SETTINGS[$provider] ?? [];
    }
}
