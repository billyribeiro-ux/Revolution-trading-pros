<?php

declare(strict_types=1);

namespace App\Jobs;

use App\Models\EmailCampaign;
use App\Models\EmailLog;
use App\Models\User;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Log;

/**
 * SendEmailCampaign Job
 * 
 * Handles the asynchronous sending of email campaigns
 * 
 * @version 1.0.0
 */
class SendEmailCampaign implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    /**
     * The number of times the job may be attempted.
     */
    public int $tries = 3;

    /**
     * The number of seconds to wait before retrying the job.
     */
    public int $backoff = 60;

    /**
     * Create a new job instance.
     */
    public function __construct(
        public EmailCampaign $campaign
    ) {}

    /**
     * Execute the job.
     */
    public function handle(): void
    {
        $campaign = $this->campaign->fresh();

        if (!$campaign || $campaign->status !== 'sending') {
            Log::info("Campaign {$campaign->id} is not in sending status, skipping.");
            return;
        }

        Log::info("Starting to send campaign: {$campaign->name} (ID: {$campaign->id})");

        try {
            // Get recipients based on segment
            $recipients = $this->getRecipients($campaign);
            
            $campaign->update(['total_recipients' => $recipients->count()]);

            foreach ($recipients as $user) {
                $this->sendToRecipient($campaign, $user);
            }

            // Mark campaign as sent
            $campaign->markAsSent();
            
            Log::info("Campaign {$campaign->id} completed. Sent: {$campaign->sent_count}");

        } catch (\Exception $e) {
            Log::error("Failed to send campaign {$campaign->id}: " . $e->getMessage());
            
            // If we've exhausted retries, mark as failed
            if ($this->attempts() >= $this->tries) {
                $campaign->update(['status' => 'draft']);
            }
            
            throw $e;
        }
    }

    /**
     * Get recipients for the campaign based on segment
     */
    private function getRecipients(EmailCampaign $campaign)
    {
        $query = User::whereNotNull('email_verified_at')
            ->whereNotNull('email');

        // TODO: Implement segment filtering based on segment_id
        // For now, return all verified users

        return $query->get();
    }

    /**
     * Send email to a single recipient
     */
    private function sendToRecipient(EmailCampaign $campaign, User $user): void
    {
        try {
            // Get template content
            $template = $campaign->template;
            if (!$template) {
                Log::warning("Campaign {$campaign->id} has no template");
                return;
            }

            // Personalize content
            $html = $this->personalizeContent($template->html_content ?? $template->content ?? '', $user);
            $subject = $this->personalizeContent($campaign->subject, $user);

            // Handle A/B testing
            if ($campaign->ab_test_config && ($campaign->ab_test_config['enabled'] ?? false)) {
                $subject = $this->getABTestSubject($campaign, $user);
            }

            // Send email (using Laravel's mail facade)
            // In production, you'd use a proper mail service like SendGrid, Mailgun, etc.
            Mail::send([], [], function ($message) use ($campaign, $user, $subject, $html) {
                $message->to($user->email, $user->name)
                    ->from($campaign->from_email ?? config('mail.from.address'), $campaign->from_name ?? config('mail.from.name'))
                    ->subject($subject)
                    ->html($html);

                if ($campaign->reply_to) {
                    $message->replyTo($campaign->reply_to);
                }
            });

            // Log the sent email
            EmailLog::create([
                'campaign_id' => $campaign->id,
                'user_id' => $user->id,
                'email' => $user->email,
                'event_type' => 'sent',
            ]);

            // Increment sent count
            $campaign->incrementSent();

        } catch (\Exception $e) {
            Log::error("Failed to send email to {$user->email} for campaign {$campaign->id}: " . $e->getMessage());
            
            // Log bounce
            EmailLog::create([
                'campaign_id' => $campaign->id,
                'user_id' => $user->id,
                'email' => $user->email,
                'event_type' => 'bounced',
                'metadata' => ['error' => $e->getMessage()],
            ]);

            $campaign->incrementBounced();
        }
    }

    /**
     * Personalize content with user data
     */
    private function personalizeContent(string $content, User $user): string
    {
        $replacements = [
            '{{first_name}}' => $user->first_name ?? $user->name,
            '{{ first_name }}' => $user->first_name ?? $user->name,
            '{{last_name}}' => $user->last_name ?? '',
            '{{ last_name }}' => $user->last_name ?? '',
            '{{name}}' => $user->name,
            '{{ name }}' => $user->name,
            '{{email}}' => $user->email,
            '{{ email }}' => $user->email,
        ];

        return str_replace(array_keys($replacements), array_values($replacements), $content);
    }

    /**
     * Get subject line for A/B test
     */
    private function getABTestSubject(EmailCampaign $campaign, User $user): string
    {
        $config = $campaign->ab_test_config;
        $splitPercentage = $config['split_percentage'] ?? 50;

        // Use user ID to consistently assign to variant
        $isVariantA = ($user->id % 100) < $splitPercentage;

        return $isVariantA ? $campaign->subject : ($config['subject_b'] ?? $campaign->subject);
    }

    /**
     * Handle a job failure.
     */
    public function failed(\Throwable $exception): void
    {
        Log::error("SendEmailCampaign job failed for campaign {$this->campaign->id}: " . $exception->getMessage());
    }
}
