<?php

namespace Database\Seeders;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use App\Models\User;
use Illuminate\Database\Seeder;

class DashboardSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Get first admin user
        $adminUser = User::where('role', 'admin')->orWhere('is_admin', true)->first();
        
        if (!$adminUser) {
            $this->command->warn('No admin user found. Skipping dashboard seeding.');
            return;
        }

        // Create admin dashboard
        $adminDashboard = Dashboard::create([
            'user_id' => $adminUser->id,
            'dashboard_type' => 'admin',
            'name' => 'Admin Dashboard',
            'description' => 'Main admin dashboard with system overview',
            'is_default' => true,
            'is_public' => false,
            'grid_columns' => 12,
            'grid_row_height' => 80,
            'grid_gap' => 16,
        ]);

        // Create admin widgets
        $adminWidgets = [
            [
                'widget_type' => 'system_health',
                'title' => 'System Health',
                'position_x' => 0,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['show_all_services' => true],
                'refresh_interval' => 60,
            ],
            [
                'widget_type' => 'revenue_mrr',
                'title' => 'Monthly Recurring Revenue',
                'position_x' => 6,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'user_growth',
                'title' => 'User Growth',
                'position_x' => 0,
                'position_y' => 4,
                'width' => 4,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'subscription_churn',
                'title' => 'Churn Rate',
                'position_x' => 4,
                'position_y' => 4,
                'width' => 4,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'recent_activity',
                'title' => 'Recent Activity',
                'position_x' => 8,
                'position_y' => 4,
                'width' => 4,
                'height' => 8,
                'config' => ['limit' => 20],
                'refresh_interval' => 60,
            ],
        ];

        foreach ($adminWidgets as $widgetData) {
            DashboardWidget::create(array_merge($widgetData, [
                'dashboard_id' => $adminDashboard->id,
                'is_visible' => true,
            ]));
        }

        // Create user dashboard template
        $userDashboard = Dashboard::create([
            'user_id' => null, // Template dashboard
            'dashboard_type' => 'user',
            'name' => 'Default User Dashboard',
            'description' => 'Default dashboard template for new users',
            'is_default' => true,
            'is_public' => true,
            'grid_columns' => 12,
            'grid_row_height' => 80,
            'grid_gap' => 16,
        ]);

        // Create user widgets
        $userWidgets = [
            [
                'widget_type' => 'subscription_status',
                'title' => 'My Subscription',
                'position_x' => 0,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => [],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'recent_courses',
                'title' => 'Recent Courses',
                'position_x' => 6,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['limit' => 5],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'trading_performance',
                'title' => 'Trading Performance',
                'position_x' => 0,
                'position_y' => 4,
                'width' => 8,
                'height' => 6,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
            ],
            [
                'widget_type' => 'notifications',
                'title' => 'Notifications',
                'position_x' => 8,
                'position_y' => 4,
                'width' => 4,
                'height' => 6,
                'config' => ['limit' => 10],
                'refresh_interval' => 60,
            ],
        ];

        foreach ($userWidgets as $widgetData) {
            DashboardWidget::create(array_merge($widgetData, [
                'dashboard_id' => $userDashboard->id,
                'is_visible' => true,
            ]));
        }

        $this->command->info('Dashboard seeding completed successfully!');
        $this->command->info("Admin Dashboard ID: {$adminDashboard->id}");
        $this->command->info("User Dashboard Template ID: {$userDashboard->id}");
    }
}
