<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('invoice_settings', function (Blueprint $table) {
            $table->id();

            // Company Information
            $table->string('company_name')->nullable();
            $table->string('company_email')->nullable();
            $table->string('company_phone', 50)->nullable();
            $table->string('company_address', 500)->nullable();
            $table->string('company_city', 100)->nullable();
            $table->string('company_state', 100)->nullable();
            $table->string('company_zip', 20)->nullable();
            $table->string('company_country', 2)->default('US');
            $table->string('tax_id', 100)->nullable();

            // Branding
            $table->string('logo_path')->nullable();
            $table->string('primary_color', 7)->default('#2563eb');
            $table->string('secondary_color', 7)->default('#1f2937');
            $table->string('accent_color', 7)->default('#10b981');
            $table->string('font_family', 100)->default('Inter, sans-serif');

            // Content
            $table->string('header_text', 100)->default('INVOICE');
            $table->string('footer_text', 500)->default('Thank you for your business!');
            $table->string('payment_terms', 255)->default('Due upon receipt');
            $table->text('notes_template')->nullable();

            // Display Options
            $table->boolean('show_logo')->default(true);
            $table->boolean('show_tax_id')->default(true);
            $table->boolean('show_payment_method')->default(true);
            $table->boolean('show_due_date')->default(true);

            // Invoice Numbering
            $table->string('invoice_prefix', 20)->default('INV-');

            // Advanced
            $table->text('custom_css')->nullable();

            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('invoice_settings');
    }
};
