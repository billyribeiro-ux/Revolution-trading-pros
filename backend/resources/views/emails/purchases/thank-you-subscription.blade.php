@extends('emails.layouts.base')

@section('title', 'Welcome to {{ $plan->name }} - Revolution Trading Pros')
@section('header-subtitle', 'Your Trading Journey Begins Now')

@section('content')
    <h2>Thank You for Subscribing, {{ $user->name }}!</h2>

    <div class="success-box">
        <p><strong>Your subscription to {{ $plan->name }} is now active!</strong></p>
    </div>

    <p>Welcome to our elite trading community! You've made an excellent decision to invest in your trading education.</p>

    <div class="order-summary">
        <h3 style="margin: 0 0 16px 0; font-size: 18px; color: #1e3a5f;">Subscription Details</h3>
        <table width="100%" cellspacing="0" cellpadding="0">
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Plan</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; font-weight: 600;">{{ $plan->name }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Billing Cycle</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">{{ ucfirst($subscription->interval) }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Start Date</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">{{ $subscription->start_date->format('M d, Y') }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Next Billing Date</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">{{ $subscription->next_payment_date->format('M d, Y') }}</td>
            </tr>
            <tr>
                <td style="padding: 16px 0 8px 0; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb;">Amount</td>
                <td style="padding: 16px 0 8px 0; text-align: right; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb; color: #059669;">${{ number_format($subscription->price, 2) }}/{{ $subscription->interval }}</td>
            </tr>
        </table>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">What's Included</h3>

    <ul class="feature-list">
        @foreach($plan->features ?? ['Access to all trading courses', 'Live trading signals', 'Private Discord community', 'Weekly market analysis', '24/7 support'] as $feature)
            <li>{{ $feature }}</li>
        @endforeach
    </ul>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Get Started</h3>

    <p>Here's how to make the most of your membership:</p>

    <ol style="padding-left: 20px; margin: 16px 0;">
        <li style="margin-bottom: 8px;"><strong>Join our Discord</strong> - Connect with fellow traders and get real-time support</li>
        <li style="margin-bottom: 8px;"><strong>Watch the Starter Guide</strong> - Get oriented with our platform and tools</li>
        <li style="margin-bottom: 8px;"><strong>Set up Alerts</strong> - Never miss a trading signal or opportunity</li>
        <li style="margin-bottom: 8px;"><strong>Book a 1-on-1</strong> - Schedule your welcome call with a mentor</li>
    </ol>

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ config('app.url') }}/dashboard" class="btn-primary">Go to Dashboard</a>
    </div>

    <div class="info-box">
        <p><strong>Need help?</strong> Reply to this email or visit our <a href="{{ config('app.url') }}/support">support center</a> anytime. We're here to help you succeed!</p>
    </div>

    <p>Here's to your trading success!</p>

    <p style="margin-top: 24px;">
        <strong>The Revolution Trading Pros Team</strong>
    </p>
@endsection
