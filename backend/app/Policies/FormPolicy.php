<?php

declare(strict_types=1);

namespace App\Policies;

use App\Models\Form;
use App\Models\User;
use Illuminate\Auth\Access\HandlesAuthorization;

/**
 * Form Policy - Authorization rules for Form resource
 *
 * Prevents IDOR vulnerabilities by ensuring users can only
 * access/modify forms they own or have explicit permission to access.
 *
 * @version 1.0.0
 * @security Critical - IDOR Prevention
 */
class FormPolicy
{
    use HandlesAuthorization;

    /**
     * Determine whether the user can view any forms.
     */
    public function viewAny(User $user): bool
    {
        // Users can list their own forms (filtered in controller)
        return true;
    }

    /**
     * Determine whether the user can view the form.
     */
    public function view(User $user, Form $form): bool
    {
        // Admin can view any form
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        // Users can only view their own forms
        return $form->user_id === $user->id;
    }

    /**
     * Determine whether the user can create forms.
     */
    public function create(User $user): bool
    {
        // Check if user has reached form limit
        $limit = $user->subscription?->form_limit ?? config('forms.default_limit', 10);
        $currentCount = Form::where('user_id', $user->id)->count();

        return $currentCount < $limit;
    }

    /**
     * Determine whether the user can update the form.
     */
    public function update(User $user, Form $form): bool
    {
        // Admin can update any form
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        // Users can only update their own forms
        return $form->user_id === $user->id;
    }

    /**
     * Determine whether the user can delete the form.
     */
    public function delete(User $user, Form $form): bool
    {
        // Admin can delete any form
        if ($user->hasRole('admin') || $user->hasRole('super_admin')) {
            return true;
        }

        // Users can only delete their own forms
        return $form->user_id === $user->id;
    }

    /**
     * Determine whether the user can duplicate the form.
     */
    public function duplicate(User $user, Form $form): bool
    {
        // Must be able to view the form to duplicate it
        if (!$this->view($user, $form)) {
            return false;
        }

        // Must have quota to create a new form
        return $this->create($user);
    }

    /**
     * Determine whether the user can view form submissions.
     */
    public function viewSubmissions(User $user, Form $form): bool
    {
        return $this->view($user, $form);
    }

    /**
     * Determine whether the user can export form submissions.
     */
    public function exportSubmissions(User $user, Form $form): bool
    {
        return $this->view($user, $form);
    }

    /**
     * Determine whether the user can manage form integrations.
     */
    public function manageIntegrations(User $user, Form $form): bool
    {
        return $this->update($user, $form);
    }

    /**
     * Determine whether the user can restore the form.
     */
    public function restore(User $user, Form $form): bool
    {
        return $this->delete($user, $form);
    }

    /**
     * Determine whether the user can permanently delete the form.
     */
    public function forceDelete(User $user, Form $form): bool
    {
        // Only admins can permanently delete
        return $user->hasRole('admin') || $user->hasRole('super_admin');
    }
}
