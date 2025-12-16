<?php

declare(strict_types=1);

namespace Tests\Unit\Services\Email;

use App\Enums\ConversationStatus;
use App\Exceptions\Email\ThreadingException;
use App\Models\Contact;
use App\Models\EmailConversation;
use App\Models\EmailMessage;
use App\Models\EmailThreadMapping;
use App\Services\Email\EmailThreadingService;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

/**
 * Unit tests for EmailThreadingService.
 *
 * @covers \App\Services\Email\EmailThreadingService
 */
class EmailThreadingServiceTest extends TestCase
{
    use RefreshDatabase;

    private EmailThreadingService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = app(EmailThreadingService::class);
    }

    /** @test */
    public function it_creates_new_conversation_for_new_thread(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $messageId = '<new-message@example.com>';

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: $messageId,
            inReplyTo: null,
            references: [],
            subject: 'New Thread Subject',
        );

        // Assert
        $this->assertInstanceOf(EmailConversation::class, $conversation);
        $this->assertEquals('New Thread Subject', $conversation->subject);
        $this->assertEquals($contact->id, $conversation->contact_id);
        $this->assertEquals(ConversationStatus::Open, $conversation->status);
    }

    /** @test */
    public function it_finds_conversation_by_in_reply_to(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $existingConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
        ]);
        EmailThreadMapping::create([
            'conversation_id' => $existingConversation->id,
            'message_id' => '<original@example.com>',
            'normalized_subject' => 'test subject',
        ]);

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<reply@example.com>',
            inReplyTo: '<original@example.com>',
            references: [],
            subject: 'Re: Test Subject',
        );

        // Assert
        $this->assertEquals($existingConversation->id, $conversation->id);
    }

    /** @test */
    public function it_finds_conversation_by_references(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $existingConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
        ]);
        EmailThreadMapping::create([
            'conversation_id' => $existingConversation->id,
            'message_id' => '<thread-root@example.com>',
            'normalized_subject' => 'test subject',
        ]);

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<reply@example.com>',
            inReplyTo: null,
            references: ['<other@example.com>', '<thread-root@example.com>'],
            subject: 'Re: Test Subject',
        );

        // Assert
        $this->assertEquals($existingConversation->id, $conversation->id);
    }

    /** @test */
    public function it_finds_conversation_by_mailbox_hash(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $existingConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
            'mailbox_hash' => 'abc123',
        ]);

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<reply@example.com>',
            inReplyTo: null,
            references: [],
            subject: 'Different Subject',
            mailboxHash: 'abc123',
        );

        // Assert
        $this->assertEquals($existingConversation->id, $conversation->id);
    }

    /** @test */
    public function it_finds_conversation_by_subject_matching(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $existingConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
            'subject' => 'Important Discussion',
            'created_at' => now()->subHours(12),
        ]);
        EmailThreadMapping::create([
            'conversation_id' => $existingConversation->id,
            'message_id' => '<original@example.com>',
            'normalized_subject' => 'important discussion',
        ]);

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<reply@example.com>',
            inReplyTo: null,
            references: [],
            subject: 'Re: Important Discussion',
        );

        // Assert
        $this->assertEquals($existingConversation->id, $conversation->id);
    }

    /** @test */
    public function it_creates_thread_mapping_for_new_message(): void
    {
        // Arrange
        $contact = Contact::factory()->create();

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<new-msg@example.com>',
            inReplyTo: null,
            references: [],
            subject: 'Test Subject',
        );

        // Assert
        $mapping = EmailThreadMapping::where('message_id', '<new-msg@example.com>')->first();
        $this->assertNotNull($mapping);
        $this->assertEquals($conversation->id, $mapping->conversation_id);
        $this->assertEquals('test subject', $mapping->normalized_subject);
    }

    /** @test */
    public function it_merges_conversations_correctly(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $targetConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
        ]);
        $sourceConversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
        ]);

        $targetMessages = EmailMessage::factory()->count(2)->create([
            'conversation_id' => $targetConversation->id,
        ]);
        $sourceMessages = EmailMessage::factory()->count(3)->create([
            'conversation_id' => $sourceConversation->id,
        ]);

        // Act
        $merged = $this->service->mergeConversations($targetConversation, $sourceConversation);

        // Assert
        $this->assertEquals($targetConversation->id, $merged->id);
        $this->assertEquals(5, $merged->messages()->count());
        $this->assertNull(EmailConversation::find($sourceConversation->id));
    }

    /** @test */
    public function it_prevents_merging_different_contact_conversations(): void
    {
        // Arrange
        $contact1 = Contact::factory()->create();
        $contact2 = Contact::factory()->create();
        $conversation1 = EmailConversation::factory()->create(['contact_id' => $contact1->id]);
        $conversation2 = EmailConversation::factory()->create(['contact_id' => $contact2->id]);

        // Assert
        $this->expectException(ThreadingException::class);

        // Act
        $this->service->mergeConversations($conversation1, $conversation2);
    }

    /** @test */
    public function it_splits_conversation_at_message(): void
    {
        // Arrange
        $contact = Contact::factory()->create();
        $conversation = EmailConversation::factory()->create([
            'contact_id' => $contact->id,
        ]);

        $message1 = EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'created_at' => now()->subHours(3),
        ]);
        $message2 = EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'message_id' => '<split-point@example.com>',
            'created_at' => now()->subHours(2),
        ]);
        $message3 = EmailMessage::factory()->create([
            'conversation_id' => $conversation->id,
            'created_at' => now()->subHour(),
        ]);

        // Act
        $newConversation = $this->service->splitConversation(
            $conversation,
            '<split-point@example.com>',
        );

        // Assert
        $this->assertNotEquals($conversation->id, $newConversation->id);
        $this->assertEquals(1, $conversation->fresh()->messages()->count());
        $this->assertEquals(2, $newConversation->messages()->count());
    }

    /** @test */
    public function it_normalizes_subject_correctly(): void
    {
        // Arrange
        $subjects = [
            'Test Subject' => 'test subject',
            'Re: Test Subject' => 'test subject',
            'RE: Test Subject' => 'test subject',
            'Fwd: Test Subject' => 'test subject',
            'FW: Test Subject' => 'test subject',
            'Re: Re: Fwd: Test Subject' => 'test subject',
            '  Re:  Test Subject  ' => 'test subject',
        ];

        $contact = Contact::factory()->create();

        // Act & Assert
        foreach ($subjects as $input => $expected) {
            $conversation = $this->service->findOrCreateConversation(
                contact: $contact,
                messageId: '<' . uniqid() . '@example.com>',
                inReplyTo: null,
                references: [],
                subject: $input,
            );

            $mapping = EmailThreadMapping::where('conversation_id', $conversation->id)->first();
            $this->assertEquals($expected, $mapping->normalized_subject, "Failed for: {$input}");
        }
    }

    /** @test */
    public function it_handles_empty_subject(): void
    {
        // Arrange
        $contact = Contact::factory()->create();

        // Act
        $conversation = $this->service->findOrCreateConversation(
            contact: $contact,
            messageId: '<no-subject@example.com>',
            inReplyTo: null,
            references: [],
            subject: '',
        );

        // Assert
        $this->assertEquals('(no subject)', $conversation->subject);
    }
}
