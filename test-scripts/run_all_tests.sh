#!/bin/bash
# run_all_tests.sh - Master Test Runner for Nuclear CMS Testing Protocol
#
# Usage:
#   BASE_URL=http://localhost:8000/api ./run_all_tests.sh
#
# With Authentication:
#   BASE_URL=http://localhost:8000/api \
#   AUTH_TOKEN=xxx \
#   USER_TOKEN=xxx \
#   ADMIN_TOKEN=xxx \
#   ./run_all_tests.sh

set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="${SCRIPT_DIR}/../test-results/${TIMESTAMP}"

# Create results directory
mkdir -p "$RESULTS_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║        NUCLEAR CMS TESTING PROTOCOL - REVOLUTION TRADING         ║"
echo "║                   Principal Engineer Level (L8+)                 ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

echo "Base URL: ${BASE_URL:-http://localhost:8000/api}"
echo "Results Directory: ${RESULTS_DIR}"
echo "Timestamp: ${TIMESTAMP}"
echo ""

# Verify server is reachable
echo "Checking server availability..."
HEALTH_CHECK=$(curl -s -o /dev/null -w "%{http_code}" "${BASE_URL:-http://localhost:8000/api}/health/live" 2>/dev/null)

if [ "$HEALTH_CHECK" = "200" ]; then
  echo -e "${GREEN}✓ Server is reachable${NC}"
else
  echo -e "${YELLOW}⚠ Server may not be running (health check returned: $HEALTH_CHECK)${NC}"
  echo "  Some tests may fail. Start the backend with:"
  echo "  cd backend && php artisan serve"
  echo ""
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo ""

# Track overall results
TOTAL_PASSED=0
TOTAL_FAILED=0
TOTAL_SKIPPED=0

run_test_suite() {
  local suite_name=$1
  local script_path=$2

  if [ ! -f "$script_path" ]; then
    echo -e "${YELLOW}⏭ Skipping ${suite_name}: Script not found${NC}"
    return
  fi

  echo -e "${BLUE}▶ Running: ${suite_name}${NC}"
  echo "  Output: ${RESULTS_DIR}/${suite_name}.log"

  # Make script executable
  chmod +x "$script_path"

  # Run the test and capture output
  bash "$script_path" > "${RESULTS_DIR}/${suite_name}.log" 2>&1
  EXIT_CODE=$?

  # Parse results from log
  SUITE_PASSED=$(grep -c "✓ PASS" "${RESULTS_DIR}/${suite_name}.log" 2>/dev/null || echo "0")
  SUITE_FAILED=$(grep -c "✗ FAIL" "${RESULTS_DIR}/${suite_name}.log" 2>/dev/null || echo "0")
  SUITE_SKIPPED=$(grep -c "⏭ SKIP" "${RESULTS_DIR}/${suite_name}.log" 2>/dev/null || echo "0")

  TOTAL_PASSED=$((TOTAL_PASSED + SUITE_PASSED))
  TOTAL_FAILED=$((TOTAL_FAILED + SUITE_FAILED))
  TOTAL_SKIPPED=$((TOTAL_SKIPPED + SUITE_SKIPPED))

  if [ "$EXIT_CODE" -eq 0 ]; then
    echo -e "${GREEN}  ✓ ${suite_name}: ${SUITE_PASSED} passed, ${SUITE_FAILED} failed, ${SUITE_SKIPPED} skipped${NC}"
  else
    echo -e "${RED}  ✗ ${suite_name}: ${SUITE_PASSED} passed, ${SUITE_FAILED} failed, ${SUITE_SKIPPED} skipped${NC}"
  fi
  echo ""
}

# Run all test suites
run_test_suite "AUTH_TESTS" "${SCRIPT_DIR}/AUTH_TESTS.sh"
run_test_suite "CRUD_TESTS" "${SCRIPT_DIR}/CRUD_TESTS.sh"
run_test_suite "SECURITY_TESTS" "${SCRIPT_DIR}/SECURITY_TESTS.sh"
run_test_suite "RBAC_TESTS" "${SCRIPT_DIR}/RBAC_TESTS.sh"

# Generate summary report
echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo ""

# Calculate pass rate
if [ $((TOTAL_PASSED + TOTAL_FAILED)) -gt 0 ]; then
  PASS_RATE=$(echo "scale=1; ($TOTAL_PASSED * 100) / ($TOTAL_PASSED + $TOTAL_FAILED)" | bc)
else
  PASS_RATE="N/A"
fi

# Determine overall status
if [ "$TOTAL_FAILED" -eq 0 ]; then
  STATUS_COLOR="${GREEN}"
  STATUS_TEXT="ALL TESTS PASSED"
elif [ "$TOTAL_FAILED" -lt 5 ]; then
  STATUS_COLOR="${YELLOW}"
  STATUS_TEXT="TESTS COMPLETED WITH WARNINGS"
else
  STATUS_COLOR="${RED}"
  STATUS_TEXT="TESTS FAILED"
fi

echo -e "${STATUS_COLOR}"
echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                      TEST EXECUTION SUMMARY                      ║"
echo "╠══════════════════════════════════════════════════════════════════╣"
printf "║  %-60s  ║\n" "Status: ${STATUS_TEXT}"
printf "║  %-60s  ║\n" "Passed: ${TOTAL_PASSED}"
printf "║  %-60s  ║\n" "Failed: ${TOTAL_FAILED}"
printf "║  %-60s  ║\n" "Skipped: ${TOTAL_SKIPPED}"
printf "║  %-60s  ║\n" "Pass Rate: ${PASS_RATE}%"
printf "║  %-60s  ║\n" ""
printf "║  %-60s  ║\n" "Results: ${RESULTS_DIR}"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Generate JSON summary
cat > "${RESULTS_DIR}/summary.json" << EOF
{
  "timestamp": "${TIMESTAMP}",
  "base_url": "${BASE_URL:-http://localhost:8000/api}",
  "results": {
    "passed": ${TOTAL_PASSED},
    "failed": ${TOTAL_FAILED},
    "skipped": ${TOTAL_SKIPPED},
    "pass_rate": "${PASS_RATE}%"
  },
  "status": "${STATUS_TEXT}",
  "test_suites": [
    "AUTH_TESTS",
    "CRUD_TESTS",
    "SECURITY_TESTS",
    "RBAC_TESTS"
  ]
}
EOF

echo ""
echo "Summary JSON: ${RESULTS_DIR}/summary.json"
echo ""

# Exit with failure count
exit $TOTAL_FAILED
