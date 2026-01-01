<?php

declare(strict_types=1);

namespace Database\Factories;

use App\Enums\AttachmentScanStatus;
use App\Models\EmailAttachment;
use App\Models\EmailMessage;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * Factory for EmailAttachment model.
 *
 * @extends Factory<EmailAttachment>
 */
class EmailAttachmentFactory extends Factory
{
    protected $model = EmailAttachment::class;

    /**
     * Define the model's default state.
     *
     * @return array<string, mixed>
     */
    public function definition(): array
    {
        $filename = $this->faker->word() . '.' . $this->faker->fileExtension();
        $size = $this->faker->numberBetween(1024, 5242880); // 1KB to 5MB

        return [
            'message_id' => EmailMessage::factory(),
            'filename' => $filename,
            'original_filename' => $filename,
            'mime_type' => $this->faker->mimeType(),
            'size' => $size,
            'storage_path' => 'attachments/' . $this->faker->uuid() . '/' . $filename,
            'storage_disk' => 'r2',
            'checksum_md5' => md5($this->faker->sha256()),
            'checksum_sha256' => $this->faker->sha256(),
            'scan_status' => AttachmentScanStatus::Pending,
            'is_inline' => false,
        ];
    }

    /**
     * Set as a PDF file.
     */
    public function pdf(): self
    {
        return $this->state(fn (array $attributes) => [
            'filename' => $this->faker->word() . '.pdf',
            'original_filename' => $this->faker->word() . '.pdf',
            'mime_type' => 'application/pdf',
        ]);
    }

    /**
     * Set as an image file.
     */
    public function image(): self
    {
        $ext = $this->faker->randomElement(['jpg', 'png', 'gif']);
        $mimeTypes = [
            'jpg' => 'image/jpeg',
            'png' => 'image/png',
            'gif' => 'image/gif',
        ];

        return $this->state(fn (array $attributes) => [
            'filename' => $this->faker->word() . '.' . $ext,
            'original_filename' => $this->faker->word() . '.' . $ext,
            'mime_type' => $mimeTypes[$ext],
        ]);
    }

    /**
     * Set as inline attachment (e.g., embedded image).
     */
    public function inline(): self
    {
        return $this->state(fn (array $attributes) => [
            'is_inline' => true,
            'content_id' => $this->faker->uuid() . '@inline',
        ]);
    }

    /**
     * Set scan status as clean.
     */
    public function clean(): self
    {
        return $this->state(fn (array $attributes) => [
            'scan_status' => AttachmentScanStatus::Clean,
            'scanned_at' => now(),
        ]);
    }

    /**
     * Set scan status as infected.
     */
    public function infected(): self
    {
        return $this->state(fn (array $attributes) => [
            'scan_status' => AttachmentScanStatus::Infected,
            'scanned_at' => now(),
            'quarantined_at' => now(),
        ]);
    }

    /**
     * Set for a specific message.
     */
    public function forMessage(EmailMessage $message): self
    {
        return $this->state(fn (array $attributes) => [
            'message_id' => $message->id,
        ]);
    }

    /**
     * Set with specific size.
     */
    public function withSize(int $bytes): self
    {
        return $this->state(fn (array $attributes) => [
            'size' => $bytes,
        ]);
    }
}
