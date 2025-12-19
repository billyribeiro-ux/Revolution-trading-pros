<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use App\Models\EmailTemplate;
use App\Models\EmailLayout;
use App\Models\EmailBlock;

class EmailTemplatesSeeder extends Seeder
{
    public function run(): void
    {
        // Create default layout
        $layout = EmailLayout::create([
            'name' => 'Default Email Layout',
            'slug' => 'default',
            'html_structure' => $this->getDefaultLayoutHtml(),
            'is_system' => true,
            'is_default' => true,
        ]);

        // 1. EMAIL VERIFICATION
        $this->createEmailVerificationTemplate($layout);
        
        // 2. PASSWORD RESET
        $this->createPasswordResetTemplate($layout);
        
        // 3. WELCOME EMAIL
        $this->createWelcomeTemplate($layout);
        
        // 4. ORDER CONFIRMATION
        $this->createOrderConfirmationTemplate($layout);
        
        // 5. SUBSCRIPTION STARTED
        $this->createSubscriptionStartedTemplate($layout);
    }

    private function createEmailVerificationTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Email Verification',
            'slug' => 'email-verification',
            'category' => 'transactional',
            'email_type' => 'verification',
            'subject' => 'Verify Your Email Address',
            'subject_template' => 'Verify Your Email Address - {{site.name}}',
            'preheader_template' => 'Please confirm your email address to get started',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        // Create blocks
        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Verify Your Email</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Thanks for signing up! Please verify your email address by clicking the button below:</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Verify Email Address',
                'url' => '{{verification_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#007bff',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p style="color: #666; font-size: 14px; margin-top: 30px;">If you didn\'t create an account, you can safely ignore this email.</p>',
            ],
            'order' => 4,
        ]);
    }

    private function createPasswordResetTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Password Reset',
            'slug' => 'password-reset',
            'category' => 'transactional',
            'email_type' => 'password_reset',
            'subject' => 'Reset Your Password',
            'subject_template' => 'Reset Your Password - {{site.name}}',
            'preheader_template' => 'Click here to reset your password',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Reset Your Password</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>We received a request to reset your password. Click the button below to create a new password:</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Reset Password',
                'url' => '{{reset_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#dc3545',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p style="color: #666; font-size: 14px; margin-top: 30px;">This link will expire in {{expiry_time}}. If you didn\'t request a password reset, please ignore this email.</p>',
            ],
            'order' => 4,
        ]);
    }

    private function createWelcomeTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Welcome Email',
            'slug' => 'welcome',
            'category' => 'transactional',
            'email_type' => 'welcome',
            'subject' => 'Welcome to Revolution Trading Pros!',
            'subject_template' => 'Welcome to {{site.name}}, {{user.first_name}}!',
            'preheader_template' => 'Let\'s get you started on your trading journey',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 32px; margin-bottom: 20px;">Welcome to Revolution Trading Pros! 🎉</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Welcome aboard! We\'re thrilled to have you join our community of traders.</p><p>Here\'s what you can do next:</p><ul><li>Complete your profile</li><li>Explore our trading courses</li><li>Join the trading room</li><li>Connect with other traders</li></ul>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Go to Dashboard',
                'url' => '{{dashboard_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#28a745',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createOrderConfirmationTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Order Confirmation',
            'slug' => 'order-confirmation',
            'category' => 'transactional',
            'email_type' => 'order_confirmation',
            'subject' => 'Order Confirmation',
            'subject_template' => 'Order Confirmation #{{order.number}}',
            'preheader_template' => 'Thank you for your order!',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Order Confirmed!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Thank you for your order! We\'ve received your payment and are processing your order.</p><p><strong>Order Number:</strong> {{order.number}}<br><strong>Order Total:</strong> {{order.total}}<br><strong>Order Date:</strong> {{order.created_at}}</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'View Order Details',
                'url' => '{{order.url}}',
            ],
            'styles' => [
                'backgroundColor' => '#007bff',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createSubscriptionStartedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Subscription Started',
            'slug' => 'subscription-started',
            'category' => 'transactional',
            'email_type' => 'subscription_started',
            'subject' => 'Your Subscription is Active',
            'subject_template' => 'Welcome to {{subscription.plan}}!',
            'preheader_template' => 'Your subscription has been activated',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Subscription Activated! 🎊</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your {{subscription.plan}} subscription is now active!</p><p><strong>Plan:</strong> {{subscription.plan}}<br><strong>Price:</strong> {{subscription.price}}<br><strong>Billing Cycle:</strong> {{subscription.billing_cycle}}<br><strong>Next Billing Date:</strong> {{subscription.next_billing_date}}</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Access Your Benefits',
                'url' => '{{dashboard_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#6f42c1',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function getDefaultLayoutHtml(): string
    {
        return <<<'HTML'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <title>{{subject}}</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f4f4f4;">
    <table border="0" cellpadding="0" cellspacing="0" width="100%" style="background-color: #f4f4f4;">
        <tr>
            <td align="center" style="padding: 40px 0;">
                <table border="0" cellpadding="0" cellspacing="0" width="600" style="background-color: #ffffff; border-radius: 8px; overflow: hidden;">
                    <!-- Header -->
                    <tr>
                        <td align="center" style="padding: 40px 40px 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
                            <h1 style="margin: 0; color: #ffffff; font-size: 24px;">{{site.name}}</h1>
                        </td>
                    </tr>
                    <!-- Content -->
                    <tr>
                        <td style="padding: 40px;">
                            {{content}}
                        </td>
                    </tr>
                    <!-- Footer -->
                    <tr>
                        <td style="padding: 30px 40px; background-color: #f8f9fa; border-top: 1px solid #e9ecef;">
                            <p style="margin: 0 0 10px; color: #6c757d; font-size: 14px; text-align: center;">
                                © {{current_year}} {{site.name}}. All rights reserved.
                            </p>
                            <p style="margin: 0; color: #6c757d; font-size: 12px; text-align: center;">
                                <a href="{{preferences_url}}" style="color: #007bff; text-decoration: none;">Email Preferences</a> | 
                                <a href="{{unsubscribe_url}}" style="color: #007bff; text-decoration: none;">Unsubscribe</a>
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>
HTML;
    }
}
