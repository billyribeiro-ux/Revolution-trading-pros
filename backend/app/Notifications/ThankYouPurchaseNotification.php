<?php

namespace App\Notifications;

use App\Models\EmailTemplate;
use App\Models\Order;
use App\Models\Product;
use App\Services\Email\EmailTemplateRenderService;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;
use Illuminate\Support\Facades\Config;

/**
 * Thank you notification for product purchases (indicators, courses, etc).
 * Supports admin-customizable templates with Blade fallback.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
class ThankYouPurchaseNotification extends Notification implements ShouldQueue
{
    use Queueable;

    public int $tries = 3;
    public int $backoff = 60;

    public function __construct(
        protected Order $order,
        protected Product $product,
        protected string $productType = 'product' // 'indicator', 'course', 'product'
    ) {}

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
        // Determine template slug based on product type
        $templateSlug = match ($this->productType) {
            'indicator' => 'thank-you-indicator',
            'course' => 'thank-you-course',
            default => 'thank-you-indicator',
        };

        // Try to use admin-customizable database template
        $template = EmailTemplate::where('slug', $templateSlug)
            ->where('status', 'published')
            ->first();

        if ($template) {
            try {
                $renderService = app(EmailTemplateRenderService::class);
                $frontendUrl = Config::get('app.frontend_url', Config::get('app.url'));

                $variables = [
                    'user_name' => $notifiable->name,
                    'product_name' => $this->product->name,
                    'order_number' => $this->order->order_number,
                    'order_total' => number_format($this->order->total, 2),
                    'purchase_date' => $this->order->created_at?->format('F d, Y'),
                    'app_url' => Config::get('app.url'),
                ];

                // Add type-specific variables
                if ($this->productType === 'indicator') {
                    $variables['download_url'] = "{$frontendUrl}/dashboard/indicators/{$this->product->id}/download";
                    $variables['documentation_url'] = "{$frontendUrl}/indicators/{$this->product->slug}/docs";
                } elseif ($this->productType === 'course') {
                    $variables['course_url'] = "{$frontendUrl}/courses/{$this->product->slug}";
                    $variables['total_lessons'] = $this->product->lessons_count ?? 0;
                }

                $rendered = $renderService->render($template, $variables);

                return (new MailMessage)
                    ->subject($rendered['subject'])
                    ->html($rendered['html']);
            } catch (\Exception $e) {
                report($e);
            }
        }

        // Fallback to Blade template
        $bladeTemplate = match ($this->productType) {
            'indicator' => 'emails.purchases.thank-you-indicator',
            'course' => 'emails.purchases.thank-you-course',
            default => 'emails.purchases.thank-you-indicator',
        };

        $subject = match ($this->productType) {
            'indicator' => "Your Indicator Purchase - {$this->product->name}",
            'course' => "Welcome to {$this->product->name} - Your Course Access",
            default => "Thank You for Your Purchase - {$this->product->name}",
        };

        return (new MailMessage)
            ->subject($subject)
            ->view($bladeTemplate, [
                'user' => $notifiable,
                'order' => $this->order,
                'product' => $this->product,
                'course' => $this->product, // Alias for course template
            ]);
    }

    /**
     * Get the array representation of the notification.
     */
    public function toArray(object $notifiable): array
    {
        return [
            'type' => "{$this->productType}_purchase",
            'order_id' => $this->order->id,
            'order_number' => $this->order->order_number,
            'product_id' => $this->product->id,
            'product_name' => $this->product->name,
            'product_type' => $this->productType,
            'amount' => $this->order->total,
            'message' => "Thank you for purchasing {$this->product->name}!",
        ];
    }
}
