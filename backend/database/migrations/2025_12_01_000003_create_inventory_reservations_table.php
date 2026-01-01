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
        Schema::create('inventory_reservations', function (Blueprint $table) {
            $table->id();

            // Unique reservation identifier
            $table->uuid('reservation_id')->unique();

            // Product reference
            $table->foreignId('product_id')->constrained()->cascadeOnDelete();

            // Quantity reserved
            $table->unsignedInteger('quantity');

            // Cart/User/Order references
            $table->string('cart_id')->nullable()->index();
            $table->foreignId('user_id')->nullable()->constrained()->nullOnDelete();
            $table->foreignId('order_id')->nullable()->constrained()->nullOnDelete();

            // Status tracking
            $table->string('status')->default('active')->index(); // active, committed, released, expired, backorder

            // TTL tracking
            $table->timestamp('expires_at')->nullable()->index();
            $table->timestamp('committed_at')->nullable();
            $table->timestamp('released_at')->nullable();

            // Additional metadata
            $table->json('metadata')->nullable();

            $table->timestamps();

            // Composite indexes
            $table->index(['product_id', 'status']);
            $table->index(['cart_id', 'status']);
            $table->index(['user_id', 'status']);
            $table->index(['status', 'expires_at']);
        });

        // Add reserved_quantity column to products table if not exists
        if (!Schema::hasColumn('products', 'reserved_quantity')) {
            Schema::table('products', function (Blueprint $table) {
                $table->unsignedInteger('reserved_quantity')->default(0)->after('stock_quantity');
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('inventory_reservations');

        if (Schema::hasColumn('products', 'reserved_quantity')) {
            Schema::table('products', function (Blueprint $table) {
                $table->dropColumn('reserved_quantity');
            });
        }
    }
};
