<?php

declare(strict_types=1);

namespace Tests\Unit\Services\Email;

use App\DataTransferObjects\Email\EmailAttachmentPayload;
use App\DataTransferObjects\Email\EmailHeader;
use App\DataTransferObjects\Email\InboundEmailPayload;
use App\Enums\ConversationStatus;
use App\Enums\EmailDirection;
use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use App\Services\Email\EmailThreadingService;
use App\Services\Email\InboundEmailService;
use Carbon\CarbonImmutable;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Mockery;
use Tests\TestCase;

/**
 * Unit tests for InboundEmailService.
 *
 * @covers \App\Services\Email\InboundEmailService
 */
class InboundEmailServiceTest extends TestCase
{
    use RefreshDatabase;

    private InboundEmailService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = app(InboundEmailService::class);
    }

    /** @test */
    public function it_processes_inbound_email_and_creates_message(): void
    {
        // Arrange
        $contact = Contact::factory()->create([
            'email' => 'sender@example.com',
        ]);

        $payload = $this->createTestPayload([
            'fromEmail' => 'sender@example.com',
            'subject' => 'Test Subject',
            'textBody' => 'Test body content',
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $this->assertInstanceOf(EmailMessage::class, $message);
        $this->assertEquals('sender@example.com', $message->from_email);
        $this->assertEquals('Test Subject', $message->subject);
        $this->assertEquals(EmailDirection::Inbound, $message->direction);
        $this->assertNotNull($message->conversation);
    }

    /** @test */
    public function it_creates_contact_for_unknown_sender(): void
    {
        // Arrange
        $payload = $this->createTestPayload([
            'fromEmail' => 'new-sender@example.com',
            'fromName' => 'New Sender',
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $contact = Contact::where('email', 'new-sender@example.com')->first();
        $this->assertNotNull($contact);
        $this->assertEquals('New Sender', $contact->first_name . ' ' . $contact->last_name);
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
        $originalMessage = EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'message_id' => '<original@example.com>',
        ]);

        $payload = $this->createTestPayload([
            'fromEmail' => 'sender@example.com',
            'subject' => 'Re: Original Subject',
            'inReplyTo' => '<original@example.com>',
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $this->assertEquals($conversation->id, $message->conversation_id);
        $this->assertEquals(2, $conversation->fresh()->messages()->count());
    }

    /** @test */
    public function it_detects_spam_above_threshold(): void
    {
        // Arrange
        $payload = $this->createTestPayload([
            'spamScore' => 8.5,
            'spamStatus' => 'Yes',
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $this->assertTrue($message->is_spam);
        $this->assertEquals(ConversationStatus::Spam, $message->conversation->status);
    }

    /** @test */
    public function it_stores_security_verdicts(): void
    {
        // Arrange
        $headers = [
            new EmailHeader('Received-SPF', 'pass'),
            new EmailHeader('Authentication-Results', 'dkim=pass; spf=pass; dmarc=pass'),
        ];

        $payload = $this->createTestPayload([
            'headers' => $headers,
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $this->assertNotNull($message->spf_verdict);
    }

    /** @test */
    public function it_handles_attachments(): void
    {
        // Arrange
        $attachment = new EmailAttachmentPayload(
            name: 'document.pdf',
            contentType: 'application/pdf',
            size: 1024,
            content: base64_encode('PDF content'),
        );

        $payload = $this->createTestPayload([
            'attachments' => [$attachment],
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert
        $this->assertEquals(1, $message->attachments()->count());
        $this->assertEquals('document.pdf', $message->attachments->first()->filename);
    }

    /** @test */
    public function it_rejects_dangerous_attachments(): void
    {
        // Arrange
        $attachment = new EmailAttachmentPayload(
            name: 'virus.exe',
            contentType: 'application/x-msdownload',
            size: 1024,
            content: base64_encode('malicious content'),
        );

        $payload = $this->createTestPayload([
            'attachments' => [$attachment],
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert - attachment should be skipped or quarantined
        $this->assertEquals(0, $message->attachments()->where('scan_status', 'clean')->count());
    }

    /** @test */
    public function it_handles_duplicate_message_ids(): void
    {
        // Arrange
        $contact = Contact::factory()->create(['email' => 'sender@example.com']);
        $existing = EmailMessage::factory()->create([
            'message_id' => '<duplicate@example.com>',
        ]);

        $payload = $this->createTestPayload([
            'messageId' => '<duplicate@example.com>',
            'fromEmail' => 'sender@example.com',
        ]);

        // Act
        $message = $this->service->processInboundEmail($payload);

        // Assert - should return existing message or handle gracefully
        $this->assertEquals($existing->id, $message->id);
    }

    /** @test */
    public function it_updates_contact_email_stats(): void
    {
        // Arrange
        $contact = Contact::factory()->create([
            'email' => 'sender@example.com',
            'conversations_count' => 0,
        ]);

        $payload = $this->createTestPayload([
            'fromEmail' => 'sender@example.com',
        ]);

        // Act
        $this->service->processInboundEmail($payload);

        // Assert
        $contact->refresh();
        $this->assertEquals(1, $contact->conversations_count);
        $this->assertNotNull($contact->last_email_received_at);
    }

    /**
     * Create a test payload with defaults.
     *
     * @param array<string, mixed> $overrides
     */
    private function createTestPayload(array $overrides = []): InboundEmailPayload
    {
        $defaults = [
            'messageId' => '<' . uniqid() . '@test.example.com>',
            'fromEmail' => 'test@example.com',
            'fromName' => 'Test Sender',
            'toEmail' => 'inbox@crm.example.com',
            'toName' => null,
            'subject' => 'Test Subject',
            'textBody' => 'Test body content',
            'htmlBody' => '<p>Test body content</p>',
            'receivedAt' => CarbonImmutable::now(),
            'inReplyTo' => null,
            'references' => [],
            'headers' => [],
            'attachments' => [],
            'mailboxHash' => null,
            'spamScore' => 0.0,
            'spamStatus' => null,
            'rawPayload' => [],
            'provider' => 'postmark',
        ];

        $data = array_merge($defaults, $overrides);

        return new InboundEmailPayload(
            messageId: $data['messageId'],
            fromEmail: $data['fromEmail'],
            fromName: $data['fromName'],
            toEmail: $data['toEmail'],
            toName: $data['toName'],
            subject: $data['subject'],
            textBody: $data['textBody'],
            htmlBody: $data['htmlBody'],
            receivedAt: $data['receivedAt'],
            inReplyTo: $data['inReplyTo'],
            references: $data['references'],
            headers: $data['headers'],
            attachments: $data['attachments'],
            mailboxHash: $data['mailboxHash'],
            spamScore: $data['spamScore'],
            spamStatus: $data['spamStatus'],
            rawPayload: $data['rawPayload'],
            provider: $data['provider'],
        );
    }
}
