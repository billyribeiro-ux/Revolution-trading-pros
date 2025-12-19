<?php

declare(strict_types=1);

namespace App\Console\Commands;

use App\Services\Integration\InventoryReservationService;
use Illuminate\Console\Command;

class ReleaseExpiredReservations extends Command
{
    /**
     * The name and signature of the console command.
     */
    protected $signature = 'inventory:release-expired';

    /**
     * The console command description.
     */
    protected $description = 'Release expired inventory reservations';

    public function __construct(
        private readonly InventoryReservationService $reservationService
    ) {
        parent::__construct();
    }

    /**
     * Execute the console command.
     */
    public function handle(): int
    {
        $released = $this->reservationService->releaseExpired();

        if ($released > 0) {
            $this->info("Released {$released} expired inventory reservation(s).");
        } else {
            $this->line('No expired reservations found.');
        }

        return Command::SUCCESS;
    }
}
