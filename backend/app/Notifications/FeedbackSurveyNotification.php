<?php

namespace App\Notifications;

use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Facades\Config;
use Illuminate\Support\Str;

/**
 * Feedback survey notification for churned members.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class FeedbackSurveyNotification extends Notification implements ShouldQueue
{
    use Queueable;

    public int $tries = 3;
    public int $backoff = 60;

    protected string $surveyToken;

    public function __construct(
        protected ?object $incentive = null
    ) {
        $this->surveyToken = Str::random(32);
    }

    /**
     * Get the notification's delivery channels.
     */
    public function via(object $notifiable): array
    {
        return ['mail', 'database'];
    }

    /**
     * Get the mail representation of the notification.
     */
    public function toMail(object $notifiable): MailMessage
    {
        $frontendUrl = Config::get('app.frontend_url', Config::get('app.url'));
        $surveyUrl = "{$frontendUrl}/feedback?token={$this->surveyToken}&user={$notifiable->id}";

        return (new MailMessage)
            ->subject("We Value Your Feedback - Revolution Trading Pros")
            ->view('emails.membership.feedback-survey', [
                'user' => $notifiable,
                'surveyUrl' => $surveyUrl,
                'incentive' => $this->incentive,
            ]);
    }

    /**
     * Get the array representation of the notification.
     */
    public function toArray(object $notifiable): array
    {
        return [
            'type' => 'feedback_survey',
            'survey_token' => $this->surveyToken,
            'has_incentive' => $this->incentive !== null,
            'message' => 'We would love to hear your feedback!',
        ];
    }
}
