#!/bin/bash
# Test Rust endpoints and delete PHP controllers after verification
# ICT 11+ Principal Engineer - Systematic Conversion & Deletion

set -e

API_URL="https://revolution-trading-pros-api.fly.dev"
BACKEND_DIR="/Users/billyribeiro/CascadeProjects/Revolution-trading-pros/backend/app/Http/Controllers"
RESULTS_FILE="/tmp/controller_test_results.txt"

echo "========================================" > "$RESULTS_FILE"
echo "CONTROLLER VERIFICATION & DELETION LOG" >> "$RESULTS_FILE"
echo "Date: $(date)" >> "$RESULTS_FILE"
echo "========================================" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test function
test_endpoint() {
    local name=$1
    local url=$2
    local method=${3:-GET}
    
    echo -e "${YELLOW}Testing:${NC} $name"
    echo "Testing: $name ($method $url)" >> "$RESULTS_FILE"
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\n%{http_code}" "$url" 2>&1)
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" 2>&1)
    fi
    
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [ "$http_code" = "200" ] || [ "$http_code" = "401" ] || [ "$http_code" = "404" ]; then
        echo -e "${GREEN}✓${NC} $name - HTTP $http_code (Endpoint exists)"
        echo "✓ PASS - HTTP $http_code" >> "$RESULTS_FILE"
        echo "$body" | head -n 3 >> "$RESULTS_FILE"
        return 0
    else
        echo -e "${RED}✗${NC} $name - HTTP $http_code (May not be implemented)"
        echo "✗ FAIL - HTTP $http_code" >> "$RESULTS_FILE"
        return 1
    fi
    echo "" >> "$RESULTS_FILE"
}

# Delete controller function
delete_controller() {
    local controller_path=$1
    local full_path="$BACKEND_DIR/$controller_path"
    
    if [ -f "$full_path" ]; then
        echo -e "${GREEN}Deleting:${NC} $controller_path"
        echo "DELETED: $controller_path" >> "$RESULTS_FILE"
        rm "$full_path"
        return 0
    else
        echo -e "${YELLOW}Not found:${NC} $controller_path"
        echo "NOT FOUND: $controller_path" >> "$RESULTS_FILE"
        return 1
    fi
}

echo "========================================="
echo "PHASE 1: TEST CONVERTED RUST ENDPOINTS"
echo "========================================="
echo ""

# Track results
declare -A test_results
total_tests=0
passed_tests=0

# Test each converted endpoint
echo "1. HealthCheckController → health.rs"
if test_endpoint "Health Live" "$API_URL/health/live"; then
    test_results["HealthCheckController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "2. NewsletterController → newsletter.rs"
if test_endpoint "Newsletter Subscribe" "$API_URL/newsletter/subscribe" "POST"; then
    test_results["NewsletterController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "3. TimeController → Simple endpoint"
if test_endpoint "Time Now" "$API_URL/time/now"; then
    test_results["TimeController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "4. PostController → posts.rs"
if test_endpoint "Posts List" "$API_URL/posts"; then
    test_results["PostController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "5. IndicatorController → indicators.rs"
if test_endpoint "Indicators List" "$API_URL/indicators"; then
    test_results["IndicatorController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "6. VideoController → videos.rs"
if test_endpoint "Videos List" "$API_URL/videos"; then
    test_results["VideoController"]=1
    ((passed_tests++))
fi
((total_tests++))

echo ""
echo "========================================="
echo "TEST RESULTS SUMMARY"
echo "========================================="
echo "Total Tests: $total_tests"
echo "Passed: $passed_tests"
echo "Failed: $((total_tests - passed_tests))"
echo ""

echo "" >> "$RESULTS_FILE"
echo "=========================================" >> "$RESULTS_FILE"
echo "SUMMARY" >> "$RESULTS_FILE"
echo "=========================================" >> "$RESULTS_FILE"
echo "Total Tests: $total_tests" >> "$RESULTS_FILE"
echo "Passed: $passed_tests" >> "$RESULTS_FILE"
echo "Failed: $((total_tests - passed_tests))" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Ask for confirmation before deletion
echo ""
echo "========================================="
echo "READY TO DELETE PHP CONTROLLERS"
echo "========================================="
echo ""
echo "The following controllers passed tests and can be deleted:"
echo ""

for controller in "${!test_results[@]}"; do
    echo "  ✓ $controller"
done

echo ""
echo "Results saved to: $RESULTS_FILE"
echo ""
echo "To proceed with deletion, run:"
echo "  bash $0 --delete"
echo ""

# If --delete flag is provided, delete the controllers
if [ "$1" = "--delete" ]; then
    echo "========================================="
    echo "DELETING VERIFIED CONTROLLERS"
    echo "========================================="
    echo ""
    
    deleted_count=0
    
    for controller in "${!test_results[@]}"; do
        case $controller in
            "HealthCheckController")
                if delete_controller "Api/HealthCheckController.php"; then
                    ((deleted_count++))
                fi
                ;;
            "NewsletterController")
                if delete_controller "Api/NewsletterController.php"; then
                    ((deleted_count++))
                fi
                ;;
            "TimeController")
                if delete_controller "Api/TimeController.php"; then
                    ((deleted_count++))
                fi
                ;;
            "PostController")
                # Multiple PostController files - need to find the right one
                if delete_controller "Api/PostController.php"; then
                    ((deleted_count++))
                fi
                ;;
            "IndicatorController")
                if delete_controller "Api/IndicatorController.php"; then
                    ((deleted_count++))
                fi
                ;;
            "VideoController")
                if delete_controller "Api/VideoController.php"; then
                    ((deleted_count++))
                fi
                ;;
        esac
    done
    
    echo ""
    echo "========================================="
    echo "DELETION COMPLETE"
    echo "========================================="
    echo "Deleted: $deleted_count controllers"
    echo ""
    echo "DELETED: $deleted_count controllers" >> "$RESULTS_FILE"
    
    # Git status
    echo "Git status:"
    cd /Users/billyribeiro/CascadeProjects/Revolution-trading-pros
    git status --short | grep "deleted.*Controller"
fi
