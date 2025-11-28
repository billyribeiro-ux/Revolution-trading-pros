<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\Request;

class MeController extends Controller
{
    public function show(Request $request)
    {
        $user = $request->user();

        // Use Spatie HasRoles to expose roles and admin flag
        $roles = method_exists($user, 'getRoleNames') ? $user->getRoleNames()->toArray() : [];
        
        // Check if user is superadmin by email (from config)
        $superadminEmails = config('superadmin.emails', []);
        $isSuperadmin = in_array(strtolower($user->email), array_map('strtolower', $superadminEmails));
        
        // Add super-admin role if superadmin by email
        if ($isSuperadmin && !in_array('super-admin', $roles)) {
            $roles[] = 'super-admin';
        }

        // Check admin status
        $isAdmin = $isSuperadmin || (method_exists($user, 'hasAnyRole')
            ? $user->hasAnyRole(['admin', 'super-admin', 'super_admin', 'administrator'])
            : false);

        // Get all permissions for the user
        $permissions = [];
        if (method_exists($user, 'getAllPermissions')) {
            $permissions = $user->getAllPermissions()->pluck('name')->toArray();
        }
        
        // Superadmin gets all permissions
        if ($isSuperadmin) {
            $permissions = ['*']; // Wildcard for all permissions
        }

        return response()->json([
            'id' => $user->id,
            'name' => $user->name,
            'first_name' => $user->first_name ?? null,
            'last_name' => $user->last_name ?? null,
            'email' => $user->email,
            'email_verified_at' => $user->email_verified_at,
            'created_at' => $user->created_at,
            'updated_at' => $user->updated_at,
            'must_change_password' => (bool) ($user->must_change_password ?? false),
            'roles' => $roles,
            'permissions' => $permissions,
            'is_admin' => $isAdmin,
            'is_superadmin' => $isSuperadmin,
        ]);
    }

    public function memberships(Request $request)
    {
        $memberships = $request->user()
            ->memberships()
            ->with('plan')
            ->where('status', 'active')
            ->get();

        return response()->json($memberships);
    }

    public function products(Request $request)
    {
        $products = $request->user()
            ->products()
            ->get();

        return response()->json($products);
    }

    public function indicators(Request $request)
    {
        $indicators = $request->user()
            ->products()
            ->where('type', 'indicator')
            ->get();

        return response()->json($indicators);
    }
}
