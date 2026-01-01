<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        if (!Schema::hasTable('products')) {
            Schema::create('products', function (Blueprint $table) {
                $table->id();
                $table->string('name');
                $table->string('slug')->unique();
                $table->enum('type', ['course', 'indicator', 'bundle']);
                $table->text('description')->nullable();
                $table->text('long_description')->nullable();
                $table->decimal('price', 10, 2);
                $table->boolean('is_active')->default(true);
                $table->json('metadata')->nullable();
                $table->string('thumbnail')->nullable();

                // SEO
                $table->string('meta_title')->nullable();
                $table->text('meta_description')->nullable();
                $table->boolean('indexable')->default(true);
                $table->string('canonical_url')->nullable();

                $table->timestamps();
            });
        }

        if (!Schema::hasTable('user_products')) {
            Schema::create('user_products', function (Blueprint $table) {
                $table->id();
                $table->foreignId('user_id')->constrained()->onDelete('cascade');
                $table->foreignId('product_id')->constrained()->onDelete('cascade');
                $table->timestamp('purchased_at');
                $table->foreignId('order_id')->nullable()->constrained();
                $table->timestamps();

                $table->unique(['user_id', 'product_id']);
            });
        }
    }

    public function down(): void
    {
        Schema::dropIfExists('user_products');
        Schema::dropIfExists('products');
    }
};
