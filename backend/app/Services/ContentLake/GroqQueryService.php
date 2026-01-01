<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use InvalidArgumentException;

/**
 * GROQ Query Service
 *
 * Implements a GROQ-like query language for the content lake.
 * GROQ (Graph-Relational Object Queries) is Sanity's query language.
 *
 * Syntax: *[_type == "post" && published == true] | order(publishedAt desc) [0...10] {
 *   _id,
 *   title,
 *   "author": author->{ name, avatar },
 *   "categories": categories[]->{ name, slug }
 * }
 */
class GroqQueryService
{
    private const CACHE_TTL = 300; // 5 minutes
    private const MAX_RESULTS = 10000;

    private array $typeModelMap = [
        'post' => \App\Models\Post::class,
        'category' => \App\Models\Category::class,
        'tag' => \App\Models\Tag::class,
        'media' => \App\Models\Media::class,
        'user' => \App\Models\User::class,
        'product' => \App\Models\Product::class,
    ];

    private array $params = [];
    private bool $useCache = true;

    /**
     * Execute a GROQ query
     */
    public function query(string $groq, array $params = [], bool $useCache = true): GroqResult
    {
        $this->params = $params;
        $this->useCache = $useCache;

        $startTime = microtime(true);

        // Check cache
        $cacheKey = $this->getCacheKey($groq, $params);
        if ($useCache && $cached = Cache::get($cacheKey)) {
            return new GroqResult($cached['data'], $cached['ms'], true);
        }

        // Parse and execute query
        $parsed = $this->parse($groq);
        $result = $this->execute($parsed);

        $executionTime = (microtime(true) - $startTime) * 1000;

        // Cache result
        if ($useCache) {
            Cache::put($cacheKey, [
                'data' => $result,
                'ms' => $executionTime,
            ], self::CACHE_TTL);

            // Store in query cache table for analytics
            DB::table('groq_query_cache')->updateOrInsert(
                ['query_hash' => hash('sha256', $groq . json_encode($params))],
                [
                    'query' => $groq,
                    'params' => json_encode($params),
                    'result' => json_encode($result),
                    'execution_time_ms' => $executionTime,
                    'cached_at' => now(),
                    'expires_at' => now()->addSeconds(self::CACHE_TTL),
                    'hit_count' => DB::raw('hit_count + 1'),
                ]
            );
        }

        return new GroqResult($result, $executionTime, false);
    }

    /**
     * Parse GROQ query into AST
     */
    private function parse(string $groq): array
    {
        $groq = trim($groq);

        // Extract main parts
        $ast = [
            'source' => '*',
            'filter' => null,
            'order' => null,
            'slice' => null,
            'projection' => null,
        ];

        // Parse filter: *[...]
        if (preg_match('/^\*\[(.+?)\]/', $groq, $matches)) {
            $ast['filter'] = $this->parseFilter($matches[1]);
            $groq = substr($groq, strlen($matches[0]));
        }

        // Parse order: | order(...)
        if (preg_match('/\|\s*order\((.+?)\)/', $groq, $matches)) {
            $ast['order'] = $this->parseOrder($matches[1]);
            $groq = preg_replace('/\|\s*order\(.+?\)/', '', $groq);
        }

        // Parse slice: [0...10] or [0..10]
        if (preg_match('/\[(\d+)\.{2,3}(\d+)\]/', $groq, $matches)) {
            $ast['slice'] = [
                'start' => (int) $matches[1],
                'end' => (int) $matches[2],
            ];
            $groq = preg_replace('/\[\d+\.{2,3}\d+\]/', '', $groq);
        }

        // Parse projection: { ... }
        if (preg_match('/\{(.+)\}\s*$/', $groq, $matches)) {
            $ast['projection'] = $this->parseProjection($matches[1]);
        }

        return $ast;
    }

    /**
     * Parse filter expression
     */
    private function parseFilter(string $filter): array
    {
        $conditions = [];

        // Split by && and ||
        $parts = preg_split('/(\s*&&\s*|\s*\|\|\s*)/', $filter, -1, PREG_SPLIT_DELIM_CAPTURE);

        $currentCondition = null;
        $currentOperator = 'AND';

        foreach ($parts as $part) {
            $part = trim($part);

            if ($part === '&&') {
                $currentOperator = 'AND';
                continue;
            }

            if ($part === '||') {
                $currentOperator = 'OR';
                continue;
            }

            if (empty($part)) {
                continue;
            }

            $condition = $this->parseCondition($part);
            $condition['boolean'] = $currentOperator;
            $conditions[] = $condition;
        }

        return $conditions;
    }

    /**
     * Parse single condition
     */
    private function parseCondition(string $condition): array
    {
        // Handle _type == "value"
        if (preg_match('/^_type\s*==\s*["\'](.+?)["\']$/', $condition, $matches)) {
            return [
                'type' => 'type_check',
                'value' => $matches[1],
            ];
        }

        // Handle field == $param
        if (preg_match('/^(\w+(?:\.\w+)*)\s*==\s*\$(\w+)$/', $condition, $matches)) {
            return [
                'type' => 'comparison',
                'field' => $matches[1],
                'operator' => '=',
                'value' => $this->params[$matches[2]] ?? null,
                'is_param' => true,
            ];
        }

        // Handle field == "value" or field == value
        if (preg_match('/^(\w+(?:\.\w+)*)\s*(==|!=|>=|<=|>|<|match|in)\s*(.+)$/', $condition, $matches)) {
            $value = $this->parseValue($matches[3]);
            return [
                'type' => 'comparison',
                'field' => $this->convertField($matches[1]),
                'operator' => $this->convertOperator($matches[2]),
                'value' => $value,
            ];
        }

        // Handle references(target_id)
        if (preg_match('/^references\(\s*["\']?(.+?)["\']?\s*\)$/', $condition, $matches)) {
            return [
                'type' => 'references',
                'target_id' => $matches[1],
            ];
        }

        // Handle defined(field)
        if (preg_match('/^defined\((\w+(?:\.\w+)*)\)$/', $condition, $matches)) {
            return [
                'type' => 'defined',
                'field' => $this->convertField($matches[1]),
            ];
        }

        // Handle !defined(field) or !(defined(field))
        if (preg_match('/^!defined\((\w+(?:\.\w+)*)\)$/', $condition, $matches)) {
            return [
                'type' => 'not_defined',
                'field' => $this->convertField($matches[1]),
            ];
        }

        // Handle field in ["a", "b"]
        if (preg_match('/^(\w+(?:\.\w+)*)\s+in\s+\[(.+)\]$/', $condition, $matches)) {
            $values = array_map(fn($v) => $this->parseValue(trim($v)), explode(',', $matches[2]));
            return [
                'type' => 'in_array',
                'field' => $this->convertField($matches[1]),
                'values' => $values,
            ];
        }

        // Handle field match "pattern*"
        if (preg_match('/^(\w+(?:\.\w+)*)\s+match\s+["\'](.+?)["\']$/', $condition, $matches)) {
            return [
                'type' => 'match',
                'field' => $this->convertField($matches[1]),
                'pattern' => $matches[2],
            ];
        }

        throw new InvalidArgumentException("Cannot parse condition: {$condition}");
    }

    /**
     * Parse order clause
     */
    private function parseOrder(string $order): array
    {
        $orderings = [];
        $parts = explode(',', $order);

        foreach ($parts as $part) {
            $part = trim($part);
            $direction = 'asc';

            if (str_ends_with($part, ' desc')) {
                $direction = 'desc';
                $part = substr($part, 0, -5);
            } elseif (str_ends_with($part, ' asc')) {
                $part = substr($part, 0, -4);
            }

            $orderings[] = [
                'field' => $this->convertField(trim($part)),
                'direction' => $direction,
            ];
        }

        return $orderings;
    }

    /**
     * Parse projection
     */
    private function parseProjection(string $projection): array
    {
        $fields = [];
        $projection = trim($projection);

        // Handle spread operator ...
        if (str_starts_with($projection, '...')) {
            $fields['_spread'] = true;
            $projection = ltrim(substr($projection, 3), ', ');
        }

        // Split by comma, respecting nested structures
        $parts = $this->splitProjectionFields($projection);

        foreach ($parts as $part) {
            $part = trim($part);

            if (empty($part)) {
                continue;
            }

            // Handle aliased field: "alias": field
            if (preg_match('/^["\'](\w+)["\']\s*:\s*(.+)$/', $part, $matches)) {
                $fields[$matches[1]] = $this->parseProjectionValue($matches[2]);
                continue;
            }

            // Handle reference traversal: author->name
            if (preg_match('/^(\w+)->(.+)$/', $part, $matches)) {
                $fields[$matches[1]] = [
                    'type' => 'reference',
                    'field' => $matches[1],
                    'projection' => $this->parseProjection($matches[2]),
                ];
                continue;
            }

            // Handle array projection: items[]->{ ... }
            if (preg_match('/^(\w+)\[\]->(.+)$/', $part, $matches)) {
                $fields[$matches[1]] = [
                    'type' => 'array_reference',
                    'field' => $matches[1],
                    'projection' => $this->parseProjection($matches[2]),
                ];
                continue;
            }

            // Handle coalesce: coalesce(field1, field2)
            if (preg_match('/^coalesce\((.+)\)$/', $part, $matches)) {
                $coalesceFields = array_map('trim', explode(',', $matches[1]));
                $fields['_coalesce'] = $coalesceFields;
                continue;
            }

            // Simple field
            $fields[$part] = ['type' => 'field', 'field' => $this->convertField($part)];
        }

        return $fields;
    }

    /**
     * Parse projection value
     */
    private function parseProjectionValue(string $value): array
    {
        $value = trim($value);

        // Handle nested projection
        if (str_starts_with($value, '{') && str_ends_with($value, '}')) {
            return [
                'type' => 'projection',
                'projection' => $this->parseProjection(substr($value, 1, -1)),
            ];
        }

        // Handle reference traversal
        if (preg_match('/^(\w+)->(.+)$/', $value, $matches)) {
            return [
                'type' => 'reference',
                'field' => $matches[1],
                'projection' => str_starts_with($matches[2], '{')
                    ? $this->parseProjection(substr($matches[2], 1, -1))
                    : $this->parseProjection($matches[2]),
            ];
        }

        // Handle count
        if (preg_match('/^count\((.+)\)$/', $value, $matches)) {
            return [
                'type' => 'count',
                'field' => $matches[1],
            ];
        }

        // Simple field reference
        return ['type' => 'field', 'field' => $this->convertField($value)];
    }

    /**
     * Split projection fields respecting nested structures
     */
    private function splitProjectionFields(string $projection): array
    {
        $fields = [];
        $current = '';
        $depth = 0;

        for ($i = 0; $i < strlen($projection); $i++) {
            $char = $projection[$i];

            if ($char === '{' || $char === '[' || $char === '(') {
                $depth++;
            } elseif ($char === '}' || $char === ']' || $char === ')') {
                $depth--;
            }

            if ($char === ',' && $depth === 0) {
                $fields[] = trim($current);
                $current = '';
            } else {
                $current .= $char;
            }
        }

        if (!empty(trim($current))) {
            $fields[] = trim($current);
        }

        return $fields;
    }

    /**
     * Execute parsed query
     */
    private function execute(array $ast): mixed
    {
        // Determine model from type filter
        $modelClass = null;
        $typeFilter = null;

        foreach ($ast['filter'] ?? [] as $condition) {
            if (($condition['type'] ?? null) === 'type_check') {
                $typeFilter = $condition['value'];
                $modelClass = $this->typeModelMap[$typeFilter] ?? null;
                break;
            }
        }

        if (!$modelClass) {
            throw new InvalidArgumentException('Query must include _type filter');
        }

        /** @var Builder $query */
        $query = $modelClass::query();

        // Apply filters
        $this->applyFilters($query, $ast['filter'] ?? [], $typeFilter);

        // Apply ordering
        if (!empty($ast['order'])) {
            foreach ($ast['order'] as $order) {
                $query->orderBy($order['field'], $order['direction']);
            }
        }

        // Apply slice/pagination
        if (!empty($ast['slice'])) {
            $query->skip($ast['slice']['start'])
                  ->take($ast['slice']['end'] - $ast['slice']['start']);
        } else {
            $query->take(self::MAX_RESULTS);
        }

        // Execute query
        $results = $query->get();

        // Apply projection
        if (!empty($ast['projection'])) {
            $results = $results->map(fn($item) => $this->applyProjection($item, $ast['projection']));
        }

        return $results->toArray();
    }

    /**
     * Apply filters to query
     */
    private function applyFilters(Builder $query, array $filters, ?string $typeFilter): void
    {
        foreach ($filters as $filter) {
            $boolean = strtolower($filter['boolean'] ?? 'and');

            switch ($filter['type'] ?? null) {
                case 'type_check':
                    // Already handled
                    break;

                case 'comparison':
                    $method = $boolean === 'or' ? 'orWhere' : 'where';
                    $query->$method($filter['field'], $filter['operator'], $filter['value']);
                    break;

                case 'defined':
                    $method = $boolean === 'or' ? 'orWhereNotNull' : 'whereNotNull';
                    $query->$method($filter['field']);
                    break;

                case 'not_defined':
                    $method = $boolean === 'or' ? 'orWhereNull' : 'whereNull';
                    $query->$method($filter['field']);
                    break;

                case 'in_array':
                    $method = $boolean === 'or' ? 'orWhereIn' : 'whereIn';
                    $query->$method($filter['field'], $filter['values']);
                    break;

                case 'match':
                    $method = $boolean === 'or' ? 'orWhere' : 'where';
                    $pattern = str_replace(['*', '?'], ['%', '_'], $filter['pattern']);
                    $query->$method($filter['field'], 'LIKE', $pattern);
                    break;

                case 'references':
                    $query->whereExists(function ($subquery) use ($filter) {
                        $subquery->select(DB::raw(1))
                            ->from('document_references')
                            ->whereColumn('document_references.source_document_id', 'document_id')
                            ->where('document_references.target_document_id', $filter['target_id']);
                    });
                    break;
            }
        }
    }

    /**
     * Apply projection to result item
     */
    private function applyProjection(Model $item, array $projection): array
    {
        $result = [];

        // Handle spread operator
        if (!empty($projection['_spread'])) {
            $result = $item->toArray();
            unset($projection['_spread']);
        }

        foreach ($projection as $key => $config) {
            if ($key === '_spread' || $key === '_coalesce') {
                continue;
            }

            switch ($config['type'] ?? 'field') {
                case 'field':
                    $result[$key] = $item->{$config['field']} ?? null;
                    break;

                case 'reference':
                    $related = $item->{$config['field']};
                    if ($related && !empty($config['projection'])) {
                        $result[$key] = $this->applyProjection($related, $config['projection']);
                    } else {
                        $result[$key] = $related?->toArray();
                    }
                    break;

                case 'array_reference':
                    $relatedItems = $item->{$config['field']};
                    if ($relatedItems && !empty($config['projection'])) {
                        $result[$key] = $relatedItems->map(
                            fn($r) => $this->applyProjection($r, $config['projection'])
                        )->toArray();
                    } else {
                        $result[$key] = $relatedItems?->toArray();
                    }
                    break;

                case 'count':
                    $result[$key] = $item->{$config['field']}()->count();
                    break;

                case 'projection':
                    $result[$key] = $this->applyProjection($item, $config['projection']);
                    break;
            }
        }

        // Handle coalesce
        if (!empty($projection['_coalesce'])) {
            foreach ($projection['_coalesce'] as $field) {
                $value = $item->{$this->convertField($field)};
                if ($value !== null) {
                    $result['_coalesced'] = $value;
                    break;
                }
            }
        }

        // Always include _id and _type
        $result['_id'] = $item->document_id ?? $item->id;
        $result['_type'] = $this->getTypeFromModel($item);

        return $result;
    }

    /**
     * Convert GROQ field name to database column
     */
    private function convertField(string $field): string
    {
        $mapping = [
            '_id' => 'id',
            '_type' => null, // Virtual field
            '_createdAt' => 'created_at',
            '_updatedAt' => 'updated_at',
            'publishedAt' => 'published_at',
            'createdAt' => 'created_at',
            'updatedAt' => 'updated_at',
        ];

        return $mapping[$field] ?? Str::snake($field);
    }

    /**
     * Convert GROQ operator to SQL operator
     */
    private function convertOperator(string $operator): string
    {
        return match ($operator) {
            '==' => '=',
            '!=' => '!=',
            '>=' => '>=',
            '<=' => '<=',
            '>' => '>',
            '<' => '<',
            'match' => 'LIKE',
            default => '=',
        };
    }

    /**
     * Parse value from string
     */
    private function parseValue(string $value): mixed
    {
        $value = trim($value);

        // Handle quoted strings
        if ((str_starts_with($value, '"') && str_ends_with($value, '"')) ||
            (str_starts_with($value, "'") && str_ends_with($value, "'"))) {
            return substr($value, 1, -1);
        }

        // Handle boolean
        if ($value === 'true') return true;
        if ($value === 'false') return false;

        // Handle null
        if ($value === 'null') return null;

        // Handle parameter reference
        if (str_starts_with($value, '$')) {
            $paramName = substr($value, 1);
            return $this->params[$paramName] ?? null;
        }

        // Handle numeric
        if (is_numeric($value)) {
            return str_contains($value, '.') ? (float) $value : (int) $value;
        }

        return $value;
    }

    /**
     * Get cache key
     */
    private function getCacheKey(string $query, array $params): string
    {
        return 'groq:' . hash('sha256', $query . json_encode($params));
    }

    /**
     * Get type name from model
     */
    private function getTypeFromModel(Model $model): string
    {
        $class = get_class($model);
        return array_search($class, $this->typeModelMap, true) ?: Str::snake(class_basename($class));
    }

    /**
     * Register custom type mapping
     */
    public function registerType(string $type, string $modelClass): self
    {
        $this->typeModelMap[$type] = $modelClass;
        return $this;
    }

    /**
     * Invalidate cache for document type
     */
    public function invalidateCache(?string $type = null): void
    {
        if ($type) {
            Cache::tags(['groq', "groq:{$type}"])->flush();
        } else {
            Cache::tags(['groq'])->flush();
        }
    }
}

/**
 * GROQ Query Result
 */
class GroqResult
{
    public function __construct(
        public readonly mixed $data,
        public readonly float $executionTimeMs,
        public readonly bool $fromCache,
    ) {}

    public function toArray(): array
    {
        return [
            'data' => $this->data,
            'ms' => round($this->executionTimeMs, 2),
            'cached' => $this->fromCache,
        ];
    }

    public function first(): mixed
    {
        return is_array($this->data) ? ($this->data[0] ?? null) : $this->data;
    }

    public function count(): int
    {
        return is_array($this->data) ? count($this->data) : 0;
    }
}
