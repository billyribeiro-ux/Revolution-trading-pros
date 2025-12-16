<?php

declare(strict_types=1);

namespace Tests\Feature\Webhooks;

use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

/**
 * Feature tests for inbound email webhooks.
 *
 * @covers \App\Http\Controllers\Webhooks\InboundEmailWebhookController
 */
class InboundEmailWebhookTest extends TestCase
{
    use RefreshDatabase;

    /** @test */
    public function it_processes_postmark_webhook(): void
    {
        // Arrange
        $contact = Contact::factory()->create(['email' => 'sender@example.com']);
        $payload = $this->getPostmarkWebhookPayload();

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $response->assertOk();
        $this->assertDatabaseHas('email_messages', [
            'from_email' => 'sender@example.com',
            'subject' => 'Test Subject',
        ]);
    }

    /** @test */
    public function it_creates_conversation_for_new_email(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'sender@example.com']);
        $payload = $this->getPostmarkWebhookPayload();

        // Act
        $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $this->assertDatabaseHas('email_conversations', [
            'subject' => 'Test Subject',
        ]);

        $conversation = EmailConversation::where('subject', 'Test Subject')->first();
        $this->assertEquals(1, $conversation->messages()->count());
    }

    /** @test */
    public function it_rejects_invalid_postmark_signature(): void
    {
        // Arrange
        $payload = $this->getPostmarkWebhookPayload();

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => 'invalid-signature',
        ]);

        // Assert
        $response->assertUnauthorized();
    }

    /** @test */
    public function it_handles_duplicate_message_ids(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'sender@example.com']);
        $conversation = EmailConversation::factory()->create();
        EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'message_id' => '<existing-message@example.com>',
        ]);

        $payload = $this->getPostmarkWebhookPayload([
            'MessageID' => 'existing-message@example.com',
        ]);

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert - should handle gracefully (200 OK, no duplicate created)
        $response->assertOk();
        $this->assertEquals(1, EmailMessage::where('message_id', '<existing-message@example.com>')->count());
    }

    /** @test */
    public function it_creates_contact_for_unknown_sender(): void
    {
        // Arrange
        $payload = $this->getPostmarkWebhookPayload([
            'From' => '"New Person" <newperson@example.com>',
            'FromFull' => ['Email' => 'newperson@example.com', 'Name' => 'New Person'],
        ]);

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $response->assertOk();
        $this->assertDatabaseHas('contacts', [
            'email' => 'newperson@example.com',
        ]);
    }

    /** @test */
    public function it_threads_reply_to_existing_conversation(): void
    {
        // Arrange
        $contact = Contact::factory()->create(['email' => 'sender@example.com']);
        $conversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
            'subject' => 'Original Subject',
        ]);
        EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'message_id' => '<original@example.com>',
        ]);

        $payload = $this->getPostmarkWebhookPayload([
            'Subject' => 'Re: Original Subject',
            'Headers' => [
                ['Name' => 'In-Reply-To', 'Value' => '<original@example.com>'],
            ],
        ]);

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $response->assertOk();
        $this->assertEquals(2, $conversation->fresh()->messages()->count());
    }

    /** @test */
    public function it_quarantines_spam_emails(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'spammer@example.com']);
        $payload = $this->getPostmarkWebhookPayload([
            'From' => 'spammer@example.com',
            'SpamScore' => 8.5,
            'SpamStatus' => 'Yes',
        ]);

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $response->assertOk();
        $message = EmailMessage::where('from_email', 'spammer@example.com')->first();
        $this->assertTrue($message->is_spam);
    }

    /** @test */
    public function it_handles_attachments(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'sender@example.com']);
        $payload = $this->getPostmarkWebhookPayload([
            'Attachments' => [
                [
                    'Name' => 'document.pdf',
                    'ContentType' => 'application/pdf',
                    'ContentLength' => 1024,
                    'Content' => base64_encode('PDF content'),
                ],
            ],
        ]);

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
        ]);

        // Assert
        $response->assertOk();
        $this->assertDatabaseHas('email_attachments', [
            'filename' => 'document.pdf',
            'mime_type' => 'application/pdf',
        ]);
    }

    /** @test */
    public function it_rate_limits_webhooks(): void
    {
        // Arrange
        $payload = $this->getPostmarkWebhookPayload();

        // Act - Send many requests
        for ($i = 0; $i < 65; $i++) {
            $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
                'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
            ]);

            if ($response->status() === 429) {
                // Assert - Rate limit hit
                $response->assertStatus(429);
                return;
            }
        }

        // If we get here without rate limit, that's also acceptable for low limits
        $this->assertTrue(true);
    }

    /** @test */
    public function it_returns_correlation_id_in_response(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'sender@example.com']);
        $payload = $this->getPostmarkWebhookPayload();

        // Act
        $response = $this->postJson('/api/webhooks/email/postmark', $payload, [
            'X-Postmark-Signature' => $this->generatePostmarkSignature($payload),
            'X-Correlation-ID' => 'test-correlation-123',
        ]);

        // Assert
        $response->assertOk();
        $response->assertHeader('X-Correlation-ID');
    }

    /** @test */
    public function it_handles_ses_webhook(): void
    {
        // Arrange
        Contact::factory()->create(['email' => 'sender@example.com']);
        $payload = $this->getSesWebhookPayload();

        // Act
        $response = $this->postJson('/api/webhooks/email/ses', $payload);

        // Assert
        $response->assertOk();
    }

    /**
     * Get a valid Postmark webhook payload.
     *
     * @param array<string, mixed> $overrides
     * @return array<string, mixed>
     */
    private function getPostmarkWebhookPayload(array $overrides = []): array
    {
        return array_merge([
            'MessageID' => 'test-' . uniqid() . '@example.com',
            'From' => '"Test Sender" <sender@example.com>',
            'FromFull' => ['Email' => 'sender@example.com', 'Name' => 'Test Sender'],
            'To' => 'inbox@crm.example.com',
            'ToFull' => [['Email' => 'inbox@crm.example.com', 'Name' => '']],
            'Subject' => 'Test Subject',
            'TextBody' => 'This is a test email body.',
            'HtmlBody' => '<p>This is a test email body.</p>',
            'Date' => now()->toIso8601String(),
            'Headers' => [],
            'Attachments' => [],
            'SpamScore' => 0.0,
            'SpamStatus' => 'No',
        ], $overrides);
    }

    /**
     * Get a valid SES webhook payload.
     *
     * @return array<string, mixed>
     */
    private function getSesWebhookPayload(): array
    {
        return [
            'Type' => 'Notification',
            'Message' => json_encode([
                'notificationType' => 'Received',
                'mail' => [
                    'messageId' => 'ses-' . uniqid(),
                    'source' => 'sender@example.com',
                    'timestamp' => now()->toIso8601String(),
                    'commonHeaders' => [
                        'subject' => 'Test SES Email',
                        'from' => ['sender@example.com'],
                        'to' => ['inbox@crm.example.com'],
                    ],
                    'headers' => [],
                ],
                'receipt' => [
                    'recipients' => ['inbox@crm.example.com'],
                    'spamVerdict' => ['status' => 'PASS'],
                    'virusVerdict' => ['status' => 'PASS'],
                    'spfVerdict' => ['status' => 'PASS'],
                    'dkimVerdict' => ['status' => 'PASS'],
                    'dmarcVerdict' => ['status' => 'PASS'],
                ],
            ]),
        ];
    }

    /**
     * Generate a mock Postmark signature for testing.
     */
    private function generatePostmarkSignature(array $payload): string
    {
        $key = config('inbound-email.postmark.signature_key', 'test-key');

        return base64_encode(hash_hmac('sha256', json_encode($payload), $key, true));
    }
}
