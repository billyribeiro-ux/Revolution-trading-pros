<?php

namespace App\Traits;

use Illuminate\Support\Facades\Auth;

/**
 * Trait for checking superadmin status
 * 
 * Use this trait in controllers, middleware, or policies to check
 * if the current user is a superadmin with full access.
 */
trait ChecksSuperadmin
{
    /**
     * Check if a user is a superadmin by email
     */
    protected function isSuperadmin($user = null): bool
    {
        $user = $user ?? Auth::user();
        
        if (!$user) {
            return false;
        }

        $superadminEmails = config('superadmin.emails', []);
        return in_array(strtolower($user->email), array_map('strtolower', $superadminEmails));
    }

    /**
     * Check if a user is an admin (superadmin or regular admin)
     */
    protected function isAdmin($user = null): bool
    {
        $user = $user ?? Auth::user();
        
        if (!$user) {
            return false;
        }

        // Superadmin is always admin
        if ($this->isSuperadmin($user)) {
            return true;
        }

        // Check is_admin flag
        if ($user->is_admin ?? false) {
            return true;
        }

        // Check roles
        if (method_exists($user, 'hasAnyRole')) {
            return $user->hasAnyRole(['admin', 'super-admin', 'super_admin', 'administrator']);
        }

        return false;
    }

    /**
     * Check if user has a specific permission
     * Superadmin always returns true
     */
    protected function userHasPermission($permission, $user = null): bool
    {
        $user = $user ?? Auth::user();
        
        if (!$user) {
            return false;
        }

        // Superadmin has all permissions
        if ($this->isSuperadmin($user)) {
            return true;
        }

        // Check Spatie permissions
        if (method_exists($user, 'hasPermissionTo')) {
            return $user->hasPermissionTo($permission);
        }

        return false;
    }

    /**
     * Check if user has any of the specified permissions
     */
    protected function userHasAnyPermission(array $permissions, $user = null): bool
    {
        $user = $user ?? Auth::user();
        
        if (!$user) {
            return false;
        }

        // Superadmin has all permissions
        if ($this->isSuperadmin($user)) {
            return true;
        }

        foreach ($permissions as $permission) {
            if ($this->userHasPermission($permission, $user)) {
                return true;
            }
        }

        return false;
    }

    /**
     * Check if user has all of the specified permissions
     */
    protected function userHasAllPermissions(array $permissions, $user = null): bool
    {
        $user = $user ?? Auth::user();
        
        if (!$user) {
            return false;
        }

        // Superadmin has all permissions
        if ($this->isSuperadmin($user)) {
            return true;
        }

        foreach ($permissions as $permission) {
            if (!$this->userHasPermission($permission, $user)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Abort if user is not a superadmin
     */
    protected function requireSuperadmin($user = null): void
    {
        if (!$this->isSuperadmin($user)) {
            abort(403, 'This action requires superadmin privileges.');
        }
    }

    /**
     * Abort if user is not an admin
     */
    protected function requireAdmin($user = null): void
    {
        if (!$this->isAdmin($user)) {
            abort(403, 'This action requires admin privileges.');
        }
    }

    /**
     * Abort if user doesn't have the required permission
     */
    protected function requirePermission(string $permission, $user = null): void
    {
        if (!$this->userHasPermission($permission, $user)) {
            abort(403, "This action requires the '{$permission}' permission.");
        }
    }
}
