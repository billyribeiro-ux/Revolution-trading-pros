<?php

namespace App\Notifications;

use App\Models\Order;
use App\Models\Product;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Notifications\Messages\MailMessage;
use Illuminate\Notifications\Notification;

/**
 * Thank you notification for product purchases (indicators, courses, etc).
 *
 * @version 1.0.0
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
        $template = match ($this->productType) {
            'indicator' => 'emails.purchases.thank-you-indicator',
            'course' => 'emails.purchases.thank-you-course',
            default => 'emails.purchases.thank-you-indicator', // Default to indicator template
        };

        $subject = match ($this->productType) {
            'indicator' => "Your Indicator Purchase - {$this->product->name}",
            'course' => "Welcome to {$this->product->name} - Your Course Access",
            default => "Thank You for Your Purchase - {$this->product->name}",
        };

        return (new MailMessage)
            ->subject($subject)
            ->view($template, [
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
