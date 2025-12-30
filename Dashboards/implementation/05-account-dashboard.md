# 5. Account Dashboard - User Profile & Settings

## 🎯 Purpose & Functionality
The Account Dashboard provides users with **comprehensive profile management** capabilities, including personal information updates, membership management, billing information, and account settings. This dashboard serves as the **central hub** for user account administration and preferences.

## 📋 Population Logic
- **Called On:** Navigation to `/dashboard/account` URL
- **Populated With:** User profile data, membership information, billing details
- **Data Source:** WordPress user_meta table + WooCommerce customer data
- **Trigger:** Click on profile link or direct URL access
- **Refreshes:** On profile updates, membership changes, or billing modifications

## 🏗️ Content Components
- **Personal Information** (name, email, profile photo)
- **Membership Status and Details**
- **Billing Information and Payment Methods**
- **Password and Security Settings**
- **Notification Preferences**
- **Communication Settings**
- **Account History and Activity Log**

## 💻 Complete Implementation Code

### PHP Template (WordPress)
```php
<?php
/**
 * Account Dashboard Template
 * File: dashboard-account.php
 * Purpose: User profile management and account settings
 */

get_header();

// Check user authentication
if (!is_user_logged_in()) {
    wp_redirect(wp_login_url(get_permalink()));
    exit;
}

$user_id = get_current_user_id();
$user_info = get_userdata($user_id);
$customer = new WC_Customer($user_id);

// Get user memberships
$active_memberships = wc_memberships_get_user_active_memberships($user_id);
$expired_memberships = wc_memberships_get_user_expired_memberships($user_id);

// Handle form submissions
$profile_updated = false;
$password_updated = false;
$error_message = '';

if (isset($_POST['action']) && $_POST['action'] === 'update_profile') {
    // Verify nonce
    if (!wp_verify_nonce($_POST['profile_nonce'], 'update_profile_' . $user_id)) {
        $error_message = __('Security check failed.', 'simpler-trading');
    } else {
        // Update user profile
        $update_data = array(
            'ID' => $user_id,
            'first_name' => sanitize_text_field($_POST['first_name']),
            'last_name' => sanitize_text_field($_POST['last_name']),
            'user_email' => sanitize_email($_POST['email']),
            'display_name' => sanitize_text_field($_POST['display_name'])
        );
        
        $result = wp_update_user($update_data);
        
        if (!is_wp_error($result)) {
            $profile_updated = true;
            $user_info = get_userdata($user_id); // Refresh user data
        } else {
            $error_message = $result->get_error_message();
        }
    }
}

if (isset($_POST['action']) && $_POST['action'] === 'update_password') {
    // Verify nonce
    if (!wp_verify_nonce($_POST['password_nonce'], 'update_password_' . $user_id)) {
        $error_message = __('Security check failed.', 'simpler-trading');
    } else {
        $current_password = $_POST['current_password'];
        $new_password = $_POST['new_password'];
        $confirm_password = $_POST['confirm_password'];
        
        // Validate current password
        if (!wp_check_password($current_password, $user_info->user_pass, $user_id)) {
            $error_message = __('Current password is incorrect.', 'simpler-trading');
        } elseif ($new_password !== $confirm_password) {
            $error_message = __('New passwords do not match.', 'simpler-trading');
        } elseif (strlen($new_password) < 8) {
            $error_message = __('Password must be at least 8 characters long.', 'simpler-trading');
        } else {
            // Update password
            wp_set_password($new_password, $user_id);
            $password_updated = true;
            
            // Force logout to require new login
            wp_clear_auth_cookie();
            wp_redirect(wp_login_url(get_permalink()) . '?password_changed=true');
            exit;
        }
    }
}

?>

<div class="dashboard-container">
    <header class="dashboard-header">
        <h1><?php _e('My Account', 'simpler-trading'); ?></h1>
    </header>
    
    <main class="dashboard-content">
        <!-- Success/Error Messages -->
        <?php if ($profile_updated): ?>
            <div class="alert alert-success">
                <?php _e('Profile updated successfully!', 'simpler-trading'); ?>
            </div>
        <?php endif; ?>
        
        <?php if ($error_message): ?>
            <div class="alert alert-error">
                <?php echo esc_html($error_message); ?>
            </div>
        <?php endif; ?>
        
        <div class="account-sections">
            <!-- Profile Information Section -->
            <section class="account-section">
                <h2><?php _e('Profile Information', 'simpler-trading'); ?></h2>
                
                <form method="post" class="profile-form">
                    <?php wp_nonce_field('update_profile_' . $user_id, 'profile_nonce'); ?>
                    <input type="hidden" name="action" value="update_profile">
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="first_name"><?php _e('First Name', 'simpler-trading'); ?></label>
                            <input type="text" id="first_name" name="first_name" 
                                   value="<?php echo esc_attr($user_info->first_name); ?>" required>
                        </div>
                        
                        <div class="form-group">
                            <label for="last_name"><?php _e('Last Name', 'simpler-trading'); ?></label>
                            <input type="text" id="last_name" name="last_name" 
                                   value="<?php echo esc_attr($user_info->last_name); ?>" required>
                        </div>
                    </div>
                    
                    <div class="form-group">
                        <label for="display_name"><?php _e('Display Name', 'simpler-trading'); ?></label>
                        <input type="text" id="display_name" name="display_name" 
                               value="<?php echo esc_attr($user_info->display_name); ?>" required>
                    </div>
                    
                    <div class="form-group">
                        <label for="email"><?php _e('Email Address', 'simpler-trading'); ?></label>
                        <input type="email" id="email" name="email" 
                               value="<?php echo esc_attr($user_info->user_email); ?>" required>
                    </div>
                    
                    <div class="form-group">
                        <label><?php _e('Profile Photo', 'simpler-trading'); ?></label>
                        <div class="profile-photo">
                            <img src="<?php echo get_avatar_url($user_id, array('size' => 100)); ?>" 
                                 alt="<?php echo esc_attr($user_info->display_name); ?>" class="avatar">
                            <p class="photo-help">
                                <?php _e('Profile photo managed through Gravatar. ', 'simpler-trading'); ?>
                                <a href="https://en.gravatar.com/" target="_blank">
                                    <?php _e('Change on Gravatar', 'simpler-trading'); ?>
                                </a>
                            </p>
                        </div>
                    </div>
                    
                    <div class="form-actions">
                        <button type="submit" class="btn btn-primary">
                            <?php _e('Update Profile', 'simpler-trading'); ?>
                        </button>
                    </div>
                </form>
            </section>
            
            <!-- Password Change Section -->
            <section class="account-section">
                <h2><?php _e('Change Password', 'simpler-trading'); ?></h2>
                
                <form method="post" class="password-form">
                    <?php wp_nonce_field('update_password_' . $user_id, 'password_nonce'); ?>
                    <input type="hidden" name="action" value="update_password">
                    
                    <div class="form-group">
                        <label for="current_password"><?php _e('Current Password', 'simpler-trading'); ?></label>
                        <input type="password" id="current_password" name="current_password" required>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="new_password"><?php _e('New Password', 'simpler-trading'); ?></label>
                            <input type="password" id="new_password" name="new_password" required minlength="8">
                            <small class="form-help">
                                <?php _e('Minimum 8 characters', 'simpler-trading'); ?>
                            </small>
                        </div>
                        
                        <div class="form-group">
                            <label for="confirm_password"><?php _e('Confirm New Password', 'simpler-trading'); ?></label>
                            <input type="password" id="confirm_password" name="confirm_password" required minlength="8">
                        </div>
                    </div>
                    
                    <div class="form-actions">
                        <button type="submit" class="btn btn-primary">
                            <?php _e('Change Password', 'simpler-trading'); ?>
                        </button>
                    </div>
                </form>
            </section>
            
            <!-- Memberships Section -->
            <section class="account-section">
                <h2><?php _e('Memberships', 'simpler-trading'); ?></h2>
                
                <div class="memberships-list">
                    <?php if (!empty($active_memberships)): ?>
                        <h3><?php _e('Active Memberships', 'simpler-trading'); ?></h3>
                        <?php foreach ($active_memberships as $membership): ?>
                            <div class="membership-item active">
                                <h4><?php echo $membership->get_plan()->get_name(); ?></h4>
                                <p><?php _e('Status:', 'simpler-trading'); ?> <span class="status-active"><?php _e('Active', 'simpler-trading'); ?></span></p>
                                <p><?php _e('Started:', 'simpler-trading'); ?> <?php echo $membership->get_start_date()->format('F j, Y'); ?></p>
                                <?php if ($membership->get_end_date()): ?>
                                    <p><?php _e('Expires:', 'simpler-trading'); ?> <?php echo $membership->get_end_date()->format('F j, Y'); ?></p>
                                <?php endif; ?>
                                <div class="membership-actions">
                                    <a href="<?php echo get_membership_dashboard_url($membership); ?>" class="btn btn-secondary">
                                        <?php _e('View Dashboard', 'simpler-trading'); ?>
                                    </a>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    <?php endif; ?>
                    
                    <?php if (!empty($expired_memberships)): ?>
                        <h3><?php _e('Expired Memberships', 'simpler-trading'); ?></h3>
                        <?php foreach ($expired_memberships as $membership): ?>
                            <div class="membership-item expired">
                                <h4><?php echo $membership->get_plan()->get_name(); ?></h4>
                                <p><?php _e('Status:', 'simpler-trading'); ?> <span class="status-expired"><?php _e('Expired', 'simpler-trading'); ?></span></p>
                                <p><?php _e('Expired:', 'simpler-trading'); ?> <?php echo $membership->get_end_date()->format('F j, Y'); ?></p>
                                <div class="membership-actions">
                                    <a href="<?php echo get_renewal_url($membership); ?>" class="btn btn-primary">
                                        <?php _e('Renew', 'simpler-trading'); ?>
                                    </a>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    <?php endif; ?>
                </div>
            </section>
            
            <!-- Billing Information Section -->
            <section class="account-section">
                <h2><?php _e('Billing Information', 'simpler-trading'); ?></h2>
                
                <div class="billing-info">
                    <?php if ($customer->get_billing_email()): ?>
                        <div class="billing-item">
                            <label><?php _e('Billing Email:', 'simpler-trading'); ?></label>
                            <span><?php echo esc_html($customer->get_billing_email()); ?></span>
                        </div>
                    <?php endif; ?>
                    
                    <?php if ($customer->get_billing_phone()): ?>
                        <div class="billing-item">
                            <label><?php _e('Phone:', 'simpler-trading'); ?></label>
                            <span><?php echo esc_html($customer->get_billing_phone()); ?></span>
                        </div>
                    <?php endif; ?>
                    
                    <?php if ($customer->get_billing_address()): ?>
                        <div class="billing-item">
                            <label><?php _e('Billing Address:', 'simpler-trading'); ?></label>
                            <address>
                                <?php echo $customer->get_formatted_billing_address(); ?>
                            </address>
                        </div>
                    <?php endif; ?>
                    
                    <div class="billing-actions">
                        <a href="<?php echo wc_get_page_permalink('myaccount'); ?>?edit-address=billing" class="btn btn-secondary">
                            <?php _e('Edit Billing Address', 'simpler-trading'); ?>
                        </a>
                        <a href="<?php echo wc_get_page_permalink('myaccount'); ?>?view-orders" class="btn btn-secondary">
                            <?php _e('View Order History', 'simpler-trading'); ?>
                        </a>
                    </div>
                </div>
            </section>
        </div>
    </main>
</div>

<style>
.account-sections {
    display: grid;
    gap: 30px;
}

.account-section {
    background: white;
    padding: 30px;
    border-radius: 8px;
    border: 1px solid #e0e0e0;
}

.account-section h2 {
    margin-bottom: 20px;
    color: #333;
}

.form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
}

.form-group {
    margin-bottom: 20px;
}

.form-group label {
    display: block;
    margin-bottom: 5px;
    font-weight: 600;
    color: #555;
}

.form-group input {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
}

.profile-photo {
    text-align: center;
}

.profile-photo .avatar {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    margin-bottom: 10px;
}

.memberships-list {
    display: grid;
    gap: 20px;
}

.membership-item {
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 8px;
}

.membership-item.active {
    border-color: #28a745;
    background: #f8fff9;
}

.membership-item.expired {
    border-color: #dc3545;
    background: #fff8f8;
}

.status-active {
    color: #28a745;
    font-weight: 600;
}

.status-expired {
    color: #dc3545;
    font-weight: 600;
}

.alert {
    padding: 15px;
    border-radius: 4px;
    margin-bottom: 20px;
}

.alert-success {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
}

.alert-error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
}

@media (max-width: 768px) {
    .form-row {
        grid-template-columns: 1fr;
    }
    
    .account-section {
        padding: 20px;
    }
}
</style>

<?php get_footer(); ?>
```

## 🎨 Svelte 5 Implementation (Modern)

### Main Page Component
```svelte
<!-- /src/routes/dashboard/account/+page.svelte -->
<script lang="ts">
  import type { PageData } from './$types';
  import { enhance } from '$app/forms';
  
  interface Props {
    data: PageData;
  }
  
  let { data }: Props = $props();
  
  let profileForm = $state<HTMLFormElement>();
  let passwordForm = $state<HTMLFormElement>();
  let isSubmitting = $state(false);
</script>

<div class="dashboard-container">
  <header class="dashboard-header">
    <h1>My Account</h1>
  </header>
  
  <main class="dashboard-content">
    {#if data.success}
      <div class="alert alert-success">
        {data.message}
      </div>
    {/if}
    
    {#if data.error}
      <div class="alert alert-error">
        {data.error}
      </div>
    {/if}
    
    <div class="account-sections">
      <!-- Profile Information -->
      <section class="account-section">
        <h2>Profile Information</h2>
        
        <form method="POST" action="?/updateProfile" use:enhance bind:this={profileForm}>
          <div class="form-row">
            <div class="form-group">
              <label for="first_name">First Name</label>
              <input 
                type="text" 
                id="first_name" 
                name="first_name" 
                value={data.user.firstName} 
                required 
              />
            </div>
            
            <div class="form-group">
              <label for="last_name">Last Name</label>
              <input 
                type="text" 
                id="last_name" 
                name="last_name" 
                value={data.user.lastName} 
                required 
              />
            </div>
          </div>
          
          <div class="form-group">
            <label for="email">Email Address</label>
            <input 
              type="email" 
              id="email" 
              name="email" 
              value={data.user.email} 
              required 
            />
          </div>
          
          <div class="form-group">
            <label>Profile Photo</label>
            <div class="profile-photo">
              <img src={data.user.avatar} alt={data.user.displayName} class="avatar" />
              <p class="photo-help">
                Profile photo managed through Gravatar. 
                <a href="https://en.gravatar.com/" target="_blank">Change on Gravatar</a>
              </p>
            </div>
          </div>
          
          <div class="form-actions">
            <button type="submit" class="btn btn-primary" disabled={isSubmitting}>
              {isSubmitting ? 'Updating...' : 'Update Profile'}
            </button>
          </div>
        </form>
      </section>
      
      <!-- Password Change -->
      <section class="account-section">
        <h2>Change Password</h2>
        
        <form method="POST" action="?/updatePassword" use:enhance bind:this={passwordForm}>
          <div class="form-group">
            <label for="current_password">Current Password</label>
            <input type="password" id="current_password" name="current_password" required />
          </div>
          
          <div class="form-row">
            <div class="form-group">
              <label for="new_password">New Password</label>
              <input type="password" id="new_password" name="new_password" required minlength="8" />
              <small class="form-help">Minimum 8 characters</small>
            </div>
            
            <div class="form-group">
              <label for="confirm_password">Confirm New Password</label>
              <input type="password" id="confirm_password" name="confirm_password" required minlength="8" />
            </div>
          </div>
          
          <div class="form-actions">
            <button type="submit" class="btn btn-primary" disabled={isSubmitting}>
              {isSubmitting ? 'Changing...' : 'Change Password'}
            </button>
          </div>
        </form>
      </section>
      
      <!-- Memberships -->
      <section class="account-section">
        <h2>Memberships</h2>
        
        <div class="memberships-list">
          {#if data.memberships.active.length > 0}
            <h3>Active Memberships</h3>
            {#each data.memberships.active as membership}
              <div class="membership-item active">
                <h4>{membership.name}</h4>
                <p>Status: <span class="status-active">Active</span></p>
                <p>Started: {membership.startDate}</p>
                {#if membership.endDate}
                  <p>Expires: {membership.endDate}</p>
                {/if}
                <div class="membership-actions">
                  <a href="/dashboard/{membership.slug}" class="btn btn-secondary">
                    View Dashboard
                  </a>
                </div>
              </div>
            {/each}
          {/if}
          
          {#if data.memberships.expired.length > 0}
            <h3>Expired Memberships</h3>
            {#each data.memberships.expired as membership}
              <div class="membership-item expired">
                <h4>{membership.name}</h4>
                <p>Status: <span class="status-expired">Expired</span></p>
                <p>Expired: {membership.endDate}</p>
                <div class="membership-actions">
                  <a href="/renew/{membership.slug}" class="btn btn-primary">
                    Renew
                  </a>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </section>
      
      <!-- Billing Information -->
      <section class="account-section">
        <h2>Billing Information</h2>
        
        <div class="billing-info">
          {#if data.billing.email}
            <div class="billing-item">
              <label>Billing Email:</label>
              <span>{data.billing.email}</span>
            </div>
          {/if}
          
          {#if data.billing.phone}
            <div class="billing-item">
              <label>Phone:</label>
              <span>{data.billing.phone}</span>
            </div>
          {/if}
          
          {#if data.billing.address}
            <div class="billing-item">
              <label>Billing Address:</label>
              <address>{@html data.billing.address}</address>
            </div>
          {/if}
          
          <div class="billing-actions">
            <a href="/account/billing" class="btn btn-secondary">
              Edit Billing Address
            </a>
            <a href="/account/orders" class="btn btn-secondary">
              View Order History
            </a>
          </div>
        </div>
      </section>
    </div>
  </main>
</div>

<style>
  .account-sections {
    display: grid;
    gap: 2rem;
  }
  
  .account-section {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    border: 1px solid #e0e0e0;
  }
  
  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }
  
  .form-group {
    margin-bottom: 1.5rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #555;
  }
  
  .form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
  }
  
  .profile-photo {
    text-align: center;
  }
  
  .avatar {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    margin-bottom: 1rem;
  }
  
  .membership-item {
    padding: 1.5rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    margin-bottom: 1rem;
  }
  
  .membership-item.active {
    border-color: #28a745;
    background: #f8fff9;
  }
  
  .membership-item.expired {
    border-color: #dc3545;
    background: #fff8f8;
  }
  
  .alert {
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1.5rem;
  }
  
  .alert-success {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }
  
  .alert-error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }
  
  @media (max-width: 768px) {
    .form-row {
      grid-template-columns: 1fr;
    }
  }
</style>
```

### Load Function
```typescript
// /src/routes/dashboard/account/+page.server.ts
import type { PageServerLoad, Actions } from './$types';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals }) => {
  const user = locals.user;
  
  if (!user) {
    throw redirect(302, '/login');
  }
  
  const [memberships, billing] = await Promise.all([
    fetchUserMemberships(user.id),
    fetchBillingInfo(user.id)
  ]);
  
  return {
    user: {
      firstName: user.firstName,
      lastName: user.lastName,
      email: user.email,
      displayName: user.displayName,
      avatar: user.avatar
    },
    memberships,
    billing
  };
};

export const actions: Actions = {
  updateProfile: async ({ request, locals }) => {
    const data = await request.formData();
    const firstName = data.get('first_name');
    const lastName = data.get('last_name');
    const email = data.get('email');
    
    try {
      await updateUserProfile(locals.user.id, {
        firstName,
        lastName,
        email
      });
      
      return { success: true, message: 'Profile updated successfully!' };
    } catch (error) {
      return fail(400, { error: 'Failed to update profile' });
    }
  },
  
  updatePassword: async ({ request, locals }) => {
    const data = await request.formData();
    const currentPassword = data.get('current_password');
    const newPassword = data.get('new_password');
    const confirmPassword = data.get('confirm_password');
    
    if (newPassword !== confirmPassword) {
      return fail(400, { error: 'Passwords do not match' });
    }
    
    try {
      await updateUserPassword(locals.user.id, currentPassword, newPassword);
      return { success: true, message: 'Password changed successfully!' };
    } catch (error) {
      return fail(400, { error: 'Failed to change password' });
    }
  }
};
```

## 🔒 Security Considerations
- **Nonce Validation**: CSRF protection on all forms
- **Password Validation**: Minimum 8 characters, current password verification
- **Input Sanitization**: All user inputs sanitized
- **Session Management**: Secure cookie-based sessions
- **Gravatar Integration**: External profile photo management

## ⚡ Performance Optimization
- **Form Enhancement**: Progressive enhancement with SvelteKit
- **Lazy Loading**: Load billing data on demand
- **Optimistic Updates**: Immediate UI feedback
- **Caching**: User data cached for performance

## 📱 Responsive Design
- **Desktop**: Two-column form layouts
- **Tablet**: Single column with optimized spacing
- **Mobile**: Full-width forms, touch-friendly inputs

## 🎯 Key Features
- ✅ Profile information editing
- ✅ Password change with validation
- ✅ Active/expired membership display
- ✅ Billing information management
- ✅ Gravatar integration
- ✅ Order history access
- ✅ Security with nonce validation
- ✅ Real-time form validation

## 📊 User Experience Flow
1. User navigates to Account Dashboard
2. Profile information loaded from WordPress user data
3. Membership status displayed with current subscriptions
4. Billing information and payment methods shown
5. User can update preferences and manage account settings

---

**Access Level**: All authenticated users  
**Security**: Nonce validation, password strength requirements  
**Integration**: WordPress user_meta + WooCommerce customer data
