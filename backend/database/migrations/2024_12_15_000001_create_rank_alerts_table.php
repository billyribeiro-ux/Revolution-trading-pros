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
        Schema::create('rank_alerts', function (Blueprint $table) {
            $table->id();
            $table->foreignId('rank_tracking_id')->constrained('rank_trackings')->onDelete('cascade');
            $table->string('type', 50)->index();
            $table->string('severity', 20)->default('info');
            $table->integer('previous_position')->nullable();
            $table->integer('current_position')->nullable();
            $table->integer('threshold')->nullable();
            $table->json('metadata')->nullable();
            $table->boolean('is_read')->default(false);
            $table->timestamp('read_at')->nullable();
            $table->timestamps();

            $table->index(['rank_tracking_id', 'is_read']);
            $table->index(['severity', 'created_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('rank_alerts');
    }
};
