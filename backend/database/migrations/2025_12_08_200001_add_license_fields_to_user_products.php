<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Add license and download tracking fields to user_products pivot table
 */
return new class extends Migration
{
    public function up(): void
    {
        Schema::table('user_products', function (Blueprint $table) {
            $table->string('license_key')->nullable()->after('order_id');
            $table->timestamp('expires_at')->nullable()->after('license_key');
            $table->integer('download_count')->default(0)->after('expires_at');
            $table->timestamp('last_downloaded_at')->nullable()->after('download_count');
            $table->timestamp('revoked_at')->nullable()->after('last_downloaded_at');
        });
    }

    public function down(): void
    {
        Schema::table('user_products', function (Blueprint $table) {
            $table->dropColumn(['license_key', 'expires_at', 'download_count', 'last_downloaded_at', 'revoked_at']);
        });
    }
};
