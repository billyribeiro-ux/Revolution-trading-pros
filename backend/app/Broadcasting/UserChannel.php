<?php

declare(strict_types=1);

namespace App\Broadcasting;

use App\Models\User;

/**
 * UserChannel - Authorization for user-specific private channels
 *
 * Handles authorization for channels like:
 * - user.{userId} (notifications)
 * - user.{userId}.cart (cart updates)
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class UserChannel
{
    /**
     * Authenticate the user's access to the channel.
     */
    public function join(User $user, int $userId): bool|array
    {
        // User can only access their own channel
        if ($user->id === $userId) {
            return [
                'id' => $user->id,
                'name' => $user->name,
                'email' => $user->email,
            ];
        }

        return false;
    }
}
