<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\Product;
use App\Models\CartItem;
use App\Models\InventoryReservation;
use App\Events\InventoryReserved;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Carbon\Carbon;

/**
 * InventoryReservationService - TTL-based Inventory Management
 *
 * Matches and exceeds Fluent Cart Pro inventory features:
 * - Soft reservation with TTL (timeout-based release)
 * - Hard reservation (committed to order)
 * - Real-time stock updates via WebSocket
 * - Low stock alerts
 * - Backorder management
 * - Multi-warehouse support (extensible)
 *
 * @author Revolution Trading Pros
 * @version 2.0.0
 */
class InventoryReservationService
{
    private const CACHE_PREFIX = 'inventory:';
    private const DEFAULT_RESERVATION_TTL = 900; // 15 minutes
    private const LOW_STOCK_THRESHOLD = 10;

    public function __construct(
        private readonly WebSocketService $webSocketService
    ) {}

    /**
     * Reserve inventory for a cart item
     *
     * @throws \Exception If insufficient stock
     */
    public function reserve(
        Product $product,
        int $quantity,
        ?string $cartId = null,
        ?int $userId = null,
        ?int $ttlSeconds = null
    ): InventoryReservation {
        $ttl = $ttlSeconds ?? self::DEFAULT_RESERVATION_TTL;
        $expiresAt = now()->addSeconds($ttl);

        return DB::transaction(function () use ($product, $quantity, $cartId, $userId, $expiresAt) {
            // Lock the product row for update
            $product = Product::lockForUpdate()->find($product->id);

            // Check available stock (total stock minus active reservations)
            $availableStock = $this->getAvailableStock($product);

            if ($availableStock < $quantity) {
                throw new \Exception(
                    "Insufficient stock for {$product->name}. Available: {$availableStock}, Requested: {$quantity}"
                );
            }

            // Create reservation
            $reservation = InventoryReservation::create([
                'reservation_id' => Str::uuid()->toString(),
                'product_id' => $product->id,
                'quantity' => $quantity,
                'cart_id' => $cartId,
                'user_id' => $userId,
                'status' => 'active',
                'expires_at' => $expiresAt,
            ]);

            // Update product reserved quantity
            $product->increment('reserved_quantity', $quantity);

            // Broadcast inventory update
            $this->webSocketService->broadcastInventoryUpdate(
                $product->id,
                $this->getAvailableStock($product),
                [
                    'reservation_id' => $reservation->reservation_id,
                    'quantity' => $quantity,
                    'expires_at' => $expiresAt->toIso8601String(),
                ]
            );

            // Check for low stock alert
            $newAvailable = $this->getAvailableStock($product);
            if ($newAvailable <= self::LOW_STOCK_THRESHOLD) {
                $this->webSocketService->broadcastLowStockAlert(
                    $product->id,
                    $newAvailable,
                    self::LOW_STOCK_THRESHOLD
                );
            }

            // Dispatch event
            event(new InventoryReserved($product, $quantity, $reservation->reservation_id, $expiresAt));

            Log::info('InventoryReservation: Stock reserved', [
                'product_id' => $product->id,
                'quantity' => $quantity,
                'reservation_id' => $reservation->reservation_id,
                'expires_at' => $expiresAt,
            ]);

            return $reservation;
        });
    }

    /**
     * Release a reservation
     */
    public function release(string $reservationId): bool
    {
        return DB::transaction(function () use ($reservationId) {
            $reservation = InventoryReservation::where('reservation_id', $reservationId)
                ->where('status', 'active')
                ->lockForUpdate()
                ->first();

            if (!$reservation) {
                return false;
            }

            $product = Product::lockForUpdate()->find($reservation->product_id);

            // Release the reserved quantity
            $product->decrement('reserved_quantity', $reservation->quantity);

            // Update reservation status
            $reservation->update([
                'status' => 'released',
                'released_at' => now(),
            ]);

            // Broadcast inventory update
            $this->webSocketService->broadcastInventoryUpdate(
                $product->id,
                $this->getAvailableStock($product)
            );

            Log::info('InventoryReservation: Released', [
                'reservation_id' => $reservationId,
                'product_id' => $product->id,
                'quantity' => $reservation->quantity,
            ]);

            return true;
        });
    }

    /**
     * Commit a reservation (convert to hard reservation for order)
     */
    public function commit(string $reservationId, int $orderId): bool
    {
        return DB::transaction(function () use ($reservationId, $orderId) {
            $reservation = InventoryReservation::where('reservation_id', $reservationId)
                ->where('status', 'active')
                ->lockForUpdate()
                ->first();

            if (!$reservation) {
                return false;
            }

            $product = Product::lockForUpdate()->find($reservation->product_id);

            // Move from reserved to committed (deduct from actual stock)
            $product->decrement('reserved_quantity', $reservation->quantity);
            $product->decrement('stock_quantity', $reservation->quantity);

            // Update reservation status
            $reservation->update([
                'status' => 'committed',
                'order_id' => $orderId,
                'committed_at' => now(),
            ]);

            // Broadcast inventory update
            $this->webSocketService->broadcastInventoryUpdate(
                $product->id,
                $this->getAvailableStock($product)
            );

            Log::info('InventoryReservation: Committed', [
                'reservation_id' => $reservationId,
                'order_id' => $orderId,
                'product_id' => $product->id,
            ]);

            return true;
        });
    }

    /**
     * Extend reservation TTL
     */
    public function extend(string $reservationId, ?int $additionalSeconds = null): bool
    {
        $additionalSeconds = $additionalSeconds ?? self::DEFAULT_RESERVATION_TTL;

        $reservation = InventoryReservation::where('reservation_id', $reservationId)
            ->where('status', 'active')
            ->first();

        if (!$reservation) {
            return false;
        }

        $newExpiry = now()->addSeconds($additionalSeconds);

        $reservation->update(['expires_at' => $newExpiry]);

        Log::info('InventoryReservation: Extended', [
            'reservation_id' => $reservationId,
            'new_expires_at' => $newExpiry,
        ]);

        return true;
    }

    /**
     * Release all expired reservations
     */
    public function releaseExpired(): int
    {
        $expiredReservations = InventoryReservation::where('status', 'active')
            ->where('expires_at', '<', now())
            ->get();

        $released = 0;

        foreach ($expiredReservations as $reservation) {
            if ($this->release($reservation->reservation_id)) {
                $released++;
            }
        }

        if ($released > 0) {
            Log::info('InventoryReservation: Released expired reservations', [
                'count' => $released,
            ]);
        }

        return $released;
    }

    /**
     * Release all reservations for a cart
     */
    public function releaseForCart(string $cartId): int
    {
        $reservations = InventoryReservation::where('cart_id', $cartId)
            ->where('status', 'active')
            ->get();

        $released = 0;

        foreach ($reservations as $reservation) {
            if ($this->release($reservation->reservation_id)) {
                $released++;
            }
        }

        return $released;
    }

    /**
     * Get available stock for a product
     */
    public function getAvailableStock(Product $product): int
    {
        // Use cached value if available
        $cacheKey = self::CACHE_PREFIX . "available:{$product->id}";

        return Cache::remember($cacheKey, 60, function () use ($product) {
            $product->refresh();
            return max(0, $product->stock_quantity - $product->reserved_quantity);
        });
    }

    /**
     * Invalidate stock cache for a product
     */
    public function invalidateStockCache(int $productId): void
    {
        Cache::forget(self::CACHE_PREFIX . "available:{$productId}");
    }

    /**
     * Check if quantity is available
     */
    public function isAvailable(Product $product, int $quantity): bool
    {
        return $this->getAvailableStock($product) >= $quantity;
    }

    /**
     * Get reservation status
     */
    public function getReservation(string $reservationId): ?InventoryReservation
    {
        return InventoryReservation::where('reservation_id', $reservationId)->first();
    }

    /**
     * Get all active reservations for a product
     */
    public function getActiveReservations(Product $product): \Illuminate\Database\Eloquent\Collection
    {
        return InventoryReservation::where('product_id', $product->id)
            ->where('status', 'active')
            ->orderBy('expires_at')
            ->get();
    }

    /**
     * Get reservation statistics
     */
    public function getStatistics(): array
    {
        return [
            'active_reservations' => InventoryReservation::where('status', 'active')->count(),
            'expired_pending' => InventoryReservation::where('status', 'active')
                ->where('expires_at', '<', now())
                ->count(),
            'committed_today' => InventoryReservation::where('status', 'committed')
                ->whereDate('committed_at', today())
                ->count(),
            'released_today' => InventoryReservation::where('status', 'released')
                ->whereDate('released_at', today())
                ->count(),
            'total_reserved_quantity' => InventoryReservation::where('status', 'active')
                ->sum('quantity'),
        ];
    }

    /**
     * Handle backorder for out-of-stock product
     */
    public function createBackorder(
        Product $product,
        int $quantity,
        int $userId,
        ?string $email = null
    ): InventoryReservation {
        $reservation = InventoryReservation::create([
            'reservation_id' => Str::uuid()->toString(),
            'product_id' => $product->id,
            'quantity' => $quantity,
            'user_id' => $userId,
            'status' => 'backorder',
            'metadata' => [
                'notification_email' => $email,
                'requested_at' => now()->toIso8601String(),
            ],
        ]);

        Log::info('InventoryReservation: Backorder created', [
            'product_id' => $product->id,
            'quantity' => $quantity,
            'user_id' => $userId,
        ]);

        return $reservation;
    }

    /**
     * Fulfill backorders when stock is replenished
     */
    public function fulfillBackorders(Product $product, int $restockedQuantity): array
    {
        $backorders = InventoryReservation::where('product_id', $product->id)
            ->where('status', 'backorder')
            ->orderBy('created_at')
            ->get();

        $fulfilled = [];
        $remainingStock = $restockedQuantity;

        foreach ($backorders as $backorder) {
            if ($remainingStock <= 0) {
                break;
            }

            if ($backorder->quantity <= $remainingStock) {
                // Can fulfill entire backorder
                $backorder->update([
                    'status' => 'active',
                    'expires_at' => now()->addHours(24), // 24-hour grace period
                ]);

                $product->increment('reserved_quantity', $backorder->quantity);
                $remainingStock -= $backorder->quantity;

                $fulfilled[] = $backorder;

                // Notify customer
                if ($backorder->user_id) {
                    $this->webSocketService->sendNotification(
                        $backorder->user_id,
                        'backorder_available',
                        'Item Now Available!',
                        "{$product->name} is now back in stock and reserved for you.",
                        ['product_id' => $product->id, 'reservation_id' => $backorder->reservation_id]
                    );
                }
            }
        }

        return $fulfilled;
    }

    /**
     * Sync cart item reservations
     */
    public function syncCartReservations(string $cartId, array $items): array
    {
        $results = [];

        // Get existing reservations for this cart
        $existingReservations = InventoryReservation::where('cart_id', $cartId)
            ->where('status', 'active')
            ->get()
            ->keyBy('product_id');

        foreach ($items as $item) {
            $productId = $item['product_id'];
            $quantity = $item['quantity'];

            $existing = $existingReservations->get($productId);

            if ($existing) {
                if ($existing->quantity === $quantity) {
                    // No change needed, just extend
                    $this->extend($existing->reservation_id);
                    $results[$productId] = ['action' => 'extended', 'reservation_id' => $existing->reservation_id];
                } elseif ($existing->quantity < $quantity) {
                    // Need more - release and re-reserve
                    $this->release($existing->reservation_id);
                    $product = Product::find($productId);
                    $newReservation = $this->reserve($product, $quantity, $cartId);
                    $results[$productId] = ['action' => 'increased', 'reservation_id' => $newReservation->reservation_id];
                } else {
                    // Need less - release and re-reserve smaller amount
                    $this->release($existing->reservation_id);
                    $product = Product::find($productId);
                    $newReservation = $this->reserve($product, $quantity, $cartId);
                    $results[$productId] = ['action' => 'decreased', 'reservation_id' => $newReservation->reservation_id];
                }
            } else {
                // New item - create reservation
                $product = Product::find($productId);
                $newReservation = $this->reserve($product, $quantity, $cartId);
                $results[$productId] = ['action' => 'created', 'reservation_id' => $newReservation->reservation_id];
            }
        }

        // Release reservations for removed items
        $currentProductIds = collect($items)->pluck('product_id')->toArray();
        foreach ($existingReservations as $productId => $reservation) {
            if (!in_array($productId, $currentProductIds)) {
                $this->release($reservation->reservation_id);
                $results[$productId] = ['action' => 'released', 'reservation_id' => $reservation->reservation_id];
            }
        }

        return $results;
    }
}
