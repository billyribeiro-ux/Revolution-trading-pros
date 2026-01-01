<?php

namespace Database\Seeders;

use App\Models\EmailTemplate;
use Illuminate\Database\Seeder;

/**
 * Seeds admin-customizable email templates for verification, purchases, and win-back campaigns.
 */
class MembershipEmailTemplatesSeeder extends Seeder
{
    public function run(): void
    {
        $templates = [
            // Email Verification
            [
                'name' => 'Email Verification',
                'slug' => 'verify-email',
                'description' => 'Sent when a new user registers to verify their email address',
                'category' => 'authentication',
                'email_type' => 'transactional',
                'subject' => 'Verify Your Email - Revolution Trading Pros',
                'preheader' => 'Click the link below to verify your email and unlock full access',
                'body_html' => $this->getVerifyEmailHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true, 'description' => 'User\'s name'],
                    'verification_url' => ['type' => 'string', 'required' => true, 'description' => 'Email verification URL'],
                ],
                'required_variables' => ['user_name', 'verification_url'],
                'status' => 'published',
                'is_system' => true,
            ],

            // Thank You - Subscription
            [
                'name' => 'Thank You - Subscription',
                'slug' => 'thank-you-subscription',
                'description' => 'Sent after a successful subscription purchase',
                'category' => 'purchases',
                'email_type' => 'transactional',
                'subject' => 'Welcome to {{plan_name}} - Revolution Trading Pros',
                'preheader' => 'Your subscription is now active! Here\'s how to get started',
                'body_html' => $this->getThankYouSubscriptionHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true],
                    'plan_name' => ['type' => 'string', 'required' => true],
                    'price' => ['type' => 'number', 'required' => true],
                    'interval' => ['type' => 'string', 'required' => true],
                    'start_date' => ['type' => 'date', 'required' => true],
                    'next_payment_date' => ['type' => 'date', 'required' => true],
                    'features' => ['type' => 'array', 'required' => false],
                ],
                'required_variables' => ['user_name', 'plan_name', 'price', 'interval'],
                'status' => 'published',
                'is_system' => true,
            ],

            // Thank You - Indicator Purchase
            [
                'name' => 'Thank You - Indicator Purchase',
                'slug' => 'thank-you-indicator',
                'description' => 'Sent after purchasing a trading indicator',
                'category' => 'purchases',
                'email_type' => 'transactional',
                'subject' => 'Your Indicator Purchase - {{product_name}}',
                'preheader' => 'Your indicator is ready for download',
                'body_html' => $this->getThankYouIndicatorHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true],
                    'product_name' => ['type' => 'string', 'required' => true],
                    'order_number' => ['type' => 'string', 'required' => true],
                    'total' => ['type' => 'number', 'required' => true],
                    'purchase_date' => ['type' => 'date', 'required' => true],
                ],
                'required_variables' => ['user_name', 'product_name', 'order_number', 'total'],
                'status' => 'published',
                'is_system' => true,
            ],

            // Thank You - Course Purchase
            [
                'name' => 'Thank You - Course Enrollment',
                'slug' => 'thank-you-course',
                'description' => 'Sent after enrolling in a trading course',
                'category' => 'purchases',
                'email_type' => 'transactional',
                'subject' => 'Welcome to {{course_name}} - Your Course Access',
                'preheader' => 'Your course enrollment is confirmed! Start learning now',
                'body_html' => $this->getThankYouCourseHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true],
                    'course_name' => ['type' => 'string', 'required' => true],
                    'course_slug' => ['type' => 'string', 'required' => true],
                    'order_number' => ['type' => 'string', 'required' => true],
                    'total' => ['type' => 'number', 'required' => true],
                ],
                'required_variables' => ['user_name', 'course_name', 'order_number'],
                'status' => 'published',
                'is_system' => true,
            ],

            // Win-Back Campaign
            [
                'name' => 'Win-Back Campaign',
                'slug' => 'win-back',
                'description' => 'Re-engagement email for churned members with special offers',
                'category' => 'marketing',
                'email_type' => 'marketing',
                'subject' => 'We Miss You, {{user_name}}! Special Offer Inside',
                'preheader' => 'Come back with an exclusive discount just for you',
                'body_html' => $this->getWinBackHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true],
                    'previous_plan' => ['type' => 'string', 'required' => true],
                    'expired_date' => ['type' => 'date', 'required' => true],
                    'discount_percent' => ['type' => 'number', 'required' => false],
                    'offer_code' => ['type' => 'string', 'required' => false],
                    'expires_in_days' => ['type' => 'number', 'required' => false],
                    'community_size' => ['type' => 'number', 'required' => false],
                ],
                'required_variables' => ['user_name', 'previous_plan'],
                'status' => 'published',
                'is_system' => true,
            ],

            // Feedback Survey
            [
                'name' => 'Feedback Survey',
                'slug' => 'feedback-survey',
                'description' => 'Feedback request for churned members',
                'category' => 'marketing',
                'email_type' => 'marketing',
                'subject' => 'We Value Your Feedback - Revolution Trading Pros',
                'preheader' => 'Help us improve by sharing your experience',
                'body_html' => $this->getFeedbackSurveyHtml(),
                'variables' => [
                    'user_name' => ['type' => 'string', 'required' => true],
                    'survey_url' => ['type' => 'string', 'required' => true],
                    'incentive_description' => ['type' => 'string', 'required' => false],
                ],
                'required_variables' => ['user_name', 'survey_url'],
                'status' => 'published',
                'is_system' => true,
            ],
        ];

        foreach ($templates as $data) {
            EmailTemplate::updateOrCreate(
                ['slug' => $data['slug']],
                $data
            );
        }
    }

    private function getVerifyEmailHtml(): string
    {
        return <<<'HTML'
<h2>Welcome, {{user_name}}!</h2>

<p>Thank you for joining Revolution Trading Pros! We're excited to have you on board.</p>

<p>Please click the button below to verify your email address and unlock full access to your account:</p>

<div style="text-align: center; margin: 32px 0;">
    <a href="{{verification_url}}" class="btn-primary">Verify Email Address</a>
</div>

<div class="info-box">
    <p><strong>Why verify?</strong> Email verification helps us keep your account secure and ensures you receive important updates about your trading journey.</p>
</div>

<p>This verification link will expire in <strong>60 minutes</strong>.</p>

<p style="font-size: 14px; color: #6b7280;">
    If you didn't create an account with Revolution Trading Pros, please ignore this email.
</p>
HTML;
    }

    private function getThankYouSubscriptionHtml(): string
    {
        return <<<'HTML'
<h2>Thank You for Subscribing, {{user_name}}!</h2>

<div class="success-box">
    <p><strong>Your subscription to {{plan_name}} is now active!</strong></p>
</div>

<p>Welcome to our elite trading community! You've made an excellent decision to invest in your trading education.</p>

<div class="order-summary">
    <h3>Subscription Details</h3>
    <table width="100%" cellspacing="0" cellpadding="0">
        <tr>
            <td>Plan</td>
            <td style="text-align: right; font-weight: 600;">{{plan_name}}</td>
        </tr>
        <tr>
            <td>Billing Cycle</td>
            <td style="text-align: right;">{{interval}}</td>
        </tr>
        <tr>
            <td>Start Date</td>
            <td style="text-align: right;">{{start_date}}</td>
        </tr>
        <tr>
            <td>Next Billing Date</td>
            <td style="text-align: right;">{{next_payment_date}}</td>
        </tr>
        <tr>
            <td style="font-weight: 600; font-size: 18px;">Amount</td>
            <td style="text-align: right; font-weight: 600; font-size: 18px; color: #059669;">${{price}}/{{interval}}</td>
        </tr>
    </table>
</div>

<div style="text-align: center; margin: 32px 0;">
    <a href="{{app_url}}/dashboard" class="btn-primary">Go to Dashboard</a>
</div>

<p>Here's to your trading success!</p>
<p><strong>The Revolution Trading Pros Team</strong></p>
HTML;
    }

    private function getThankYouIndicatorHtml(): string
    {
        return <<<'HTML'
<h2>Thank You for Your Purchase, {{user_name}}!</h2>

<div class="success-box">
    <p><strong>Your purchase of {{product_name}} is complete!</strong></p>
</div>

<p>Congratulations! You now have access to one of our powerful trading indicators.</p>

<div class="order-summary">
    <h3>Order Summary</h3>
    <table width="100%" cellspacing="0" cellpadding="0">
        <tr>
            <td>Order Number</td>
            <td style="text-align: right; font-family: monospace;">{{order_number}}</td>
        </tr>
        <tr>
            <td>Product</td>
            <td style="text-align: right; font-weight: 600;">{{product_name}}</td>
        </tr>
        <tr>
            <td>Purchase Date</td>
            <td style="text-align: right;">{{purchase_date}}</td>
        </tr>
        <tr>
            <td style="font-weight: 600; font-size: 18px;">Total Paid</td>
            <td style="text-align: right; font-weight: 600; font-size: 18px; color: #059669;">${{total}}</td>
        </tr>
    </table>
</div>

<div style="text-align: center; margin: 32px 0;">
    <a href="{{app_url}}/dashboard/downloads" class="btn-primary">Download Now</a>
</div>

<div class="warning-box">
    <p><strong>Important:</strong> Your download link is tied to your account. Please do not share it with others.</p>
</div>

<p>Happy trading!</p>
<p><strong>The Revolution Trading Pros Team</strong></p>
HTML;
    }

    private function getThankYouCourseHtml(): string
    {
        return <<<'HTML'
<h2>Welcome to {{course_name}}, {{user_name}}!</h2>

<div class="success-box">
    <p><strong>Your enrollment in {{course_name}} is confirmed!</strong></p>
</div>

<p>You've made an excellent investment in your trading education. This course has transformed thousands of traders.</p>

<div class="order-summary">
    <h3>Enrollment Details</h3>
    <table width="100%" cellspacing="0" cellpadding="0">
        <tr>
            <td>Order Number</td>
            <td style="text-align: right; font-family: monospace;">{{order_number}}</td>
        </tr>
        <tr>
            <td>Course</td>
            <td style="text-align: right; font-weight: 600;">{{course_name}}</td>
        </tr>
        <tr>
            <td>Access</td>
            <td style="text-align: right;">Lifetime Access</td>
        </tr>
        <tr>
            <td style="font-weight: 600; font-size: 18px;">Total Paid</td>
            <td style="text-align: right; font-weight: 600; font-size: 18px; color: #059669;">${{total}}</td>
        </tr>
    </table>
</div>

<div style="text-align: center; margin: 32px 0;">
    <a href="{{app_url}}/courses/{{course_slug}}" class="btn-primary">Start Course</a>
</div>

<p>We're thrilled to have you as a student!</p>
<p><strong>The Revolution Trading Pros Team</strong></p>
HTML;
    }

    private function getWinBackHtml(): string
    {
        return <<<'HTML'
<h2>We Miss You, {{user_name}}!</h2>

<p>It's been a while since we've seen you at Revolution Trading Pros, and we wanted to reach out personally.</p>

<p>We noticed your <strong>{{previous_plan}}</strong> membership ended on <strong>{{expired_date}}</strong>, and we'd love to have you back in our trading community.</p>

{{#if discount_percent}}
<div style="background: linear-gradient(135deg, #059669 0%, #10b981 100%); border-radius: 12px; padding: 32px; margin: 32px 0; text-align: center;">
    <p style="color: rgba(255,255,255,0.9); font-size: 14px; text-transform: uppercase; margin: 0 0 8px 0;">Exclusive Comeback Offer</p>
    <p style="color: #ffffff; font-size: 36px; font-weight: 700; margin: 0 0 8px 0;">{{discount_percent}}% OFF</p>
    {{#if offer_code}}
    <p style="background: rgba(255,255,255,0.2); display: inline-block; padding: 8px 16px; border-radius: 4px; color: #ffffff; font-family: monospace; font-size: 18px; margin: 0;">{{offer_code}}</p>
    {{/if}}
</div>
{{/if}}

<div style="text-align: center; margin: 32px 0;">
    <a href="{{app_url}}/reactivate{{#if offer_code}}?code={{offer_code}}{{/if}}" class="btn-primary">Rejoin Now & Save</a>
</div>

{{#if expires_in_days}}
<div class="warning-box">
    <p><strong>Hurry!</strong> This offer expires in <strong>{{expires_in_days}} days</strong>.</p>
</div>
{{/if}}

<p>Looking forward to seeing you back,</p>
<p><strong>The Revolution Trading Pros Team</strong></p>
HTML;
    }

    private function getFeedbackSurveyHtml(): string
    {
        return <<<'HTML'
<h2>Hi {{user_name}},</h2>

<p>We noticed that your membership with Revolution Trading Pros recently ended, and we wanted to reach out personally.</p>

<p>Your experience matters to us, and we'd love to hear your honest feedback. Whether positive or constructive, your input helps us improve and serve our trading community better.</p>

<div class="info-box">
    <p>This quick survey takes less than <strong>2 minutes</strong> to complete.</p>
</div>

<div style="text-align: center; margin: 32px 0;">
    <a href="{{survey_url}}" class="btn-primary">Share Your Feedback</a>
</div>

<h3>Quick Questions</h3>
<p>If you prefer not to click through, just reply to this email with your thoughts on:</p>
<ol>
    <li><strong>What did you enjoy most</strong> about your membership?</li>
    <li><strong>What could we have done better</strong> during your time with us?</li>
    <li><strong>What was the main reason</strong> you decided to cancel?</li>
    <li><strong>Would you consider rejoining</strong> in the future?</li>
</ol>

{{#if incentive_description}}
<div class="success-box">
    <p><strong>Thank You Gift:</strong> Complete the survey and receive {{incentive_description}} as our way of saying thanks!</p>
</div>
{{/if}}

<p>Thank you for being part of our journey,</p>
<p><strong>The Revolution Trading Pros Team</strong></p>
HTML;
    }
}
