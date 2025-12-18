<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Resources\SeoSettingResource;
use App\Http\Resources\SeoAlertResource;
use App\Http\Resources\SitemapConfigResource;
use App\Services\Seo\SeoSettingsService;
use App\Services\Seo\SitemapService;
use App\Services\Seo\LocalBusinessService;
use App\Services\Seo\SeoAlertService;
use App\Models\SeoSetting;
use App\Models\SeoAlert;
use App\Models\SitemapConfig;
use App\Enums\SeoSettingType;
use App\Enums\SeoAlertSeverity;
use App\Events\SeoSettingUpdated;
use App\Events\CriticalSeoAlert;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Validator;
use Illuminate\Validation\Rule;
use Illuminate\Validation\ValidationException;
use Symfony\Component\HttpFoundation\Response;

class SeoSettingsController extends Controller
{
    private const CACHE_TTL = 3600; // 1 hour
    private const CACHE_PREFIX = 'seo:settings:';
    
    /**
     * Allowed settings with their validation rules and defaults
     */
    private const SETTING_DEFINITIONS = [
        'default_meta_title_template' => [
            'type' => 'string',
            'rules' => 'string|max:255',
            'default' => '{title} | {site_name}',
            'description' => 'Template for meta titles',
            'category' => 'meta',
        ],
        'default_meta_description_template' => [
            'type' => 'string',
            'rules' => 'string|max:500',
            'default' => '{excerpt}',
            'description' => 'Template for meta descriptions',
            'category' => 'meta',
        ],
        'enable_auto_meta_generation' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Automatically generate missing meta tags',
            'category' => 'automation',
        ],
        'enable_broken_link_monitoring' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Monitor for broken internal and external links',
            'category' => 'monitoring',
        ],
        'enable_sitemap_generation' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Automatically generate XML sitemaps',
            'category' => 'sitemap',
        ],
        'sitemap_update_frequency' => [
            'type' => 'string',
            'rules' => ['string', 'in:always,hourly,daily,weekly,monthly,yearly,never'],
            'default' => 'daily',
            'description' => 'How often to regenerate sitemaps',
            'category' => 'sitemap',
        ],
        'max_sitemap_entries' => [
            'type' => 'integer',
            'rules' => 'integer|min:100|max:50000',
            'default' => 10000,
            'description' => 'Maximum entries per sitemap file',
            'category' => 'sitemap',
        ],
        'enable_image_optimization' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Automatically optimize images for SEO',
            'category' => 'media',
        ],
        'image_alt_text_required' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Require alt text for all images',
            'category' => 'media',
        ],
        'enable_schema_markup' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Add structured data to pages',
            'category' => 'schema',
        ],
        'default_schema_type' => [
            'type' => 'string',
            'rules' => ['string', 'in:Article,WebPage,Product,LocalBusiness,Organization'],
            'default' => 'WebPage',
            'description' => 'Default Schema.org type for pages',
            'category' => 'schema',
        ],
        'robots_txt_content' => [
            'type' => 'text',
            'rules' => 'string|max:5000',
            'default' => "User-agent: *\nAllow: /\n\nSitemap: /sitemap.xml",
            'description' => 'Content of robots.txt file',
            'category' => 'crawling',
        ],
        'enable_redirect_management' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Track and manage 301/302 redirects',
            'category' => 'redirects',
        ],
        'max_redirect_chains' => [
            'type' => 'integer',
            'rules' => 'integer|min:1|max:5',
            'default' => 3,
            'description' => 'Maximum allowed redirect chain length',
            'category' => 'redirects',
        ],
        'enable_keyword_tracking' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => true,
            'description' => 'Track keyword rankings and performance',
            'category' => 'keywords',
        ],
        'keyword_tracking_frequency' => [
            'type' => 'string',
            'rules' => ['string', 'in:daily,weekly,biweekly,monthly'],
            'default' => 'weekly',
            'description' => 'How often to check keyword rankings',
            'category' => 'keywords',
        ],
        'enable_competitor_monitoring' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => false,
            'description' => 'Monitor competitor SEO changes',
            'category' => 'competitive',
        ],
        'alert_email_recipients' => [
            'type' => 'json',
            'rules' => 'array',
            'default' => [],
            'description' => 'Email addresses for SEO alerts',
            'category' => 'alerts',
        ],
        'alert_severity_threshold' => [
            'type' => 'string',
            'rules' => ['string', 'in:low,medium,high,critical'],
            'default' => 'medium',
            'description' => 'Minimum severity for email alerts',
            'category' => 'alerts',
        ],
        'enable_auto_fix' => [
            'type' => 'boolean',
            'rules' => 'boolean',
            'default' => false,
            'description' => 'Automatically fix simple SEO issues',
            'category' => 'automation',
        ],
        'auto_fix_types' => [
            'type' => 'json',
            'rules' => 'array',
            'default' => ['missing_alt_text', 'duplicate_titles', 'empty_meta_descriptions'],
            'description' => 'Types of issues to auto-fix',
            'category' => 'automation',
        ],
    ];

    public function __construct(
        private readonly SeoSettingsService $settingsService,
        private readonly SitemapService $sitemapService,
        private readonly LocalBusinessService $localBusinessService,
        private readonly SeoAlertService $alertService
    ) {}

    /**
     * Get all SEO settings with metadata.
     */
    public function index(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'category' => 'nullable|string',
            'search' => 'nullable|string|max:100',
            'include_defaults' => 'boolean',
        ]);

        $cacheKey = $this->getCacheKey('all', $validated);
        
        $settings = Cache::remember($cacheKey, self::CACHE_TTL, function () use ($validated) {
            $query = SeoSetting::query();
            
            // Filter by category if specified
            if (!empty($validated['category'])) {
                $categoryKeys = $this->getSettingKeysByCategory($validated['category']);
                $query->whereIn('key', $categoryKeys);
            }
            
            // Search in keys and descriptions
            if (!empty($validated['search'])) {
                $query->where(function ($q) use ($validated) {
                    $q->where('key', 'like', "%{$validated['search']}%")
                      ->orWhere('description', 'like', "%{$validated['search']}%");
                });
            }
            
            $settings = $query->get();
            
            // Include defaults if requested
            if ($validated['include_defaults'] ?? false) {
                $settings = $this->mergeWithDefaults($settings);
            }
            
            return $settings;
        });

        // Transform settings for response
        $transformed = $this->transformSettings($settings);
        
        return response()->json([
            'settings' => $transformed,
            'categories' => $this->getCategories(),
            'total' => count($transformed),
        ]);
    }

    /**
     * Get a specific setting with full metadata.
     */
    public function show(string $key): JsonResponse
    {
        if (!$this->isValidSettingKey($key)) {
            return response()->json([
                'error' => 'Invalid setting key',
                'valid_keys' => array_keys(self::SETTING_DEFINITIONS),
            ], Response::HTTP_BAD_REQUEST);
        }

        $setting = $this->settingsService->get($key);
        $definition = self::SETTING_DEFINITIONS[$key];
        
        return response()->json([
            'setting' => [
                'key' => $key,
                'value' => $setting,
                'type' => $definition['type'],
                'default' => $definition['default'],
                'description' => $definition['description'],
                'category' => $definition['category'],
                'rules' => $definition['rules'],
                'last_modified' => SeoSetting::where('key', $key)->first()?->updated_at,
            ],
        ]);
    }

    /**
     * Update a setting with validation and event dispatching.
     */
    public function update(string $key, Request $request): JsonResponse
    {
        if (!$this->isValidSettingKey($key)) {
            return response()->json([
                'error' => 'Invalid setting key',
                'message' => "The setting key '{$key}' is not recognized",
            ], Response::HTTP_BAD_REQUEST);
        }

        $definition = self::SETTING_DEFINITIONS[$key];
        
        // Validate the value based on setting definition
        $validator = Validator::make(
            ['value' => $request->value],
            ['value' => $definition['rules']]
        );

        if ($validator->fails()) {
            throw new ValidationException($validator);
        }

        DB::beginTransaction();
        
        try {
            $oldValue = $this->settingsService->get($key);
            
            $setting = SeoSetting::updateOrCreate(
                ['key' => $key],
                [
                    'value' => $this->castValue($request->value, $definition['type']),
                    'type' => $definition['type'],
                    'description' => $definition['description'],
                    'category' => $definition['category'],
                ]
            );

            // Clear caches
            $this->clearSettingCaches($key);

            // Dispatch event for setting change
            event(new SeoSettingUpdated($key, $oldValue, $setting->value));

            // Handle special settings that require immediate action
            $this->handleSpecialSettings($key, $setting->value);

            DB::commit();

            // Log the change
            Log::info('SEO setting updated', [
                'key' => $key,
                'old_value' => $oldValue,
                'new_value' => $setting->value,
                'user_id' => $request->user()->id,
            ]);

            return response()->json([
                'setting' => new SeoSettingResource($setting),
                'message' => 'Setting updated successfully',
                'requires_reindex' => $this->requiresReindex($key),
            ]);

        } catch (\Exception $e) {
            DB::rollback();
            
            Log::error('Failed to update SEO setting', [
                'key' => $key,
                'error' => $e->getMessage(),
            ]);
            
            return response()->json([
                'error' => 'Failed to update setting',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Bulk update multiple settings.
     */
    public function bulkUpdate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'settings' => 'required|array',
            'settings.*.key' => 'required|string',
            'settings.*.value' => 'required',
        ]);

        $results = [
            'success' => [],
            'failed' => [],
        ];

        DB::beginTransaction();

        try {
            foreach ($validated['settings'] as $item) {
                if (!$this->isValidSettingKey($item['key'])) {
                    $results['failed'][] = [
                        'key' => $item['key'],
                        'error' => 'Invalid setting key',
                    ];
                    continue;
                }

                $definition = self::SETTING_DEFINITIONS[$item['key']];
                
                $validator = Validator::make(
                    ['value' => $item['value']],
                    ['value' => $definition['rules']]
                );

                if ($validator->fails()) {
                    $results['failed'][] = [
                        'key' => $item['key'],
                        'error' => $validator->errors()->first('value'),
                    ];
                    continue;
                }

                $setting = SeoSetting::updateOrCreate(
                    ['key' => $item['key']],
                    [
                        'value' => $this->castValue($item['value'], $definition['type']),
                        'type' => $definition['type'],
                        'description' => $definition['description'],
                        'category' => $definition['category'],
                    ]
                );

                $results['success'][] = $item['key'];
            }

            if (empty($results['failed'])) {
                DB::commit();
                
                // Clear all caches
                $this->clearAllSettingCaches();
                
                return response()->json([
                    'message' => 'All settings updated successfully',
                    'results' => $results,
                ]);
            } else {
                DB::rollback();
                
                return response()->json([
                    'error' => 'Some settings failed validation',
                    'results' => $results,
                ], Response::HTTP_UNPROCESSABLE_ENTITY);
            }

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Bulk update failed',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Reset a setting to its default value.
     */
    public function reset(string $key): JsonResponse
    {
        if (!$this->isValidSettingKey($key)) {
            return response()->json([
                'error' => 'Invalid setting key',
            ], Response::HTTP_BAD_REQUEST);
        }

        $definition = self::SETTING_DEFINITIONS[$key];
        
        $setting = SeoSetting::updateOrCreate(
            ['key' => $key],
            [
                'value' => $definition['default'],
                'type' => $definition['type'],
                'description' => $definition['description'],
                'category' => $definition['category'],
            ]
        );

        $this->clearSettingCaches($key);

        return response()->json([
            'setting' => new SeoSettingResource($setting),
            'message' => 'Setting reset to default value',
        ]);
    }

    /**
     * Get sitemap configurations with full details.
     */
    public function getSitemapConfigs(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'active_only' => 'boolean',
            'include_stats' => 'boolean',
        ]);

        $cacheKey = $this->getCacheKey('sitemaps', $validated);
        
        $configs = Cache::remember($cacheKey, self::CACHE_TTL, function () use ($validated) {
            $query = SitemapConfig::query()
                ->with(['entries' => function ($q) {
                    $q->select('sitemap_config_id', DB::raw('COUNT(*) as count'))
                      ->groupBy('sitemap_config_id');
                }]);
            
            if ($validated['active_only'] ?? false) {
                $query->where('is_active', true);
            }
            
            return $query->get();
        });

        $transformed = $configs->map(function ($config) use ($validated) {
            $data = [
                'id' => $config->id,
                'name' => $config->name,
                'url' => $config->url,
                'type' => $config->type,
                'frequency' => $config->frequency,
                'priority' => $config->priority,
                'is_active' => $config->is_active,
                'include_images' => $config->include_images,
                'include_videos' => $config->include_videos,
                'max_entries' => $config->max_entries,
                'last_generated' => $config->last_generated_at,
                'next_generation' => $this->calculateNextGeneration($config),
            ];
            
            if ($validated['include_stats'] ?? false) {
                $stats = $this->sitemapService->getStats($config->id);
                $data['stats'] = $stats;
            }
            
            return $data;
        });

        return response()->json([
            'configs' => $transformed,
            'total' => $configs->count(),
            'generation_status' => $this->sitemapService->getGenerationStatus(),
        ]);
    }

    /**
     * Create a new sitemap configuration.
     */
    public function createSitemapConfig(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:100|unique:sitemap_configs',
            'type' => ['required', Rule::in(['posts', 'pages', 'products', 'categories', 'tags', 'custom'])],
            'frequency' => ['required', Rule::in(['always', 'hourly', 'daily', 'weekly', 'monthly', 'yearly', 'never'])],
            'priority' => 'required|numeric|min:0|max:1',
            'filters' => 'nullable|array',
            'include_images' => 'boolean',
            'include_videos' => 'boolean',
            'max_entries' => 'nullable|integer|min:1|max:50000',
            'is_active' => 'boolean',
        ]);

        DB::beginTransaction();

        try {
            $config = SitemapConfig::create([
                'name' => $validated['name'],
                'url' => $this->generateSitemapUrl($validated['name']),
                'type' => $validated['type'],
                'frequency' => $validated['frequency'],
                'priority' => $validated['priority'],
                'filters' => $validated['filters'] ?? [],
                'include_images' => $validated['include_images'] ?? false,
                'include_videos' => $validated['include_videos'] ?? false,
                'max_entries' => $validated['max_entries'] ?? 10000,
                'is_active' => $validated['is_active'] ?? true,
            ]);

            // Generate initial sitemap
            $this->sitemapService->generate($config->id);

            DB::commit();

            return response()->json([
                'config' => new SitemapConfigResource($config),
                'message' => 'Sitemap configuration created successfully',
            ], Response::HTTP_CREATED);

        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Failed to create sitemap configuration',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Update sitemap configuration with validation.
     */
    public function updateSitemapConfig(int $id, Request $request): JsonResponse
    {
        $config = SitemapConfig::findOrFail($id);
        
        $validated = $request->validate([
            'name' => 'string|max:100|unique:sitemap_configs,name,' . $id,
            'frequency' => [Rule::in(['always', 'hourly', 'daily', 'weekly', 'monthly', 'yearly', 'never'])],
            'priority' => 'numeric|min:0|max:1',
            'filters' => 'nullable|array',
            'include_images' => 'boolean',
            'include_videos' => 'boolean',
            'max_entries' => 'nullable|integer|min:1|max:50000',
            'is_active' => 'boolean',
            'regenerate' => 'boolean',
        ]);

        $config->update($validated);

        if ($validated['regenerate'] ?? false) {
            $this->sitemapService->generate($config->id);
        }

        Cache::forget($this->getCacheKey('sitemaps', []));

        return response()->json([
            'config' => new SitemapConfigResource($config),
            'message' => 'Sitemap configuration updated successfully',
        ]);
    }

    /**
     * Delete a sitemap configuration.
     */
    public function deleteSitemapConfig(int $id): JsonResponse
    {
        $config = SitemapConfig::findOrFail($id);
        
        // Delete the physical sitemap file
        $this->sitemapService->deleteFile($config->url);
        
        $config->delete();
        
        Cache::forget($this->getCacheKey('sitemaps', []));

        return response()->json([
            'message' => 'Sitemap configuration deleted successfully',
        ]);
    }

    /**
     * Manually generate a sitemap.
     */
    public function generateSitemap(int $id): JsonResponse
    {
        $config = SitemapConfig::findOrFail($id);
        
        try {
            $result = $this->sitemapService->generate($config->id);
            
            return response()->json([
                'message' => 'Sitemap generated successfully',
                'url' => $result['url'],
                'entries' => $result['entries'],
                'size' => $result['size'],
            ]);
            
        } catch (\Exception $e) {
            return response()->json([
                'error' => 'Failed to generate sitemap',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Get local business data with validation.
     */
    public function getLocalBusiness(): JsonResponse
    {
        $data = $this->localBusinessService->get();
        
        return response()->json([
            'business' => $data,
            'schema' => $this->localBusinessService->generateSchema($data),
            'validation' => $this->localBusinessService->validate($data),
        ]);
    }

    /**
     * Update local business data with comprehensive validation.
     */
    public function updateLocalBusiness(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'description' => 'nullable|string|max:1000',
            'address' => 'required|array',
            'address.street' => 'required|string|max:255',
            'address.city' => 'required|string|max:100',
            'address.state' => 'required|string|max:100',
            'address.postal_code' => 'required|string|max:20',
            'address.country' => 'required|string|size:2',
            'phone' => 'required|string|max:50',
            'email' => 'nullable|email|max:255',
            'website' => 'nullable|url|max:500',
            'hours' => 'required|array',
            'hours.*.day' => ['required', Rule::in(['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'])],
            'hours.*.open' => 'required|date_format:H:i',
            'hours.*.close' => 'required|date_format:H:i|after:hours.*.open',
            'hours.*.closed' => 'boolean',
            'social_media' => 'nullable|array',
            'social_media.facebook' => 'nullable|url',
            'social_media.twitter' => 'nullable|url',
            'social_media.instagram' => 'nullable|url',
            'social_media.linkedin' => 'nullable|url',
            'logo_url' => 'nullable|url|max:500',
            'images' => 'nullable|array|max:10',
            'images.*' => 'url|max:500',
            'categories' => 'nullable|array|max:5',
            'categories.*' => 'string|max:100',
            'price_range' => ['nullable', Rule::in(['$', '$$', '$$$', '$$$$'])],
            'accepts_reservations' => 'boolean',
            'delivery_available' => 'boolean',
            'takeout_available' => 'boolean',
            'wheelchair_accessible' => 'boolean',
            'parking_available' => 'boolean',
            'wifi_available' => 'boolean',
        ]);

        try {
            $this->localBusinessService->update($validated);
            
            // Generate updated schema
            $schema = $this->localBusinessService->generateSchema($validated);
            
            // Update sitemap with new business info
            $this->sitemapService->updateLocalBusinessEntry($validated);
            
            // Clear caches
            Cache::forget('local_business_data');
            Cache::forget('local_business_schema');
            
            return response()->json([
                'message' => 'Local business data updated successfully',
                'business' => $validated,
                'schema' => $schema,
            ]);
            
        } catch (\Exception $e) {
            return response()->json([
                'error' => 'Failed to update local business data',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Check for SEO alerts with filtering and pagination.
     */
    public function checkAlerts(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'severity' => ['nullable', Rule::in(['low', 'medium', 'high', 'critical'])],
            'category' => 'nullable|string',
            'is_resolved' => 'nullable|boolean',
            'date_from' => 'nullable|date',
            'date_to' => 'nullable|date|after_or_equal:date_from',
            'page' => 'integer|min:1',
            'per_page' => 'integer|min:1|max:100',
        ]);

        $query = SeoAlert::query()
            ->with(['resolvedBy', 'affectedContent']);

        // Apply filters
        if (isset($validated['is_resolved'])) {
            $query->where('is_resolved', $validated['is_resolved']);
        }

        if (!empty($validated['severity'])) {
            $query->where('severity', $validated['severity']);
        }

        if (!empty($validated['category'])) {
            $query->where('category', $validated['category']);
        }

        if (!empty($validated['date_from'])) {
            $query->where('created_at', '>=', $validated['date_from']);
        }

        if (!empty($validated['date_to'])) {
            $query->where('created_at', '<=', $validated['date_to'] . ' 23:59:59');
        }

        // Order by severity and recency
        $query->orderByRaw("FIELD(severity, 'critical', 'high', 'medium', 'low')")
              ->orderBy('created_at', 'desc');

        $alerts = $query->paginate($validated['per_page'] ?? 20);

        // Check for new critical alerts
        $this->checkCriticalAlerts();

        return response()->json([
            'alerts' => SeoAlertResource::collection($alerts),
            'pagination' => [
                'total' => $alerts->total(),
                'per_page' => $alerts->perPage(),
                'current_page' => $alerts->currentPage(),
                'last_page' => $alerts->lastPage(),
            ],
            'summary' => $this->getAlertsSummary(),
        ]);
    }

    /**
     * Resolve an alert.
     */
    public function resolveAlert(int $alertId, Request $request): JsonResponse
    {
        $alert = SeoAlert::findOrFail($alertId);
        
        $validated = $request->validate([
            'resolution_notes' => 'required|string|max:1000',
            'action_taken' => 'required|string|max:500',
        ]);

        $alert->update([
            'is_resolved' => true,
            'resolved_at' => now(),
            'resolved_by' => $request->user()->id,
            'resolution_notes' => $validated['resolution_notes'],
            'action_taken' => $validated['action_taken'],
        ]);

        return response()->json([
            'alert' => new SeoAlertResource($alert),
            'message' => 'Alert resolved successfully',
        ]);
    }

    /**
     * Create a manual alert.
     */
    public function createAlert(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'type' => 'required|string|max:100',
            'severity' => ['required', Rule::in(['low', 'medium', 'high', 'critical'])],
            'category' => 'required|string|max:50',
            'title' => 'required|string|max:255',
            'description' => 'required|string|max:1000',
            'affected_url' => 'nullable|url|max:500',
            'affected_content_type' => 'nullable|string',
            'affected_content_id' => 'nullable|integer',
            'metadata' => 'nullable|array',
        ]);

        $alert = SeoAlert::create([
            'type' => $validated['type'],
            'severity' => $validated['severity'],
            'category' => $validated['category'],
            'title' => $validated['title'],
            'description' => $validated['description'],
            'affected_url' => $validated['affected_url'],
            'affected_content_type' => $validated['affected_content_type'],
            'affected_content_id' => $validated['affected_content_id'],
            'metadata' => $validated['metadata'] ?? [],
            'source' => 'manual',
            'created_by' => $request->user()->id,
        ]);

        // Send notification for critical alerts
        if ($alert->severity === 'critical') {
            event(new CriticalSeoAlert($alert));
        }

        return response()->json([
            'alert' => new SeoAlertResource($alert),
            'message' => 'Alert created successfully',
        ], Response::HTTP_CREATED);
    }

    /**
     * Export settings for backup.
     */
    public function exportSettings(): JsonResponse
    {
        $settings = SeoSetting::all();
        $sitemapConfigs = SitemapConfig::all();
        $localBusiness = $this->localBusinessService->get();
        
        $export = [
            'version' => '1.0',
            'exported_at' => now()->toIso8601String(),
            'settings' => $settings->map(fn($s) => [
                'key' => $s->key,
                'value' => $s->value,
                'type' => $s->type,
            ]),
            'sitemap_configs' => $sitemapConfigs,
            'local_business' => $localBusiness,
        ];
        
        return response()->json($export)
            ->header('Content-Disposition', 'attachment; filename=seo-settings-export.json');
    }

    /**
     * Import settings from backup.
     */
    public function importSettings(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'file' => 'required|file|mimes:json|max:5120',
            'overwrite' => 'boolean',
        ]);
        
        try {
            $content = json_decode($request->file('file')->get(), true);
            
            if (!isset($content['version']) || !isset($content['settings'])) {
                throw new \InvalidArgumentException('Invalid export file format');
            }
            
            DB::beginTransaction();
            
            // Import settings
            foreach ($content['settings'] as $setting) {
                if ($this->isValidSettingKey($setting['key'])) {
                    if ($validated['overwrite'] ?? false) {
                        SeoSetting::updateOrCreate(
                            ['key' => $setting['key']],
                            [
                                'value' => $setting['value'],
                                'type' => $setting['type'],
                            ]
                        );
                    } else {
                        SeoSetting::firstOrCreate(
                            ['key' => $setting['key']],
                            [
                                'value' => $setting['value'],
                                'type' => $setting['type'],
                            ]
                        );
                    }
                }
            }
            
            // Import sitemap configs if present
            if (isset($content['sitemap_configs'])) {
                foreach ($content['sitemap_configs'] as $config) {
                    SitemapConfig::updateOrCreate(
                        ['name' => $config['name']],
                        $config
                    );
                }
            }
            
            // Import local business if present
            if (isset($content['local_business'])) {
                $this->localBusinessService->update($content['local_business']);
            }
            
            DB::commit();
            
            // Clear all caches
            $this->clearAllSettingCaches();
            
            return response()->json([
                'message' => 'Settings imported successfully',
                'imported' => [
                    'settings' => count($content['settings']),
                    'sitemaps' => count($content['sitemap_configs'] ?? []),
                    'local_business' => isset($content['local_business']),
                ],
            ]);
            
        } catch (\Exception $e) {
            DB::rollback();
            
            return response()->json([
                'error' => 'Import failed',
                'message' => $e->getMessage(),
            ], Response::HTTP_BAD_REQUEST);
        }
    }

    /**
     * Get robots.txt content.
     */
    public function getRobotsTxt(): JsonResponse
    {
        $content = $this->settingsService->get('robots_txt_content');
        
        return response()->json([
            'content' => $content,
            'preview_url' => url('/robots.txt'),
            'validation' => $this->validateRobotsTxt($content),
        ]);
    }

    /**
     * Update robots.txt content.
     */
    public function updateRobotsTxt(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'content' => 'required|string|max:5000',
        ]);
        
        // Validate robots.txt syntax
        $validation = $this->validateRobotsTxt($validated['content']);
        
        if (!$validation['valid']) {
            return response()->json([
                'error' => 'Invalid robots.txt syntax',
                'errors' => $validation['errors'],
            ], Response::HTTP_UNPROCESSABLE_ENTITY);
        }
        
        $this->settingsService->set('robots_txt_content', $validated['content']);
        
        // Update actual robots.txt file
        file_put_contents(public_path('robots.txt'), $validated['content']);
        
        return response()->json([
            'message' => 'Robots.txt updated successfully',
            'content' => $validated['content'],
        ]);
    }

    // Helper methods

    private function isValidSettingKey(string $key): bool
    {
        return array_key_exists($key, self::SETTING_DEFINITIONS);
    }

    private function getSettingKeysByCategory(string $category): array
    {
        return array_keys(array_filter(
            self::SETTING_DEFINITIONS,
            fn($def) => $def['category'] === $category
        ));
    }

    private function mergeWithDefaults($settings): array
    {
        $merged = [];
        $existingKeys = $settings->pluck('key')->toArray();
        
        // Add existing settings
        foreach ($settings as $setting) {
            $merged[] = $setting;
        }
        
        // Add missing defaults
        foreach (self::SETTING_DEFINITIONS as $key => $definition) {
            if (!in_array($key, $existingKeys)) {
                $merged[] = new SeoSetting([
                    'key' => $key,
                    'value' => $definition['default'],
                    'type' => $definition['type'],
                    'description' => $definition['description'],
                    'category' => $definition['category'],
                ]);
            }
        }
        
        return $merged;
    }

    private function transformSettings($settings): array
    {
        return collect($settings)->map(function ($setting) {
            $definition = self::SETTING_DEFINITIONS[$setting->key] ?? null;
            
            return [
                'key' => $setting->key,
                'value' => $setting->value,
                'type' => $setting->type,
                'category' => $setting->category ?? $definition['category'] ?? 'general',
                'description' => $setting->description ?? $definition['description'] ?? '',
                'default' => $definition['default'] ?? null,
                'last_modified' => $setting->updated_at,
            ];
        })->toArray();
    }

    private function getCategories(): array
    {
        $categories = array_unique(array_column(self::SETTING_DEFINITIONS, 'category'));
        sort($categories);
        
        return array_map(fn($cat) => [
            'key' => $cat,
            'label' => ucfirst(str_replace('_', ' ', $cat)),
            'count' => count($this->getSettingKeysByCategory($cat)),
        ], $categories);
    }

    private function castValue($value, string $type)
    {
        return match($type) {
            'boolean' => (bool) $value,
            'integer' => (int) $value,
            'float' => (float) $value,
            'json' => is_array($value) ? $value : json_decode($value, true),
            default => $value,
        };
    }

    private function handleSpecialSettings(string $key, $value): void
    {
        // Handle settings that require immediate action
        match($key) {
            'enable_sitemap_generation' => $this->toggleSitemapGeneration($value),
            'enable_broken_link_monitoring' => $this->toggleBrokenLinkMonitoring($value),
            'enable_keyword_tracking' => $this->toggleKeywordTracking($value),
            'enable_auto_fix' => $this->toggleAutoFix($value),
            default => null,
        };
    }

    private function toggleSitemapGeneration(bool $enabled): void
    {
        if ($enabled) {
            // Schedule sitemap generation
            $this->sitemapService->scheduleGeneration();
        } else {
            // Cancel scheduled generations
            $this->sitemapService->cancelScheduledGeneration();
        }
    }

    private function toggleBrokenLinkMonitoring(bool $enabled): void
    {
        // Implementation for toggling broken link monitoring
        if ($enabled) {
            dispatch(new \App\Jobs\MonitorBrokenLinksJob)->onQueue('monitoring');
        }
    }

    private function toggleKeywordTracking(bool $enabled): void
    {
        // Implementation for toggling keyword tracking
        if ($enabled) {
            dispatch(new \App\Jobs\TrackKeywordRankingsJob)->onQueue('tracking');
        }
    }

    private function toggleAutoFix(bool $enabled): void
    {
        // Implementation for toggling auto-fix
        config(['seo.auto_fix.enabled' => $enabled]);
    }

    private function requiresReindex(string $key): bool
    {
        $reindexKeys = [
            'enable_schema_markup',
            'default_schema_type',
            'enable_auto_meta_generation',
            'default_meta_title_template',
            'default_meta_description_template',
        ];
        
        return in_array($key, $reindexKeys);
    }

    private function getCacheKey(string $type, array $params = []): string
    {
        $key = self::CACHE_PREFIX . $type;
        
        if (!empty($params)) {
            $key .= ':' . md5(json_encode($params));
        }
        
        return $key;
    }

    private function clearSettingCaches(string $key): void
    {
        Cache::forget(self::CACHE_PREFIX . 'all');
        Cache::forget(self::CACHE_PREFIX . $key);
        Cache::tags(['seo_settings'])->flush();
    }

    private function clearAllSettingCaches(): void
    {
        Cache::forget(self::CACHE_PREFIX . 'all');
        Cache::tags(['seo_settings'])->flush();
    }

    private function generateSitemapUrl(string $name): string
    {
        $slug = Str::slug($name);
        return "/sitemap-{$slug}.xml";
    }

    private function calculateNextGeneration(SitemapConfig $config): ?string
    {
        if (!$config->last_generated_at) {
            return 'Never generated';
        }
        
        $next = match($config->frequency) {
            'hourly' => $config->last_generated_at->addHour(),
            'daily' => $config->last_generated_at->addDay(),
            'weekly' => $config->last_generated_at->addWeek(),
            'monthly' => $config->last_generated_at->addMonth(),
            'yearly' => $config->last_generated_at->addYear(),
            'never' => null,
            default => $config->last_generated_at->addDay(),
        };
        
        return $next?->toIso8601String();
    }

    private function checkCriticalAlerts(): void
    {
        $criticalAlerts = SeoAlert::where('severity', 'critical')
            ->where('is_resolved', false)
            ->where('created_at', '>=', now()->subHour())
            ->get();
        
        foreach ($criticalAlerts as $alert) {
            if (!Cache::has("alert_notified:{$alert->id}")) {
                event(new CriticalSeoAlert($alert));
                Cache::put("alert_notified:{$alert->id}", true, 3600);
            }
        }
    }

    private function getAlertsSummary(): array
    {
        return [
            'total_unresolved' => SeoAlert::where('is_resolved', false)->count(),
            'by_severity' => [
                'critical' => SeoAlert::where('is_resolved', false)->where('severity', 'critical')->count(),
                'high' => SeoAlert::where('is_resolved', false)->where('severity', 'high')->count(),
                'medium' => SeoAlert::where('is_resolved', false)->where('severity', 'medium')->count(),
                'low' => SeoAlert::where('is_resolved', false)->where('severity', 'low')->count(),
            ],
            'recent_24h' => SeoAlert::where('created_at', '>=', now()->subDay())->count(),
        ];
    }

    private function validateRobotsTxt(string $content): array
    {
        $errors = [];
        $lines = explode("\n", $content);
        $currentUserAgent = null;
        
        foreach ($lines as $lineNum => $line) {
            $line = trim($line);
            
            if (empty($line) || str_starts_with($line, '#')) {
                continue;
            }
            
            if (preg_match('/^User-agent:\s*(.+)$/i', $line, $matches)) {
                $currentUserAgent = $matches[1];
            } elseif (preg_match('/^(Allow|Disallow|Crawl-delay|Sitemap):\s*(.*)$/i', $line, $matches)) {
                if ($currentUserAgent === null && strtolower($matches[1]) !== 'sitemap') {
                    $errors[] = "Line {$lineNum}: Directive without User-agent";
                }
            } else {
                $errors[] = "Line {$lineNum}: Invalid directive format";
            }
        }
        
        return [
            'valid' => empty($errors),
            'errors' => $errors,
        ];
    }
}