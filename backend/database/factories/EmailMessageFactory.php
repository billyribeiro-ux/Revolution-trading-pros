<?php

declare(strict_types=1);

namespace Database\Factories;

use App\Enums\EmailDirection;
use App\Enums\EmailMessageStatus;
use App\Enums\SecurityVerdict;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * Factory for EmailMessage model.
 *
 * @extends Factory<EmailMessage>
 */
class EmailMessageFactory extends Factory
{
    protected $model = EmailMessage::class;

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {
        $fromEmail = $this->faker->safeEmail();
        $toEmail = $this->faker->safeEmail();

        return [
            'conversation_id' => EmailConversation::factory(),
            'message_id' => '<' . $this->faker->uuid() . '@' . $this->faker->domainName() . '>',
            'in_reply_to' => null,
            'direction' => EmailDirection::Inbound,
            'status' => EmailMessageStatus::Processed,
            'from_email' => $fromEmail,
            'from_name' => $this->faker->name(),
            'to_email' => $toEmail,
            'to_name' => null,
            'subject' => $this->faker->sentence(),
            'text_body' => $this->faker->paragraphs(3, true),
            'html_body' => '<p>' . $this->faker->paragraphs(3, true) . '</p>',
            'is_read' => false,
            'is_spam' => false,
            'spam_score' => $this->faker->randomFloat(2, 0, 2),
            'spam_verdict' => null,
            'spf_verdict' => SecurityVerdict::Pass,
            'dkim_verdict' => SecurityVerdict::Pass,
            'dmarc_verdict' => SecurityVerdict::Pass,
            'received_at' => now(),
            'processed_at' => now(),
        ];
    }

    /**
     * Set as inbound message.
     */
    public function inbound(): self
    {
        return $this->state(fn (array $attributes) => [
            'direction' => EmailDirection::Inbound,
        ]);
    }

    /**
     * Set as outbound message.
     */
    public function outbound(): self
    {
        return $this->state(fn (array $attributes) => [
            'direction' => EmailDirection::Outbound,
        ]);
    }

    /**
     * Set as read.
     */
    public function read(): self
    {
        return $this->state(fn (array $attributes) => [
            'is_read' => true,
            'read_at' => now(),
        ]);
    }

    /**
     * Set as unread.
     */
    public function unread(): self
    {
        return $this->state(fn (array $attributes) => [
            'is_read' => false,
            'read_at' => null,
        ]);
    }

    /**
     * Set as spam.
     */
    public function spam(float $score = 8.0): self
    {
        return $this->state(fn (array $attributes) => [
            'is_spam' => true,
            'spam_score' => $score,
            'spam_verdict' => 'SPAM',
            'status' => EmailMessageStatus::Spam,
        ]);
    }

    /**
     * Set as a reply.
     */
    public function replyTo(EmailMessage $parent): self
    {
        return $this->state(fn (array $attributes) => [
            'conversation_id' => $parent->conversation_id,
            'in_reply_to' => $parent->message_id,
            'subject' => 'Re: ' . $parent->subject,
        ]);
    }

    /**
     * Set with specific from address.
     */
    public function from(string $email, ?string $name = null): self
    {
        return $this->state(fn (array $attributes) => [
            'from_email' => $email,
            'from_name' => $name,
        ]);
    }

    /**
     * Set with security failures.
     */
    public function withSecurityFailure(string $check = 'spf'): self
    {
        $verdicts = [
            'spf_verdict' => SecurityVerdict::Pass,
            'dkim_verdict' => SecurityVerdict::Pass,
            'dmarc_verdict' => SecurityVerdict::Pass,
        ];

        $verdicts["{$check}_verdict"] = SecurityVerdict::Fail;

        return $this->state(fn (array $attributes) => $verdicts);
    }

    /**
     * Set for a specific conversation.
     */
    public function forConversation(EmailConversation $conversation): self
    {
        return $this->state(fn (array $attributes) => [
            'conversation_id' => $conversation->id,
        ]);
    }

    /**
     * Set with a specific message ID.
     */
    public function withMessageId(string $messageId): self
    {
        return $this->state(fn (array $attributes) => [
            'message_id' => $messageId,
        ]);
    }
}
