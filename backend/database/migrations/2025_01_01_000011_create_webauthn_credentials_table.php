<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('web_authn_credentials', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('credential_id', 512)->unique();
            $table->text('public_key');
            $table->string('name', 100)->default('Passkey');
            $table->unsignedBigInteger('counter')->default(0);
            $table->json('transports')->nullable();
            $table->string('aaguid', 36)->nullable();
            $table->timestamp('last_used_at')->nullable();
            $table->timestamps();

            // Index for efficient lookup by user
            $table->index('user_id');
            // Index for credential lookup during authentication
            $table->index('credential_id');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('web_authn_credentials');
    }
};
