<?php

/**
 * Superadmin Configuration
 * 
 * This file defines superadmin emails that have FULL unrestricted access
 * to all features and functions of the application.
 * 
 * SECURITY: Keep this list minimal and secure.
 */

return [
    /*
    |--------------------------------------------------------------------------
    | Superadmin Emails
    |--------------------------------------------------------------------------
    |
    | These email addresses have full superadmin access regardless of their
    | database roles. They bypass all permission checks.
    |
    */
    'emails' => [
        'welberribeirodrums@gmail.com',
    ],

    /*
    |--------------------------------------------------------------------------
    | Role Hierarchy
    |--------------------------------------------------------------------------
    |
    | Defines the role hierarchy from lowest to highest privilege.
    |
    */
    'role_hierarchy' => [
        'user',
        'member', 
        'admin',
        'super-admin',
    ],

    /*
    |--------------------------------------------------------------------------
    | Default Permissions by Role
    |--------------------------------------------------------------------------
    |
    | Default permissions assigned to each role. Superadmin has all permissions.
    |
    */
    'role_permissions' => [
        'user' => [
            'dashboard.view',
        ],
        'member' => [
            'dashboard.view',
            'trading_rooms.access',
            'media.view',
        ],
        'admin' => [
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
        ],
        'super-admin' => [
            '*', // All permissions
        ],
    ],
];
