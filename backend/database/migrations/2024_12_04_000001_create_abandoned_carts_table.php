<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('abandoned_carts', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->nullable()->constrained('users')->nullOnDelete();
            $table->string('session_id')->index();
            $table->string('email')->index();
            $table->json('items');
            $table->decimal('subtotal', 10, 2)->default(0);
            $table->decimal('total', 10, 2)->default(0);
            $table->string('currency', 3)->default('USD');
            $table->enum('status', ['pending', 'email_sent', 'clicked', 'recovered', 'expired', 'unsubscribed'])
                  ->default('pending')
                  ->index();
            $table->unsignedTinyInteger('recovery_attempts')->default(0);
            $table->timestamp('last_recovery_at')->nullable();
            $table->timestamp('recovered_at')->nullable();
            $table->string('recovery_code', 20)->nullable()->index();
            $table->decimal('recovery_discount', 5, 2)->nullable();
            $table->string('source', 50)->nullable();
            $table->json('utm_params')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamp('abandoned_at')->index();
            $table->timestamps();

            // Composite indexes for common queries
            $table->index(['status', 'recovery_attempts', 'abandoned_at']);
            $table->index(['email', 'status']);
        });

        // Add email_logs campaign_id column if not exists
        if (!Schema::hasColumn('email_logs', 'campaign_id')) {
            Schema::table('email_logs', function (Blueprint $table) {
                $table->string('campaign_id', 50)->nullable()->after('id')->index();
                $table->string('template', 50)->nullable()->after('campaign_type');
                $table->string('offer_code', 50)->nullable()->after('template');
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('abandoned_carts');

        if (Schema::hasColumn('email_logs', 'campaign_id')) {
            Schema::table('email_logs', function (Blueprint $table) {
                $table->dropColumn(['campaign_id', 'template', 'offer_code']);
            });
        }
    }
};
