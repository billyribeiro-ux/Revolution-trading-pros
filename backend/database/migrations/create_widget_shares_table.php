<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('widget_shares', function (Blueprint $table) {
            $table->id();
            $table->foreignId('widget_id')->constrained('dashboard_widgets')->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->boolean('can_view')->default(true);
            $table->boolean('can_edit')->default(false);
            $table->boolean('can_delete')->default(false);
            $table->timestamp('shared_at');
            $table->timestamps();

            $table->unique(['widget_id', 'user_id']);
            $table->index('user_id');
        });

        Schema::create('widget_share_links', function (Blueprint $table) {
            $table->id();
            $table->foreignId('widget_id')->constrained('dashboard_widgets')->onDelete('cascade');
            $table->string('token', 64)->unique();
            $table->timestamp('expires_at')->nullable();
            $table->integer('view_count')->default(0);
            $table->timestamps();

            $table->index('token');
            $table->index(['widget_id', 'expires_at']);
        });

        Schema::create('widget_views', function (Blueprint $table) {
            $table->id();
            $table->foreignId('widget_id')->constrained('dashboard_widgets')->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->onDelete('set null');
            $table->string('ip_address')->nullable();
            $table->timestamp('viewed_at');

            $table->index(['widget_id', 'viewed_at']);
            $table->index('user_id');
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('widget_views');
        Schema::dropIfExists('widget_share_links');
        Schema::dropIfExists('widget_shares');
    }
};
