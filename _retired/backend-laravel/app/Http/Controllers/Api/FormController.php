<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Form;
use App\Models\FormField;
use App\Models\FormSubmission;
use App\Models\FormSubmissionData;
use Illuminate\Auth\Access\AuthorizationException;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Validator;

/**
 * FormController - Enterprise-grade Form API Controller
 *
 * Handles all form CRUD operations with proper validation,
 * error handling, authorization, and transaction support.
 *
 * @version 2.0.0
 * @security IDOR Prevention via Policies
 */
class FormController extends Controller
{
    /**
     * Allowed sortable columns (prevent SQL injection)
     */
    private const SORTABLE_COLUMNS = [
        'created_at',
        'updated_at',
        'title',
        'status',
        'submissions_count',
    ];

    /**
     * Display a listing of forms
     */
    public function index(Request $request): JsonResponse
    {
        $this->authorize('viewAny', Form::class);

        $query = Form::query()
            ->where('created_by', auth()->id()) // User can only see their own forms
            ->withCount('submissions')
            ->with('creator:id,name');

        // Filter by status (validate against allowed values)
        if ($request->has('status')) {
            $allowedStatuses = ['draft', 'active', 'inactive', 'archived'];
            if (in_array($request->status, $allowedStatuses)) {
                $query->where('status', $request->status);
            }
        }

        // Search by title (sanitized)
        if ($request->has('search') && strlen($request->search) > 0) {
            $search = trim($request->search);
            if (strlen($search) <= 100) { // Limit search length
                $query->where('title', 'like', '%' . $search . '%');
            }
        }

        // Sort (whitelist validation)
        $sortBy = $request->get('sort_by', 'created_at');
        $sortOrder = strtolower($request->get('sort_order', 'desc'));

        if (!in_array($sortBy, self::SORTABLE_COLUMNS)) {
            $sortBy = 'created_at';
        }
        if (!in_array($sortOrder, ['asc', 'desc'])) {
            $sortOrder = 'desc';
        }

        $query->orderBy($sortBy, $sortOrder);

        $perPage = min((int) $request->get('per_page', 15), 100); // Cap at 100
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
            // Log the actual error for debugging
            Log::error('Form creation failed', [
                'user_id' => auth()->id(),
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            // Return sanitized error message (never expose internal details)
            return response()->json([
                'success' => false,
                'message' => 'Failed to create form. Please try again later.',
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

        // Authorization check - prevent IDOR
        $this->authorize('view', $form);

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

        // Check submission limit
        if ($form->submission_limit && $form->submission_count >= $form->submission_limit) {
            return response()->json([
                'success' => false,
                'message' => 'This form has reached its submission limit',
            ], 403);
        }

        // Check if form has expired
        if ($form->expires_at && $form->expires_at->isPast()) {
            return response()->json([
                'success' => false,
                'message' => 'This form has expired',
            ], 403);
        }

        // Enforce anti-spam settings
        $antiSpamResult = $this->checkAntiSpam($request, $form);
        if (!$antiSpamResult['passed']) {
            return response()->json([
                'success' => false,
                'message' => $antiSpamResult['message'],
            ], 422);
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
                    $fieldRules[] = 'email:rfc,dns';
                    break;
                case 'url':
                    $fieldRules[] = 'url';
                    break;
                case 'number':
                    $fieldRules[] = 'numeric';
                    break;
                case 'phone':
                    $fieldRules[] = 'regex:/^[\+]?[(]?[0-9]{1,4}[)]?[-\s\.]?[0-9]{1,4}[-\s\.]?[0-9]{1,9}$/';
                    break;
                case 'file':
                    $fieldRules[] = 'file';
                    $fieldRules[] = 'max:10240'; // 10MB max
                    break;
            }

            // Add custom validation from field config
            if (!empty($field->validation)) {
                $customRules = $field->validation;
                if (isset($customRules['min'])) {
                    $fieldRules[] = 'min:' . $customRules['min'];
                }
                if (isset($customRules['max'])) {
                    $fieldRules[] = 'max:' . $customRules['max'];
                }
                if (isset($customRules['pattern'])) {
                    $fieldRules[] = 'regex:' . $customRules['pattern'];
                }
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
                        'form_load_time' => $request->input('_form_load_time'),
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
                    'message' => $form->success_message ?? 'Form submitted successfully',
                    'data' => [
                        'submission_id' => $submission->submission_id,
                        'redirect_url' => $form->redirect_url,
                    ],
                ], 201);
            });
        } catch (\Exception $e) {
            return response()->json([
                'success' => false,
                'message' => $form->error_message ?? 'Failed to submit form',
                'error' => config('app.debug') ? $e->getMessage() : null,
            ], 500);
        }
    }

    /**
     * Check anti-spam settings for form submission
     */
    private function checkAntiSpam(Request $request, Form $form): array
    {
        $settings = $form->anti_spam_settings ?? [];

        // 1. Honeypot check - if honeypot field is filled, it's a bot
        if (!empty($settings['honeypot_enabled'])) {
            $honeypotField = $settings['honeypot_field'] ?? '_website';
            if ($request->filled($honeypotField)) {
                \Log::warning('Form spam detected: honeypot filled', [
                    'form_id' => $form->id,
                    'ip' => $request->ip(),
                ]);
                return [
                    'passed' => false,
                    'message' => 'Submission rejected',
                ];
            }
        }

        // 2. Time-based check - forms submitted too quickly are likely bots
        if (!empty($settings['min_submit_time'])) {
            $loadTime = $request->input('_form_load_time');
            $submitTime = now()->timestamp;

            if ($loadTime && ($submitTime - $loadTime) < $settings['min_submit_time']) {
                \Log::warning('Form spam detected: submitted too quickly', [
                    'form_id' => $form->id,
                    'ip' => $request->ip(),
                    'time_diff' => $submitTime - $loadTime,
                ]);
                return [
                    'passed' => false,
                    'message' => 'Please take your time filling out the form',
                ];
            }
        }

        // 3. Rate limiting by IP - prevent submission flooding
        if (!empty($settings['rate_limit'])) {
            $limit = $settings['rate_limit'];
            $period = $settings['rate_limit_period'] ?? 60; // default 60 seconds

            $recentSubmissions = FormSubmission::where('form_id', $form->id)
                ->where('ip_address', $request->ip())
                ->where('created_at', '>=', now()->subSeconds($period))
                ->count();

            if ($recentSubmissions >= $limit) {
                \Log::warning('Form spam detected: rate limit exceeded', [
                    'form_id' => $form->id,
                    'ip' => $request->ip(),
                    'submissions' => $recentSubmissions,
                ]);
                return [
                    'passed' => false,
                    'message' => 'Too many submissions. Please try again later.',
                ];
            }
        }

        // 4. reCAPTCHA verification
        if (!empty($settings['recaptcha_enabled']) && !empty($settings['recaptcha_secret'])) {
            $recaptchaToken = $request->input('g-recaptcha-response') ?? $request->input('recaptcha_token');

            if (empty($recaptchaToken)) {
                return [
                    'passed' => false,
                    'message' => 'Please complete the CAPTCHA verification',
                ];
            }

            $verified = $this->verifyRecaptcha($recaptchaToken, $settings['recaptcha_secret']);
            if (!$verified) {
                \Log::warning('Form spam detected: reCAPTCHA failed', [
                    'form_id' => $form->id,
                    'ip' => $request->ip(),
                ]);
                return [
                    'passed' => false,
                    'message' => 'CAPTCHA verification failed. Please try again.',
                ];
            }
        }

        // 5. Block known spam patterns in content
        if (!empty($settings['block_spam_patterns'])) {
            $spamPatterns = [
                '/\[url=/',           // BBCode links
                '/\<a\s+href/',       // HTML links
                '/viagra|cialis|casino|lottery|winner|congratulations.*won/i',
                '/click here.*free/i',
                '/earn money.*fast/i',
            ];

            $content = implode(' ', $request->except(['_token', '_form_load_time', 'g-recaptcha-response']));

            foreach ($spamPatterns as $pattern) {
                if (preg_match($pattern, $content)) {
                    \Log::warning('Form spam detected: spam pattern matched', [
                        'form_id' => $form->id,
                        'ip' => $request->ip(),
                        'pattern' => $pattern,
                    ]);
                    return [
                        'passed' => false,
                        'message' => 'Your submission contains blocked content',
                    ];
                }
            }
        }

        return ['passed' => true, 'message' => 'OK'];
    }

    /**
     * Verify reCAPTCHA token with Google
     */
    private function verifyRecaptcha(string $token, string $secret): bool
    {
        try {
            $response = \Http::asForm()->post('https://www.google.com/recaptcha/api/siteverify', [
                'secret' => $secret,
                'response' => $token,
            ]);

            $result = $response->json();

            return $result['success'] ?? false;
        } catch (\Exception $e) {
            \Log::error('reCAPTCHA verification failed', [
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }
}
