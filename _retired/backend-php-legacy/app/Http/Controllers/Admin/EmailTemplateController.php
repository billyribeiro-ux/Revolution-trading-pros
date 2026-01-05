<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailTemplate;
use App\Services\TemplateRenderService;
use Illuminate\Http\Request;

class EmailTemplateController extends Controller
{
    public function __construct(
        protected TemplateRenderService $templateService
    ) {}

    /**
     * List all templates
     */
    public function index(Request $request)
    {
        $query = EmailTemplate::query();

        if ($request->has('email_type')) {
            $query->byType($request->email_type);
        }

        $templates = $query->latest()->get();

        return response()->json($templates);
    }

    /**
     * Get single template
     */
    public function show($id)
    {
        $template = EmailTemplate::findOrFail($id);

        return response()->json($template);
    }

    /**
     * Create new template
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|unique:email_templates,slug',
            'subject' => 'required|string',
            'body_html' => 'required|string',
            'body_text' => 'nullable|string',
            'email_type' => 'required|string',
            'variables' => 'nullable|array',
            'is_active' => 'boolean',
        ]);

        $template = EmailTemplate::create($validated);

        return response()->json([
            'message' => 'Template created successfully',
            'template' => $template,
        ], 201);
    }

    /**
     * Update template
     */
    public function update(Request $request, $id)
    {
        $template = EmailTemplate::findOrFail($id);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'subject' => 'string',
            'body_html' => 'string',
            'body_text' => 'nullable|string',
            'email_type' => 'string',
            'variables' => 'nullable|array',
            'is_active' => 'boolean',
        ]);

        $template->update($validated);

        return response()->json([
            'message' => 'Template updated successfully',
            'template' => $template,
        ]);
    }

    /**
     * Delete template
     */
    public function destroy($id)
    {
        $template = EmailTemplate::findOrFail($id);
        $template->delete();

        return response()->json([
            'message' => 'Template deleted successfully',
        ]);
    }

    /**
     * Preview template with sample data
     */
    public function preview(Request $request, $id)
    {
        $template = EmailTemplate::findOrFail($id);

        $sampleData = $request->input('data', [
            'user' => ['name' => 'John Doe', 'email' => 'john@example.com'],
            'order' => ['id' => '12345', 'total' => '$99.99'],
        ]);

        $rendered = [
            'subject' => $this->templateService->render($template->subject, $sampleData),
            'body_html' => $this->templateService->render($template->body_html, $sampleData),
        ];

        return response()->json($rendered);
    }
}
