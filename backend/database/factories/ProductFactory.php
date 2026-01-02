<?php

namespace Database\Factories;

use App\Models\Product;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Str;

/**
 * Product Factory for Testing
 *
 * @extends Factory<Product>
 */
class ProductFactory extends Factory
{
    protected $model = Product::class;

    public function definition(): array
    {
        $name = $this->faker->words(3, true);

        return [
            'name' => ucwords($name),
            'slug' => Str::slug($name),
            'type' => $this->faker->randomElement(['digital', 'physical', 'service', 'subscription', 'indicator', 'course']),
            'description' => $this->faker->paragraph(),
            'price' => $this->faker->randomFloat(2, 9.99, 999.99),
            'sale_price' => null,
            'cost_price' => null,
            'sku' => strtoupper($this->faker->unique()->bothify('SKU-####-??')),
            'is_active' => true,
            'is_featured' => false,
            'is_taxable' => true,
            'is_digital' => true,
            'stock_quantity' => null,
            'meta' => null,
        ];
    }

    /**
     * Indicator product state
     */
    public function indicator(): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'indicator',
            'is_digital' => true,
        ]);
    }

    /**
     * Course product state
     */
    public function course(): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'course',
            'is_digital' => true,
        ]);
    }

    /**
     * Subscription product state
     */
    public function subscription(): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'subscription',
            'is_digital' => true,
        ]);
    }

    /**
     * Inactive product state
     */
    public function inactive(): static
    {
        return $this->state(fn (array $attributes) => [
            'is_active' => false,
        ]);
    }

    /**
     * On sale product state
     */
    public function onSale(): static
    {
        return $this->state(function (array $attributes) {
            $price = $attributes['price'] ?? 99.99;
            return [
                'sale_price' => $price * 0.8, // 20% off
            ];
        });
    }
}
