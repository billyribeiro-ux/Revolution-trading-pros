<?php

declare(strict_types=1);

namespace App\Policies;

use App\Models\Popup;
use App\Models\User;
use Illuminate\Auth\Access\HandlesAuthorization;

/**
 * Popup Policy - Authorization rules for Popup resource
 *
 * Prevents IDOR vulnerabilities by ensuring users can only
 * access/modify popups they own or have explicit permission to access.
 *
 * @version 1.0.0
 * @security Critical - IDOR Prevention
 */
class PopupPolicy
{
    use HandlesAuthorization;

    /**
     * Determine whether the user can view any popups.
     */
    public function viewAny(User $user): bool
    {
        return true;
    }

    /**
     * Determine whether the user can view the popup.
     */
    public function view(User $user, Popup $popup): bool
    {
        // Admin can view any popup
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        // Users can only view their own popups
        return $popup->user_id === $user->id;
    }

    /**
     * Determine whether the user can create popups.
     */
    public function create(User $user): bool
    {
        // Check subscription limits
        $limit = $user->subscription?->popup_limit ?? config('popups.default_limit', 5);
        $currentCount = Popup::where('user_id', $user->id)->count();

        return $currentCount < $limit;
    }

    /**
     * Determine whether the user can update the popup.
     */
    public function update(User $user, Popup $popup): bool
    {
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        return $popup->user_id === $user->id;
    }

    /**
     * Determine whether the user can delete the popup.
     */
    public function delete(User $user, Popup $popup): bool
    {
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        return $popup->user_id === $user->id;
    }

    /**
     * Determine whether the user can duplicate the popup.
     */
    public function duplicate(User $user, Popup $popup): bool
    {
        if (!$this->view($user, $popup)) {
            return false;
        }

        return $this->create($user);
    }

    /**
     * Determine whether the user can view popup analytics.
     */
    public function viewAnalytics(User $user, Popup $popup): bool
    {
        return $this->view($user, $popup);
    }

    /**
     * Determine whether the user can toggle popup status.
     */
    public function toggleStatus(User $user, Popup $popup): bool
    {
        return $this->update($user, $popup);
    }

    /**
     * Determine whether the user can restore the popup.
     */
    public function restore(User $user, Popup $popup): bool
    {
        return $this->delete($user, $popup);
    }

    /**
     * Determine whether the user can permanently delete the popup.
     */
    public function forceDelete(User $user, Popup $popup): bool
    {
        return $user->hasRole('admin') || $user->hasRole('super_admin');
    }
}
