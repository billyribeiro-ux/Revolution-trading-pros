# Final Verification Plan - ICT 11+ Principal Engineer
**Date:** January 4, 2026  
**Objective:** Verify all dashboard pages work with authentication

---

## 🎯 Test Scenarios

### **Scenario 1: Authentication Flow**

**Prerequisites:**
- User account exists in database
- Email is verified (or verification bypassed)

**Steps:**
1. Navigate to https://revolution-trading-pros.pages.dev/login
2. Enter credentials
3. Submit login form
4. Verify redirect to dashboard
5. Check browser DevTools → Application → Cookies
6. Confirm `rtp_access_token` cookie exists

**Expected Result:**
- ✅ Login successful
- ✅ Token stored in cookies
- ✅ Redirected to dashboard

---

### **Scenario 2: Subscriptions Page**

**URL:** `/dashboard/account/subscriptions`

**Test Steps:**
1. Navigate to subscriptions page
2. Open DevTools → Network tab
3. Check request to `/api/my/subscriptions`
4. Verify `Authorization: Bearer <token>` header present
5. Check response status code
6. Verify response format: `{ subscriptions: [...] }`

**Expected Result:**
- ✅ HTTP 200 OK
- ✅ Authorization header present
- ✅ Data loads successfully
- ✅ No 401 errors in console

---

### **Scenario 3: Coupons Page**

**URL:** `/dashboard/account/coupons`

**Test Steps:**
1. Navigate to coupons page
2. Check request to `/api/coupons/user/available`
3. Verify Authorization header
4. Check response status

**Expected Result:**
- ✅ HTTP 200 OK
- ✅ Authorization header present
- ✅ Coupons list loads (or empty array)

---

### **Scenario 4: Edit Address Page**

**URL:** `/dashboard/account/edit-address`

**Test Steps:**
1. Navigate to edit address page
2. Check request to `/api/user/profile`
3. Verify Authorization header
4. Verify form populates with user data
5. Submit form (optional)
6. Check PUT request has Authorization header

**Expected Result:**
- ✅ HTTP 200 OK on GET
- ✅ Form loads with user data
- ✅ Authorization header on both GET and PUT

---

### **Scenario 5: Payment Methods Page**

**URL:** `/dashboard/account/payment-methods`

**Test Steps:**
1. Navigate to payment methods page
2. Check request to `/api/user/payment-methods`
3. Verify Authorization header
4. Check response (stub implementation returns empty array)

**Expected Result:**
- ✅ HTTP 200 OK
- ✅ Authorization header present
- ✅ Empty payment methods array (stub)

---

### **Scenario 6: Edit Account Page**

**URL:** `/dashboard/account/edit-account`

**Test Steps:**
1. Navigate to edit account page
2. Check request to `/api/user/profile`
3. Verify Authorization header
4. Verify form populates with account data

**Expected Result:**
- ✅ HTTP 200 OK
- ✅ Form loads with user data
- ✅ Authorization header present

---

### **Scenario 7: Orders Page**

**URL:** `/dashboard/account/orders`

**Test Steps:**
1. Navigate to orders page
2. Check request to `/api/my/orders`
3. Verify Authorization header
4. Check response format

**Expected Result:**
- ✅ HTTP 200 OK
- ✅ Authorization header present
- ✅ Orders list loads (or empty array)

---

## 🔧 Troubleshooting Guide

### **If 401 Errors Persist:**

1. **Check Cookie Storage:**
   - Open DevTools → Application → Cookies
   - Verify `rtp_access_token` exists
   - Check cookie domain and path

2. **Check Authorization Header:**
   - Open DevTools → Network tab
   - Click on API request
   - Check Headers tab
   - Verify `Authorization: Bearer <token>` is present

3. **Check Token Validity:**
   - Copy token from cookie
   - Test manually:
   ```bash
   curl -H "Authorization: Bearer <TOKEN>" \
     https://revolution-trading-pros-api.fly.dev/api/auth/me
   ```

4. **Check Cloudflare Proxy:**
   - Verify `_redirects` file is in frontend root
   - Check Cloudflare Pages build logs
   - Confirm proxy is working

---

## 📊 Success Criteria

**All 6 dashboard pages must:**
- ✅ Return HTTP 200 (not 401)
- ✅ Include Authorization header in requests
- ✅ Load data successfully
- ✅ Display no console errors

**Authentication must:**
- ✅ Store token in cookies
- ✅ Forward token to API requests
- ✅ Validate token on backend
- ✅ Return user data

---

## 🎓 Known Limitations

1. **Payment Methods:** Stub implementation (returns empty array)
2. **Subscriptions:** May return empty if no subscriptions exist
3. **Orders:** May return empty if no orders exist
4. **Email Verification:** May block login if not verified

---

## 📝 Test Evidence Template

```markdown
## Test Results - [Date/Time]

### Subscriptions Page
- URL: /dashboard/account/subscriptions
- Status: [200/401/500]
- Auth Header: [Present/Missing]
- Data Loaded: [Yes/No]
- Screenshot: [Link]

### Coupons Page
- URL: /dashboard/account/coupons
- Status: [200/401/500]
- Auth Header: [Present/Missing]
- Data Loaded: [Yes/No]

[Continue for all 6 pages...]
```

---

**Ready for final verification once Cloudflare Pages deployment completes.**
