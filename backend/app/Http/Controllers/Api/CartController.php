<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\MembershipPlan;
use App\Models\Order;
use App\Models\Product;
use Illuminate\Http\Request;
use Illuminate\Support\Str;

class CartController extends Controller
{
    public function checkout(Request $request)
    {
        $request->validate([
            'items' => 'required|array',
            'items.*.type' => 'required|in:product,membership',
            'items.*.id' => 'required|integer',
            'items.*.quantity' => 'integer|min:1',
        ]);

        $user = $request->user();
        $items = $request->items;

        $subtotal = 0;
        $orderItems = [];

        foreach ($items as $item) {
            if ($item['type'] === 'product') {
                $product = Product::findOrFail($item['id']);
                $price = $product->price;
                $name = $product->name;
            } else {
                $plan = MembershipPlan::findOrFail($item['id']);
                $price = $plan->price;
                $name = $plan->name;
            }

            $quantity = $item['quantity'] ?? 1;
            $subtotal += $price * $quantity;

            $orderItems[] = [
                'item_type' => $item['type'],
                'item_id' => $item['id'],
                'name' => $name,
                'price' => $price,
                'quantity' => $quantity,
            ];
        }

        $order = Order::create([
            'user_id' => $user->id,
            'order_number' => 'ORD-'.strtoupper(Str::random(10)),
            'subtotal' => $subtotal,
            'tax' => 0,
            'total' => $subtotal,
            'status' => 'pending',
        ]);

        foreach ($orderItems as $item) {
            $order->items()->create($item);
        }

        // TODO: Integrate payment provider

        return response()->json([
            'order' => $order->load('items'),
            'message' => 'Order created. Proceed with payment.',
        ], 201);
    }
}
