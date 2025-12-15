<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('search_engines', function (Blueprint $table) {
            $table->id();
            $table->string('key', 50)->unique();
            $table->string('name', 100);
            $table->string('api_endpoint')->nullable();
            $table->boolean('supports_local')->default(false);
            $table->integer('max_results')->default(100);
            $table->boolean('is_active')->default(true);
            $table->json('config')->nullable();
            $table->timestamps();

            $table->index('is_active');
        });

        // Seed default search engines
        DB::table('search_engines')->insert([
            [
                'key' => 'google',
                'name' => 'Google',
                'api_endpoint' => 'https://serpapi.com/search',
                'supports_local' => true,
                'max_results' => 100,
                'is_active' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'key' => 'bing',
                'name' => 'Bing',
                'api_endpoint' => 'https://serpapi.com/search',
                'supports_local' => true,
                'max_results' => 50,
                'is_active' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'key' => 'yahoo',
                'name' => 'Yahoo',
                'api_endpoint' => 'https://serpapi.com/search',
                'supports_local' => false,
                'max_results' => 50,
                'is_active' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'key' => 'duckduckgo',
                'name' => 'DuckDuckGo',
                'api_endpoint' => 'https://serpapi.com/search',
                'supports_local' => false,
                'max_results' => 30,
                'is_active' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
        ]);
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('search_engines');
    }
};
