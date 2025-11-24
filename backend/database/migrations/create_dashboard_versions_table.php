<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('dashboard_versions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('dashboard_id')->constrained()->onDelete('cascade');
            $table->integer('version_number');
            $table->string('description')->nullable();
            $table->json('dashboard_snapshot');
            $table->json('widgets_snapshot');
            $table->foreignId('created_by')->constrained('users')->onDelete('cascade');
            $table->timestamp('created_at');

            $table->unique(['dashboard_id', 'version_number']);
            $table->index(['dashboard_id', 'created_at']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('dashboard_versions');
    }
};
