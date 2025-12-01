<?php

namespace App\Models;

use App\Notifications\VerifyEmailNotification;
use Illuminate\Contracts\Auth\MustVerifyEmail;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Foundation\Auth\User as Authenticatable;
use Illuminate\Notifications\Notifiable;
use Laravel\Sanctum\HasApiTokens;
use Spatie\Permission\Traits\HasRoles;

class User extends Authenticatable implements MustVerifyEmail
{
    use HasApiTokens, HasFactory, HasRoles, Notifiable;

    protected $fillable = [
        'first_name',
        'last_name',
        'name',
        'email',
        'password',
        'must_change_password',
        'mfa_enabled',
        'mfa_secret',
        'mfa_backup_codes',
        'mfa_enabled_at',
        'stripe_customer_id',
    ];

    protected $hidden = [
        'password',
        'remember_token',
        'mfa_secret',
        'mfa_backup_codes',
    ];

    protected $casts = [
        'email_verified_at' => 'datetime',
        'password' => 'hashed',
        'must_change_password' => 'boolean',
        'mfa_enabled' => 'boolean',
        'mfa_backup_codes' => 'array',
        'mfa_enabled_at' => 'datetime',
    ];

    public function memberships()
    {
        return $this->hasMany(UserMembership::class);
    }

    public function products()
    {
        return $this->belongsToMany(Product::class, 'user_products')
            ->withPivot('purchased_at', 'order_id')
            ->withTimestamps();
    }

    public function orders()
    {
        return $this->hasMany(Order::class);
    }

    public function securityEvents()
    {
        return $this->hasMany(SecurityEvent::class);
    }

    /**
     * Get the user's sessions.
     */
    public function sessions()
    {
        return $this->hasMany(UserSession::class);
    }

    /**
     * Get the user's active sessions.
     */
    public function activeSessions()
    {
        return $this->sessions()
            ->where('is_active', true)
            ->where('expires_at', '>', now());
    }

    /**
     * Get the user's subscriptions.
     */
    public function subscriptions()
    {
        return $this->hasMany(UserSubscription::class);
    }

    /**
     * Get the user's active subscriptions.
     */
    public function activeSubscriptions()
    {
        return $this->subscriptions()->where('status', 'active');
    }

    /**
     * Check if user has an active subscription to a specific plan.
     */
    public function hasActiveSubscription(?int $planId = null): bool
    {
        $query = $this->subscriptions()->where('status', 'active');

        if ($planId) {
            $query->where('subscription_plan_id', $planId);
        }

        return $query->exists();
    }

    /**
     * Get the user's WebAuthn credentials (passkeys)
     */
    public function webAuthnCredentials()
    {
        return $this->hasMany(WebAuthnCredential::class);
    }

    /**
     * Check if user has WebAuthn credentials registered
     */
    public function hasWebAuthnCredentials(): bool
    {
        return $this->webAuthnCredentials()->exists();
    }

    /**
     * Get the user's integration credentials
     */
    public function integrationCredentials()
    {
        return $this->hasMany(IntegrationCredential::class);
    }

    /**
     * Send the email verification notification.
     * Overrides the default to use our branded template.
     */
    public function sendEmailVerificationNotification(): void
    {
        $this->notify(new VerifyEmailNotification());
    }

    /**
     * Get the user's expired/cancelled memberships (past members).
     */
    public function expiredMemberships()
    {
        return $this->memberships()
            ->whereIn('status', ['expired', 'cancelled'])
            ->orWhere('expires_at', '<', now());
    }

    /**
     * Check if user is a past member (had active membership that expired/cancelled).
     */
    public function isPastMember(): bool
    {
        return $this->expiredMemberships()->exists();
    }

    /**
     * Get the user's last active membership.
     */
    public function lastMembership()
    {
        return $this->memberships()
            ->orderBy('expires_at', 'desc')
            ->first();
    }
}