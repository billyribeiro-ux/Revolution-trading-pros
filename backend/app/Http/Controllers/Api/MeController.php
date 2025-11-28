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
        $roles = method_exists($user, 'getRoleNames') ? $user->getRoleNames() : collect();

        return response()->json([
            'id' => $user->id,
            'name' => $user->name,
            'email' => $user->email,
            'email_verified_at' => $user->email_verified_at,
            'created_at' => $user->created_at,
            'updated_at' => $user->updated_at,
            'must_change_password' => (bool) ($user->must_change_password ?? false),
            'roles' => $roles,
            'is_admin' => method_exists($user, 'hasAnyRole')
                ? $user->hasAnyRole(['admin', 'super-admin', 'super_admin'])
                : false,
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
