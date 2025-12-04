<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * Form PDF Templates Migration
 *
 * Creates tables for PDF generation from form submissions.
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('form_pdf_templates', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->string('name');
            $table->text('description')->nullable();
            $table->string('template_type', 50)->default('custom')->index();
            $table->boolean('active')->default(true)->index();

            // Paper settings
            $table->string('paper_size', 20)->default('letter');
            $table->string('orientation', 20)->default('portrait');

            // Template content
            $table->text('header_html')->nullable();
            $table->text('body_html')->nullable();
            $table->text('footer_html')->nullable();
            $table->text('cover_letter_html')->nullable(); // Cover letter page

            // Settings
            $table->json('field_settings')->nullable(); // Which fields to include/exclude
            $table->json('style_settings')->nullable(); // Colors, fonts, margins
            $table->json('conditional_logic')->nullable(); // When to apply template

            // Attachment options
            $table->boolean('attach_to_email')->default(false);
            $table->boolean('attach_to_confirmation')->default(false);

            // File options
            $table->string('filename_pattern')->nullable();
            $table->string('password')->nullable(); // PDF password protection
            $table->boolean('flatten_form')->default(false);

            // FluentForm PDF Generator features
            // Logo/Branding
            $table->string('logo_url', 500)->nullable();
            $table->string('logo_position', 20)->default('left'); // left, center, right

            // Watermark settings
            $table->string('watermark_text')->nullable();
            $table->string('watermark_image', 500)->nullable();
            $table->decimal('watermark_opacity', 3, 2)->default(0.10);

            // Display options
            $table->boolean('show_page_numbers')->default(true);
            $table->boolean('show_form_title')->default(true);
            $table->boolean('show_submission_date')->default(true);
            $table->boolean('show_field_labels')->default(true);
            $table->boolean('include_empty_fields')->default(false);
            $table->string('entry_view', 20)->default('list'); // list, table, grid

            // Text/Font settings (FluentForm compatible)
            $table->string('text_direction', 5)->default('ltr'); // ltr, rtl
            $table->string('font_family', 100)->default('DejaVu Sans');
            $table->unsignedTinyInteger('font_size')->default(12);
            $table->string('font_color', 20)->default('#111827');
            $table->string('heading_color', 20)->default('#1e3a8a');
            $table->string('accent_color', 20)->default('#3b82f6');
            $table->string('background_color', 20)->default('#ffffff');
            $table->string('border_style', 20)->default('solid'); // none, solid, dashed, dotted

            $table->timestamps();
            $table->softDeletes();

            // Indexes
            $table->index(['form_id', 'active']);
            $table->index(['form_id', 'template_type']);
        });

        // Table to store generated PDFs
        Schema::create('form_pdf_files', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->foreignId('submission_id')->constrained('form_submissions')->onDelete('cascade');
            $table->foreignId('template_id')->nullable()->constrained('form_pdf_templates')->onDelete('set null');
            $table->string('filename');
            $table->string('path');
            $table->unsignedInteger('file_size')->nullable();
            $table->string('mime_type', 50)->default('application/pdf');
            $table->string('status', 20)->default('generated'); // generated, sent, downloaded
            $table->timestamp('generated_at');
            $table->timestamp('sent_at')->nullable();
            $table->timestamp('downloaded_at')->nullable();
            $table->unsignedSmallInteger('download_count')->default(0);
            $table->timestamps();

            // Indexes
            $table->index(['form_id', 'submission_id']);
            $table->index(['submission_id', 'template_id']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('form_pdf_files');
        Schema::dropIfExists('form_pdf_templates');
    }
};
