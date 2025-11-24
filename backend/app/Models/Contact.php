<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;

class Contact extends Model
{
    use HasUuids;

    protected $fillable = [
        'user_id', 'email', 'first_name', 'last_name', 'phone', 'mobile',
        'company_id', 'job_title', 'department',
        'address_line1', 'address_line2', 'city', 'state', 'postal_code', 'country', 'timezone',
        'website', 'linkedin_url', 'twitter_handle',
        'source', 'source_details', 'owner_id', 'status', 'lifecycle_stage',
        'lead_score', 'health_score', 'engagement_score', 'value_score',
        'is_verified', 'is_unsubscribed', 'do_not_contact', 'is_vip',
        'custom_fields', 'tags',
        'first_touch_channel', 'first_touch_campaign', 'last_touch_channel', 'last_touch_campaign',
        'total_sessions', 'last_session_at', 'avg_engagement_score', 'avg_intent_score', 'friction_events_count',
        'email_opens', 'email_clicks', 'last_email_opened_at', 'last_email_clicked_at',
        'subscription_status', 'subscription_plan_id', 'subscription_mrr', 'lifetime_value',
        'last_activity_at', 'last_contacted_at', 'next_followup_at',
        'activities_count', 'notes_count', 'tasks_count', 'deals_count',
        'last_seen_at', 'converted_at'
    ];

    protected $casts = [
        'is_verified' => 'boolean',
        'is_unsubscribed' => 'boolean',
        'do_not_contact' => 'boolean',
        'is_vip' => 'boolean',
        'custom_fields' => 'array',
        'tags' => 'array',
        'last_session_at' => 'datetime',
        'last_email_opened_at' => 'datetime',
        'last_email_clicked_at' => 'datetime',
        'last_activity_at' => 'datetime',
        'last_contacted_at' => 'datetime',
        'next_followup_at' => 'datetime',
        'last_seen_at' => 'datetime',
        'converted_at' => 'datetime',
    ];

    protected $appends = ['full_name'];

    public function getFullNameAttribute(): string
    {
        return trim("{$this->first_name} {$this->last_name}");
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public function owner(): BelongsTo
    {
        return $this->belongsTo(User::class, 'owner_id');
    }

    public function deals(): HasMany
    {
        return $this->hasMany(Deal::class);
    }

    public function activities(): HasMany
    {
        return $this->hasMany(CrmActivity::class, 'subject_id')
            ->where('subject_type', 'contact');
    }

    public function notes(): HasMany
    {
        return $this->hasMany(CrmNote::class);
    }

    public function segments(): BelongsToMany
    {
        return $this->belongsToMany(ContactSegment::class, 'contact_segment_members');
    }

    public function scoreHistory(): HasMany
    {
        return $this->hasMany(LeadScoreLog::class);
    }
}
