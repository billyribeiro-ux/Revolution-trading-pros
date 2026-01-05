#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# Run Schedule System Database Migration
# ═══════════════════════════════════════════════════════════════════════════

set -e

echo "═══════════════════════════════════════════════════════════════════════════"
echo "Schedule System Database Migration"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ ERROR: DATABASE_URL environment variable is not set"
    echo ""
    echo "Please set it first:"
    echo "  export DATABASE_URL='postgresql://your-real-credentials'"
    echo ""
    echo "Or run with:"
    echo "  DATABASE_URL='your-connection-string' ./run-migration.sh"
    exit 1
fi

echo "✓ DATABASE_URL found"
echo ""

# Check if psql is available
if ! command -v psql &> /dev/null; then
    echo "❌ ERROR: psql is not installed"
    echo ""
    echo "Install PostgreSQL client:"
    echo "  macOS: brew install postgresql"
    echo "  Ubuntu: sudo apt-get install postgresql-client"
    echo ""
    exit 1
fi

echo "✓ psql found"
echo ""

# Run the migration
echo "Running migration: 013_trading_room_schedules.sql"
echo ""

psql "$DATABASE_URL" -f api/migrations/013_trading_room_schedules.sql

if [ $? -eq 0 ]; then
    echo ""
    echo "═══════════════════════════════════════════════════════════════════════════"
    echo "✅ MIGRATION SUCCESSFUL!"
    echo "═══════════════════════════════════════════════════════════════════════════"
    echo ""
    echo "Created tables:"
    echo "  - trading_room_schedules"
    echo "  - schedule_exceptions"
    echo ""
    echo "Seeded data:"
    echo "  - 6 events for Day Trading Room"
    echo "  - 3 events for Swing Trading Room"
    echo "  - 3 events for Small Account Mentorship"
    echo ""
    echo "Your API should now work! Test it:"
    echo "  curl https://your-api-url.com/api/schedules/day-trading-room/upcoming?days=7"
    echo ""
else
    echo ""
    echo "═══════════════════════════════════════════════════════════════════════════"
    echo "❌ MIGRATION FAILED"
    echo "═══════════════════════════════════════════════════════════════════════════"
    exit 1
fi
