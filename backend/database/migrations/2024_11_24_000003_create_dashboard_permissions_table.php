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
        Schema::create('dashboard_permissions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('dashboard_id')->constrained()->onDelete('cascade');
            $table->string('role')->nullable(); // admin, user, subscriber, etc.
            $table->foreignId('user_id')->nullable()->constrained()->onDelete('cascade');
            $table->boolean('can_view')->default(true);
            $table->boolean('can_edit')->default(false);
            $table->boolean('can_delete')->default(false);
            $table->boolean('can_share')->default(false);
            $table->timestamps();

            $table->index(['dashboard_id', 'role']);
            $table->index(['dashboard_id', 'user_id']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('dashboard_permissions');
    }
};
