<?php

namespace Database\Factories;

use App\Enums\SubscriptionInterval;
use App\Models\SubscriptionPlan;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Str;

class SubscriptionPlanFactory extends Factory
{
    protected $model = SubscriptionPlan::class;

    public function definition(): array
    {
        $name = $this->faker->words(3, true);
        return [
            'name' => $name,
            'slug' => Str::slug($name),
            'description' => $this->faker->sentence,
            'price' => $this->faker->randomFloat(2, 10, 100),
            'billing_period' => SubscriptionInterval::Monthly,
            'billing_interval' => 1,
            'trial_days' => 14,
            'signup_fee' => 0,
            'max_users' => 1,
            'features' => ['feature1', 'feature2'],
            'is_active' => true,
            'is_featured' => false,
            'sort_order' => 0,
        ];
    }
}
