<?php

declare(strict_types=1);

namespace App\Jobs;

use App\Contracts\Email\EmailAttachmentStorageInterface;
use App\Enums\AttachmentScanStatus;
use App\Models\EmailAttachment;
use App\Support\Logging\StructuredLogger;
use App\Support\Metrics\MetricsCollector;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Throwable;

/**
 * Job to scan email attachments for viruses.
 *
 * Runs asynchronously to avoid blocking email processing.
 *
 * @version 1.0.0
 */
class ScanAttachmentForVirusJob implements ShouldQueue
{
    use Dispatchable;
    use InteractsWithQueue;
    use Queueable;
    use SerializesModels;

    /**
     * Maximum retry attempts.
     */
    public int $tries = 3;

    /**
     * Timeout in seconds.
     */
    public int $timeout = 60;

    /**
     * Create a new job instance.
     */
    public function __construct(
        public readonly EmailAttachment $attachment,
    ) {
        $this->onQueue('security');
    }

    /**
     * Calculate backoff times.
     *
     * @return array<int>
     */
    public function backoff(): array
    {
        return [30, 60, 120];
    }

    /**
     * Execute the job.
     */
    public function handle(
        EmailAttachmentStorageInterface $storage,
        MetricsCollector $metrics,
    ): void {
        $logger = StructuredLogger::security()->withContext([
            'attachment_id' => $this->attachment->id,
            'filename' => $this->attachment->filename,
        ]);

        $logger->operationStart('virus_scan');
        $startTime = microtime(true);

        try {
            // Mark as scanning
            $this->attachment->update(['scan_status' => AttachmentScanStatus::Pending]);

            // Perform scan
            $isClean = $storage->scanForVirus($this->attachment);

            // Update status
            $status = $isClean ? AttachmentScanStatus::Clean : AttachmentScanStatus::Infected;
            $this->attachment->update([
                'scan_status' => $status,
                'scanned_at' => now(),
            ]);

            $duration = (microtime(true) - $startTime) * 1000;

            if ($isClean) {
                $logger->operationComplete('virus_scan', $duration, [
                    'result' => 'clean',
                ]);
                $metrics->recordAttachmentProcessed('clean', $this->attachment->size);
            } else {
                $logger->securityEvent('virus_detected', 'high', [
                    'attachment_id' => $this->attachment->id,
                    'filename' => $this->attachment->filename,
                    'message_id' => $this->attachment->message_id,
                ]);
                $metrics->recordAttachmentProcessed('infected', $this->attachment->size);

                // Quarantine infected file
                $this->quarantineAttachment();
            }

        } catch (Throwable $e) {
            $duration = (microtime(true) - $startTime) * 1000;

            $this->attachment->update(['scan_status' => AttachmentScanStatus::Error]);

            $logger->operationFailed('virus_scan', $e->getMessage(), $duration);

            throw $e;
        }
    }

    /**
     * Quarantine an infected attachment.
     */
    private function quarantineAttachment(): void
    {
        // Move to quarantine storage, notify admins, etc.
        $this->attachment->update([
            'quarantined_at' => now(),
        ]);
    }

    /**
     * Handle job failure.
     */
    public function failed(?Throwable $exception): void
    {
        $this->attachment->update(['scan_status' => AttachmentScanStatus::Error]);

        StructuredLogger::security()->error('Virus scan job failed', [
            'attachment_id' => $this->attachment->id,
            'exception' => $exception?->getMessage(),
        ]);
    }
}
