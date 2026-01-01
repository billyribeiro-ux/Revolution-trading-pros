<?php

declare(strict_types=1);

namespace Tests\Unit\DataTransferObjects\Email;

use App\DataTransferObjects\Email\PostmarkInboundPayload;
use App\Exceptions\Email\InboundEmailException;
use Tests\TestCase;

/**
 * Unit tests for PostmarkInboundPayload.
 *
 * @covers \App\DataTransferObjects\Email\PostmarkInboundPayload
 */
class PostmarkInboundPayloadTest extends TestCase
{
    /** @test */
    public function it_parses_valid_postmark_payload(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload();

        // Act
        $dto = PostmarkInboundPayload::fromWebhook($rawPayload);
        $payload = $dto->getPayload();

        // Assert
        $this->assertEquals('test@example.com', $payload->fromEmail);
        $this->assertEquals('Test Sender', $payload->fromName);
        $this->assertEquals('inbox@crm.example.com', $payload->toEmail);
        $this->assertEquals('Test Subject', $payload->subject);
        $this->assertEquals('postmark', $payload->provider);
    }

    /** @test */
    public function it_parses_email_body(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'TextBody' => 'Plain text body',
            'HtmlBody' => '<p>HTML body</p>',
        ]);

        // Act
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertEquals('Plain text body', $payload->textBody);
        $this->assertEquals('<p>HTML body</p>', $payload->htmlBody);
    }

    /** @test */
    public function it_parses_headers(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'Headers' => [
                ['Name' => 'X-Custom-Header', 'Value' => 'custom-value'],
                ['Name' => 'In-Reply-To', 'Value' => '<original@example.com>'],
            ],
        ]);

        // Act
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertEquals('custom-value', $payload->getHeader('X-Custom-Header'));
        $this->assertEquals('<original@example.com>', $payload->inReplyTo);
    }

    /** @test */
    public function it_parses_references_header(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'Headers' => [
                ['Name' => 'References', 'Value' => '<ref1@example.com> <ref2@example.com>'],
            ],
        ]);

        // Act
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertCount(2, $payload->references);
        $this->assertContains('<ref1@example.com>', $payload->references);
        $this->assertContains('<ref2@example.com>', $payload->references);
    }

    /** @test */
    public function it_parses_attachments(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
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
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertTrue($payload->hasAttachments());
        $this->assertEquals(1, $payload->attachmentCount());
        $this->assertEquals('document.pdf', $payload->attachments[0]->name);
        $this->assertEquals('pdf', $payload->attachments[0]->getExtension());
    }

    /** @test */
    public function it_parses_spam_score(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'SpamScore' => 3.5,
            'SpamStatus' => 'No',
        ]);

        // Act
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertEquals(3.5, $payload->spamScore);
        $this->assertEquals('No', $payload->spamStatus);
        $this->assertFalse($payload->isAboveSpamThreshold(5.0));
        $this->assertTrue($payload->isAboveSpamThreshold(3.0));
    }

    /** @test */
    public function it_parses_mailbox_hash(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'MailboxHash' => 'abc123xyz',
        ]);

        // Act
        $payload = PostmarkInboundPayload::fromWebhook($rawPayload)->getPayload();

        // Assert
        $this->assertEquals('abc123xyz', $payload->mailboxHash);
    }

    /** @test */
    public function it_detects_reply_emails(): void
    {
        // Test with In-Reply-To
        $payload1 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'Headers' => [['Name' => 'In-Reply-To', 'Value' => '<original@example.com>']],
        ]))->getPayload();
        $this->assertTrue($payload1->isReply());

        // Test with Re: subject
        $payload2 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'Subject' => 'Re: Original Subject',
        ]))->getPayload();
        $this->assertTrue($payload2->isReply());

        // Test with mailbox hash
        $payload3 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'MailboxHash' => 'abc123',
        ]))->getPayload();
        $this->assertTrue($payload3->isReply());

        // Test non-reply
        $payload4 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload())->getPayload();
        $this->assertFalse($payload4->isReply());
    }

    /** @test */
    public function it_normalizes_subject(): void
    {
        $subjects = [
            'Re: Test' => 'Test',
            'RE: Test' => 'Test',
            'Fwd: Test' => 'Test',
            'FW: Test' => 'Test',
            'Re: Fwd: Test' => 'Test',
            'Normal Subject' => 'Normal Subject',
        ];

        foreach ($subjects as $input => $expected) {
            $payload = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
                'Subject' => $input,
            ]))->getPayload();

            $this->assertEquals($expected, $payload->getNormalizedSubject(), "Failed for: {$input}");
        }
    }

    /** @test */
    public function it_provides_body_preview(): void
    {
        // Arrange
        $longBody = str_repeat('This is a test. ', 50);
        $payload = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'TextBody' => $longBody,
        ]))->getPayload();

        // Act
        $preview = $payload->getBodyPreview(50);

        // Assert
        $this->assertLessThanOrEqual(50, strlen($preview));
        $this->assertStringEndsWith('...', $preview);
    }

    /** @test */
    public function it_throws_on_missing_from(): void
    {
        // Arrange
        $rawPayload = [
            'Subject' => 'Test',
            'To' => 'inbox@example.com',
        ];

        // Assert
        $this->expectException(InboundEmailException::class);

        // Act
        PostmarkInboundPayload::fromWebhook($rawPayload);
    }

    /** @test */
    public function it_throws_on_missing_recipient(): void
    {
        // Arrange
        $rawPayload = [
            'From' => 'test@example.com',
            'Subject' => 'Test',
        ];

        // Assert
        $this->expectException(InboundEmailException::class);

        // Act
        PostmarkInboundPayload::fromWebhook($rawPayload);
    }

    /** @test */
    public function it_detects_auto_replies(): void
    {
        // With Auto-Submitted header
        $dto1 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'Headers' => [['Name' => 'Auto-Submitted', 'Value' => 'auto-replied']],
        ]));
        $this->assertTrue($dto1->isAutoReply());

        // With X-Auto-Response-Suppress header
        $dto2 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload([
            'Headers' => [['Name' => 'X-Auto-Response-Suppress', 'Value' => 'All']],
        ]));
        $this->assertTrue($dto2->isAutoReply());

        // Normal email
        $dto3 = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload());
        $this->assertFalse($dto3->isAutoReply());
    }

    /** @test */
    public function it_parses_cc_recipients(): void
    {
        // Arrange
        $rawPayload = $this->getValidPostmarkPayload([
            'CcFull' => [
                ['Email' => 'cc1@example.com', 'Name' => 'CC One'],
                ['Email' => 'cc2@example.com', 'Name' => 'CC Two'],
            ],
        ]);

        // Act
        $dto = PostmarkInboundPayload::fromWebhook($rawPayload);
        $ccRecipients = $dto->getCcRecipients();

        // Assert
        $this->assertCount(2, $ccRecipients);
        $this->assertEquals('cc1@example.com', $ccRecipients[0]['email']);
        $this->assertEquals('CC One', $ccRecipients[0]['name']);
    }

    /** @test */
    public function it_converts_to_loggable_format(): void
    {
        // Arrange
        $payload = PostmarkInboundPayload::fromWebhook($this->getValidPostmarkPayload())->getPayload();

        // Act
        $loggable = $payload->toLoggable();

        // Assert
        $this->assertArrayHasKey('message_id', $loggable);
        $this->assertArrayHasKey('from_email', $loggable);
        $this->assertArrayHasKey('subject', $loggable);
        $this->assertArrayNotHasKey('text_body', $loggable); // Sensitive data excluded
        $this->assertArrayNotHasKey('html_body', $loggable); // Sensitive data excluded
    }

    /**
     * Get a valid Postmark payload with optional overrides.
     *
     * @param array<string, mixed> $overrides
     * @return array<string, mixed>
     */
    private function getValidPostmarkPayload(array $overrides = []): array
    {
        return array_merge([
            'MessageID' => 'test-message-id-123',
            'From' => '"Test Sender" <test@example.com>',
            'FromFull' => [
                'Email' => 'test@example.com',
                'Name' => 'Test Sender',
            ],
            'To' => 'inbox@crm.example.com',
            'ToFull' => [
                ['Email' => 'inbox@crm.example.com', 'Name' => ''],
            ],
            'Subject' => 'Test Subject',
            'TextBody' => 'Test text body',
            'HtmlBody' => '<p>Test HTML body</p>',
            'Date' => '2025-01-15T10:30:00Z',
            'Headers' => [],
            'Attachments' => [],
            'SpamScore' => 0.0,
            'SpamStatus' => 'No',
        ], $overrides);
    }
}
