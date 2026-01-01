<?php

declare(strict_types=1);

namespace App\Broadcasting;

use App\Models\User;

/**
 * AdminChannel - Authorization for admin-only channels
 *
 * Handles authorization for channels like:
 * - admin.notifications
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class AdminChannel
{
    /**
     * Authenticate the user's access to the admin channel.
     */
    public function join(User $user): bool|array
    {
        // Only admin users can access admin channels
        if ($user->hasRole(['admin', 'super-admin'])) {
            return [
                'id' => $user->id,
                'name' => $user->name,
                'role' => $user->roles->first()?->name ?? 'admin',
            ];
        }

        return false;
    }
}
