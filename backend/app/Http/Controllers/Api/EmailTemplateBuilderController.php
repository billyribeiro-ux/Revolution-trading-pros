<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\EmailTemplate;
use App\Models\EmailBlock;
use App\Models\EmailLayout;
use App\Services\Email\EmailTemplateRenderService;
use App\Services\Email\VariableResolverService;
use Illuminate\Http\Request;

class EmailTemplateBuilderController extends Controller
{
    public function __construct(
        private EmailTemplateRenderService $renderService,
        private VariableResolverService $variableResolver
    ) {}

    /**
     * Get all templates
     */
    public function index(Request $request)
    {
        $query = EmailTemplate::with(['layout', 'blocks']);
        
        if ($request->has('category')) {
            $query->where('category', $request->category);
        }
        
        if ($request->has('email_type')) {
            $query->where('email_type', $request->email_type);
        }
        
        $templates = $query->latest()->get();
        
        return response()->json($templates);
    }

    /**
     * Get single template with blocks
     */
    public function show($id)
    {
        $template = EmailTemplate::with(['layout', 'blocks' => function($query) {
            $query->ordered();
        }])->findOrFail($id);
        
        return response()->json($template);
    }

    /**
     * Create new template
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string',
            'slug' => 'required|string|unique:email_templates',
            'category' => 'required|in:transactional,marketing,automation,system',
            'email_type' => 'required|string',
            'subject' => 'required|string',
            'subject_template' => 'nullable|string',
            'preheader_template' => 'nullable|string',
            'layout_id' => 'nullable|exists:email_layouts,id',
            'is_active' => 'boolean',
        ]);
        
        $template = EmailTemplate::create($validated);
        
        return response()->json($template, 201);
    }

    /**
     * Update template
     */
    public function update(Request $request, $id)
    {
        $template = EmailTemplate::findOrFail($id);
        
        $validated = $request->validate([
            'name' => 'string',
            'subject' => 'string',
            'subject_template' => 'nullable|string',
            'preheader_template' => 'nullable|string',
            'layout_id' => 'nullable|exists:email_layouts,id',
            'global_styles' => 'nullable|array',
            'is_active' => 'boolean',
        ]);
        
        $template->update($validated);
        
        return response()->json($template);
    }

    /**
     * Delete template
     */
    public function destroy($id)
    {
        $template = EmailTemplate::findOrFail($id);
        
        if ($template->is_system) {
            return response()->json(['error' => 'Cannot delete system template'], 403);
        }
        
        $template->delete();
        
        return response()->json(['message' => 'Template deleted']);
    }

    /**
     * Add block to template
     */
    public function addBlock(Request $request, $templateId)
    {
        $validated = $request->validate([
            'block_type' => 'required|in:text,image,button,divider,spacer,columns,header,footer,product,social',
            'content' => 'required|array',
            'styles' => 'nullable|array',
            'settings' => 'nullable|array',
            'order' => 'nullable|integer',
            'parent_block_id' => 'nullable|exists:email_blocks,id',
        ]);
        
        $validated['template_id'] = $templateId;
        
        $block = EmailBlock::create($validated);
        
        return response()->json($block, 201);
    }

    /**
     * Update block
     */
    public function updateBlock(Request $request, $templateId, $blockId)
    {
        $block = EmailBlock::where('template_id', $templateId)
            ->where('id', $blockId)
            ->firstOrFail();
        
        $validated = $request->validate([
            'content' => 'array',
            'styles' => 'nullable|array',
            'settings' => 'nullable|array',
            'order' => 'nullable|integer',
        ]);
        
        $block->update($validated);
        
        return response()->json($block);
    }

    /**
     * Delete block
     */
    public function deleteBlock($templateId, $blockId)
    {
        $block = EmailBlock::where('template_id', $templateId)
            ->where('id', $blockId)
            ->firstOrFail();
        
        $block->delete();
        
        return response()->json(['message' => 'Block deleted']);
    }

    /**
     * Reorder blocks
     */
    public function reorderBlocks(Request $request, $templateId)
    {
        $validated = $request->validate([
            'blocks' => 'required|array',
            'blocks.*.id' => 'required|exists:email_blocks,id',
            'blocks.*.order' => 'required|integer',
        ]);
        
        foreach ($validated['blocks'] as $blockData) {
            EmailBlock::where('id', $blockData['id'])
                ->where('template_id', $templateId)
                ->update(['order' => $blockData['order']]);
        }
        
        return response()->json(['message' => 'Blocks reordered']);
    }

    /**
     * Preview template
     */
    public function preview(Request $request, $id)
    {
        $template = EmailTemplate::findOrFail($id);
        
        $sampleData = $request->input('data', [
            'user' => [
                'name' => 'John Doe',
                'first_name' => 'John',
                'last_name' => 'Doe',
                'email' => 'john@example.com',
            ],
            'site' => [
                'name' => config('app.name'),
                'url' => config('app.url'),
            ],
            'current_year' => date('Y'),
        ]);
        
        $rendered = $this->renderService->render($template, $sampleData);
        
        return response()->json($rendered);
    }

    /**
     * Get available variables
     */
    public function getVariables(Request $request)
    {
        $category = $request->input('category');
        $variables = $this->variableResolver->getAvailableVariables($category);
        
        return response()->json($variables);
    }

    /**
     * Get all layouts
     */
    public function getLayouts()
    {
        $layouts = EmailLayout::all();
        
        return response()->json($layouts);
    }

    /**
     * Duplicate template
     */
    public function duplicate($id)
    {
        $original = EmailTemplate::with('blocks')->findOrFail($id);
        
        $template = $original->replicate();
        $template->name = $original->name . ' (Copy)';
        $template->slug = $original->slug . '-copy-' . time();
        $template->is_system = false;
        $template->save();
        
        // Duplicate blocks
        foreach ($original->blocks as $block) {
            $newBlock = $block->replicate();
            $newBlock->template_id = $template->id;
            $newBlock->save();
        }
        
        return response()->json($template, 201);
    }
}
