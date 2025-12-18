<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use Spatie\Permission\Models\Role;
use Spatie\Permission\Models\Permission;
use App\Models\User;

/**
 * Seeder for roles and permissions
 * 
 * Run with: php artisan db:seed --class=RolesAndPermissionsSeeder
 */
class RolesAndPermissionsSeeder extends Seeder
{
    /**
     * Superadmin emails from config
     */
    private function getSuperadminEmails(): array
    {
        return config('superadmin.emails', ['welberribeirodrums@gmail.com']);
    }

    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Reset cached roles and permissions
        app()[\Spatie\Permission\PermissionRegistrar::class]->forgetCachedPermissions();

        // Create permissions
        $permissions = [
            // Dashboard
            'dashboard.view',
            'dashboard.analytics',
            
            // Users
            'users.view',
            'users.create',
            'users.edit',
            'users.delete',
            'users.impersonate',
            
            // Content
            'posts.view',
            'posts.create',
            'posts.edit',
            'posts.delete',
            'posts.publish',
            
            // Products
            'products.view',
            'products.create',
            'products.edit',
            'products.delete',
            
            // Orders
            'orders.view',
            'orders.manage',
            'orders.refund',
            
            // Coupons
            'coupons.view',
            'coupons.create',
            'coupons.edit',
            'coupons.delete',
            
            // Popups
            'popups.view',
            'popups.create',
            'popups.edit',
            'popups.delete',
            
            // Forms
            'forms.view',
            'forms.create',
            'forms.edit',
            'forms.delete',
            'forms.submissions',
            
            // Settings
            'settings.view',
            'settings.edit',
            'settings.system',
            
            // Memberships
            'memberships.view',
            'memberships.manage',
            
            // Trading Rooms
            'trading_rooms.access',
            'trading_rooms.moderate',
            'trading_rooms.admin',
            
            // Media
            'media.view',
            'media.upload',
            'media.delete',
            
            // System
            'system.logs',
            'system.cache',
            'system.maintenance',
        ];

        foreach ($permissions as $permission) {
            Permission::firstOrCreate(['name' => $permission, 'guard_name' => 'web']);
        }

        $this->command->info('Created ' . count($permissions) . ' permissions');

        // Create roles
        $userRole = Role::firstOrCreate(['name' => 'user', 'guard_name' => 'web']);
        $memberRole = Role::firstOrCreate(['name' => 'member', 'guard_name' => 'web']);
        $adminRole = Role::firstOrCreate(['name' => 'admin', 'guard_name' => 'web']);
        $superadminRole = Role::firstOrCreate(['name' => 'super-admin', 'guard_name' => 'web']);

        $this->command->info('Created roles: user, member, admin, super-admin');

        // Assign permissions to roles
        $userRole->syncPermissions([
            'dashboard.view',
        ]);

        $memberRole->syncPermissions([
            'dashboard.view',
            'trading_rooms.access',
            'media.view',
        ]);

        $adminRole->syncPermissions([
            'dashboard.view',
            'dashboard.analytics',
            'users.view',
            'users.create',
            'users.edit',
            'posts.view',
            'posts.create',
            'posts.edit',
            'posts.delete',
            'posts.publish',
            'products.view',
            'products.create',
            'products.edit',
            'orders.view',
            'orders.manage',
            'coupons.view',
            'coupons.create',
            'coupons.edit',
            'popups.view',
            'popups.create',
            'popups.edit',
            'forms.view',
            'forms.create',
            'forms.edit',
            'forms.submissions',
            'settings.view',
            'memberships.view',
            'memberships.manage',
            'trading_rooms.access',
            'trading_rooms.moderate',
            'media.view',
            'media.upload',
        ]);

        // Superadmin gets ALL permissions
        $superadminRole->syncPermissions(Permission::all());

        $this->command->info('Assigned permissions to roles');

        // Assign super-admin role to superadmin emails
        foreach ($this->getSuperadminEmails() as $email) {
            $user = User::where('email', strtolower($email))->first();
            if ($user) {
                $user->syncRoles(['super-admin']);
                $this->command->info("Assigned super-admin role to: {$email}");
            } else {
                $this->command->warn("User not found for superadmin email: {$email}");
            }
        }

        $this->command->info('Roles and permissions seeding completed!');
    }
}
