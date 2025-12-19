<?php

declare(strict_types=1);

namespace App\DataTransferObjects\Email;

/**
 * DTO for email attachment payloads.
 *
 * @version 1.0.0
 */
readonly class EmailAttachmentPayload
{
    /**
     * Dangerous file extensions that should be blocked.
     */
    private const DANGEROUS_EXTENSIONS = [
        'exe', 'dll', 'bat', 'cmd', 'com', 'pif', 'scr', 'vbs', 'vbe',
        'js', 'jse', 'ws', 'wsf', 'wsc', 'wsh', 'ps1', 'ps1xml', 'ps2',
        'ps2xml', 'psc1', 'psc2', 'msc', 'msi', 'msp', 'mst', 'hta',
        'cpl', 'mshxml', 'gadget', 'application', 'appref-ms',
    ];

    public function __construct(
        public string $name,
        public string $contentType,
        public int $size,
        public string $content, // Base64 encoded
        public ?string $contentId = null,
    ) {}

    /**
     * Create from Postmark payload format.
     *
     * @param array<string, mixed> $data
     */
    public static function fromPostmark(array $data): self
    {
        return new self(
            name: $data['Name'] ?? 'attachment',
            contentType: $data['ContentType'] ?? 'application/octet-stream',
            size: $data['ContentLength'] ?? strlen(base64_decode($data['Content'] ?? '')),
            content: $data['Content'] ?? '',
            contentId: $data['ContentID'] ?? null,
        );
    }

    /**
     * Create from SES/S3 format.
     *
     * @param array<string, mixed> $data
     */
    public static function fromSes(array $data): self
    {
        return new self(
            name: $data['filename'] ?? $data['name'] ?? 'attachment',
            contentType: $data['contentType'] ?? $data['mimeType'] ?? 'application/octet-stream',
            size: $data['size'] ?? strlen(base64_decode($data['content'] ?? '')),
            content: $data['content'] ?? '',
            contentId: $data['contentId'] ?? null,
        );
    }

    /**
     * Get file extension (lowercase).
     */
    public function getExtension(): string
    {
        $parts = explode('.', $this->name);

        return strtolower(end($parts));
    }

    /**
     * Check if file type is potentially dangerous.
     */
    public function isDangerousType(): bool
    {
        return in_array($this->getExtension(), self::DANGEROUS_EXTENSIONS, true);
    }

    /**
     * Check if attachment is inline (embedded image, etc).
     */
    public function isInline(): bool
    {
        return $this->contentId !== null;
    }

    /**
     * Get decoded content.
     */
    public function getDecodedContent(): string
    {
        return base64_decode($this->content);
    }

    /**
     * Get MD5 checksum of content.
     */
    public function getChecksum(): string
    {
        return md5($this->getDecodedContent());
    }

    /**
     * Get SHA-256 hash of content.
     */
    public function getSha256Hash(): string
    {
        return hash('sha256', $this->getDecodedContent());
    }

    /**
     * Check if size exceeds limit.
     */
    public function exceedsSize(int $maxBytes): bool
    {
        return $this->size > $maxBytes;
    }

    /**
     * Check if content type matches.
     */
    public function isContentType(string $type): bool
    {
        return str_starts_with($this->contentType, $type);
    }

    /**
     * Check if this is an image.
     */
    public function isImage(): bool
    {
        return $this->isContentType('image/');
    }

    /**
     * Check if this is a PDF.
     */
    public function isPdf(): bool
    {
        return $this->contentType === 'application/pdf';
    }

    /**
     * Convert to array for storage.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'name' => $this->name,
            'content_type' => $this->contentType,
            'size' => $this->size,
            'extension' => $this->getExtension(),
            'content_id' => $this->contentId,
            'is_inline' => $this->isInline(),
            'checksum' => $this->getChecksum(),
        ];
    }

    /**
     * Convert to loggable format (without content).
     *
     * @return array<string, mixed>
     */
    public function toLoggable(): array
    {
        return [
            'name' => $this->name,
            'content_type' => $this->contentType,
            'size' => $this->size,
            'extension' => $this->getExtension(),
            'is_dangerous' => $this->isDangerousType(),
        ];
    }
}
