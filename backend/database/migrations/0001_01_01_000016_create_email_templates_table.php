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
        Schema::create('email_templates', function (Blueprint $table) {
            $table->id();
            $table->string('name'); // User-friendly name
            $table->string('slug')->unique(); // welcome-email, order-confirmation, etc.
            $table->string('subject');
            $table->text('body_html'); // HTML version
            $table->text('body_text')->nullable(); // Plain text version
            $table->json('variables')->nullable(); // Available variables like {{user.name}}
            $table->string('email_type'); // welcome, order, subscription, newsletter, etc.
            $table->boolean('is_active')->default(true);
            $table->timestamps();
            $table->softDeletes();

            $table->index('slug');
            $table->index('email_type');
            $table->index('is_active');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('email_templates');
    }
};
