<?php

declare(strict_types=1);

namespace App\Services\Forms;

use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\FormPdfTemplate;
use Barryvdh\DomPDF\Facade\Pdf;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\View;
use Illuminate\Support\Str;

/**
 * PDF Generation Service for Form Submissions
 *
 * Generates PDF documents from form submissions using customizable templates.
 * Supports multiple paper sizes, orientations, and template types.
 */
class PdfGenerationService
{
    /**
     * Generate PDF for a form submission.
     */
    public function generate(FormSubmission $submission, ?FormPdfTemplate $template = null): string
    {
        // Get or create default template
        $template = $template ?? $this->getDefaultTemplate($submission->form);

        // Prepare data for PDF
        $data = $this->prepareData($submission, $template);

        // Generate HTML content
        $html = $this->renderHtml($data, $template);

        // Generate PDF
        $pdf = Pdf::loadHTML($html);

        // Configure PDF settings
        $this->configurePdf($pdf, $template);

        // Generate unique filename
        $filename = $template->generateFilename($submission);
        $path = "pdfs/submissions/{$submission->form_id}/{$filename}";

        // Save PDF to storage
        Storage::put($path, $pdf->output());

        return $path;
    }

    /**
     * Generate PDF and return as download response.
     */
    public function download(FormSubmission $submission, ?FormPdfTemplate $template = null)
    {
        $template = $template ?? $this->getDefaultTemplate($submission->form);
        $data = $this->prepareData($submission, $template);
        $html = $this->renderHtml($data, $template);

        $pdf = Pdf::loadHTML($html);
        $this->configurePdf($pdf, $template);

        $filename = $template->generateFilename($submission);

        return $pdf->download($filename);
    }

    /**
     * Generate PDF and return as stream response.
     */
    public function stream(FormSubmission $submission, ?FormPdfTemplate $template = null)
    {
        $template = $template ?? $this->getDefaultTemplate($submission->form);
        $data = $this->prepareData($submission, $template);
        $html = $this->renderHtml($data, $template);

        $pdf = Pdf::loadHTML($html);
        $this->configurePdf($pdf, $template);

        $filename = $template->generateFilename($submission);

        return $pdf->stream($filename);
    }

    /**
     * Generate PDF and return raw content.
     */
    public function output(FormSubmission $submission, ?FormPdfTemplate $template = null): string
    {
        $template = $template ?? $this->getDefaultTemplate($submission->form);
        $data = $this->prepareData($submission, $template);
        $html = $this->renderHtml($data, $template);

        $pdf = Pdf::loadHTML($html);
        $this->configurePdf($pdf, $template);

        return $pdf->output();
    }

    /**
     * Generate PDF for email attachment.
     */
    public function generateForEmail(FormSubmission $submission): ?array
    {
        $templates = FormPdfTemplate::forForm($submission->form_id)
            ->active()
            ->attachToEmail()
            ->get();

        if ($templates->isEmpty()) {
            return null;
        }

        $attachments = [];

        foreach ($templates as $template) {
            if ($template->shouldApplyTo($submission)) {
                $content = $this->output($submission, $template);
                $filename = $template->generateFilename($submission);

                $attachments[] = [
                    'content' => $content,
                    'filename' => $filename,
                    'mime' => 'application/pdf',
                ];
            }
        }

        return $attachments;
    }

    /**
     * Prepare data for PDF generation.
     */
    protected function prepareData(FormSubmission $submission, FormPdfTemplate $template): array
    {
        $form = $submission->form;
        $formattedData = $submission->getFormattedData();

        // Get field settings from template
        $fieldSettings = $template->field_settings ?? [];

        // Prepare fields array
        $fields = [];
        foreach ($form->fields as $field) {
            // Skip hidden fields unless explicitly included
            if (isset($fieldSettings[$field->name]['hidden']) && $fieldSettings[$field->name]['hidden']) {
                continue;
            }

            $value = $formattedData[$field->name] ?? '';

            // Format value based on field type
            $formattedValue = $this->formatFieldValue($field, $value);

            $fields[] = [
                'name' => $field->name,
                'label' => $fieldSettings[$field->name]['label'] ?? $field->label,
                'value' => $formattedValue,
                'type' => $field->type,
            ];
        }

        // Get payment data if available
        $payment = $submission->payment;

        return [
            // Form info
            'form_id' => $form->id,
            'form_title' => $form->title,
            'form_description' => $form->description,

            // Submission info
            'submission_id' => $submission->id,
            'submission_date' => $submission->created_at->format('F j, Y'),
            'submission_time' => $submission->created_at->format('g:i A'),
            'submission_datetime' => $submission->created_at->format('F j, Y \a\t g:i A'),

            // User info
            'user_name' => $submission->user?->name ?? 'Guest',
            'user_email' => $submission->email ?? $submission->user?->email ?? '',
            'ip_address' => $submission->ip_address,

            // Fields
            'fields' => $fields,

            // Payment info (if applicable)
            'has_payment' => $payment !== null,
            'payment' => $payment ? [
                'amount' => number_format($payment->amount, 2),
                'currency' => $payment->currency,
                'status' => $payment->status,
                'method' => $payment->payment_method,
                'transaction_id' => $payment->transaction_id,
                'date' => $payment->created_at?->format('F j, Y'),
            ] : null,

            // Meta
            'current_date' => now()->format('F j, Y'),
            'current_time' => now()->format('g:i A'),
            'year' => now()->format('Y'),

            // Template settings
            'template' => $template,
            'styles' => array_merge(
                FormPdfTemplate::getDefaultStyles(),
                $template->style_settings ?? []
            ),
        ];
    }

    /**
     * Format field value based on type.
     */
    protected function formatFieldValue($field, $value): string
    {
        if (is_array($value)) {
            return implode(', ', array_filter($value));
        }

        return match($field->type) {
            'date' => $this->formatDate($value),
            'datetime' => $this->formatDateTime($value),
            'file', 'image' => $this->formatFile($value),
            'checkbox' => $value ? 'Yes' : 'No',
            'currency', 'payment' => $this->formatCurrency($value),
            'url' => "<a href=\"{$value}\">{$value}</a>",
            'email' => "<a href=\"mailto:{$value}\">{$value}</a>",
            'phone', 'phone_intl' => "<a href=\"tel:{$value}\">{$value}</a>",
            'textarea', 'rich_text_input' => nl2br(e($value)),
            'rating' => str_repeat('★', (int)$value) . str_repeat('☆', 5 - (int)$value),
            'nps' => "{$value}/10",
            default => e((string)$value),
        };
    }

    /**
     * Format date value.
     */
    protected function formatDate($value): string
    {
        if (empty($value)) {
            return '';
        }

        try {
            return \Carbon\Carbon::parse($value)->format('F j, Y');
        } catch (\Exception $e) {
            return (string)$value;
        }
    }

    /**
     * Format datetime value.
     */
    protected function formatDateTime($value): string
    {
        if (empty($value)) {
            return '';
        }

        try {
            return \Carbon\Carbon::parse($value)->format('F j, Y \a\t g:i A');
        } catch (\Exception $e) {
            return (string)$value;
        }
    }

    /**
     * Format file value.
     */
    protected function formatFile($value): string
    {
        if (empty($value)) {
            return '';
        }

        if (is_array($value)) {
            $files = array_map(fn($f) => basename($f), $value);
            return implode(', ', $files);
        }

        return basename($value);
    }

    /**
     * Format currency value.
     */
    protected function formatCurrency($value): string
    {
        if (empty($value)) {
            return '';
        }

        return '$' . number_format((float)$value, 2);
    }

    /**
     * Render HTML from template.
     */
    protected function renderHtml(array $data, FormPdfTemplate $template): string
    {
        $styles = $data['styles'];

        // Process template sections with variable replacement
        $header = $this->processTemplate(
            $template->header_html ?? FormPdfTemplate::getDefaultHeader(),
            $data
        );

        $body = $this->processTemplate(
            $template->body_html ?? FormPdfTemplate::getDefaultBody(),
            $data
        );

        $footer = $this->processTemplate(
            $template->footer_html ?? FormPdfTemplate::getDefaultFooter(),
            $data
        );

        // Build complete HTML document
        return <<<HTML
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title>{$data['form_title']} - Submission #{$data['submission_id']}</title>
    <style>
        @page {
            margin: {$styles['margin_top']}mm {$styles['margin_right']}mm {$styles['margin_bottom']}mm {$styles['margin_left']}mm;
        }
        body {
            font-family: {$styles['font_family']};
            font-size: {$styles['font_size']}pt;
            line-height: {$styles['line_height']};
            color: {$styles['text_color']};
            background-color: {$styles['background_color']};
            margin: 0;
            padding: 0;
        }
        .header {
            position: fixed;
            top: -{$styles['margin_top']}mm;
            left: 0;
            right: 0;
            height: 60px;
        }
        .footer {
            position: fixed;
            bottom: -{$styles['margin_bottom']}mm;
            left: 0;
            right: 0;
            height: 50px;
        }
        .content {
            margin-top: 40px;
            margin-bottom: 40px;
        }
        .page-break {
            page-break-after: always;
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th, td {
            padding: 8px 12px;
            text-align: left;
            border-bottom: 1px solid {$styles['border_color']};
        }
        th {
            background-color: #f3f4f6;
            font-weight: 600;
        }
        .field-row {
            margin-bottom: 15px;
            padding-bottom: 15px;
            border-bottom: 1px solid {$styles['border_color']};
        }
        .field-label {
            font-weight: 600;
            color: {$styles['secondary_color']};
            margin-bottom: 5px;
            display: block;
        }
        .field-value {
            color: {$styles['text_color']};
        }
        .payment-box {
            background-color: #f0fdf4;
            border: 1px solid #86efac;
            border-radius: 8px;
            padding: 15px;
            margin-top: 20px;
        }
        .payment-total {
            font-size: 18pt;
            font-weight: 700;
            color: #059669;
        }
        a {
            color: {$styles['primary_color']};
            text-decoration: none;
        }
        .text-center { text-align: center; }
        .text-right { text-align: right; }
        .font-bold { font-weight: 700; }
        .text-sm { font-size: 10pt; }
        .text-lg { font-size: 14pt; }
        .mb-4 { margin-bottom: 16px; }
        .mt-4 { margin-top: 16px; }
    </style>
</head>
<body>
    <div class="header">
        {$header}
    </div>

    <div class="footer">
        {$footer}
    </div>

    <div class="content">
        {$body}
    </div>
</body>
</html>
HTML;
    }

    /**
     * Process template with variable replacement.
     */
    protected function processTemplate(string $template, array $data): string
    {
        // Simple variable replacement {{variable}}
        $template = preg_replace_callback('/\{\{(\w+)\}\}/', function ($matches) use ($data) {
            $key = $matches[1];
            return $data[$key] ?? '';
        }, $template);

        // Handle {{#each fields}} loop
        if (str_contains($template, '{{#each fields}}')) {
            $fieldsHtml = '';
            foreach ($data['fields'] as $field) {
                $fieldTemplate = preg_replace(
                    '/\{\{#each fields\}\}(.*?)\{\{\/each\}\}/s',
                    '$1',
                    $template
                );

                $fieldHtml = $fieldTemplate;
                $fieldHtml = str_replace('{{label}}', $field['label'], $fieldHtml);
                $fieldHtml = str_replace('{{value}}', $field['value'], $fieldHtml);
                $fieldHtml = str_replace('{{name}}', $field['name'], $fieldHtml);
                $fieldHtml = str_replace('{{type}}', $field['type'], $fieldHtml);

                $fieldsHtml .= $fieldHtml;
            }

            $template = preg_replace(
                '/\{\{#each fields\}\}.*?\{\{\/each\}\}/s',
                $fieldsHtml,
                $template
            );
        }

        // Handle conditional {{#if payment}}
        if ($data['has_payment'] && $data['payment']) {
            $template = preg_replace_callback(
                '/\{\{#if payment\}\}(.*?)\{\{\/if\}\}/s',
                function ($matches) use ($data) {
                    $content = $matches[1];
                    foreach ($data['payment'] as $key => $value) {
                        $content = str_replace("{{payment.{$key}}}", (string)$value, $content);
                    }
                    return $content;
                },
                $template
            );
        } else {
            $template = preg_replace('/\{\{#if payment\}\}.*?\{\{\/if\}\}/s', '', $template);
        }

        return $template;
    }

    /**
     * Configure PDF settings.
     */
    protected function configurePdf($pdf, FormPdfTemplate $template): void
    {
        $dimensions = $template->paper_dimensions;

        $pdf->setPaper([0, 0, $dimensions['width'], $dimensions['height']]);

        $options = $pdf->getOptions();
        $options->set('isHtml5ParserEnabled', true);
        $options->set('isRemoteEnabled', true);
        $options->set('defaultFont', 'DejaVu Sans');

        if ($template->password) {
            $pdf->setEncryption($template->password);
        }
    }

    /**
     * Get or create default template for form.
     */
    protected function getDefaultTemplate(Form $form): FormPdfTemplate
    {
        $template = FormPdfTemplate::forForm($form->id)
            ->active()
            ->ofType(FormPdfTemplate::TYPE_SUMMARY)
            ->first();

        if ($template) {
            return $template;
        }

        // Create a virtual template (not saved)
        $template = new FormPdfTemplate([
            'form_id' => $form->id,
            'name' => 'Default Summary',
            'template_type' => FormPdfTemplate::TYPE_SUMMARY,
            'active' => true,
            'paper_size' => FormPdfTemplate::SIZE_LETTER,
            'orientation' => FormPdfTemplate::ORIENTATION_PORTRAIT,
            'header_html' => FormPdfTemplate::getDefaultHeader(),
            'body_html' => FormPdfTemplate::getDefaultBody(),
            'footer_html' => FormPdfTemplate::getDefaultFooter(),
            'style_settings' => FormPdfTemplate::getDefaultStyles(),
        ]);

        return $template;
    }

    /**
     * Generate receipt PDF for payment.
     */
    public function generateReceipt(FormSubmission $submission): ?string
    {
        if (!$submission->payment) {
            return null;
        }

        $template = FormPdfTemplate::forForm($submission->form_id)
            ->active()
            ->ofType(FormPdfTemplate::TYPE_RECEIPT)
            ->first();

        if (!$template) {
            // Create receipt template
            $template = new FormPdfTemplate([
                'form_id' => $submission->form_id,
                'name' => 'Payment Receipt',
                'template_type' => FormPdfTemplate::TYPE_RECEIPT,
                'header_html' => $this->getReceiptHeader(),
                'body_html' => $this->getReceiptBody(),
                'footer_html' => FormPdfTemplate::getDefaultFooter(),
            ]);
        }

        return $this->generate($submission, $template);
    }

    /**
     * Get receipt header HTML.
     */
    protected function getReceiptHeader(): string
    {
        return <<<'HTML'
<div style="text-align: center; padding: 20px 0; border-bottom: 3px solid #059669;">
    <h1 style="margin: 0; color: #059669; font-size: 28px;">PAYMENT RECEIPT</h1>
    <p style="margin: 10px 0 0; color: #6b7280; font-size: 14px;">Receipt #{{submission_id}}</p>
</div>
HTML;
    }

    /**
     * Get receipt body HTML.
     */
    protected function getReceiptBody(): string
    {
        return <<<'HTML'
<div style="padding: 30px 0;">
    <div style="margin-bottom: 30px;">
        <h3 style="color: #374151; margin-bottom: 10px;">Payment Details</h3>
        <table style="width: 100%;">
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;"><strong>Date:</strong></td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">{{#if payment}}{{payment.date}}{{/if}}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;"><strong>Transaction ID:</strong></td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">{{#if payment}}{{payment.transaction_id}}{{/if}}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;"><strong>Payment Method:</strong></td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">{{#if payment}}{{payment.method}}{{/if}}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;"><strong>Status:</strong></td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">{{#if payment}}{{payment.status}}{{/if}}</td>
            </tr>
        </table>
    </div>

    <div style="background: #f0fdf4; border: 2px solid #059669; border-radius: 8px; padding: 20px; text-align: center; margin: 20px 0;">
        <p style="margin: 0; color: #6b7280; font-size: 14px;">Amount Paid</p>
        <p style="margin: 5px 0 0; color: #059669; font-size: 32px; font-weight: 700;">
            {{#if payment}}${{payment.amount}} {{payment.currency}}{{/if}}
        </p>
    </div>

    <div style="margin-top: 30px;">
        <h3 style="color: #374151; margin-bottom: 10px;">Customer Information</h3>
        <p style="margin: 5px 0;"><strong>Name:</strong> {{user_name}}</p>
        <p style="margin: 5px 0;"><strong>Email:</strong> {{user_email}}</p>
    </div>

    <div style="margin-top: 30px;">
        <h3 style="color: #374151; margin-bottom: 10px;">Form: {{form_title}}</h3>
        {{#each fields}}
        <div style="margin-bottom: 10px; padding-bottom: 10px; border-bottom: 1px dashed #e5e7eb;">
            <span style="color: #6b7280;">{{label}}:</span> {{value}}
        </div>
        {{/each}}
    </div>
</div>
HTML;
    }
}
