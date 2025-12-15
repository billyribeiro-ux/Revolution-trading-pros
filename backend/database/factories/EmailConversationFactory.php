<?php

declare(strict_types=1);

namespace Database\Factories;

use App\Enums\ConversationPriority;
use App\Enums\ConversationStatus;
use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * Factory for EmailConversation model.
 *
 * @extends Factory<EmailConversation>
 */
class EmailConversationFactory extends Factory
{
    protected $model = EmailConversation::class;

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {
        return [
            'contact_id' => Contact::factory(),
            'assigned_to' => null,
            'subject' => $this->faker->sentence(),
            'status' => ConversationStatus::Open,
            'priority' => ConversationPriority::Normal,
            'mailbox_hash' => $this->faker->optional()->regexify('[a-zA-Z0-9]{12}'),
            'message_count' => 0,
            'unread_count' => 0,
            'first_message_at' => now(),
            'last_message_at' => now(),
            'sla_deadline_at' => null,
        ];
    }

    /**
     * Set the conversation as assigned.
     */
    public function assigned(?User $user = null): self
    {
        return $this->state(fn (array $attributes) => [
            'assigned_to' => $user?->id ?? User::factory(),
        ]);
    }

    /**
     * Set the conversation status.
     */
    public function withStatus(ConversationStatus $status): self
    {
        return $this->state(fn (array $attributes) => [
            'status' => $status,
        ]);
    }

    /**
     * Set as open status.
     */
    public function open(): self
    {
        return $this->withStatus(ConversationStatus::Open);
    }

    /**
     * Set as pending status.
     */
    public function pending(): self
    {
        return $this->withStatus(ConversationStatus::Pending);
    }

    /**
     * Set as resolved status.
     */
    public function resolved(): self
    {
        return $this->state(fn (array $attributes) => [
            'status' => ConversationStatus::Resolved,
            'resolved_at' => now(),
        ]);
    }

    /**
     * Set as spam.
     */
    public function spam(): self
    {
        return $this->withStatus(ConversationStatus::Spam);
    }

    /**
     * Set the conversation priority.
     */
    public function withPriority(ConversationPriority $priority): self
    {
        return $this->state(fn (array $attributes) => [
            'priority' => $priority,
            'sla_deadline_at' => now()->addMinutes($priority->slaMinutes()),
        ]);
    }

    /**
     * Set as urgent priority.
     */
    public function urgent(): self
    {
        return $this->withPriority(ConversationPriority::Urgent);
    }

    /**
     * Set as high priority.
     */
    public function highPriority(): self
    {
        return $this->withPriority(ConversationPriority::High);
    }

    /**
     * Set with unread messages.
     */
    public function withUnread(int $count = 1): self
    {
        return $this->state(fn (array $attributes) => [
            'unread_count' => $count,
            'message_count' => max($attributes['message_count'] ?? 0, $count),
        ]);
    }

    /**
     * Set with a specific contact.
     */
    public function forContact(Contact $contact): self
    {
        return $this->state(fn (array $attributes) => [
            'contact_id' => $contact->id,
        ]);
    }
}
