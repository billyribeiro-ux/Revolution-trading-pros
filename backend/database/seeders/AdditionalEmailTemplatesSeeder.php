<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use App\Models\EmailTemplate;
use App\Models\EmailLayout;
use App\Models\EmailBlock;

class AdditionalEmailTemplatesSeeder extends Seeder
{
    public function run(): void
    {
        $layout = EmailLayout::where('slug', 'default')->first();

        // 6. SUBSCRIPTION EXPIRING
        $this->createSubscriptionExpiringTemplate($layout);
        
        // 7. SUBSCRIPTION RENEWED
        $this->createSubscriptionRenewedTemplate($layout);
        
        // 8. SUBSCRIPTION CANCELLED
        $this->createSubscriptionCancelledTemplate($layout);
        
        // 9. ORDER SHIPPED
        $this->createOrderShippedTemplate($layout);
        
        // 10. PAYMENT SUCCESS
        $this->createPaymentSuccessTemplate($layout);
        
        // 11. PAYMENT FAILED
        $this->createPaymentFailedTemplate($layout);
        
        // 12. NEWSLETTER
        $this->createNewsletterTemplate($layout);
        
        // 13. ABANDONED CART
        $this->createAbandonedCartTemplate($layout);
        
        // 14. COURSE ENROLLMENT
        $this->createCourseEnrollmentTemplate($layout);
        
        // 15. SUPPORT TICKET
        $this->createSupportTicketTemplate($layout);
        
        // 16. ACCOUNT UPDATED
        $this->createAccountUpdatedTemplate($layout);
        
        // 17. TWO FACTOR AUTH
        $this->createTwoFactorTemplate($layout);
        
        // 18. REFUND PROCESSED
        $this->createRefundProcessedTemplate($layout);
    }

    private function createSubscriptionExpiringTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Subscription Expiring',
            'slug' => 'subscription-expiring',
            'category' => 'transactional',
            'email_type' => 'subscription_expiring',
            'subject' => 'Your Subscription is Expiring Soon',
            'subject_template' => 'Your {{subscription.plan}} expires in 7 days',
            'preheader_template' => 'Renew now to keep your benefits',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #ff6b6b; font-size: 28px; margin-bottom: 20px;">⚠️ Your Subscription is Expiring</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your {{subscription.plan}} subscription will expire on <strong>{{subscription.next_billing_date}}</strong>.</p><p>Renew now to continue enjoying all your benefits without interruption!</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Renew Subscription',
                'url' => '{{renew_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#ffc107',
                'color' => '#000000',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createSubscriptionRenewedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Subscription Renewed',
            'slug' => 'subscription-renewed',
            'category' => 'transactional',
            'email_type' => 'subscription_renewed',
            'subject' => 'Subscription Renewed Successfully',
            'subject_template' => 'Your {{subscription.plan}} has been renewed',
            'preheader_template' => 'Thank you for continuing with us!',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #28a745; font-size: 28px; margin-bottom: 20px;">✅ Subscription Renewed!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Great news! Your {{subscription.plan}} subscription has been successfully renewed.</p><p><strong>Next Billing Date:</strong> {{subscription.next_billing_date}}<br><strong>Amount:</strong> {{subscription.price}}</p>',
            ],
            'order' => 2,
        ]);
    }

    private function createSubscriptionCancelledTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Subscription Cancelled',
            'slug' => 'subscription-cancelled',
            'category' => 'transactional',
            'email_type' => 'subscription_cancelled',
            'subject' => 'Subscription Cancelled',
            'subject_template' => 'Your {{subscription.plan}} has been cancelled',
            'preheader_template' => 'We\'re sorry to see you go',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Subscription Cancelled</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your {{subscription.plan}} subscription has been cancelled as requested.</p><p>You\'ll continue to have access until {{subscription.end_date}}.</p><p>We\'d love to have you back anytime!</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Reactivate Subscription',
                'url' => '{{reactivate_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#6c757d',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createOrderShippedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Order Shipped',
            'slug' => 'order-shipped',
            'category' => 'transactional',
            'email_type' => 'order_shipped',
            'subject' => 'Your Order Has Shipped!',
            'subject_template' => 'Order #{{order.number}} is on its way!',
            'preheader_template' => 'Track your package',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">📦 Your Order Has Shipped!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Good news! Your order #{{order.number}} has been shipped and is on its way to you.</p><p><strong>Tracking Number:</strong> {{order.tracking_number}}<br><strong>Estimated Delivery:</strong> {{order.estimated_delivery}}</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Track Package',
                'url' => '{{tracking_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#17a2b8',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createPaymentSuccessTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Payment Success',
            'slug' => 'payment-success',
            'category' => 'transactional',
            'email_type' => 'payment_success',
            'subject' => 'Payment Received',
            'subject_template' => 'Payment of {{payment.amount}} received',
            'preheader_template' => 'Thank you for your payment',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #28a745; font-size: 28px; margin-bottom: 20px;">✅ Payment Successful!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>We\'ve successfully received your payment of <strong>{{payment.amount}}</strong>.</p><p><strong>Transaction ID:</strong> {{payment.transaction_id}}<br><strong>Date:</strong> {{payment.date}}</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'View Receipt',
                'url' => '{{receipt_url}}',
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

    private function createPaymentFailedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Payment Failed',
            'slug' => 'payment-failed',
            'category' => 'transactional',
            'email_type' => 'payment_failed',
            'subject' => 'Payment Failed',
            'subject_template' => 'Payment of {{payment.amount}} failed',
            'preheader_template' => 'Please update your payment method',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #dc3545; font-size: 28px; margin-bottom: 20px;">❌ Payment Failed</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>We were unable to process your payment of <strong>{{payment.amount}}</strong>.</p><p><strong>Reason:</strong> {{payment.error_message}}</p><p>Please update your payment method to continue your service.</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Update Payment Method',
                'url' => '{{update_payment_url}}',
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
    }

    private function createNewsletterTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Newsletter',
            'slug' => 'newsletter',
            'category' => 'marketing',
            'email_type' => 'newsletter',
            'subject' => 'Newsletter',
            'subject_template' => '{{newsletter.title}}',
            'preheader_template' => '{{newsletter.preview}}',
            'layout_id' => $layout->id,
            'is_system' => false,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 32px; margin-bottom: 20px;">{{newsletter.title}}</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>{{newsletter.content}}</p>',
            ],
            'order' => 2,
        ]);
    }

    private function createAbandonedCartTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Abandoned Cart',
            'slug' => 'abandoned-cart',
            'category' => 'marketing',
            'email_type' => 'abandoned_cart',
            'subject' => 'You Left Something Behind!',
            'subject_template' => 'Complete your order - {{cart.item_count}} items waiting',
            'preheader_template' => 'Your cart is waiting for you',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">🛒 You Left Something Behind!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>We noticed you left {{cart.item_count}} items in your cart. Complete your purchase now!</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Complete Your Order',
                'url' => '{{checkout_url}}',
            ],
            'styles' => [
                'backgroundColor' => '#ff6b6b',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createCourseEnrollmentTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Course Enrollment',
            'slug' => 'course-enrollment',
            'category' => 'transactional',
            'email_type' => 'course_enrollment',
            'subject' => 'Course Enrollment Confirmed',
            'subject_template' => 'Welcome to {{course.name}}!',
            'preheader_template' => 'Start learning today',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">📚 Welcome to {{course.name}}!</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Congratulations! You\'re now enrolled in <strong>{{course.name}}</strong>.</p><p>Start learning right away and unlock your potential!</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'Start Course',
                'url' => '{{course.url}}',
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

    private function createSupportTicketTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Support Ticket Created',
            'slug' => 'support-ticket-created',
            'category' => 'transactional',
            'email_type' => 'support_ticket',
            'subject' => 'Support Ticket Created',
            'subject_template' => 'Ticket #{{ticket.number}} - {{ticket.subject}}',
            'preheader_template' => 'We\'ve received your support request',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">🎫 Support Ticket Created</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>We\'ve received your support request and our team will respond shortly.</p><p><strong>Ticket #:</strong> {{ticket.number}}<br><strong>Subject:</strong> {{ticket.subject}}</p>',
            ],
            'order' => 2,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'button',
            'content' => [
                'text' => 'View Ticket',
                'url' => '{{ticket.url}}',
            ],
            'styles' => [
                'backgroundColor' => '#17a2b8',
                'color' => '#ffffff',
                'padding' => '14px 40px',
                'borderRadius' => '6px',
                'fontSize' => '16px',
                'fontWeight' => 'bold',
            ],
            'order' => 3,
        ]);
    }

    private function createAccountUpdatedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Account Updated',
            'slug' => 'account-updated',
            'category' => 'transactional',
            'email_type' => 'account_updated',
            'subject' => 'Account Information Updated',
            'subject_template' => 'Your account has been updated',
            'preheader_template' => 'Recent changes to your account',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">Account Updated</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your account information has been updated successfully.</p><p>If you didn\'t make these changes, please contact support immediately.</p>',
            ],
            'order' => 2,
        ]);
    }

    private function createTwoFactorTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Two-Factor Authentication',
            'slug' => 'two-factor-auth',
            'category' => 'transactional',
            'email_type' => '2fa',
            'subject' => 'Your Verification Code',
            'subject_template' => 'Your verification code: {{code}}',
            'preheader_template' => 'Use this code to sign in',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">🔐 Your Verification Code</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your verification code is:</p><h2 style="font-size: 36px; letter-spacing: 8px; text-align: center; padding: 20px; background: #f8f9fa; border-radius: 8px; margin: 20px 0;">{{code}}</h2><p style="color: #666; font-size: 14px;">This code will expire in 10 minutes.</p>',
            ],
            'order' => 2,
        ]);
    }

    private function createRefundProcessedTemplate($layout)
    {
        $template = EmailTemplate::create([
            'name' => 'Refund Processed',
            'slug' => 'refund-processed',
            'category' => 'transactional',
            'email_type' => 'refund',
            'subject' => 'Refund Processed',
            'subject_template' => 'Refund of {{refund.amount}} processed',
            'preheader_template' => 'Your refund has been issued',
            'layout_id' => $layout->id,
            'is_system' => true,
            'is_active' => true,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<h1 style="color: #333; font-size: 28px; margin-bottom: 20px;">💰 Refund Processed</h1>',
            ],
            'order' => 1,
        ]);

        EmailBlock::create([
            'template_id' => $template->id,
            'block_type' => 'text',
            'content' => [
                'html' => '<p>Hi {{user.name}},</p><p>Your refund of <strong>{{refund.amount}}</strong> has been processed.</p><p>The funds should appear in your account within 5-10 business days.</p><p><strong>Refund ID:</strong> {{refund.id}}<br><strong>Original Order:</strong> {{order.number}}</p>',
            ],
            'order' => 2,
        ]);
    }
}
