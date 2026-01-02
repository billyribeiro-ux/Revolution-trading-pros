<?php

namespace Database\Factories;

use App\Models\Coupon;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Str;

/**
 * Coupon Factory for Testing
 *
 * @extends Factory<Coupon>
 */
class CouponFactory extends Factory
{
    protected $model = Coupon::class;

    public function definition(): array
    {
        return [
            'code' => strtoupper($this->faker->unique()->bothify('????####')),
            'type' => $this->faker->randomElement(['percentage', 'fixed']),
            'value' => $this->faker->randomFloat(2, 5, 50),
            'description' => $this->faker->sentence(),
            'is_active' => true,
            'max_uses' => $this->faker->optional()->numberBetween(10, 1000),
            'uses_count' => 0,
            'max_discount' => null,
            'min_purchase' => null,
            'starts_at' => null,
            'expires_at' => null,
        ];
    }

    /**
     * 100% discount coupon
     */
    public function fullDiscount(): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'percentage',
            'value' => 100,
        ]);
    }

    /**
     * Percentage discount coupon
     */
    public function percentage(float $percent = 20): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'percentage',
            'value' => $percent,
        ]);
    }

    /**
     * Fixed amount discount coupon
     */
    public function fixed(float $amount = 10): static
    {
        return $this->state(fn (array $attributes) => [
            'type' => 'fixed',
            'value' => $amount,
        ]);
    }

    /**
     * Expired coupon
     */
    public function expired(): static
    {
        return $this->state(fn (array $attributes) => [
            'expires_at' => now()->subDay(),
        ]);
    }

    /**
     * Not yet started coupon
     */
    public function future(): static
    {
        return $this->state(fn (array $attributes) => [
            'starts_at' => now()->addDay(),
        ]);
    }

    /**
     * Inactive coupon
     */
    public function inactive(): static
    {
        return $this->state(fn (array $attributes) => [
            'is_active' => false,
        ]);
    }

    /**
     * Limited use coupon
     */
    public function limitedUse(int $maxUses = 1): static
    {
        return $this->state(fn (array $attributes) => [
            'max_uses' => $maxUses,
            'uses_count' => 0,
        ]);
    }

    /**
     * Exhausted coupon (max uses reached)
     */
    public function exhausted(): static
    {
        return $this->state(fn (array $attributes) => [
            'max_uses' => 1,
            'uses_count' => 1,
        ]);
    }
}
