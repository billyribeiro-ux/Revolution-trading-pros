<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('settings', function (Blueprint $table) {
            $table->id();
            $table->string('key')->unique();
            $table->text('value')->nullable();
            $table->string('type')->default('string'); // string, boolean, integer, json
            $table->string('group')->default('general'); // general, seo, payment, etc.
            $table->text('description')->nullable();
            $table->boolean('is_public')->default(false);
            $table->timestamps();
        });

        // Insert default settings
        DB::table('settings')->insert([
            ['key' => 'site_name', 'value' => 'Revolution Trading Pros', 'type' => 'string', 'group' => 'general', 'is_public' => true, 'created_at' => now(), 'updated_at' => now()],
            ['key' => 'site_description', 'value' => 'Professional Trading Education', 'type' => 'string', 'group' => 'general', 'is_public' => true, 'created_at' => now(), 'updated_at' => now()],
            ['key' => 'contact_email', 'value' => 'contact@revolutiontradingpros.com', 'type' => 'string', 'group' => 'general', 'is_public' => true, 'created_at' => now(), 'updated_at' => now()],
            ['key' => 'maintenance_mode', 'value' => 'false', 'type' => 'boolean', 'group' => 'general', 'is_public' => false, 'created_at' => now(), 'updated_at' => now()],
        ]);
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('settings');
    }
};
