<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Carbon\Carbon;

/**
 * Funnel Subscriber Model (Contacts in automation funnels)
 *
 * @property string $id
 * @property string $funnel_id
 * @property string $contact_id
 * @property string|null $current_action_id
 * @property string $status
 * @property Carbon $entered_at
 * @property Carbon|null $next_execution_at
 * @property Carbon|null $completed_at
 * @property array|null $execution_log
 * @property int $actions_completed
 */
class FunnelSubscriber extends Model
{
    use HasUuids;

    protected $fillable = [
        'funnel_id',
        'contact_id',
        'current_action_id',
        'status',
        'entered_at',
        'next_execution_at',
        'completed_at',
        'execution_log',
        'actions_completed',
    ];

    protected function casts(): array
    {
        return [
            'entered_at' => 'datetime',
            'next_execution_at' => 'datetime',
            'completed_at' => 'datetime',
            'execution_log' => 'array',
            'actions_completed' => 'integer',
        ];
    }

    protected static function booted(): void
    {
        static::creating(function (self $subscriber): void {
            $subscriber->status ??= 'active';
            $subscriber->entered_at ??= now();
            $subscriber->execution_log ??= [];
        });
    }

    // Relationships
    public function funnel(): BelongsTo
    {
        return $this->belongsTo(AutomationFunnel::class, 'funnel_id');
    }

    public function contact(): BelongsTo
    {
        return $this->belongsTo(Contact::class);
    }

    public function currentAction(): BelongsTo
    {
        return $this->belongsTo(FunnelAction::class, 'current_action_id');
    }

    // Scopes
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', 'active');
    }

    public function scopeWaiting(Builder $query): Builder
    {
        return $query->where('status', 'waiting');
    }

    public function scopeDueForExecution(Builder $query): Builder
    {
        return $query->whereIn('status', ['active', 'waiting'])
            ->whereNotNull('next_execution_at')
            ->where('next_execution_at', '<=', now());
    }

    // Business Logic
    public function executeCurrentAction(): array
    {
        $action = $this->currentAction;

        if (!$action) {
            $this->complete();
            return ['success' => true, 'message' => 'No action - funnel completed'];
        }

        $result = $action->execute($this->contact, $this);

        $this->logExecution($action, $result);

        if ($result['success']) {
            $this->increment('actions_completed');
            $this->advanceToNextAction($result['condition_result'] ?? null);
        } else {
            $this->update(['status' => 'failed']);
        }

        return $result;
    }

    public function advanceToNextAction(?bool $conditionResult = null): void
    {
        $currentAction = $this->currentAction;

        if (!$currentAction) {
            $this->complete();
            return;
        }

        $nextAction = $currentAction->getNextAction($conditionResult);

        if ($nextAction) {
            $this->update([
                'current_action_id' => $nextAction->id,
                'status' => 'active',
                'next_execution_at' => now()->addSeconds($nextAction->delay_seconds),
            ]);
        } else {
            $this->complete();
        }
    }

    public function complete(): void
    {
        $this->update([
            'status' => 'completed',
            'completed_at' => now(),
            'next_execution_at' => null,
        ]);

        $this->funnel->increment('completed_count');
        $this->logExecution(null, ['action' => 'funnel_completed']);
    }

    public function cancel(string $reason = 'manual'): void
    {
        $this->update([
            'status' => 'cancelled',
            'next_execution_at' => null,
        ]);

        $this->logExecution(null, ['action' => 'cancelled', 'reason' => $reason]);
    }

    public function pause(): void
    {
        $this->update(['status' => 'paused']);
        $this->logExecution(null, ['action' => 'paused']);
    }

    public function resume(): void
    {
        $this->update([
            'status' => 'active',
            'next_execution_at' => now(),
        ]);
        $this->logExecution(null, ['action' => 'resumed']);
    }

    public function retry(): void
    {
        if ($this->status !== 'failed') {
            return;
        }

        $this->update([
            'status' => 'active',
            'next_execution_at' => now(),
        ]);
        $this->logExecution(null, ['action' => 'retried']);
    }

    private function logExecution(?FunnelAction $action, array $result): void
    {
        $log = $this->execution_log ?? [];
        $log[] = [
            'action_id' => $action?->id,
            'action_type' => $action?->action_type,
            'result' => $result,
            'at' => now()->toIso8601String(),
        ];

        $this->update(['execution_log' => $log]);
    }

    public function getProgressPercentage(): float
    {
        $totalActions = $this->funnel->actions()->count();
        if ($totalActions === 0) {
            return 100;
        }

        return round(($this->actions_completed / $totalActions) * 100, 1);
    }

    public function getDurationInFunnel(): string
    {
        if ($this->completed_at) {
            return $this->entered_at->diffForHumans($this->completed_at, true);
        }

        return $this->entered_at->diffForHumans(now(), true);
    }
}
