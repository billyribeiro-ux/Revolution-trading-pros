<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Carbon\Carbon;

/**
 * Smart Link Click Model (Tracking clicks on smart links)
 *
 * @property string $id
 * @property string $smart_link_id
 * @property string|null $contact_id
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property string|null $referrer
 * @property string|null $country
 * @property string|null $city
 * @property string|null $device
 * @property string|null $browser
 * @property string|null $os
 * @property array|null $utm_params
 * @property Carbon $clicked_at
 */
class SmartLinkClick extends Model
{
    use HasUuids;

    public $timestamps = false;

    protected $fillable = [
        'smart_link_id',
        'contact_id',
        'ip_address',
        'user_agent',
        'referrer',
        'country',
        'city',
        'device',
        'browser',
        'os',
        'utm_params',
        'clicked_at',
    ];

    protected function casts(): array
    {
        return [
            'utm_params' => 'array',
            'clicked_at' => 'datetime',
        ];
    }

    // Relationships
    public function smartLink(): BelongsTo
    {
        return $this->belongsTo(SmartLink::class);
    }

    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }
}
