<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\FormPdfFile;
use App\Models\FormPdfTemplate;
use App\Services\Forms\PdfGenerationService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Storage;

/**
 * FormPdfController - PDF Generation for Form Submissions
 *
 * Handles generating, downloading, and previewing PDFs from form submissions.
 * Integrates with the FluentForm Pro PDF feature equivalent.
 */
class FormPdfController extends Controller
{
    public function __construct(
        protected PdfGenerationService $pdfService
    ) {}

    /**
     * List all PDFs for a submission.
     */
    public function index(int $formId, int $submissionId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->firstOrFail();

        $pdfs = FormPdfFile::forSubmission($submissionId)
            ->orderByDesc('created_at')
            ->get()
            ->map(fn($pdf) => [
                'id' => $pdf->id,
                'filename' => $pdf->filename,
                'file_size_formatted' => $pdf->file_size_formatted,
                'generated_at' => $pdf->generated_at?->toIso8601String(),
                'download_count' => $pdf->download_count,
                'url' => $pdf->url,
            ]);

        return response()->json([
            'success' => true,
            'pdfs' => $pdfs,
        ]);
    }

    /**
     * Generate a new PDF for a submission.
     */
    public function generate(Request $request, int $formId, int $submissionId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->with(['form', 'user', 'payment'])
            ->firstOrFail();

        // Get optional template
        $template = null;
        if ($request->has('template_id')) {
            $template = FormPdfTemplate::where('id', $request->template_id)
                ->where('form_id', $formId)
                ->active()
                ->first();
        }

        try {
            // Generate PDF and save to storage
            $path = $this->pdfService->generate($submission, $template);

            // Get file info
            $fileSize = Storage::size($path);
            $filename = basename($path);

            // Create PDF file record
            $pdfFile = FormPdfFile::create([
                'form_id' => $formId,
                'submission_id' => $submissionId,
                'template_id' => $template?->id,
                'filename' => $filename,
                'path' => $path,
                'file_size' => $fileSize,
                'mime_type' => 'application/pdf',
                'status' => FormPdfFile::STATUS_GENERATED,
                'generated_at' => now(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'PDF generated successfully',
                'pdf' => [
                    'id' => $pdfFile->id,
                    'filename' => $pdfFile->filename,
                    'file_size_formatted' => $pdfFile->file_size_formatted,
                    'generated_at' => $pdfFile->generated_at?->toIso8601String(),
                    'download_count' => $pdfFile->download_count,
                    'url' => $pdfFile->url,
                ],
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to generate PDF: ' . $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Download a PDF file.
     */
    public function download(int $formId, int $submissionId, int $pdfId): Response|JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $pdfFile = FormPdfFile::where('id', $pdfId)
            ->where('form_id', $formId)
            ->where('submission_id', $submissionId)
            ->firstOrFail();

        if (!$pdfFile->exists()) {
            return response()->json([
                'success' => false,
                'message' => 'PDF file not found in storage',
            ], 404);
        }

        // Record the download
        $pdfFile->recordDownload();

        $content = $pdfFile->getContents();

        return response($content)
            ->header('Content-Type', 'application/pdf')
            ->header('Content-Disposition', 'attachment; filename="' . $pdfFile->filename . '"')
            ->header('Content-Length', (string)strlen($content));
    }

    /**
     * Preview a PDF file (stream in browser).
     */
    public function preview(int $formId, int $submissionId, int $pdfId): Response|JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $pdfFile = FormPdfFile::where('id', $pdfId)
            ->where('form_id', $formId)
            ->where('submission_id', $submissionId)
            ->firstOrFail();

        if (!$pdfFile->exists()) {
            return response()->json([
                'success' => false,
                'message' => 'PDF file not found in storage',
            ], 404);
        }

        $content = $pdfFile->getContents();

        return response($content)
            ->header('Content-Type', 'application/pdf')
            ->header('Content-Disposition', 'inline; filename="' . $pdfFile->filename . '"')
            ->header('Content-Length', (string)strlen($content));
    }

    /**
     * Delete a PDF file.
     */
    public function destroy(int $formId, int $submissionId, int $pdfId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('delete', $form);

        $pdfFile = FormPdfFile::where('id', $pdfId)
            ->where('form_id', $formId)
            ->where('submission_id', $submissionId)
            ->firstOrFail();

        $pdfFile->delete();

        return response()->json([
            'success' => true,
            'message' => 'PDF deleted successfully',
        ]);
    }

    /**
     * Generate and download PDF immediately (without saving).
     */
    public function downloadDirect(Request $request, int $formId, int $submissionId)
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->with(['form', 'user', 'payment'])
            ->firstOrFail();

        // Get optional template
        $template = null;
        if ($request->has('template_id')) {
            $template = FormPdfTemplate::where('id', $request->template_id)
                ->where('form_id', $formId)
                ->active()
                ->first();
        }

        return $this->pdfService->download($submission, $template);
    }

    /**
     * Stream PDF preview directly (without saving).
     */
    public function streamDirect(Request $request, int $formId, int $submissionId)
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->with(['form', 'user', 'payment'])
            ->firstOrFail();

        // Get optional template
        $template = null;
        if ($request->has('template_id')) {
            $template = FormPdfTemplate::where('id', $request->template_id)
                ->where('form_id', $formId)
                ->active()
                ->first();
        }

        return $this->pdfService->stream($submission, $template);
    }

    /**
     * List available PDF templates for a form.
     */
    public function templates(int $formId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('view', $form);

        $templates = FormPdfTemplate::forForm($formId)
            ->active()
            ->orderBy('name')
            ->get()
            ->map(fn($template) => [
                'id' => $template->id,
                'name' => $template->name,
                'description' => $template->description,
                'template_type' => $template->template_type,
                'paper_size' => $template->paper_size,
                'orientation' => $template->orientation,
            ]);

        return response()->json([
            'success' => true,
            'templates' => $templates,
        ]);
    }

    /**
     * Create a new PDF template.
     */
    public function storeTemplate(Request $request, int $formId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('update', $form);

        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'template_type' => 'required|in:' . implode(',', array_keys(FormPdfTemplate::TEMPLATE_TYPES)),
            'paper_size' => 'nullable|in:' . implode(',', array_keys(FormPdfTemplate::PAPER_SIZES)),
            'orientation' => 'nullable|in:portrait,landscape',
            'header_html' => 'nullable|string',
            'body_html' => 'nullable|string',
            'footer_html' => 'nullable|string',
            'style_settings' => 'nullable|array',
            'field_settings' => 'nullable|array',
            'conditional_logic' => 'nullable|array',
            'attach_to_email' => 'nullable|boolean',
            'password' => 'nullable|string|max:255',
            'active' => 'nullable|boolean',
        ]);

        $template = FormPdfTemplate::create([
            'form_id' => $formId,
            ...$validated,
        ]);

        return response()->json([
            'success' => true,
            'message' => 'Template created successfully',
            'template' => $template,
        ], 201);
    }

    /**
     * Update a PDF template.
     */
    public function updateTemplate(Request $request, int $formId, int $templateId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('update', $form);

        $template = FormPdfTemplate::where('id', $templateId)
            ->where('form_id', $formId)
            ->firstOrFail();

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'description' => 'nullable|string|max:1000',
            'template_type' => 'sometimes|in:' . implode(',', array_keys(FormPdfTemplate::TEMPLATE_TYPES)),
            'paper_size' => 'nullable|in:' . implode(',', array_keys(FormPdfTemplate::PAPER_SIZES)),
            'orientation' => 'nullable|in:portrait,landscape',
            'header_html' => 'nullable|string',
            'body_html' => 'nullable|string',
            'footer_html' => 'nullable|string',
            'style_settings' => 'nullable|array',
            'field_settings' => 'nullable|array',
            'conditional_logic' => 'nullable|array',
            'attach_to_email' => 'nullable|boolean',
            'password' => 'nullable|string|max:255',
            'active' => 'nullable|boolean',
        ]);

        $template->update($validated);

        return response()->json([
            'success' => true,
            'message' => 'Template updated successfully',
            'template' => $template->fresh(),
        ]);
    }

    /**
     * Delete a PDF template.
     */
    public function destroyTemplate(int $formId, int $templateId): JsonResponse
    {
        $form = Form::findOrFail($formId);
        $this->authorize('update', $form);

        $template = FormPdfTemplate::where('id', $templateId)
            ->where('form_id', $formId)
            ->firstOrFail();

        $template->delete();

        return response()->json([
            'success' => true,
            'message' => 'Template deleted successfully',
        ]);
    }

    /**
     * Preview a template with sample data.
     */
    public function previewTemplate(Request $request, int $formId, int $templateId)
    {
        $form = Form::findOrFail($formId);
        $this->authorize('view', $form);

        $template = FormPdfTemplate::where('id', $templateId)
            ->where('form_id', $formId)
            ->firstOrFail();

        // Get the most recent submission or create sample data
        $submission = FormSubmission::where('form_id', $formId)
            ->with(['form', 'user', 'payment'])
            ->latest()
            ->first();

        if (!$submission) {
            return response()->json([
                'success' => false,
                'message' => 'No submissions available for preview. Submit a form entry first.',
            ], 404);
        }

        return $this->pdfService->stream($submission, $template);
    }

    /**
     * Generate payment receipt PDF.
     */
    public function receipt(int $formId, int $submissionId)
    {
        $form = Form::findOrFail($formId);
        $this->authorize('viewSubmissions', $form);

        $submission = FormSubmission::where('form_id', $formId)
            ->where('id', $submissionId)
            ->with(['form', 'user', 'payment'])
            ->firstOrFail();

        if (!$submission->payment) {
            return response()->json([
                'success' => false,
                'message' => 'No payment associated with this submission',
            ], 404);
        }

        try {
            $path = $this->pdfService->generateReceipt($submission);

            if (!$path) {
                return response()->json([
                    'success' => false,
                    'message' => 'Failed to generate receipt',
                ], 500);
            }

            // Get file info
            $fileSize = Storage::size($path);
            $filename = basename($path);

            // Create PDF file record
            $pdfFile = FormPdfFile::create([
                'form_id' => $formId,
                'submission_id' => $submissionId,
                'filename' => $filename,
                'path' => $path,
                'file_size' => $fileSize,
                'mime_type' => 'application/pdf',
                'status' => FormPdfFile::STATUS_GENERATED,
                'generated_at' => now(),
            ]);

            return response()->json([
                'success' => true,
                'message' => 'Receipt generated successfully',
                'pdf' => [
                    'id' => $pdfFile->id,
                    'filename' => $pdfFile->filename,
                    'file_size_formatted' => $pdfFile->file_size_formatted,
                    'generated_at' => $pdfFile->generated_at?->toIso8601String(),
                    'download_count' => $pdfFile->download_count,
                    'url' => $pdfFile->url,
                ],
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to generate receipt: ' . $e->getMessage(),
            ], 500);
        }
    }
}
