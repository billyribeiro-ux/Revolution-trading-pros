@extends('emails.layouts.base')

@section('title', 'We Value Your Feedback - Revolution Trading Pros')
@section('header-subtitle', 'Help Us Improve')

@section('content')
    <h2>Hi {{ $user->name }},</h2>

    <p>We noticed that your membership with Revolution Trading Pros recently ended, and we wanted to reach out personally.</p>

    <p>Your experience matters to us, and we'd love to hear your honest feedback. Whether positive or constructive, your input helps us improve and serve our trading community better.</p>

    <div class="info-box">
        <p>This quick survey takes less than <strong>2 minutes</strong> to complete.</p>
    </div>

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ $surveyUrl }}" class="btn-primary">Share Your Feedback</a>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Quick Questions</h3>

    <p>If you prefer not to click through, just reply to this email with your thoughts on:</p>

    <ol style="padding-left: 20px; margin: 16px 0; color: #374151;">
        <li style="margin-bottom: 12px;"><strong>What did you enjoy most</strong> about your membership?</li>
        <li style="margin-bottom: 12px;"><strong>What could we have done better</strong> during your time with us?</li>
        <li style="margin-bottom: 12px;"><strong>What was the main reason</strong> you decided to cancel?</li>
        <li style="margin-bottom: 12px;"><strong>Would you consider rejoining</strong> in the future? If so, what would bring you back?</li>
        <li style="margin-bottom: 12px;"><strong>Any additional suggestions</strong> or feedback you'd like to share?</li>
    </ol>

    <div style="background: #f9fafb; border-radius: 8px; padding: 24px; margin: 32px 0;">
        <h4 style="margin: 0 0 16px 0; color: #1e3a5f;">How would you rate your overall experience?</h4>
        <table width="100%" cellspacing="0" cellpadding="0">
            <tr>
                @foreach([1, 2, 3, 4, 5] as $rating)
                <td style="text-align: center;">
                    <a href="{{ $surveyUrl }}?rating={{ $rating }}" style="text-decoration: none; display: inline-block; width: 48px; height: 48px; line-height: 48px; background: {{ $rating <= 2 ? '#fecaca' : ($rating == 3 ? '#fef3c7' : '#bbf7d0') }}; border-radius: 50%; font-size: 20px;">
                        {{ $rating == 1 ? '😞' : ($rating == 2 ? '😐' : ($rating == 3 ? '🙂' : ($rating == 4 ? '😊' : '🤩'))) }}
                    </a>
                    <p style="margin: 8px 0 0 0; font-size: 12px; color: #6b7280;">{{ $rating }}</p>
                </td>
                @endforeach
            </tr>
        </table>
    </div>

    @if($incentive)
    <div class="success-box">
        <p><strong>Thank You Gift:</strong> Complete the survey and receive {{ $incentive->description }} as our way of saying thanks for your time!</p>
    </div>
    @endif

    <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 32px 0;">

    <p style="font-size: 14px; color: #6b7280;">
        Your feedback is completely confidential and will only be used to improve our services. We read every response personally.
    </p>

    <p style="margin-top: 24px;">
        Thank you for being part of our journey,<br><br>
        <strong>The Revolution Trading Pros Team</strong>
    </p>

    <p style="font-size: 12px; color: #9ca3af; margin-top: 24px;">
        P.S. Even if you're not returning, we genuinely appreciate the time you spent with us and wish you the best in your trading journey!
    </p>
@endsection
