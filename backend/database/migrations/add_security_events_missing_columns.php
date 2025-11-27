<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

/**
 * Add Missing Columns to Security Events Table
 * 
 * Google L11 Enterprise Standard - Complete schema alignment
 * Adds all columns required by the SecurityEvent model
 */
return new class extends Migration
{
    public function up(): void
    {
        // Add missing columns
        if (!Schema::hasColumn('security_events', 'session_id')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->string('session_id')->nullable()->after('metadata'));
        }
        
        if (!Schema::hasColumn('security_events', 'device_fingerprint')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->string('device_fingerprint')->nullable()->after('session_id'));
        }
        
        if (!Schema::hasColumn('security_events', 'is_suspicious')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->boolean('is_suspicious')->default(false)->after('device_fingerprint'));
        }
        
        if (!Schema::hasColumn('security_events', 'is_blocked')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->boolean('is_blocked')->default(false)->after('is_suspicious'));
        }
        
        if (!Schema::hasColumn('security_events', 'blocked_reason')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->string('blocked_reason')->nullable()->after('is_blocked'));
        }
        
        if (!Schema::hasColumn('security_events', 'threat_level')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->string('threat_level')->default('none')->after('blocked_reason'));
        }
        
        if (!Schema::hasColumn('security_events', 'risk_score')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->float('risk_score')->default(0)->after('threat_level'));
        }
        
        if (!Schema::hasColumn('security_events', 'action_taken')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->string('action_taken')->default('none')->after('risk_score'));
        }
        
        if (!Schema::hasColumn('security_events', 'related_event_id')) {
            Schema::table('security_events', fn(Blueprint $t) => 
                $t->unsignedBigInteger('related_event_id')->nullable()->after('action_taken'));
        }

        // Add indexes (ignore if they exist)
        $this->addIndexSafely('security_events', 'type', 'idx_security_events_type');
        $this->addIndexSafely('security_events', 'severity', 'idx_security_events_severity');
        $this->addIndexSafely('security_events', 'user_id', 'idx_security_events_user_id');
        $this->addIndexSafely('security_events', 'ip_address', 'idx_security_events_ip');
        $this->addIndexSafely('security_events', 'is_suspicious', 'idx_security_events_suspicious');
        $this->addIndexSafely('security_events', 'created_at', 'idx_security_events_created');
    }

    private function addIndexSafely(string $table, string $column, string $indexName): void
    {
        try {
            Schema::table($table, fn(Blueprint $t) => $t->index($column, $indexName));
        } catch (\Exception $e) {
            // Index already exists, ignore
        }
    }

    public function down(): void
    {
        $columns = [
            'session_id',
            'device_fingerprint', 
            'is_suspicious',
            'is_blocked',
            'blocked_reason',
            'threat_level',
            'risk_score',
            'action_taken',
            'related_event_id',
        ];

        foreach ($columns as $column) {
            if (Schema::hasColumn('security_events', $column)) {
                Schema::table('security_events', fn(Blueprint $t) => $t->dropColumn($column));
            }
        }
    }
};
