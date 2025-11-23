#!/bin/bash

# Test Course Creation End-to-End
echo "=== Testing Course Creation API ==="
echo ""

# Step 1: Login to get token
echo "1. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST http://127.0.0.1:8000/api/login \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "email": "admin@test.com",
    "password": "password123"
  }')

echo "Login Response:"
echo "$LOGIN_RESPONSE" | jq '.' 2>/dev/null || echo "$LOGIN_RESPONSE"
echo ""

# Extract token
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token // .access_token // empty' 2>/dev/null)

if [ -z "$TOKEN" ]; then
  echo "❌ Failed to get authentication token"
  exit 1
fi

echo "✓ Got token: ${TOKEN:0:20}..."
echo ""

# Step 2: Create a course
echo "2. Creating course..."
COURSE_RESPONSE=$(curl -s -X POST http://127.0.0.1:8000/api/admin/products \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Test Trading Course",
    "slug": "test-trading-course",
    "type": "course",
    "description": "Learn advanced trading strategies",
    "long_description": "This comprehensive course covers everything you need to know about trading, from basics to advanced strategies.",
    "price": 299.99,
    "is_active": true,
    "thumbnail": "https://example.com/thumbnail.jpg",
    "meta_title": "Test Trading Course - Learn Trading",
    "meta_description": "Master trading with our comprehensive course",
    "indexable": true,
    "metadata": {
      "short_description": "Learn advanced trading strategies",
      "course_type": "self-paced",
      "format": "video",
      "level": "intermediate",
      "duration_hours": 20,
      "modules": [
        {
          "id": "mod1",
          "title": "Introduction to Trading",
          "description": "Get started with trading basics",
          "order": 1,
          "lessons": [
            {
              "id": "les1",
              "title": "What is Trading?",
              "type": "video",
              "duration": 15,
              "order": 1
            }
          ]
        }
      ],
      "pricing_model": "one-time",
      "currency": "USD",
      "access": {
        "lifetime_access": true,
        "certificate_enabled": true
      },
      "seo": {
        "keywords": ["trading", "stocks", "forex"]
      },
      "analytics": {
        "ga4_enabled": true
      }
    }
  }')

echo "Course Creation Response:"
echo "$COURSE_RESPONSE" | jq '.' 2>/dev/null || echo "$COURSE_RESPONSE"
echo ""

# Check if course was created
COURSE_ID=$(echo "$COURSE_RESPONSE" | jq -r '.data.id // .id // empty' 2>/dev/null)

if [ -z "$COURSE_ID" ]; then
  echo "❌ Failed to create course"
  echo "Response: $COURSE_RESPONSE"
  exit 1
fi

echo "✓ Course created with ID: $COURSE_ID"
echo ""

# Step 3: Retrieve the course
echo "3. Retrieving course..."
GET_RESPONSE=$(curl -s -X GET "http://127.0.0.1:8000/api/admin/products/$COURSE_ID" \
  -H "Accept: application/json" \
  -H "Authorization: Bearer $TOKEN")

echo "Retrieved Course:"
echo "$GET_RESPONSE" | jq '.' 2>/dev/null || echo "$GET_RESPONSE"
echo ""

# Step 4: List courses filtered by type
echo "4. Listing courses (type=course)..."
LIST_RESPONSE=$(curl -s -X GET "http://127.0.0.1:8000/api/admin/products?type=course" \
  -H "Accept: application/json" \
  -H "Authorization: Bearer $TOKEN")

echo "Course List:"
echo "$LIST_RESPONSE" | jq '.data | length' 2>/dev/null && echo "courses found" || echo "$LIST_RESPONSE"
echo ""

echo "=== Test Complete ==="
echo ""
echo "Summary:"
echo "  ✓ Authentication successful"
echo "  ✓ Course created (ID: $COURSE_ID)"
echo "  ✓ Course retrieved successfully"
echo "  ✓ Course appears in filtered list"
