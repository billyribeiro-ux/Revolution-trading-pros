<?php

declare(strict_types=1);

namespace App\Services\Forms;

use App\Models\Form;
use App\Models\FormField;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Collection;

/**
 * FormInventoryService - FluentForms 6.1.8 (December 2025)
 *
 * Manages product inventory for form product/payment fields.
 * Features:
 * - Real-time stock tracking
 * - Low stock alerts
 * - Stock reservation during checkout
 * - Automatic stock reduction on submission
 * - Inventory reports and analytics
 */
class FormInventoryService
{
    private const CACHE_PREFIX = 'form_inventory:';
    private const RESERVATION_TTL = 900; // 15 minutes reservation timeout

    /**
     * Get inventory for a form field
     */
    public function getFieldInventory(int $fieldId): array
    {
        $field = FormField::find($fieldId);
        if (!$field) {
            return [];
        }

        $options = $field->settings['options'] ?? [];
        $inventory = [];

        foreach ($options as $option) {
            $productId = $option['id'] ?? $option['value'] ?? null;
            if (!$productId) {
                continue;
            }

            $stock = $this->getStock($fieldId, $productId);
            $reserved = $this->getReservedStock($fieldId, $productId);

            $inventory[] = [
                'id' => $productId,
                'name' => $option['label'] ?? $option['name'] ?? $productId,
                'price' => (float) ($option['price'] ?? 0),
                'stock' => $stock,
                'reserved' => $reserved,
                'available' => max(0, $stock - $reserved),
                'max_per_order' => $option['max_per_order'] ?? null,
                'sku' => $option['sku'] ?? null,
                'low_stock_threshold' => $option['low_stock_threshold'] ?? 5,
                'is_low_stock' => $stock <= ($option['low_stock_threshold'] ?? 5),
                'is_out_of_stock' => ($stock - $reserved) <= 0,
            ];
        }

        return $inventory;
    }

    /**
     * Get current stock for a product
     */
    public function getStock(int $fieldId, string $productId): int
    {
        $cacheKey = $this->getStockCacheKey($fieldId, $productId);

        return Cache::remember($cacheKey, 3600, function () use ($fieldId, $productId) {
            $field = FormField::find($fieldId);
            if (!$field) {
                return 0;
            }

            $options = $field->settings['options'] ?? [];
            foreach ($options as $option) {
                $id = $option['id'] ?? $option['value'] ?? null;
                if ($id === $productId) {
                    return (int) ($option['stock'] ?? 0);
                }
            }

            return 0;
        });
    }

    /**
     * Update stock for a product
     */
    public function updateStock(int $fieldId, string $productId, int $newStock): bool
    {
        $field = FormField::find($fieldId);
        if (!$field) {
            return false;
        }

        $settings = $field->settings;
        $options = $settings['options'] ?? [];
        $updated = false;

        foreach ($options as $index => $option) {
            $id = $option['id'] ?? $option['value'] ?? null;
            if ($id === $productId) {
                $options[$index]['stock'] = max(0, $newStock);
                $updated = true;
                break;
            }
        }

        if (!$updated) {
            return false;
        }

        $settings['options'] = $options;
        $field->settings = $settings;
        $field->save();

        // Clear cache
        $this->clearStockCache($fieldId, $productId);

        Log::info('FormInventoryService: Stock updated', [
            'field_id' => $fieldId,
            'product_id' => $productId,
            'new_stock' => $newStock,
        ]);

        return true;
    }

    /**
     * Reduce stock after successful submission
     */
    public function reduceStock(int $fieldId, string $productId, int $quantity): bool
    {
        $currentStock = $this->getStock($fieldId, $productId);
        $newStock = max(0, $currentStock - $quantity);

        return $this->updateStock($fieldId, $productId, $newStock);
    }

    /**
     * Increment stock (for refunds/cancellations)
     */
    public function incrementStock(int $fieldId, string $productId, int $quantity): bool
    {
        $currentStock = $this->getStock($fieldId, $productId);
        $newStock = $currentStock + $quantity;

        return $this->updateStock($fieldId, $productId, $newStock);
    }

    /**
     * Reserve stock during checkout process
     */
    public function reserveStock(int $fieldId, string $productId, int $quantity, string $sessionId): array
    {
        $available = $this->getAvailableStock($fieldId, $productId);

        if ($available < $quantity) {
            return [
                'success' => false,
                'message' => 'Insufficient stock available.',
                'available' => $available,
                'requested' => $quantity,
            ];
        }

        $reservationKey = $this->getReservationKey($fieldId, $productId, $sessionId);
        $reservationsKey = $this->getProductReservationsKey($fieldId, $productId);

        // Store individual reservation
        Cache::put($reservationKey, [
            'quantity' => $quantity,
            'created_at' => now()->toIso8601String(),
        ], self::RESERVATION_TTL);

        // Update total reservations
        $currentReservations = Cache::get($reservationsKey, []);
        $currentReservations[$sessionId] = $quantity;
        Cache::put($reservationsKey, $currentReservations, self::RESERVATION_TTL);

        Log::info('FormInventoryService: Stock reserved', [
            'field_id' => $fieldId,
            'product_id' => $productId,
            'quantity' => $quantity,
            'session_id' => substr($sessionId, 0, 8) . '...',
        ]);

        return [
            'success' => true,
            'message' => 'Stock reserved successfully.',
            'reservation_expires_at' => now()->addSeconds(self::RESERVATION_TTL)->toIso8601String(),
        ];
    }

    /**
     * Release reserved stock
     */
    public function releaseReservation(int $fieldId, string $productId, string $sessionId): bool
    {
        $reservationKey = $this->getReservationKey($fieldId, $productId, $sessionId);
        $reservationsKey = $this->getProductReservationsKey($fieldId, $productId);

        // Remove individual reservation
        Cache::forget($reservationKey);

        // Update total reservations
        $currentReservations = Cache::get($reservationsKey, []);
        unset($currentReservations[$sessionId]);

        if (empty($currentReservations)) {
            Cache::forget($reservationsKey);
        } else {
            Cache::put($reservationsKey, $currentReservations, self::RESERVATION_TTL);
        }

        return true;
    }

    /**
     * Confirm reservation and reduce actual stock
     */
    public function confirmReservation(int $fieldId, string $productId, int $quantity, string $sessionId): bool
    {
        // Release the reservation
        $this->releaseReservation($fieldId, $productId, $sessionId);

        // Reduce actual stock
        return $this->reduceStock($fieldId, $productId, $quantity);
    }

    /**
     * Get available stock (total - reserved)
     */
    public function getAvailableStock(int $fieldId, string $productId): int
    {
        $stock = $this->getStock($fieldId, $productId);
        $reserved = $this->getReservedStock($fieldId, $productId);

        return max(0, $stock - $reserved);
    }

    /**
     * Get total reserved stock for a product
     */
    public function getReservedStock(int $fieldId, string $productId): int
    {
        $reservationsKey = $this->getProductReservationsKey($fieldId, $productId);
        $reservations = Cache::get($reservationsKey, []);

        return array_sum($reservations);
    }

    /**
     * Check if product is available
     */
    public function isAvailable(int $fieldId, string $productId, int $quantity = 1): bool
    {
        return $this->getAvailableStock($fieldId, $productId) >= $quantity;
    }

    /**
     * Validate cart items against inventory
     */
    public function validateCart(array $cartItems): array
    {
        $errors = [];
        $validItems = [];

        foreach ($cartItems as $item) {
            $fieldId = $item['field_id'] ?? 0;
            $productId = $item['product_id'] ?? '';
            $quantity = $item['quantity'] ?? 1;

            if (!$fieldId || !$productId) {
                $errors[] = [
                    'item' => $item,
                    'error' => 'Invalid item data',
                ];
                continue;
            }

            $available = $this->getAvailableStock($fieldId, $productId);

            if ($available < $quantity) {
                $errors[] = [
                    'item' => $item,
                    'error' => "Only {$available} available",
                    'available' => $available,
                    'requested' => $quantity,
                ];
            } else {
                $validItems[] = $item;
            }
        }

        return [
            'valid' => empty($errors),
            'valid_items' => $validItems,
            'errors' => $errors,
        ];
    }

    /**
     * Get low stock products for a form
     */
    public function getLowStockProducts(int $formId): Collection
    {
        $form = Form::with('fields')->find($formId);
        if (!$form) {
            return collect();
        }

        $lowStockProducts = collect();

        foreach ($form->fields as $field) {
            if (!in_array($field->type, ['product', 'payment', 'inventory'])) {
                continue;
            }

            $inventory = $this->getFieldInventory($field->id);
            foreach ($inventory as $product) {
                if ($product['is_low_stock'] || $product['is_out_of_stock']) {
                    $lowStockProducts->push([
                        'field_id' => $field->id,
                        'field_name' => $field->name,
                        ...$product,
                    ]);
                }
            }
        }

        return $lowStockProducts;
    }

    /**
     * Get inventory report for a form
     */
    public function getInventoryReport(int $formId): array
    {
        $form = Form::with('fields')->find($formId);
        if (!$form) {
            return [];
        }

        $report = [
            'form_id' => $formId,
            'form_name' => $form->name,
            'generated_at' => now()->toIso8601String(),
            'summary' => [
                'total_products' => 0,
                'in_stock' => 0,
                'low_stock' => 0,
                'out_of_stock' => 0,
                'total_value' => 0,
            ],
            'products' => [],
        ];

        foreach ($form->fields as $field) {
            if (!in_array($field->type, ['product', 'payment', 'inventory'])) {
                continue;
            }

            $inventory = $this->getFieldInventory($field->id);
            foreach ($inventory as $product) {
                $report['summary']['total_products']++;
                $report['summary']['total_value'] += $product['price'] * $product['stock'];

                if ($product['is_out_of_stock']) {
                    $report['summary']['out_of_stock']++;
                } elseif ($product['is_low_stock']) {
                    $report['summary']['low_stock']++;
                } else {
                    $report['summary']['in_stock']++;
                }

                $report['products'][] = [
                    'field_id' => $field->id,
                    'field_name' => $field->name,
                    ...$product,
                ];
            }
        }

        return $report;
    }

    /**
     * Bulk update inventory
     */
    public function bulkUpdateInventory(array $updates): array
    {
        $results = ['updated' => 0, 'failed' => 0, 'errors' => []];

        foreach ($updates as $update) {
            $fieldId = $update['field_id'] ?? 0;
            $productId = $update['product_id'] ?? '';
            $stock = $update['stock'] ?? null;

            if (!$fieldId || !$productId || $stock === null) {
                $results['failed']++;
                $results['errors'][] = 'Invalid update data';
                continue;
            }

            if ($this->updateStock($fieldId, $productId, (int) $stock)) {
                $results['updated']++;
            } else {
                $results['failed']++;
                $results['errors'][] = "Failed to update {$productId}";
            }
        }

        return $results;
    }

    // =========================================================================
    // CACHE KEY HELPERS
    // =========================================================================

    private function getStockCacheKey(int $fieldId, string $productId): string
    {
        return self::CACHE_PREFIX . "stock:{$fieldId}:{$productId}";
    }

    private function getReservationKey(int $fieldId, string $productId, string $sessionId): string
    {
        return self::CACHE_PREFIX . "reservation:{$fieldId}:{$productId}:{$sessionId}";
    }

    private function getProductReservationsKey(int $fieldId, string $productId): string
    {
        return self::CACHE_PREFIX . "reservations:{$fieldId}:{$productId}";
    }

    private function clearStockCache(int $fieldId, string $productId): void
    {
        Cache::forget($this->getStockCacheKey($fieldId, $productId));
    }
}
