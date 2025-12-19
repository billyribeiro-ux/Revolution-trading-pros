@extends('emails.layouts.base')

@section('title', 'Your Indicator Purchase - Revolution Trading Pros')
@section('header-subtitle', 'Your New Trading Tool is Ready')

@section('content')
    <h2>Thank You for Your Purchase, {{ $user->name }}!</h2>

    <div class="success-box">
        <p><strong>Your purchase of {{ $product->name }} is complete!</strong></p>
    </div>

    <p>Congratulations! You now have access to one of our powerful trading indicators. This tool has helped thousands of traders improve their market analysis.</p>

    <div class="order-summary">
        <h3 style="margin: 0 0 16px 0; font-size: 18px; color: #1e3a5f;">Order Summary</h3>
        <table width="100%" cellspacing="0" cellpadding="0">
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Order Number</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; font-family: monospace;">{{ $order->order_number }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Product</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; font-weight: 600;">{{ $product->name }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Purchase Date</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">{{ $order->created_at->format('M d, Y') }}</td>
            </tr>
            @if($order->discount_amount > 0)
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Discount</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; color: #059669;">-${{ number_format($order->discount_amount, 2) }}</td>
            </tr>
            @endif
            <tr>
                <td style="padding: 16px 0 8px 0; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb;">Total Paid</td>
                <td style="padding: 16px 0 8px 0; text-align: right; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb; color: #059669;">${{ number_format($order->total, 2) }}</td>
            </tr>
        </table>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Download Your Indicator</h3>

    <p>Your indicator is ready for download. Click the button below to access your files and installation guide:</p>

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ config('app.url') }}/dashboard/downloads" class="btn-primary">Download Now</a>
    </div>

    <div class="warning-box">
        <p><strong>Important:</strong> Your download link is tied to your account. Please do not share it with others. Each license is for single-user use only.</p>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Installation Guide</h3>

    <ol style="padding-left: 20px; margin: 16px 0;">
        <li style="margin-bottom: 8px;"><strong>Download</strong> the indicator file from your dashboard</li>
        <li style="margin-bottom: 8px;"><strong>Open</strong> your trading platform (TradingView/MT4/MT5)</li>
        <li style="margin-bottom: 8px;"><strong>Import</strong> the indicator following our step-by-step guide</li>
        <li style="margin-bottom: 8px;"><strong>Configure</strong> the settings based on your trading style</li>
    </ol>

    <p>Check out our <a href="{{ config('app.url') }}/tutorials/indicator-setup">detailed setup tutorial</a> for platform-specific instructions.</p>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">What's Included</h3>

    <ul class="feature-list">
        <li>{{ $product->name }} indicator file</li>
        <li>Installation guide (PDF)</li>
        <li>Video walkthrough</li>
        <li>Strategy guide with best practices</li>
        <li>Lifetime updates included</li>
        <li>30-day money-back guarantee</li>
    </ul>

    <div class="info-box">
        <p><strong>Questions?</strong> Our support team is here to help with installation and setup. Reply to this email or visit our <a href="{{ config('app.url') }}/support">support center</a>.</p>
    </div>

    <p>Happy trading!</p>

    <p style="margin-top: 24px;">
        <strong>The Revolution Trading Pros Team</strong>
    </p>
@endsection
