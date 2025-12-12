<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormSubmission;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Queue;

/**
 * Form Notification Service - Multi-channel notifications
 *
 * Features:
 * - Email notifications (SMTP, SES, Mailgun)
 * - Slack integration
 * - Discord webhooks
 * - Microsoft Teams
 * - Generic webhooks
 * - SMS (Twilio)
 * - Push notifications
 * - Conditional notifications
 * - Rate limiting
 * - Retry logic
 * - Notification templates
 * - Delivery tracking
 *
 * @version 1.0.0
 */
class FormNotificationService
{
    /**
     * Notification channels
     */
    private const CHANNELS = [
        'email' => 'Email',
        'slack' => 'Slack',
        'discord' => 'Discord',
        'teams' => 'Microsoft Teams',
        'webhook' => 'Custom Webhook',
        'sms' => 'SMS',
        'push' => 'Push Notification',
    ];

    /**
     * Retry configuration
     */
    private const MAX_RETRIES = 3;
    private const RETRY_DELAY = 5; // seconds

    /**
     * Rate limit configuration (per channel per form)
     */
    private const RATE_LIMITS = [
        'email' => 100,    // per hour
        'slack' => 60,     // per hour
        'discord' => 60,   // per hour
        'teams' => 60,     // per hour
        'webhook' => 120,  // per hour
        'sms' => 20,       // per hour
    ];

    /**
     * Send notification for form submission
     */
    public function sendSubmissionNotification(Form $form, FormSubmission $submission): array
    {
        $results = [];
        $notifications = $form->notifications ?? [];

        foreach ($notifications as $notification) {
            if (!$this->shouldSend($notification, $submission)) {
                continue;
            }

            $channel = $notification['channel'] ?? 'email';

            // Check rate limit
            if (!$this->checkRateLimit($form->id, $channel)) {
                $results[] = [
                    'channel' => $channel,
                    'status' => 'rate_limited',
                ];
                continue;
            }

            $result = $this->send($channel, $notification, $form, $submission);
            $results[] = $result;

            // Track delivery
            $this->trackDelivery($form->id, $submission->id, $result);
        }

        return $results;
    }

    /**
     * Send notification via channel
     */
    public function send(string $channel, array $config, Form $form, FormSubmission $submission): array
    {
        $result = match ($channel) {
            'email' => $this->sendEmail($config, $form, $submission),
            'slack' => $this->sendSlack($config, $form, $submission),
            'discord' => $this->sendDiscord($config, $form, $submission),
            'teams' => $this->sendTeams($config, $form, $submission),
            'webhook' => $this->sendWebhook($config, $form, $submission),
            'sms' => $this->sendSms($config, $form, $submission),
            default => ['status' => 'error', 'error' => 'Unknown channel'],
        };

        return array_merge(['channel' => $channel], $result);
    }

    /**
     * Send email notification
     */
    private function sendEmail(array $config, Form $form, FormSubmission $submission): array
    {
        try {
            $to = $this->parseRecipients($config['to'] ?? '', $submission);
            $cc = $this->parseRecipients($config['cc'] ?? '', $submission);
            $bcc = $this->parseRecipients($config['bcc'] ?? '', $submission);

            if (empty($to)) {
                return ['status' => 'error', 'error' => 'No recipients'];
            }

            $subject = $this->processTemplate($config['subject'] ?? 'New submission: {form_title}', $form, $submission);
            $body = $this->processTemplate($config['body'] ?? $this->getDefaultEmailBody(), $form, $submission);

            Mail::send([], [], function ($message) use ($to, $cc, $bcc, $subject, $body, $config) {
                $message->to($to);

                if (!empty($cc)) {
                    $message->cc($cc);
                }
                if (!empty($bcc)) {
                    $message->bcc($bcc);
                }

                $message->subject($subject);
                $message->html($body);

                if (!empty($config['reply_to'])) {
                    $message->replyTo($config['reply_to']);
                }
            });

            return ['status' => 'sent', 'recipients' => count($to)];
        } catch (\Exception $e) {
            Log::error('Form email notification failed', [
                'form_id' => $form->id,
                'error' => $e->getMessage(),
            ]);
            return ['status' => 'error', 'error' => $e->getMessage()];
        }
    }

    /**
     * Send Slack notification
     */
    private function sendSlack(array $config, Form $form, FormSubmission $submission): array
    {
        $webhookUrl = $config['webhook_url'] ?? null;

        if (!$webhookUrl) {
            return ['status' => 'error', 'error' => 'No webhook URL'];
        }

        $message = $this->buildSlackMessage($config, $form, $submission);

        return $this->sendToWebhook($webhookUrl, $message);
    }

    /**
     * Build Slack message payload
     */
    private function buildSlackMessage(array $config, Form $form, FormSubmission $submission): array
    {
        $data = $submission->data ?? [];
        $fields = [];

        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $value = implode(', ', $value);
            }
            $fields[] = [
                'type' => 'mrkdwn',
                'text' => "*{$key}:*\n{$value}",
            ];
        }

        return [
            'username' => $config['username'] ?? 'Form Bot',
            'icon_emoji' => $config['icon'] ?? ':clipboard:',
            'attachments' => [
                [
                    'color' => $config['color'] ?? '#3b82f6',
                    'blocks' => [
                        [
                            'type' => 'header',
                            'text' => [
                                'type' => 'plain_text',
                                'text' => "New submission: {$form->title}",
                            ],
                        ],
                        [
                            'type' => 'section',
                            'fields' => array_slice($fields, 0, 10), // Slack limit
                        ],
                        [
                            'type' => 'context',
                            'elements' => [
                                [
                                    'type' => 'mrkdwn',
                                    'text' => "Submitted at: {$submission->created_at->format('M j, Y g:i A')}",
                                ],
                            ],
                        ],
                        [
                            'type' => 'actions',
                            'elements' => [
                                [
                                    'type' => 'button',
                                    'text' => ['type' => 'plain_text', 'text' => 'View Submission'],
                                    'url' => config('app.url') . "/admin/forms/{$form->id}/submissions/{$submission->id}",
                                ],
                            ],
                        ],
                    ],
                ],
            ],
        ];
    }

    /**
     * Send Discord notification
     */
    private function sendDiscord(array $config, Form $form, FormSubmission $submission): array
    {
        $webhookUrl = $config['webhook_url'] ?? null;

        if (!$webhookUrl) {
            return ['status' => 'error', 'error' => 'No webhook URL'];
        }

        $message = $this->buildDiscordMessage($config, $form, $submission);

        return $this->sendToWebhook($webhookUrl, $message);
    }

    /**
     * Build Discord message payload
     */
    private function buildDiscordMessage(array $config, Form $form, FormSubmission $submission): array
    {
        $data = $submission->data ?? [];
        $fields = [];

        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $value = implode(', ', $value);
            }
            $fields[] = [
                'name' => $key,
                'value' => substr((string)$value, 0, 1024) ?: '-',
                'inline' => strlen((string)$value) < 50,
            ];
        }

        return [
            'username' => $config['username'] ?? 'Form Notifications',
            'avatar_url' => $config['avatar_url'] ?? null,
            'embeds' => [
                [
                    'title' => "New submission: {$form->title}",
                    'color' => hexdec(ltrim($config['color'] ?? '#3b82f6', '#')),
                    'fields' => array_slice($fields, 0, 25), // Discord limit
                    'footer' => [
                        'text' => "Submission ID: {$submission->id}",
                    ],
                    'timestamp' => $submission->created_at->toIso8601String(),
                ],
            ],
        ];
    }

    /**
     * Send Microsoft Teams notification
     */
    private function sendTeams(array $config, Form $form, FormSubmission $submission): array
    {
        $webhookUrl = $config['webhook_url'] ?? null;

        if (!$webhookUrl) {
            return ['status' => 'error', 'error' => 'No webhook URL'];
        }

        $message = $this->buildTeamsMessage($config, $form, $submission);

        return $this->sendToWebhook($webhookUrl, $message);
    }

    /**
     * Build Microsoft Teams message payload
     */
    private function buildTeamsMessage(array $config, Form $form, FormSubmission $submission): array
    {
        $data = $submission->data ?? [];
        $facts = [];

        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $value = implode(', ', $value);
            }
            $facts[] = [
                'name' => $key,
                'value' => (string)$value,
            ];
        }

        return [
            '@type' => 'MessageCard',
            '@context' => 'http://schema.org/extensions',
            'themeColor' => ltrim($config['color'] ?? '#3b82f6', '#'),
            'summary' => "New submission: {$form->title}",
            'sections' => [
                [
                    'activityTitle' => "New submission: {$form->title}",
                    'activitySubtitle' => $submission->created_at->format('M j, Y g:i A'),
                    'facts' => array_slice($facts, 0, 20),
                    'markdown' => true,
                ],
            ],
            'potentialAction' => [
                [
                    '@type' => 'OpenUri',
                    'name' => 'View Submission',
                    'targets' => [
                        ['os' => 'default', 'uri' => config('app.url') . "/admin/forms/{$form->id}/submissions/{$submission->id}"],
                    ],
                ],
            ],
        ];
    }

    /**
     * Send generic webhook notification
     */
    private function sendWebhook(array $config, Form $form, FormSubmission $submission): array
    {
        $webhookUrl = $config['url'] ?? null;

        if (!$webhookUrl) {
            return ['status' => 'error', 'error' => 'No webhook URL'];
        }

        $payload = [
            'event' => 'form.submission',
            'timestamp' => now()->toIso8601String(),
            'form' => [
                'id' => $form->id,
                'title' => $form->title,
                'slug' => $form->slug,
            ],
            'submission' => [
                'id' => $submission->id,
                'data' => $submission->data,
                'submitted_at' => $submission->created_at->toIso8601String(),
            ],
        ];

        // Add custom headers
        $headers = $config['headers'] ?? [];

        // Add signature if secret provided
        if (!empty($config['secret'])) {
            $signature = hash_hmac('sha256', json_encode($payload), $config['secret']);
            $headers['X-Signature'] = $signature;
        }

        return $this->sendToWebhook($webhookUrl, $payload, $headers);
    }

    /**
     * Send SMS notification
     */
    private function sendSms(array $config, Form $form, FormSubmission $submission): array
    {
        $to = $config['to'] ?? null;
        $message = $this->processTemplate(
            $config['message'] ?? "New form submission: {form_title}",
            $form,
            $submission
        );

        if (!$to) {
            return ['status' => 'error', 'error' => 'No phone number'];
        }

        // Using Twilio
        $accountSid = config('services.twilio.sid');
        $authToken = config('services.twilio.token');
        $from = config('services.twilio.from');

        if (!$accountSid || !$authToken) {
            return ['status' => 'error', 'error' => 'SMS not configured'];
        }

        try {
            $response = Http::withBasicAuth($accountSid, $authToken)
                ->asForm()
                ->post("https://api.twilio.com/2010-04-01/Accounts/{$accountSid}/Messages.json", [
                    'To' => $to,
                    'From' => $from,
                    'Body' => substr($message, 0, 1600), // SMS limit
                ]);

            if ($response->successful()) {
                return ['status' => 'sent', 'sid' => $response->json('sid')];
            }

            return ['status' => 'error', 'error' => $response->json('message')];
        } catch (\Exception $e) {
            return ['status' => 'error', 'error' => $e->getMessage()];
        }
    }

    /**
     * Send to webhook URL with retry logic
     */
    private function sendToWebhook(string $url, array $payload, array $headers = []): array
    {
        $attempt = 0;

        while ($attempt < self::MAX_RETRIES) {
            try {
                $response = Http::timeout(30)
                    ->withHeaders($headers)
                    ->post($url, $payload);

                if ($response->successful()) {
                    return ['status' => 'sent', 'code' => $response->status()];
                }

                if ($response->status() >= 400 && $response->status() < 500) {
                    // Client error, don't retry
                    return ['status' => 'error', 'error' => "HTTP {$response->status()}", 'body' => $response->body()];
                }
            } catch (\Exception $e) {
                Log::warning('Webhook delivery attempt failed', [
                    'url' => $url,
                    'attempt' => $attempt + 1,
                    'error' => $e->getMessage(),
                ]);
            }

            $attempt++;
            if ($attempt < self::MAX_RETRIES) {
                sleep(self::RETRY_DELAY * $attempt);
            }
        }

        return ['status' => 'error', 'error' => 'Max retries exceeded'];
    }

    /**
     * Check if notification should be sent (conditional logic)
     */
    private function shouldSend(array $notification, FormSubmission $submission): bool
    {
        if (!($notification['enabled'] ?? true)) {
            return false;
        }

        $conditions = $notification['conditions'] ?? [];

        if (empty($conditions)) {
            return true;
        }

        $data = $submission->data ?? [];

        foreach ($conditions as $condition) {
            $field = $condition['field'] ?? null;
            $operator = $condition['operator'] ?? 'equals';
            $value = $condition['value'] ?? null;
            $fieldValue = $data[$field] ?? null;

            $matches = match ($operator) {
                'equals' => $fieldValue == $value,
                'not_equals' => $fieldValue != $value,
                'contains' => is_string($fieldValue) && str_contains($fieldValue, $value),
                'not_contains' => is_string($fieldValue) && !str_contains($fieldValue, $value),
                'empty' => empty($fieldValue),
                'not_empty' => !empty($fieldValue),
                'greater_than' => is_numeric($fieldValue) && $fieldValue > $value,
                'less_than' => is_numeric($fieldValue) && $fieldValue < $value,
                default => true,
            };

            // If any condition fails with AND logic
            if (!$matches && ($notification['condition_logic'] ?? 'and') === 'and') {
                return false;
            }

            // If any condition passes with OR logic
            if ($matches && ($notification['condition_logic'] ?? 'and') === 'or') {
                return true;
            }
        }

        return ($notification['condition_logic'] ?? 'and') === 'and';
    }

    /**
     * Parse recipients from string (supports field references)
     */
    private function parseRecipients(string $recipients, FormSubmission $submission): array
    {
        if (empty($recipients)) {
            return [];
        }

        $data = $submission->data ?? [];
        $emails = [];

        foreach (explode(',', $recipients) as $recipient) {
            $recipient = trim($recipient);

            // Check if it's a field reference {field_name}
            if (preg_match('/^\{([^}]+)\}$/', $recipient, $matches)) {
                $fieldValue = $data[$matches[1]] ?? null;
                if (filter_var($fieldValue, FILTER_VALIDATE_EMAIL)) {
                    $emails[] = $fieldValue;
                }
            } elseif (filter_var($recipient, FILTER_VALIDATE_EMAIL)) {
                $emails[] = $recipient;
            }
        }

        return array_unique($emails);
    }

    /**
     * Process template with form/submission data
     */
    private function processTemplate(string $template, Form $form, FormSubmission $submission): string
    {
        $data = $submission->data ?? [];

        // Form variables
        $template = str_replace('{form_title}', $form->title, $template);
        $template = str_replace('{form_id}', (string)$form->id, $template);
        $template = str_replace('{submission_id}', (string)$submission->id, $template);
        $template = str_replace('{submission_date}', $submission->created_at->format('M j, Y g:i A'), $template);

        // Field variables
        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $value = implode(', ', $value);
            }
            $template = str_replace("{{$key}}", (string)$value, $template);
        }

        // All fields
        $template = str_replace('{all_fields}', $this->formatAllFields($data), $template);

        return $template;
    }

    /**
     * Format all fields for display
     */
    private function formatAllFields(array $data): string
    {
        $lines = [];
        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $value = implode(', ', $value);
            }
            $lines[] = "<strong>{$key}:</strong> {$value}";
        }
        return implode('<br>', $lines);
    }

    /**
     * Get default email body template
     */
    private function getDefaultEmailBody(): string
    {
        return <<<HTML
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #3b82f6; color: white; padding: 20px; border-radius: 8px 8px 0 0; }
        .content { background: #f9fafb; padding: 20px; border: 1px solid #e5e7eb; }
        .field { margin-bottom: 15px; }
        .field-label { font-weight: 600; color: #374151; }
        .field-value { margin-top: 4px; }
        .footer { padding: 15px; text-align: center; color: #6b7280; font-size: 12px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2 style="margin: 0;">New Form Submission</h2>
            <p style="margin: 5px 0 0 0; opacity: 0.9;">{form_title}</p>
        </div>
        <div class="content">
            {all_fields}
        </div>
        <div class="footer">
            Submitted on {submission_date}
        </div>
    </div>
</body>
</html>
HTML;
    }

    /**
     * Check rate limit
     */
    private function checkRateLimit(int $formId, string $channel): bool
    {
        $key = "form_notification_rate:{$formId}:{$channel}";
        $limit = self::RATE_LIMITS[$channel] ?? 60;

        $current = cache()->get($key, 0);

        if ($current >= $limit) {
            return false;
        }

        cache()->put($key, $current + 1, now()->addHour());
        return true;
    }

    /**
     * Track delivery
     */
    private function trackDelivery(int $formId, int $submissionId, array $result): void
    {
        \DB::table('form_notification_logs')->insert([
            'form_id' => $formId,
            'submission_id' => $submissionId,
            'channel' => $result['channel'],
            'status' => $result['status'],
            'error' => $result['error'] ?? null,
            'created_at' => now(),
        ]);
    }

    /**
     * Get notification logs
     */
    public function getNotificationLogs(int $formId, int $limit = 100): array
    {
        return \DB::table('form_notification_logs')
            ->where('form_id', $formId)
            ->orderByDesc('created_at')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get delivery stats
     */
    public function getDeliveryStats(int $formId, int $days = 30): array
    {
        $since = now()->subDays($days);

        return [
            'total' => \DB::table('form_notification_logs')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->count(),

            'by_status' => \DB::table('form_notification_logs')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->select('status')
                ->selectRaw('COUNT(*) as count')
                ->groupBy('status')
                ->pluck('count', 'status')
                ->toArray(),

            'by_channel' => \DB::table('form_notification_logs')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->select('channel')
                ->selectRaw('COUNT(*) as count')
                ->groupBy('channel')
                ->pluck('count', 'channel')
                ->toArray(),

            'success_rate' => $this->calculateSuccessRate($formId, $since),
        ];
    }

    /**
     * Calculate success rate
     */
    private function calculateSuccessRate(int $formId, $since): float
    {
        $total = \DB::table('form_notification_logs')
            ->where('form_id', $formId)
            ->where('created_at', '>=', $since)
            ->count();

        if ($total === 0) {
            return 100.0;
        }

        $successful = \DB::table('form_notification_logs')
            ->where('form_id', $formId)
            ->where('created_at', '>=', $since)
            ->where('status', 'sent')
            ->count();

        return round(($successful / $total) * 100, 2);
    }

    /**
     * Get available channels
     */
    public function getAvailableChannels(): array
    {
        return self::CHANNELS;
    }

    /**
     * Test notification
     */
    public function testNotification(string $channel, array $config): array
    {
        // Create mock form and submission
        $mockForm = new Form([
            'id' => 0,
            'title' => 'Test Form',
            'slug' => 'test-form',
        ]);

        $mockSubmission = new FormSubmission([
            'id' => 0,
            'data' => [
                'name' => 'John Doe',
                'email' => 'john@example.com',
                'message' => 'This is a test submission',
            ],
            'created_at' => now(),
        ]);

        return $this->send($channel, $config, $mockForm, $mockSubmission);
    }
}
