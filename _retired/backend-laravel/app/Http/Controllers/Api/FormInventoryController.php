<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Services\Forms\FormInventoryService;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

/**
 * FormInventoryController - FluentForms 6.1.8 (December 2025)
 *
 * API endpoints for form inventory management.
 */
class FormInventoryController extends Controller
{
    public function __construct(
        private readonly FormInventoryService $inventoryService
    ) {}

    /**
     * Get inventory for a field
     */
    public function fieldInventory(int $fieldId): JsonResponse
    {
        $inventory = $this->inventoryService->getFieldInventory($fieldId);

        return response()->json([
            'success' => true,
            'data' => $inventory,
        ]);
    }

    /**
     * Get stock for a specific product
     */
    public function getStock(int $fieldId, string $productId): JsonResponse
    {
        $stock = $this->inventoryService->getStock($fieldId, $productId);
        $available = $this->inventoryService->getAvailableStock($fieldId, $productId);

        return response()->json([
            'success' => true,
            'data' => [
                'stock' => $stock,
                'available' => $available,
                'reserved' => $stock - $available,
            ],
        ]);
    }

    /**
     * Update stock for a product
     */
    public function updateStock(Request $request, int $fieldId, string $productId): JsonResponse
    {
        $request->validate([
            'stock' => 'required|integer|min:0',
        ]);

        $result = $this->inventoryService->updateStock(
            $fieldId,
            $productId,
            (int) $request->input('stock')
        );

        return response()->json([
            'success' => $result,
            'message' => $result ? 'Stock updated successfully.' : 'Failed to update stock.',
        ]);
    }

    /**
     * Reserve stock for checkout
     */
    public function reserve(Request $request): JsonResponse
    {
        $request->validate([
            'field_id' => 'required|integer',
            'product_id' => 'required|string',
            'quantity' => 'required|integer|min:1',
        ]);

        $sessionId = session()->getId();
        $result = $this->inventoryService->reserveStock(
            (int) $request->input('field_id'),
            $request->input('product_id'),
            (int) $request->input('quantity'),
            $sessionId
        );

        return response()->json($result, $result['success'] ? 200 : 400);
    }

    /**
     * Release reservation
     */
    public function release(Request $request): JsonResponse
    {
        $request->validate([
            'field_id' => 'required|integer',
            'product_id' => 'required|string',
        ]);

        $sessionId = session()->getId();
        $this->inventoryService->releaseReservation(
            (int) $request->input('field_id'),
            $request->input('product_id'),
            $sessionId
        );

        return response()->json([
            'success' => true,
            'message' => 'Reservation released.',
        ]);
    }

    /**
     * Check product availability
     */
    public function checkAvailability(Request $request): JsonResponse
    {
        $request->validate([
            'field_id' => 'required|integer',
            'product_id' => 'required|string',
            'quantity' => 'integer|min:1',
        ]);

        $isAvailable = $this->inventoryService->isAvailable(
            (int) $request->input('field_id'),
            $request->input('product_id'),
            (int) $request->input('quantity', 1)
        );

        $available = $this->inventoryService->getAvailableStock(
            (int) $request->input('field_id'),
            $request->input('product_id')
        );

        return response()->json([
            'success' => true,
            'available' => $isAvailable,
            'quantity_available' => $available,
        ]);
    }

    /**
     * Validate cart items
     */
    public function validateCart(Request $request): JsonResponse
    {
        $request->validate([
            'items' => 'required|array',
            'items.*.field_id' => 'required|integer',
            'items.*.product_id' => 'required|string',
            'items.*.quantity' => 'required|integer|min:1',
        ]);

        $result = $this->inventoryService->validateCart($request->input('items'));

        return response()->json([
            'success' => $result['valid'],
            'data' => $result,
        ]);
    }

    /**
     * Get low stock products for a form
     */
    public function lowStock(int $formId): JsonResponse
    {
        $products = $this->inventoryService->getLowStockProducts($formId);

        return response()->json([
            'success' => true,
            'data' => $products,
            'count' => $products->count(),
        ]);
    }

    /**
     * Get inventory report for a form
     */
    public function report(int $formId): JsonResponse
    {
        $report = $this->inventoryService->getInventoryReport($formId);

        return response()->json([
            'success' => true,
            'data' => $report,
        ]);
    }

    /**
     * Bulk update inventory
     */
    public function bulkUpdate(Request $request): JsonResponse
    {
        $request->validate([
            'updates' => 'required|array',
            'updates.*.field_id' => 'required|integer',
            'updates.*.product_id' => 'required|string',
            'updates.*.stock' => 'required|integer|min:0',
        ]);

        $result = $this->inventoryService->bulkUpdateInventory($request->input('updates'));

        return response()->json([
            'success' => $result['failed'] === 0,
            'data' => $result,
        ]);
    }
}
