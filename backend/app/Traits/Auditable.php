<?php

declare(strict_types=1);

namespace App\Traits;

/**
 * Auditable Trait
 *
 * Track changes to model for audit logging.
 */
trait Auditable
{
    /**
     * Boot the trait.
     */
    protected static function bootAuditable(): void
    {
        static::created(function ($model) {
            $model->logAudit('created');
        });

        static::updated(function ($model) {
            $model->logAudit('updated', $model->getDirty());
        });

        static::deleted(function ($model) {
            $model->logAudit('deleted');
        });
    }

    /**
     * Log audit entry.
     */
    protected function logAudit(string $action, array $changes = []): void
    {
        // Audit logging can be implemented based on requirements
    }

    /**
     * Get audit log for this model.
     */
    public function getAuditLog(): array
    {
        return [];
    }
}
