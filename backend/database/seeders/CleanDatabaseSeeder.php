<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\DB;
use App\Models\User;
use App\Models\Product;
use App\Models\Post;

class CleanDatabaseSeeder extends Seeder
{
    /**
     * Clean database and keep only production-ready data
     */
    public function run(): void
    {
        // Remove test users (keep admin only)
        User::where('email', 'LIKE', '%test%')
            ->where('email', '!=', 'admin@test.com')
            ->delete();

        // Remove test products
        Product::where('name', 'LIKE', '%test%')
            ->orWhere('slug', 'LIKE', '%test%')
            ->delete();

        // Remove draft/unpublished posts older than 30 days
        Post::where('status', 'draft')
            ->where('created_at', '<', now()->subDays(30))
            ->delete();

        // Clean up orphaned records
        DB::statement('DELETE FROM cart_items WHERE cart_id NOT IN (SELECT id FROM carts)');
        DB::statement('DELETE FROM order_items WHERE order_id NOT IN (SELECT id FROM orders)');

        $this->command->info('Database cleaned successfully!');
        $this->command->info('- Removed test users');
        $this->command->info('- Removed test products');
        $this->command->info('- Removed old draft posts');
        $this->command->info('- Cleaned orphaned records');
    }
}
