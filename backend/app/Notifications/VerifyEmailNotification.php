<?php

namespace App\Notifications;

use App\Models\EmailTemplate;
use App\Services\Email\EmailTemplateRenderService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Carbon;
use Illuminate\Support\Facades\Config;
use Illuminate\Support\Facades\URL;

/**
 * Custom email verification notification with branded template.
 * Supports admin-customizable templates with Blade fallback.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
class VerifyEmailNotification extends Notification implements ShouldQueue
{
    use Queueable;

    /**
     * The number of times the job may be attempted.
     */
    public int $tries = 3;

    /**
     * The number of seconds to wait before retrying.
     */
    public int $backoff = 60;

    /**
     * Get the notification's delivery channels.
     */
    public function via(object $notifiable): array
    {
        return ['mail'];
    }

    /**
     * Get the mail representation of the notification.
     */
    public function toMail(object $notifiable): MailMessage
    {
        $verificationUrl = $this->verificationUrl($notifiable);

        // Try to use admin-customizable database template
        $template = EmailTemplate::where('slug', 'verify-email')
            ->where('status', 'published')
            ->first();

        if ($template) {
            try {
                $renderService = app(EmailTemplateRenderService::class);
                $rendered = $renderService->render($template, [
                    'user_name' => $notifiable->name,
                    'verification_url' => $verificationUrl,
                    'app_url' => Config::get('app.url'),
                ]);

                return (new MailMessage)
                    ->subject($rendered['subject'])
                    ->html($rendered['html']);
            } catch (\Exception $e) {
                // Fall through to Blade template on error
                report($e);
            }
        }

        // Fallback to Blade template
        return (new MailMessage)
            ->subject('Verify Your Email - Revolution Trading Pros')
            ->view('emails.auth.verify-email', [
                'user' => $notifiable,
                'verificationUrl' => $verificationUrl,
            ]);
    }

    /**
     * Get the verification URL for the given notifiable.
     */
    protected function verificationUrl(object $notifiable): string
    {
        $frontendUrl = Config::get('app.frontend_url', Config::get('app.url'));

        // Create signed URL for API verification
        $apiUrl = URL::temporarySignedRoute(
            'verification.verify',
            Carbon::now()->addMinutes(Config::get('auth.verification.expire', 60)),
            [
                'id' => $notifiable->getKey(),
                'hash' => sha1($notifiable->getEmailForVerification()),
            ]
        );

        // Extract signature and expiration for frontend
        $parsed = parse_url($apiUrl);
        parse_str($parsed['query'] ?? '', $query);

        // Build frontend verification URL
        return $frontendUrl . '/verify-email?' . http_build_query([
            'id' => $notifiable->getKey(),
            'hash' => sha1($notifiable->getEmailForVerification()),
            'expires' => $query['expires'] ?? '',
            'signature' => $query['signature'] ?? '',
        ]);
    }

    /**
     * Get the array representation of the notification.
     */
    public function toArray(object $notifiable): array
    {
        return [
            'user_id' => $notifiable->id,
            'email' => $notifiable->email,
            'type' => 'email_verification',
        ];
    }
}
