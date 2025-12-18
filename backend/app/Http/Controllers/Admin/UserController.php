<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\User;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Str;

class UserController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index()
    {
        // Get superadmin emails from config
        $superadminEmails = config('superadmin.emails', []);
        
        // Get users with admin/super-admin roles OR superadmin emails
        $admins = User::where(function ($query) use ($superadminEmails) {
            // Users with admin roles
            $query->whereHas('roles', function ($q) {
                $q->whereIn('name', ['admin', 'super-admin']);
            });
            
            // OR users with superadmin emails
            if (!empty($superadminEmails)) {
                $query->orWhereIn('email', array_map('strtolower', $superadminEmails));
            }
        })
            ->with('roles')
            ->orderBy('created_at', 'desc')
            ->paginate(request('per_page', 20));

        return response()->json([
            'data' => $admins->items(),
            'total' => $admins->total(),
            'current_page' => $admins->currentPage(),
            'last_page' => $admins->lastPage(),
        ]);
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'first_name' => 'sometimes|string|max:100',
            'last_name' => 'sometimes|string|max:100',
            'email' => 'required|email|max:255|unique:users,email',
            'password' => 'required|string|min:8|confirmed',
            'roles' => 'sometimes|array',
            'roles.*' => 'string|exists:roles,name',
        ]);

        $user = User::create([
            'first_name' => $validated['first_name'] ?? '',
            'last_name' => $validated['last_name'] ?? '',
            'name' => $validated['name'] ?? trim(($validated['first_name'] ?? '').' '.($validated['last_name'] ?? '')) ?: $validated['email'],
            'email' => $validated['email'],
            'password' => Hash::make($validated['password']),
            'email_verified_at' => now(), // Auto-verify admin users
        ]);

        // Assign roles if provided
        if (isset($validated['roles']) && method_exists($user, 'assignRole')) {
            $user->assignRole($validated['roles']);
        } else if (method_exists($user, 'assignRole')) {
            // Default to admin role if no roles specified
            $user->assignRole('admin');
        }

        // Reload user with roles
        $user = $user->fresh(['roles']);

        return response()->json([
            'user' => $user,
            'message' => 'User created successfully',
        ], 201);
    }

    /**
     * Display the specified resource.
     */
    public function show(string $id)
    {
        $user = User::with('roles')->findOrFail($id);

        return response()->json($user);
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, string $id)
    {
        $user = User::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|string|max:255',
            'first_name' => 'sometimes|string|max:100',
            'last_name' => 'sometimes|string|max:100',
            'email' => 'sometimes|email|max:255|unique:users,email,'.$user->id,
            'password' => 'sometimes|nullable|string|min:8|confirmed',
            'roles' => 'sometimes|array',
            'roles.*' => 'string|exists:roles,name',
        ]);

        // Update basic fields
        $user->fill($validated);
        
        // Update name if first_name or last_name changed
        if (isset($validated['first_name']) || isset($validated['last_name'])) {
            $user->name = trim(($user->first_name ?? '').' '.($user->last_name ?? '')) ?: $user->name;
        }

        // Update password if provided
        if (!empty($validated['password'])) {
            $user->password = Hash::make($validated['password']);
        }

        $user->save();

        // Update roles if provided
        if (isset($validated['roles']) && method_exists($user, 'syncRoles')) {
            $user->syncRoles($validated['roles']);
        }

        // Reload user with roles
        $user = $user->fresh(['roles']);

        return response()->json([
            'message' => 'User updated successfully',
            'user' => $user,
        ]);
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(Request $request, string $id)
    {
        $user = User::findOrFail($id);

        // Prevent deleting yourself
        if ($request->user() && $request->user()->id === $user->id) {
            return response()->json([
                'message' => 'You cannot delete your own admin account.',
            ], 422);
        }

        $user->delete();

        return response()->json([
            'message' => 'Admin user deleted successfully',
        ]);
    }

    /**
     * Get admin user statistics
     */
    public function stats()
    {
        $totalAdmins = User::whereHas('roles', function ($q) {
            $q->whereIn('name', ['admin', 'super-admin']);
        })->count();

        $superAdmins = User::whereHas('roles', function ($q) {
            $q->where('name', 'super-admin');
        })->count();

        $regularAdmins = User::whereHas('roles', function ($q) {
            $q->where('name', 'admin');
        })->count();

        $newThisMonth = User::whereHas('roles', function ($q) {
            $q->whereIn('name', ['admin', 'super-admin']);
        })->where('created_at', '>=', now()->startOfMonth())->count();

        return response()->json([
            'total_admins' => $totalAdmins,
            'super_admins' => $superAdmins,
            'regular_admins' => $regularAdmins,
            'new_this_month' => $newThisMonth,
        ]);
    }
}
