<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Localization (i18n) Service
 *
 * Provides comprehensive internationalization features:
 * - Multi-language content management
 * - Translation workflows
 * - Locale-aware content delivery
 * - Language fallbacks
 * - Translation memory
 * - Missing translation detection
 * - RTL language support
 * - Pluralization rules
 */
class LocalizationService
{
    private const CACHE_TTL = 3600;

    private array $supportedLocales = [];
    private string $defaultLocale = 'en';
    private array $fallbackChain = [];

    public function __construct()
    {
        $this->loadConfiguration();
    }

    /**
     * Get localized content for a document
     */
    public function getLocalizedContent(string $documentId, string $locale, array $options = []): array
    {
        $cacheKey = "localized:{$documentId}:{$locale}";

        if (!($options['skipCache'] ?? false)) {
            $cached = Cache::get($cacheKey);
            if ($cached) {
                return $cached;
            }
        }

        // Get base document
        $document = $this->getBaseDocument($documentId);

        if (!$document) {
            return ['error' => 'Document not found'];
        }

        // Get translation
        $translation = $this->getTranslation($documentId, $locale);

        if ($translation) {
            $result = $this->mergeTranslation($document, $translation, $locale);
        } else {
            // Try fallback chain
            $result = $this->getWithFallback($document, $documentId, $locale);
        }

        $result['_locale'] = $locale;
        $result['_isTranslated'] = $translation !== null;
        $result['_availableLocales'] = $this->getAvailableLocales($documentId);

        Cache::put($cacheKey, $result, self::CACHE_TTL);

        return $result;
    }

    /**
     * Save a translation
     */
    public function saveTranslation(string $documentId, string $locale, array $content, ?int $userId = null): array
    {
        $translation = DB::table('document_translations')
            ->where('document_id', $documentId)
            ->where('locale', $locale)
            ->first();

        $data = [
            'document_id' => $documentId,
            'locale' => $locale,
            'content' => json_encode($content),
            'translated_by' => $userId,
            'translated_at' => now(),
            'updated_at' => now(),
        ];

        if ($translation) {
            // Update existing
            $data['revision'] = $translation->revision + 1;
            DB::table('document_translations')
                ->where('id', $translation->id)
                ->update($data);
            $id = $translation->id;
        } else {
            // Create new
            $data['id'] = Str::uuid()->toString();
            $data['revision'] = 1;
            $data['created_at'] = now();
            DB::table('document_translations')->insert($data);
            $id = $data['id'];
        }

        // Add to translation memory
        $this->updateTranslationMemory($content, $locale);

        // Clear cache
        Cache::forget("localized:{$documentId}:{$locale}");
        Cache::forget("available_locales:{$documentId}");

        return [
            'id' => $id,
            'documentId' => $documentId,
            'locale' => $locale,
            'revision' => $data['revision'],
        ];
    }

    /**
     * Get translation status for a document
     */
    public function getTranslationStatus(string $documentId): array
    {
        $translations = DB::table('document_translations')
            ->where('document_id', $documentId)
            ->get();

        $statuses = [];
        foreach ($this->supportedLocales as $locale => $config) {
            $translation = $translations->firstWhere('locale', $locale);

            $statuses[$locale] = [
                'locale' => $locale,
                'name' => $config['name'],
                'nativeName' => $config['nativeName'],
                'isTranslated' => $translation !== null,
                'translatedAt' => $translation?->translated_at,
                'translatedBy' => $translation?->translated_by,
                'revision' => $translation?->revision ?? 0,
                'completeness' => $translation ? $this->calculateCompleteness($documentId, $translation) : 0,
            ];
        }

        return [
            'documentId' => $documentId,
            'defaultLocale' => $this->defaultLocale,
            'locales' => $statuses,
            'totalLocales' => count($this->supportedLocales),
            'translatedCount' => $translations->count(),
        ];
    }

    /**
     * Get missing translations for a document
     */
    public function getMissingTranslations(string $documentId): array
    {
        $baseDocument = $this->getBaseDocument($documentId);

        if (!$baseDocument) {
            return [];
        }

        $translatableFields = $this->getTranslatableFields($baseDocument);
        $missing = [];

        foreach ($this->supportedLocales as $locale => $config) {
            if ($locale === $this->defaultLocale) continue;

            $translation = $this->getTranslation($documentId, $locale);
            $translatedContent = $translation ? json_decode($translation->content, true) : [];

            $missingFields = [];
            foreach ($translatableFields as $field) {
                if (!isset($translatedContent[$field]) || empty($translatedContent[$field])) {
                    $missingFields[] = [
                        'field' => $field,
                        'originalValue' => $this->getFieldValue($baseDocument, $field),
                    ];
                }
            }

            if (!empty($missingFields)) {
                $missing[$locale] = [
                    'locale' => $locale,
                    'localeName' => $config['name'],
                    'missingFields' => $missingFields,
                    'completeness' => 1 - (count($missingFields) / count($translatableFields)),
                ];
            }
        }

        return $missing;
    }

    /**
     * Bulk translate using translation memory
     */
    public function autoTranslate(string $documentId, string $targetLocale): array
    {
        $baseDocument = $this->getBaseDocument($documentId);

        if (!$baseDocument) {
            return ['error' => 'Document not found'];
        }

        $translatableFields = $this->getTranslatableFields($baseDocument);
        $translated = [];
        $notFound = [];

        foreach ($translatableFields as $field) {
            $originalValue = $this->getFieldValue($baseDocument, $field);

            if (!$originalValue) continue;

            // Check translation memory
            $memoryTranslation = $this->findInTranslationMemory($originalValue, $targetLocale);

            if ($memoryTranslation) {
                $translated[$field] = $memoryTranslation;
            } else {
                $notFound[] = $field;
            }
        }

        return [
            'documentId' => $documentId,
            'targetLocale' => $targetLocale,
            'translated' => $translated,
            'notFound' => $notFound,
            'completeness' => count($translated) / max(1, count($translatableFields)),
        ];
    }

    /**
     * Configure supported locales
     */
    public function configureLocales(array $locales): void
    {
        $this->supportedLocales = $locales;

        DB::table('locale_config')->updateOrInsert(
            ['key' => 'supported_locales'],
            ['value' => json_encode($locales), 'updated_at' => now()]
        );

        Cache::forget('locale_config');
    }

    /**
     * Set default locale
     */
    public function setDefaultLocale(string $locale): void
    {
        if (!isset($this->supportedLocales[$locale])) {
            throw new \InvalidArgumentException("Locale not supported: {$locale}");
        }

        $this->defaultLocale = $locale;

        DB::table('locale_config')->updateOrInsert(
            ['key' => 'default_locale'],
            ['value' => $locale, 'updated_at' => now()]
        );

        Cache::forget('locale_config');
    }

    /**
     * Set fallback chain
     */
    public function setFallbackChain(array $chain): void
    {
        $this->fallbackChain = $chain;

        DB::table('locale_config')->updateOrInsert(
            ['key' => 'fallback_chain'],
            ['value' => json_encode($chain), 'updated_at' => now()]
        );

        Cache::forget('locale_config');
    }

    /**
     * Get locale metadata
     */
    public function getLocaleMetadata(string $locale): array
    {
        $config = $this->supportedLocales[$locale] ?? null;

        if (!$config) {
            return ['error' => 'Locale not found'];
        }

        return [
            'code' => $locale,
            'name' => $config['name'],
            'nativeName' => $config['nativeName'],
            'direction' => $config['direction'] ?? 'ltr',
            'dateFormat' => $config['dateFormat'] ?? 'Y-m-d',
            'timeFormat' => $config['timeFormat'] ?? 'H:i',
            'numberFormat' => $config['numberFormat'] ?? ['decimal' => '.', 'thousand' => ','],
            'pluralRules' => $config['pluralRules'] ?? 'default',
            'isRTL' => ($config['direction'] ?? 'ltr') === 'rtl',
        ];
    }

    /**
     * Format date for locale
     */
    public function formatDate(\DateTime $date, string $locale, string $format = 'medium'): string
    {
        $metadata = $this->getLocaleMetadata($locale);
        $dateFormat = $metadata['dateFormat'] ?? 'Y-m-d';

        return match ($format) {
            'short' => $date->format('n/j/y'),
            'medium' => $date->format($dateFormat),
            'long' => $date->format('F j, Y'),
            'full' => $date->format('l, F j, Y'),
            default => $date->format($dateFormat),
        };
    }

    /**
     * Format number for locale
     */
    public function formatNumber(float $number, string $locale, int $decimals = 2): string
    {
        $metadata = $this->getLocaleMetadata($locale);
        $format = $metadata['numberFormat'] ?? ['decimal' => '.', 'thousand' => ','];

        return number_format($number, $decimals, $format['decimal'], $format['thousand']);
    }

    /**
     * Pluralize text
     */
    public function pluralize(int $count, array $forms, string $locale): string
    {
        $metadata = $this->getLocaleMetadata($locale);
        $rules = $metadata['pluralRules'] ?? 'default';

        $index = $this->getPluralIndex($count, $rules);

        return $forms[$index] ?? $forms[0] ?? '';
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function loadConfiguration(): void
    {
        $config = Cache::remember('locale_config', self::CACHE_TTL, function () {
            return DB::table('locale_config')->get()->keyBy('key');
        });

        if (isset($config['supported_locales'])) {
            $this->supportedLocales = json_decode($config['supported_locales']->value, true) ?? [];
        } else {
            // Default locales
            $this->supportedLocales = [
                'en' => ['name' => 'English', 'nativeName' => 'English', 'direction' => 'ltr'],
                'es' => ['name' => 'Spanish', 'nativeName' => 'Español', 'direction' => 'ltr'],
                'fr' => ['name' => 'French', 'nativeName' => 'Français', 'direction' => 'ltr'],
                'de' => ['name' => 'German', 'nativeName' => 'Deutsch', 'direction' => 'ltr'],
                'pt' => ['name' => 'Portuguese', 'nativeName' => 'Português', 'direction' => 'ltr'],
                'zh' => ['name' => 'Chinese', 'nativeName' => '中文', 'direction' => 'ltr'],
                'ja' => ['name' => 'Japanese', 'nativeName' => '日本語', 'direction' => 'ltr'],
                'ar' => ['name' => 'Arabic', 'nativeName' => 'العربية', 'direction' => 'rtl'],
                'he' => ['name' => 'Hebrew', 'nativeName' => 'עברית', 'direction' => 'rtl'],
            ];
        }

        if (isset($config['default_locale'])) {
            $this->defaultLocale = $config['default_locale']->value;
        }

        if (isset($config['fallback_chain'])) {
            $this->fallbackChain = json_decode($config['fallback_chain']->value, true) ?? [];
        }
    }

    private function getBaseDocument(string $documentId): ?array
    {
        $post = DB::table('posts')->find($documentId);

        if (!$post) {
            return null;
        }

        return (array) $post;
    }

    private function getTranslation(string $documentId, string $locale): ?object
    {
        return DB::table('document_translations')
            ->where('document_id', $documentId)
            ->where('locale', $locale)
            ->first();
    }

    private function mergeTranslation(array $document, object $translation, string $locale): array
    {
        $translatedContent = json_decode($translation->content, true) ?? [];

        // Merge translated fields over base document
        foreach ($translatedContent as $field => $value) {
            if ($value !== null && $value !== '') {
                $document[$field] = $value;
            }
        }

        return $document;
    }

    private function getWithFallback(array $document, string $documentId, string $locale): array
    {
        $chain = $this->fallbackChain[$locale] ?? [$this->defaultLocale];

        foreach ($chain as $fallbackLocale) {
            $translation = $this->getTranslation($documentId, $fallbackLocale);

            if ($translation) {
                $result = $this->mergeTranslation($document, $translation, $fallbackLocale);
                $result['_fallbackLocale'] = $fallbackLocale;
                return $result;
            }
        }

        // Return base document
        $document['_fallbackLocale'] = $this->defaultLocale;
        return $document;
    }

    private function getAvailableLocales(string $documentId): array
    {
        return Cache::remember("available_locales:{$documentId}", self::CACHE_TTL, function () use ($documentId) {
            $locales = [$this->defaultLocale]; // Base language always available

            $translations = DB::table('document_translations')
                ->where('document_id', $documentId)
                ->pluck('locale')
                ->toArray();

            return array_unique(array_merge($locales, $translations));
        });
    }

    private function calculateCompleteness(string $documentId, object $translation): float
    {
        $baseDocument = $this->getBaseDocument($documentId);

        if (!$baseDocument) {
            return 0;
        }

        $translatableFields = $this->getTranslatableFields($baseDocument);
        $translatedContent = json_decode($translation->content, true) ?? [];

        $translated = 0;
        foreach ($translatableFields as $field) {
            if (isset($translatedContent[$field]) && !empty($translatedContent[$field])) {
                $translated++;
            }
        }

        return $translated / max(1, count($translatableFields));
    }

    private function getTranslatableFields(array $document): array
    {
        // Fields that should be translated
        $translatable = ['title', 'excerpt', 'content', 'meta_title', 'meta_description'];

        return array_filter($translatable, fn($field) => isset($document[$field]));
    }

    private function getFieldValue(array $document, string $field): mixed
    {
        return $document[$field] ?? null;
    }

    private function updateTranslationMemory(array $content, string $locale): void
    {
        foreach ($content as $field => $value) {
            if (!is_string($value) || strlen($value) < 10) continue;

            $hash = md5($value);

            DB::table('translation_memory')->updateOrInsert(
                ['hash' => $hash, 'locale' => $locale],
                [
                    'source_text' => $value,
                    'locale' => $locale,
                    'updated_at' => now(),
                ]
            );
        }
    }

    private function findInTranslationMemory(string $text, string $targetLocale): ?string
    {
        // First find source text hash
        $hash = md5($text);

        $memory = DB::table('translation_memory')
            ->where('hash', $hash)
            ->where('locale', $targetLocale)
            ->first();

        return $memory?->source_text;
    }

    private function getPluralIndex(int $count, string $rules): int
    {
        // Common plural rules
        return match ($rules) {
            'slavic' => $count === 1 ? 0 : ($count >= 2 && $count <= 4 ? 1 : 2),
            'arabic' => $count === 0 ? 0 : ($count === 1 ? 1 : ($count === 2 ? 2 : ($count % 100 >= 3 && $count % 100 <= 10 ? 3 : ($count % 100 >= 11 ? 4 : 5)))),
            'chinese', 'japanese', 'korean' => 0, // No plural forms
            default => $count === 1 ? 0 : 1, // English-like
        };
    }
}
