<?php

declare(strict_types=1);

namespace Database\Seeders;

use App\Models\NewsletterCategory;
use Illuminate\Database\Seeder;

/**
 * NewsletterCategorySeeder
 *
 * Seeds default newsletter categories for trading platform.
 *
 * @level ICT11 Principal Engineer
 */
class NewsletterCategorySeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        NewsletterCategory::seedDefaults();

        $this->command->info('Newsletter categories seeded successfully.');
    }
}
