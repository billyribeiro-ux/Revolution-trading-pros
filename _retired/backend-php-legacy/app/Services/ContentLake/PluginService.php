<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Plugin/Extension Service
 *
 * Provides a comprehensive plugin architecture:
 * - Plugin registration and lifecycle
 * - Hook system for extensibility
 * - Custom field types
 * - Custom block types for Portable Text
 * - API extensions
 * - Middleware hooks
 * - Event listeners
 * - Configuration management
 * - Dependency resolution
 */
class PluginService
{
    private array $registeredPlugins = [];
    private array $hooks = [];
    private array $fieldTypes = [];
    private array $blockTypes = [];
    private array $apiExtensions = [];
    private array $middlewares = [];

    public function __construct()
    {
        $this->loadRegisteredPlugins();
    }

    /**
     * Register a plugin
     */
    public function register(array $plugin): array
    {
        $this->validatePlugin($plugin);

        $pluginId = $plugin['id'] ?? Str::slug($plugin['name']);

        // Check for conflicts
        if (isset($this->registeredPlugins[$pluginId])) {
            throw new \InvalidArgumentException("Plugin already registered: {$pluginId}");
        }

        // Resolve dependencies
        $this->resolveDependencies($plugin);

        // Store plugin
        DB::table('cms_plugins')->insert([
            'id' => $pluginId,
            'name' => $plugin['name'],
            'version' => $plugin['version'],
            'description' => $plugin['description'] ?? null,
            'author' => $plugin['author'] ?? null,
            'homepage' => $plugin['homepage'] ?? null,
            'dependencies' => json_encode($plugin['dependencies'] ?? []),
            'config_schema' => json_encode($plugin['configSchema'] ?? []),
            'config' => json_encode($plugin['defaultConfig'] ?? []),
            'hooks' => json_encode($plugin['hooks'] ?? []),
            'field_types' => json_encode($plugin['fieldTypes'] ?? []),
            'block_types' => json_encode($plugin['blockTypes'] ?? []),
            'api_extensions' => json_encode($plugin['apiExtensions'] ?? []),
            'status' => 'active',
            'installed_at' => now(),
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        // Initialize plugin
        $this->initializePlugin($pluginId, $plugin);

        Cache::forget('cms_plugins');

        return $this->getPlugin($pluginId);
    }

    /**
     * Get plugin info
     */
    public function getPlugin(string $pluginId): ?array
    {
        $plugin = DB::table('cms_plugins')->find($pluginId);

        if (!$plugin) {
            return null;
        }

        return $this->formatPlugin($plugin);
    }

    /**
     * List all plugins
     */
    public function listPlugins(?string $status = null): array
    {
        $query = DB::table('cms_plugins');

        if ($status) {
            $query->where('status', $status);
        }

        return $query->get()->map(fn($p) => $this->formatPlugin($p))->toArray();
    }

    /**
     * Enable a plugin
     */
    public function enable(string $pluginId): array
    {
        $plugin = $this->getPlugin($pluginId);

        if (!$plugin) {
            throw new \InvalidArgumentException("Plugin not found: {$pluginId}");
        }

        // Resolve dependencies again
        $this->resolveDependencies($plugin);

        DB::table('cms_plugins')
            ->where('id', $pluginId)
            ->update(['status' => 'active', 'updated_at' => now()]);

        // Re-initialize
        $this->initializePlugin($pluginId, $plugin);

        Cache::forget('cms_plugins');

        return $this->getPlugin($pluginId);
    }

    /**
     * Disable a plugin
     */
    public function disable(string $pluginId): array
    {
        // Check if other plugins depend on this
        $dependents = $this->findDependents($pluginId);

        if (!empty($dependents)) {
            throw new \InvalidArgumentException(
                "Cannot disable: other plugins depend on this: " . implode(', ', $dependents)
            );
        }

        DB::table('cms_plugins')
            ->where('id', $pluginId)
            ->update(['status' => 'disabled', 'updated_at' => now()]);

        // Unload plugin
        $this->unloadPlugin($pluginId);

        Cache::forget('cms_plugins');

        return $this->getPlugin($pluginId);
    }

    /**
     * Uninstall a plugin
     */
    public function uninstall(string $pluginId): bool
    {
        $plugin = $this->getPlugin($pluginId);

        if (!$plugin) {
            return false;
        }

        // Check dependencies
        $dependents = $this->findDependents($pluginId);

        if (!empty($dependents)) {
            throw new \InvalidArgumentException(
                "Cannot uninstall: other plugins depend on this: " . implode(', ', $dependents)
            );
        }

        // Run uninstall hooks
        $this->executeHook("{$pluginId}:uninstall", ['plugin' => $plugin]);

        // Remove from database
        DB::table('cms_plugins')->where('id', $pluginId)->delete();

        // Unload
        $this->unloadPlugin($pluginId);

        Cache::forget('cms_plugins');

        return true;
    }

    /**
     * Update plugin configuration
     */
    public function configure(string $pluginId, array $config): array
    {
        $plugin = $this->getPlugin($pluginId);

        if (!$plugin) {
            throw new \InvalidArgumentException("Plugin not found: {$pluginId}");
        }

        // Validate config against schema
        $this->validateConfig($config, $plugin['configSchema']);

        // Merge with existing config
        $newConfig = array_merge($plugin['config'], $config);

        DB::table('cms_plugins')
            ->where('id', $pluginId)
            ->update(['config' => json_encode($newConfig), 'updated_at' => now()]);

        // Execute config changed hook
        $this->executeHook("{$pluginId}:config_changed", [
            'oldConfig' => $plugin['config'],
            'newConfig' => $newConfig,
        ]);

        Cache::forget('cms_plugins');
        Cache::forget("plugin_config:{$pluginId}");

        return $this->getPlugin($pluginId);
    }

    /**
     * Get plugin configuration
     */
    public function getConfig(string $pluginId): array
    {
        return Cache::remember("plugin_config:{$pluginId}", 3600, function () use ($pluginId) {
            $plugin = $this->getPlugin($pluginId);
            return $plugin ? $plugin['config'] : [];
        });
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HOOK SYSTEM
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Register a hook handler
     */
    public function addHook(string $hookName, callable $handler, int $priority = 10): void
    {
        if (!isset($this->hooks[$hookName])) {
            $this->hooks[$hookName] = [];
        }

        $this->hooks[$hookName][] = [
            'handler' => $handler,
            'priority' => $priority,
        ];

        // Sort by priority
        usort($this->hooks[$hookName], fn($a, $b) => $a['priority'] <=> $b['priority']);
    }

    /**
     * Execute a hook
     */
    public function executeHook(string $hookName, array $args = []): mixed
    {
        if (!isset($this->hooks[$hookName])) {
            return $args['default'] ?? null;
        }

        $result = $args['initial'] ?? null;

        foreach ($this->hooks[$hookName] as $hook) {
            $result = call_user_func($hook['handler'], $result, $args);
        }

        return $result;
    }

    /**
     * Filter a value through hooks
     */
    public function filter(string $hookName, mixed $value, array $args = []): mixed
    {
        return $this->executeHook($hookName, array_merge($args, ['initial' => $value]));
    }

    /**
     * Get registered hooks
     */
    public function getHooks(): array
    {
        return array_keys($this->hooks);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CUSTOM FIELD TYPES
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Register a custom field type
     */
    public function registerFieldType(string $name, array $definition): void
    {
        $this->validateFieldTypeDefinition($definition);

        $this->fieldTypes[$name] = [
            'name' => $name,
            'title' => $definition['title'] ?? ucfirst($name),
            'description' => $definition['description'] ?? null,
            'icon' => $definition['icon'] ?? null,
            'inputComponent' => $definition['inputComponent'] ?? null,
            'previewComponent' => $definition['previewComponent'] ?? null,
            'validation' => $definition['validation'] ?? [],
            'options' => $definition['options'] ?? [],
            'prepare' => $definition['prepare'] ?? null,
            'serialize' => $definition['serialize'] ?? null,
            'deserialize' => $definition['deserialize'] ?? null,
        ];
    }

    /**
     * Get all registered field types
     */
    public function getFieldTypes(): array
    {
        return $this->fieldTypes;
    }

    /**
     * Get a specific field type
     */
    public function getFieldType(string $name): ?array
    {
        return $this->fieldTypes[$name] ?? null;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CUSTOM BLOCK TYPES
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Register a custom block type for Portable Text
     */
    public function registerBlockType(string $name, array $definition): void
    {
        $this->blockTypes[$name] = [
            'name' => $name,
            'title' => $definition['title'] ?? ucfirst($name),
            'description' => $definition['description'] ?? null,
            'icon' => $definition['icon'] ?? null,
            'fields' => $definition['fields'] ?? [],
            'preview' => $definition['preview'] ?? null,
            'component' => $definition['component'] ?? null,
            'serializer' => $definition['serializer'] ?? null,
        ];
    }

    /**
     * Get all registered block types
     */
    public function getBlockTypes(): array
    {
        return $this->blockTypes;
    }

    /**
     * Get a specific block type
     */
    public function getBlockType(string $name): ?array
    {
        return $this->blockTypes[$name] ?? null;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API EXTENSIONS
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Register an API extension
     */
    public function registerApiExtension(string $name, array $definition): void
    {
        $this->apiExtensions[$name] = [
            'name' => $name,
            'prefix' => $definition['prefix'] ?? $name,
            'routes' => $definition['routes'] ?? [],
            'middleware' => $definition['middleware'] ?? [],
            'controller' => $definition['controller'] ?? null,
        ];
    }

    /**
     * Get all API extensions
     */
    public function getApiExtensions(): array
    {
        return $this->apiExtensions;
    }

    /**
     * Get API routes from all extensions
     */
    public function getExtensionRoutes(): array
    {
        $routes = [];

        foreach ($this->apiExtensions as $extension) {
            foreach ($extension['routes'] as $route) {
                $routes[] = [
                    'method' => $route['method'] ?? 'GET',
                    'path' => "/{$extension['prefix']}/{$route['path']}",
                    'handler' => $route['handler'],
                    'middleware' => array_merge($extension['middleware'], $route['middleware'] ?? []),
                    'name' => "{$extension['name']}.{$route['name']}",
                ];
            }
        }

        return $routes;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MIDDLEWARE
    // ═══════════════════════════════════════════════════════════════════════

    /**
     * Register middleware
     */
    public function registerMiddleware(string $name, callable $middleware): void
    {
        $this->middlewares[$name] = $middleware;
    }

    /**
     * Get registered middlewares
     */
    public function getMiddlewares(): array
    {
        return $this->middlewares;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function loadRegisteredPlugins(): void
    {
        $plugins = Cache::remember('cms_plugins', 3600, function () {
            return DB::table('cms_plugins')
                ->where('status', 'active')
                ->get();
        });

        foreach ($plugins as $plugin) {
            $this->initializePlugin($plugin->id, $this->formatPlugin($plugin));
        }
    }

    private function validatePlugin(array $plugin): void
    {
        $required = ['name', 'version'];

        foreach ($required as $field) {
            if (empty($plugin[$field])) {
                throw new \InvalidArgumentException("Plugin must have: {$field}");
            }
        }

        // Validate version format
        if (!preg_match('/^\d+\.\d+\.\d+/', $plugin['version'])) {
            throw new \InvalidArgumentException('Invalid version format. Use semantic versioning (e.g., 1.0.0)');
        }
    }

    private function resolveDependencies(array $plugin): void
    {
        $dependencies = $plugin['dependencies'] ?? [];

        foreach ($dependencies as $depId => $versionConstraint) {
            $dep = $this->getPlugin($depId);

            if (!$dep) {
                throw new \InvalidArgumentException("Missing dependency: {$depId}");
            }

            if ($dep['status'] !== 'active') {
                throw new \InvalidArgumentException("Dependency not active: {$depId}");
            }

            // Check version constraint
            if (!$this->satisfiesVersionConstraint($dep['version'], $versionConstraint)) {
                throw new \InvalidArgumentException(
                    "Dependency version mismatch: {$depId} requires {$versionConstraint}, found {$dep['version']}"
                );
            }
        }
    }

    private function satisfiesVersionConstraint(string $version, string $constraint): bool
    {
        // Simple constraint checking
        if ($constraint === '*') {
            return true;
        }

        if (str_starts_with($constraint, '^')) {
            $minVersion = substr($constraint, 1);
            return version_compare($version, $minVersion, '>=');
        }

        if (str_starts_with($constraint, '>=')) {
            $minVersion = substr($constraint, 2);
            return version_compare($version, $minVersion, '>=');
        }

        return version_compare($version, $constraint, '>=');
    }

    private function findDependents(string $pluginId): array
    {
        $dependents = [];
        $plugins = $this->listPlugins('active');

        foreach ($plugins as $plugin) {
            $deps = $plugin['dependencies'] ?? [];
            if (isset($deps[$pluginId])) {
                $dependents[] = $plugin['id'];
            }
        }

        return $dependents;
    }

    private function initializePlugin(string $pluginId, array $plugin): void
    {
        $this->registeredPlugins[$pluginId] = $plugin;

        // Register hooks
        foreach ($plugin['hooks'] ?? [] as $hookName => $handler) {
            if (is_callable($handler)) {
                $this->addHook($hookName, $handler);
            }
        }

        // Register field types
        foreach ($plugin['fieldTypes'] ?? [] as $name => $definition) {
            $this->registerFieldType($name, $definition);
        }

        // Register block types
        foreach ($plugin['blockTypes'] ?? [] as $name => $definition) {
            $this->registerBlockType($name, $definition);
        }

        // Register API extensions
        foreach ($plugin['apiExtensions'] ?? [] as $name => $definition) {
            $this->registerApiExtension($name, $definition);
        }

        // Execute init hook
        $this->executeHook("{$pluginId}:init", ['plugin' => $plugin]);
    }

    private function unloadPlugin(string $pluginId): void
    {
        unset($this->registeredPlugins[$pluginId]);

        // Remove hooks registered by this plugin
        // (In a real implementation, we'd track which hooks belong to which plugin)
    }

    private function formatPlugin(object $plugin): array
    {
        return [
            'id' => $plugin->id,
            'name' => $plugin->name,
            'version' => $plugin->version,
            'description' => $plugin->description,
            'author' => $plugin->author,
            'homepage' => $plugin->homepage,
            'dependencies' => json_decode($plugin->dependencies, true) ?? [],
            'configSchema' => json_decode($plugin->config_schema, true) ?? [],
            'config' => json_decode($plugin->config, true) ?? [],
            'hooks' => json_decode($plugin->hooks, true) ?? [],
            'fieldTypes' => json_decode($plugin->field_types, true) ?? [],
            'blockTypes' => json_decode($plugin->block_types, true) ?? [],
            'apiExtensions' => json_decode($plugin->api_extensions, true) ?? [],
            'status' => $plugin->status,
            'installedAt' => $plugin->installed_at,
        ];
    }

    private function validateConfig(array $config, array $schema): void
    {
        foreach ($schema as $field => $rules) {
            if (($rules['required'] ?? false) && !isset($config[$field])) {
                throw new \InvalidArgumentException("Required config field missing: {$field}");
            }

            if (isset($config[$field]) && isset($rules['type'])) {
                $type = gettype($config[$field]);
                $expectedType = $rules['type'];

                if ($type !== $expectedType) {
                    throw new \InvalidArgumentException(
                        "Config field {$field} must be {$expectedType}, got {$type}"
                    );
                }
            }
        }
    }

    private function validateFieldTypeDefinition(array $definition): void
    {
        // Field type validation logic
    }
}
