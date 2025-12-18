<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class SubscriptionPayment extends Model
{
    use HasFactory;

    protected $fillable = [
        'user_subscription_id',
        'user_id',
        'amount',
        'status',
        'payment_method',
        'transaction_id',
        'paid_at',
        'failure_reason',
        'metadata',
    ];

    protected $casts = [
        'amount' => 'decimal:2',
        'paid_at' => 'datetime',
        'metadata' => 'array',
    ];

    public function subscription()
    {
        return $this->belongsTo(UserSubscription::class, 'user_subscription_id');
    }

    public function user()
    {
        return $this->belongsTo(User::class);
    }
}