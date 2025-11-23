<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Form;
use App\Models\FormField;
use App\Models\FormSubmission;
use App\Models\FormSubmissionData;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Validator;

/**
 * FormController - Enterprise-grade Form API Controller
 *
 * Handles all form CRUD operations with proper validation,
 * error handling, and transaction support.
 */
class FormController extends Controller
{
    /**
     * Display a listing of forms
     */
    public function index(Request $request): JsonResponse
    {
        $query = Form::query()
            ->withCount('submissions')
            ->with('creator:id,name');

        // Filter by status
        if ($request->has('status')) {
            $query->where('status', $request->status);
        }

        // Search by title
        if ($request->has('search')) {
            $query->where('title', 'like', '%' . $request->search . '%');
        }

        // Sort
        $sortBy = $request->get('sort_by', 'created_at');
        $sortOrder = $request->get('sort_order', 'desc');
        $query->orderBy($sortBy, $sortOrder);

        $perPage = $request->get('per_page', 15);
        $forms = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $forms,
        ]);
    }

    /**
     * Store a newly created form
     */
    public function store(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'title' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:forms',
            'description' => 'nullable|string',
            'settings' => 'nullable|array',
            'styles' => 'nullable|array',
            'fields' => 'nullable|array',
            'fields.*.field_type' => 'required_with:fields|string',
            'fields.*.label' => 'required_with:fields|string',
            'fields.*.name' => 'required_with:fields|string',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'message' => 'Validation failed',
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            return DB::transaction(function () use ($request) {
                $form = Form::create([
                    'title' => $request->title,
                    'slug' => $request->slug,
                    'description' => $request->description,
                    'settings' => $request->settings ?? [],
                    'styles' => $request->styles ?? [],
                    'status' => 'draft',
                    'created_by' => auth()->id(),
                ]);

                // Create fields if provided
                if ($request->has('fields')) {
                    foreach ($request->fields as $index => $fieldData) {
                        FormField::create([
                            'form_id' => $form->id,
                            'field_type' => $fieldData['field_type'],
                            'label' => $fieldData['label'],
                            'name' => $fieldData['name'],
                            'placeholder' => $fieldData['placeholder'] ?? null,
                            'help_text' => $fieldData['help_text'] ?? null,
                            'default_value' => $fieldData['default_value'] ?? null,
                            'options' => $fieldData['options'] ?? null,
                            'validation' => $fieldData['validation'] ?? null,
                            'conditional_logic' => $fieldData['conditional_logic'] ?? null,
                            'attributes' => $fieldData['attributes'] ?? null,
                            'required' => $fieldData['required'] ?? false,
                            'order' => $fieldData['order'] ?? $index,
                            'width' => $fieldData['width'] ?? 100,
                        ]);
                    }
                }

                return response()->json([
                    'success' => true,
                    'message' => 'Form created successfully',
                    'data' => $form->load('fields'),
                ], 201);
            });
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to create form',
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Display the specified form
     */
    public function show(int $id): JsonResponse
    {
        $form = Form::with(['fields', 'creator:id,name'])
            ->withCount('submissions')
            ->find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => $form,
        ]);
    }

    /**
     * Update the specified form
     */
    public function update(Request $request, int $id): JsonResponse
    {
        $form = Form::find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        $validator = Validator::make($request->all(), [
            'title' => 'sometimes|required|string|max:255',
            'slug' => 'sometimes|string|max:255|unique:forms,slug,' . $id,
            'description' => 'nullable|string',
            'settings' => 'nullable|array',
            'styles' => 'nullable|array',
            'fields' => 'nullable|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'message' => 'Validation failed',
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            return DB::transaction(function () use ($request, $form) {
                $form->update($request->only([
                    'title', 'slug', 'description', 'settings', 'styles'
                ]));

                // Update fields if provided
                if ($request->has('fields')) {
                    // Delete existing fields and recreate
                    $form->fields()->delete();

                    foreach ($request->fields as $index => $fieldData) {
                        FormField::create([
                            'form_id' => $form->id,
                            'field_type' => $fieldData['field_type'],
                            'label' => $fieldData['label'],
                            'name' => $fieldData['name'],
                            'placeholder' => $fieldData['placeholder'] ?? null,
                            'help_text' => $fieldData['help_text'] ?? null,
                            'default_value' => $fieldData['default_value'] ?? null,
                            'options' => $fieldData['options'] ?? null,
                            'validation' => $fieldData['validation'] ?? null,
                            'conditional_logic' => $fieldData['conditional_logic'] ?? null,
                            'attributes' => $fieldData['attributes'] ?? null,
                            'required' => $fieldData['required'] ?? false,
                            'order' => $fieldData['order'] ?? $index,
                            'width' => $fieldData['width'] ?? 100,
                        ]);
                    }
                }

                return response()->json([
                    'success' => true,
                    'message' => 'Form updated successfully',
                    'data' => $form->fresh()->load('fields'),
                ]);
            });
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to update form',
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Remove the specified form
     */
    public function destroy(int $id): JsonResponse
    {
        $form = Form::find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        try {
            $form->delete();

            return response()->json([
                'success' => true,
                'message' => 'Form deleted successfully',
            ]);
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to delete form',
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Publish a form
     */
    public function publish(int $id): JsonResponse
    {
        $form = Form::find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        $form->publish();

        return response()->json([
            'success' => true,
            'message' => 'Form published successfully',
            'data' => $form,
        ]);
    }

    /**
     * Unpublish a form
     */
    public function unpublish(int $id): JsonResponse
    {
        $form = Form::find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        $form->unpublish();

        return response()->json([
            'success' => true,
            'message' => 'Form unpublished successfully',
            'data' => $form,
        ]);
    }

    /**
     * Duplicate a form
     */
    public function duplicate(int $id): JsonResponse
    {
        $form = Form::with('fields')->find($id);

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found',
            ], 404);
        }

        try {
            return DB::transaction(function () use ($form) {
                $newForm = $form->replicate();
                $newForm->title = $form->title . ' (Copy)';
                $newForm->slug = null; // Let model generate unique slug
                $newForm->status = 'draft';
                $newForm->submission_count = 0;
                $newForm->published_at = null;
                $newForm->created_by = auth()->id();
                $newForm->save();

                // Duplicate fields
                foreach ($form->fields as $field) {
                    $newField = $field->replicate();
                    $newField->form_id = $newForm->id;
                    $newField->save();
                }

                return response()->json([
                    'success' => true,
                    'message' => 'Form duplicated successfully',
                    'data' => $newForm->load('fields'),
                ], 201);
            });
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to duplicate form',
                'error' => $e->getMessage(),
            ], 500);
        }
    }

    /**
     * Get available field types
     */
    public function fieldTypes(): JsonResponse
    {
        return response()->json([
            'success' => true,
            'data' => FormField::FIELD_TYPES,
        ]);
    }

    /**
     * Get form statistics
     */
    public function stats(): JsonResponse
    {
        $stats = [
            'total_forms' => Form::count(),
            'published_forms' => Form::published()->count(),
            'draft_forms' => Form::draft()->count(),
            'total_submissions' => FormSubmission::count(),
            'unread_submissions' => FormSubmission::unread()->count(),
        ];

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Preview a form by slug (public endpoint)
     */
    public function preview(string $slug): JsonResponse
    {
        $form = Form::with('fields')
            ->where('slug', $slug)
            ->published()
            ->first();

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found or not published',
            ], 404);
        }

        return response()->json([
            'success' => true,
            'data' => $form,
        ]);
    }

    /**
     * Submit a form (public endpoint)
     */
    public function submit(Request $request, string $slug): JsonResponse
    {
        $form = Form::with('fields')
            ->where('slug', $slug)
            ->published()
            ->first();

        if (!$form) {
            return response()->json([
                'success' => false,
                'message' => 'Form not found or not published',
            ], 404);
        }

        // Build validation rules from fields
        $rules = [];
        foreach ($form->fields as $field) {
            $fieldRules = [];
            if ($field->required) {
                $fieldRules[] = 'required';
            } else {
                $fieldRules[] = 'nullable';
            }

            // Add type-specific rules
            switch ($field->field_type) {
                case 'email':
                    $fieldRules[] = 'email';
                    break;
                case 'url':
                    $fieldRules[] = 'url';
                    break;
                case 'number':
                    $fieldRules[] = 'numeric';
                    break;
            }

            $rules[$field->name] = $fieldRules;
        }

        $validator = Validator::make($request->all(), $rules);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'message' => 'Validation failed',
                'errors' => $validator->errors(),
            ], 422);
        }

        try {
            return DB::transaction(function () use ($request, $form) {
                $submission = FormSubmission::create([
                    'form_id' => $form->id,
                    'user_id' => auth()->id(),
                    'ip_address' => $request->ip(),
                    'user_agent' => $request->userAgent(),
                    'referrer' => $request->header('referer'),
                    'metadata' => [
                        'submitted_at' => now()->toIso8601String(),
                    ],
                ]);

                // Save submission data
                foreach ($form->fields as $field) {
                    if ($request->has($field->name)) {
                        FormSubmissionData::create([
                            'submission_id' => $submission->id,
                            'field_id' => $field->id,
                            'field_name' => $field->name,
                            'value' => is_array($request->input($field->name))
                                ? json_encode($request->input($field->name))
                                : $request->input($field->name),
                        ]);
                    }
                }

                // Increment form submission count
                $form->incrementSubmissions();

                return response()->json([
                    'success' => true,
                    'message' => 'Form submitted successfully',
                    'data' => [
                        'submission_id' => $submission->submission_id,
                    ],
                ], 201);
            });
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => 'Failed to submit form',
                'error' => $e->getMessage(),
            ], 500);
        }
    }
}
