@extends('emails.layouts.base')

@section('title', 'We Miss You! - Revolution Trading Pros')
@section('header-subtitle', 'A Special Offer Just For You')

@section('content')
    <h2>We Miss You, {{ $user->name }}!</h2>

    <p>It's been a while since we've seen you at Revolution Trading Pros, and we wanted to reach out personally.</p>

    <p>We noticed your <strong>{{ $previousPlan->name }}</strong> membership ended on <strong>{{ $membership->expires_at->format('F d, Y') }}</strong>, and we'd love to have you back in our trading community.</p>

    @if($offer)
    <div style="background: linear-gradient(135deg, #059669 0%, #10b981 100%); border-radius: 12px; padding: 32px; margin: 32px 0; text-align: center;">
        <p style="color: rgba(255,255,255,0.9); font-size: 14px; text-transform: uppercase; letter-spacing: 1px; margin: 0 0 8px 0;">Exclusive Comeback Offer</p>
        <p style="color: #ffffff; font-size: 36px; font-weight: 700; margin: 0 0 8px 0;">{{ $offer->discount_percent }}% OFF</p>
        <p style="color: rgba(255,255,255,0.9); font-size: 16px; margin: 0 0 16px 0;">Your first {{ $offer->discount_months }} months</p>
        <p style="background: rgba(255,255,255,0.2); display: inline-block; padding: 8px 16px; border-radius: 4px; color: #ffffff; font-family: monospace; font-size: 18px; letter-spacing: 2px; margin: 0;">{{ $offer->code }}</p>
    </div>
    @endif

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Here's What You've Been Missing</h3>

    <ul class="feature-list">
        <li>New advanced trading strategies added weekly</li>
        <li>Updated indicators with enhanced accuracy</li>
        <li>Growing Discord community (now {{ $communitySize ?? '5,000+' }} members!)</li>
        <li>Live trading sessions every weekday</li>
        <li>Exclusive market analysis and trade setups</li>
    </ul>

    @if($testimonial)
    <div style="background: #f9fafb; border-radius: 8px; padding: 24px; margin: 32px 0; border-left: 4px solid #2563eb;">
        <p style="font-style: italic; font-size: 16px; margin: 0 0 12px 0; color: #374151;">"{{ $testimonial->content }}"</p>
        <p style="margin: 0; color: #6b7280; font-size: 14px;"><strong>{{ $testimonial->author }}</strong> - {{ $testimonial->title }}</p>
    </div>
    @endif

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ config('app.url') }}/reactivate?code={{ $offer->code ?? '' }}&user={{ $user->id }}" class="btn-primary">Rejoin Now & Save</a>
    </div>

    @if($offer)
    <div class="warning-box">
        <p><strong>Hurry!</strong> This offer expires in <strong>{{ $offer->expires_in_days }} days</strong> and is limited to former members only.</p>
    </div>
    @endif

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Why Come Back?</h3>

    <table width="100%" cellspacing="0" cellpadding="0">
        <tr>
            <td style="padding: 16px; background: #f0f9ff; border-radius: 8px; margin-bottom: 12px;">
                <p style="margin: 0; font-weight: 600; color: #1e3a5f;">Continuous Improvement</p>
                <p style="margin: 8px 0 0 0; color: #374151; font-size: 14px;">We've added 50+ new lessons and updated our core strategies based on current market conditions.</p>
            </td>
        </tr>
        <tr><td style="height: 12px;"></td></tr>
        <tr>
            <td style="padding: 16px; background: #f0fdf4; border-radius: 8px;">
                <p style="margin: 0; font-weight: 600; color: #047857;">Proven Results</p>
                <p style="margin: 8px 0 0 0; color: #374151; font-size: 14px;">Our members have reported an average 40% improvement in their trading performance.</p>
            </td>
        </tr>
        <tr><td style="height: 12px;"></td></tr>
        <tr>
            <td style="padding: 16px; background: #fef3c7; border-radius: 8px;">
                <p style="margin: 0; font-weight: 600; color: #92400e;">Risk-Free Trial</p>
                <p style="margin: 8px 0 0 0; color: #374151; font-size: 14px;">Not sure? Try us again for 7 days free. Cancel anytime if it's not for you.</p>
            </td>
        </tr>
    </table>

    <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 32px 0;">

    <p style="font-size: 14px; color: #6b7280;">
        Have questions before rejoining? Simply reply to this email - we're here to help!
    </p>

    <p style="margin-top: 24px;">
        Looking forward to seeing you back,<br><br>
        <strong>The Revolution Trading Pros Team</strong>
    </p>
@endsection
