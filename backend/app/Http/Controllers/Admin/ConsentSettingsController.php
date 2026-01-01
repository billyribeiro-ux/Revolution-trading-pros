<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\ConsentSettings;
use App\Models\ConsentBannerTemplate;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * ConsentSettingsController
 *
 * Complete API for managing consent settings and banner templates.
 * Mirrors functionality from Consent Magic Pro WordPress plugin.
 */
class ConsentSettingsController extends Controller
{
    /**
     * Get all consent settings
     */
    public function index(Request $request): JsonResponse
    {
        $group = $request->query('group');

        $settings = $group
            ? ConsentSettings::getByGroup($group)
            : ConsentSettings::getAllSettings();

        return response()->json([
            'success' => true,
            'data' => $settings,
        ]);
    }

    /**
     * Get public settings (for frontend)
     */
    public function publicSettings(): JsonResponse
    {
        $settings = ConsentSettings::getPublicSettings();
        $activeTemplate = ConsentBannerTemplate::getActive();

        return response()->json([
            'success' => true,
            'data' => [
                'settings' => $settings,
                'template' => $activeTemplate?->toExportArray(),
                'cssVariables' => $activeTemplate?->getCssVariables(),
            ],
        ]);
    }

    /**
     * Get a specific setting
     */
    public function show(string $key): JsonResponse
    {
        $setting = ConsentSettings::where('key', $key)->first();

        if (!$setting) {
            return response()->json([
                'success' => false,
                'message' => 'Setting not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'key' => $setting->key,
                'value' => $setting->getCastedValue(),
                'type' => $setting->type,
                'group' => $setting->group,
                'description' => $setting->description,
                'is_public' => $setting->is_public,
            ],
        ]);
    }

    /**
     * Update a specific setting
     */
    public function update(Request $request, string $key): JsonResponse
    {
        $validated = $request->validate([
            'value' => 'required',
            'type' => 'nullable|string|in:string,integer,boolean,json,array',
            'group' => 'nullable|string|max:50',
            'description' => 'nullable|string',
            'is_public' => 'nullable|boolean',
        ]);

        $setting = ConsentSettings::setValue($key, $validated['value'], [
            'type' => $validated['type'] ?? null,
            'group' => $validated['group'] ?? 'general',
            'description' => $validated['description'] ?? null,
            'is_public' => $validated['is_public'] ?? false,
        ]);

        return response()->json([
            'success' => true,
            'message' => 'Setting updated successfully',
            'data' => [
                'key' => $setting->key,
                'value' => $setting->getCastedValue(),
            ],
        ]);
    }

    /**
     * Bulk update settings
     */
    public function bulkUpdate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'settings' => 'required|array',
            'settings.*' => 'required',
        ]);

        ConsentSettings::bulkUpdate($validated['settings']);

        return response()->json([
            'success' => true,
            'message' => 'Settings updated successfully',
        ]);
    }

    /**
     * Reset settings to defaults
     */
    public function reset(): JsonResponse
    {
        // Clear existing settings
        ConsentSettings::truncate();

        // Initialize defaults
        ConsentSettings::initializeDefaults();

        // Clear cache
        ConsentSettings::clearCache();

        return response()->json([
            'success' => true,
            'message' => 'Settings reset to defaults',
            'data' => ConsentSettings::getAllSettings(),
        ]);
    }

    // =========================================================================
    // BANNER TEMPLATES
    // =========================================================================

    /**
     * Get all banner templates
     */
    public function templates(Request $request): JsonResponse
    {
        $templates = ConsentBannerTemplate::getAllTemplates();

        return response()->json([
            'success' => true,
            'data' => [
                'templates' => $templates,
                'activeId' => ConsentBannerTemplate::getActive()?->id,
            ],
        ]);
    }

    /**
     * Get active banner template
     */
    public function activeTemplate(): JsonResponse
    {
        $template = ConsentBannerTemplate::getActive();

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'No active template found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => $template,
        ]);
    }

    /**
     * Get a specific template
     */
    public function showTemplate(int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => $template,
        ]);
    }

    /**
     * Create a new template
     */
    public function createTemplate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:100',
            'category' => 'nullable|string|max:50',
            'description' => 'nullable|string',
            'layout_type' => 'nullable|string|in:bar,popup,floating,drawer',
            'position' => 'nullable|string|in:top,bottom,center',
            'position_horizontal' => 'nullable|string|in:left,center,right',
            'background_color' => 'nullable|string|max:20',
            'text_color' => 'nullable|string|max:20',
            'link_color' => 'nullable|string|max:20',
            'title_color' => 'nullable|string|max:20',
            'border_color' => 'nullable|string|max:20',
            'border_style' => 'nullable|string|max:20',
            'border_width' => 'nullable|integer|min:0|max:20',
            'accept_btn_bg' => 'nullable|string|max:20',
            'accept_btn_text' => 'nullable|string|max:20',
            'accept_btn_hover_bg' => 'nullable|string|max:20',
            'reject_btn_bg' => 'nullable|string|max:20',
            'reject_btn_text' => 'nullable|string|max:20',
            'reject_btn_hover_bg' => 'nullable|string|max:20',
            'settings_btn_bg' => 'nullable|string|max:20',
            'settings_btn_text' => 'nullable|string|max:20',
            'settings_btn_border' => 'nullable|string|max:20',
            'toggle_active_color' => 'nullable|string|max:20',
            'toggle_inactive_color' => 'nullable|string|max:20',
            'font_family' => 'nullable|string|max:100',
            'title_font_size' => 'nullable|integer|min:10|max:48',
            'title_font_weight' => 'nullable|integer|min:100|max:900',
            'body_font_size' => 'nullable|integer|min:10|max:32',
            'body_font_weight' => 'nullable|integer|min:100|max:900',
            'btn_font_size' => 'nullable|integer|min:10|max:24',
            'btn_font_weight' => 'nullable|integer|min:100|max:900',
            'padding_top' => 'nullable|integer|min:0|max:100',
            'padding_bottom' => 'nullable|integer|min:0|max:100',
            'padding_left' => 'nullable|integer|min:0|max:100',
            'padding_right' => 'nullable|integer|min:0|max:100',
            'btn_padding_x' => 'nullable|integer|min:0|max:60',
            'btn_padding_y' => 'nullable|integer|min:0|max:40',
            'btn_margin' => 'nullable|integer|min:0|max:40',
            'btn_border_radius' => 'nullable|integer|min:0|max:50',
            'container_border_radius' => 'nullable|integer|min:0|max:50',
            'container_max_width' => 'nullable|integer|min:300|max:2000',
            'animation_type' => 'nullable|string|in:slide,fade,none',
            'animation_duration' => 'nullable|integer|min:0|max:1000',
            'title' => 'nullable|string|max:255',
            'accept_btn_text' => 'nullable|string|max:50',
            'reject_btn_text' => 'nullable|string|max:50',
            'settings_btn_text' => 'nullable|string|max:50',
            'privacy_link_text' => 'nullable|string|max:100',
            'privacy_link_url' => 'nullable|url|max:255',
            'cookie_link_text' => 'nullable|string|max:100',
            'cookie_link_url' => 'nullable|url|max:255',
            'show_reject_btn' => 'nullable|boolean',
            'show_settings_btn' => 'nullable|boolean',
            'show_privacy_link' => 'nullable|boolean',
            'show_cookie_link' => 'nullable|boolean',
            'close_on_scroll' => 'nullable|boolean',
            'close_on_scroll_distance' => 'nullable|integer|min:0|max:500',
            'show_close_btn' => 'nullable|boolean',
            'block_page_scroll' => 'nullable|boolean',
            'show_powered_by' => 'nullable|boolean',
            'logo_url' => 'nullable|url|max:255',
            'logo_size' => 'nullable|integer|min:20|max:200',
            'logo_position' => 'nullable|string|in:left,center,right',
        ]);

        $validated['slug'] = Str::slug($validated['name']) . '-' . time();
        $validated['is_system'] = false;

        $template = ConsentBannerTemplate::create($validated);

        return response()->json([
            'success' => true,
            'message' => 'Template created successfully',
            'data' => $template,
        ], 201);
    }

    /**
     * Update a template
     */
    public function updateTemplate(Request $request, int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        if ($template->is_system) {
            return response()->json([
                'success' => false,
                'message' => 'Cannot modify system templates',
            ], 403);
        }

        $validated = $request->validate([
            'name' => 'sometimes|string|max:100',
            'category' => 'nullable|string|max:50',
            'description' => 'nullable|string',
            'layout_type' => 'nullable|string|in:bar,popup,floating,drawer',
            'position' => 'nullable|string|in:top,bottom,center',
            'position_horizontal' => 'nullable|string|in:left,center,right',
            'background_color' => 'nullable|string|max:20',
            'text_color' => 'nullable|string|max:20',
            'link_color' => 'nullable|string|max:20',
            'title_color' => 'nullable|string|max:20',
            'border_color' => 'nullable|string|max:20',
            'border_style' => 'nullable|string|max:20',
            'border_width' => 'nullable|integer|min:0|max:20',
            'accept_btn_bg' => 'nullable|string|max:20',
            'accept_btn_text' => 'nullable|string|max:20',
            'accept_btn_hover_bg' => 'nullable|string|max:20',
            'reject_btn_bg' => 'nullable|string|max:20',
            'reject_btn_text' => 'nullable|string|max:20',
            'reject_btn_hover_bg' => 'nullable|string|max:20',
            'settings_btn_bg' => 'nullable|string|max:20',
            'settings_btn_text' => 'nullable|string|max:20',
            'settings_btn_border' => 'nullable|string|max:20',
            'toggle_active_color' => 'nullable|string|max:20',
            'toggle_inactive_color' => 'nullable|string|max:20',
            'font_family' => 'nullable|string|max:100',
            'title_font_size' => 'nullable|integer|min:10|max:48',
            'title_font_weight' => 'nullable|integer|min:100|max:900',
            'body_font_size' => 'nullable|integer|min:10|max:32',
            'body_font_weight' => 'nullable|integer|min:100|max:900',
            'btn_font_size' => 'nullable|integer|min:10|max:24',
            'btn_font_weight' => 'nullable|integer|min:100|max:900',
            'padding_top' => 'nullable|integer|min:0|max:100',
            'padding_bottom' => 'nullable|integer|min:0|max:100',
            'padding_left' => 'nullable|integer|min:0|max:100',
            'padding_right' => 'nullable|integer|min:0|max:100',
            'btn_padding_x' => 'nullable|integer|min:0|max:60',
            'btn_padding_y' => 'nullable|integer|min:0|max:40',
            'btn_margin' => 'nullable|integer|min:0|max:40',
            'btn_border_radius' => 'nullable|integer|min:0|max:50',
            'container_border_radius' => 'nullable|integer|min:0|max:50',
            'container_max_width' => 'nullable|integer|min:300|max:2000',
            'animation_type' => 'nullable|string|in:slide,fade,none',
            'animation_duration' => 'nullable|integer|min:0|max:1000',
            'title' => 'nullable|string|max:255',
            'accept_btn_text' => 'nullable|string|max:50',
            'reject_btn_text' => 'nullable|string|max:50',
            'settings_btn_text' => 'nullable|string|max:50',
            'privacy_link_text' => 'nullable|string|max:100',
            'privacy_link_url' => 'nullable|url|max:255',
            'cookie_link_text' => 'nullable|string|max:100',
            'cookie_link_url' => 'nullable|url|max:255',
            'show_reject_btn' => 'nullable|boolean',
            'show_settings_btn' => 'nullable|boolean',
            'show_privacy_link' => 'nullable|boolean',
            'show_cookie_link' => 'nullable|boolean',
            'close_on_scroll' => 'nullable|boolean',
            'close_on_scroll_distance' => 'nullable|integer|min:0|max:500',
            'show_close_btn' => 'nullable|boolean',
            'block_page_scroll' => 'nullable|boolean',
            'show_powered_by' => 'nullable|boolean',
            'logo_url' => 'nullable|url|max:255',
            'logo_size' => 'nullable|integer|min:20|max:200',
            'logo_position' => 'nullable|string|in:left,center,right',
        ]);

        $template->update($validated);

        return response()->json([
            'success' => true,
            'message' => 'Template updated successfully',
            'data' => $template->fresh(),
        ]);
    }

    /**
     * Delete a template
     */
    public function deleteTemplate(int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        if ($template->is_system) {
            return response()->json([
                'success' => false,
                'message' => 'Cannot delete system templates',
            ], 403);
        }

        $template->delete();

        return response()->json([
            'success' => true,
            'message' => 'Template deleted successfully',
        ]);
    }

    /**
     * Set a template as active
     */
    public function activateTemplate(int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        $template->setAsActive();

        return response()->json([
            'success' => true,
            'message' => 'Template activated successfully',
            'data' => $template,
        ]);
    }

    /**
     * Duplicate a template
     */
    public function duplicateTemplate(int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        $newTemplate = $template->replicate();
        $newTemplate->name = $template->name . ' (Copy)';
        $newTemplate->slug = Str::slug($newTemplate->name) . '-' . time();
        $newTemplate->is_system = false;
        $newTemplate->is_active = false;
        $newTemplate->is_default = false;
        $newTemplate->save();

        return response()->json([
            'success' => true,
            'message' => 'Template duplicated successfully',
            'data' => $newTemplate,
        ], 201);
    }

    /**
     * Export template configuration
     */
    public function exportTemplate(int $id): JsonResponse
    {
        $template = ConsentBannerTemplate::find($id);

        if (!$template) {
            return response()->json([
                'success' => false,
                'message' => 'Template not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => $template->toExportArray(),
        ]);
    }

    /**
     * Import template configuration
     */
    public function importTemplate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'template' => 'required|array',
            'template.name' => 'required|string|max:100',
        ]);

        try {
            $template = ConsentBannerTemplate::createFromImport($validated['template']);

            return response()->json([
                'success' => true,
                'message' => 'Template imported successfully',
                'data' => $template,
            ], 201);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to import template: ' . $e->getMessage(),
            ], 400);
        }
    }

    /**
     * Initialize default templates and settings
     */
    public function initialize(): JsonResponse
    {
        ConsentSettings::initializeDefaults();
        ConsentBannerTemplate::initializeSystemTemplates();

        return response()->json([
            'success' => true,
            'message' => 'Consent system initialized successfully',
        ]);
    }
}
