@extends('emails.layouts.base')

@section('title', 'Your Course Access - Revolution Trading Pros')
@section('header-subtitle', 'Your Trading Education Starts Now')

@section('content')
    <h2>Welcome to {{ $course->name }}, {{ $user->name }}!</h2>

    <div class="success-box">
        <p><strong>Your enrollment in {{ $course->name }} is confirmed!</strong></p>
    </div>

    <p>You've made an excellent investment in your trading education. This course has transformed thousands of traders, and we're confident it will do the same for you.</p>

    <div class="order-summary">
        <h3 style="margin: 0 0 16px 0; font-size: 18px; color: #1e3a5f;">Enrollment Details</h3>
        <table width="100%" cellspacing="0" cellpadding="0">
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Order Number</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; font-family: monospace;">{{ $order->order_number }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Course</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; font-weight: 600;">{{ $course->name }}</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Access</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">Lifetime Access</td>
            </tr>
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Enrollment Date</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right;">{{ $order->created_at->format('M d, Y') }}</td>
            </tr>
            @if($order->discount_amount > 0)
            <tr>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb;">Discount Applied</td>
                <td style="padding: 8px 0; border-bottom: 1px solid #e5e7eb; text-align: right; color: #059669;">-${{ number_format($order->discount_amount, 2) }}</td>
            </tr>
            @endif
            <tr>
                <td style="padding: 16px 0 8px 0; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb;">Total Paid</td>
                <td style="padding: 16px 0 8px 0; text-align: right; font-weight: 600; font-size: 18px; border-top: 2px solid #e5e7eb; color: #059669;">${{ number_format($order->total, 2) }}</td>
            </tr>
        </table>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Course Overview</h3>

    @if($course->description)
    <p>{{ $course->description }}</p>
    @endif

    <ul class="feature-list">
        <li>{{ $course->lesson_count ?? 'Multiple' }} in-depth video lessons</li>
        <li>Downloadable resources and cheat sheets</li>
        <li>Real-world trading examples</li>
        <li>Quizzes to test your knowledge</li>
        <li>Certificate of completion</li>
        <li>Access to course community</li>
    </ul>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Start Learning Now</h3>

    <p>Your course is ready and waiting. Click below to begin your first lesson:</p>

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ config('app.url') }}/courses/{{ $course->slug }}" class="btn-primary">Start Course</a>
    </div>

    <h3 style="color: #1e3a5f; margin: 32px 0 16px 0;">Tips for Success</h3>

    <ol style="padding-left: 20px; margin: 16px 0;">
        <li style="margin-bottom: 8px;"><strong>Set a schedule</strong> - Dedicate specific times for learning</li>
        <li style="margin-bottom: 8px;"><strong>Take notes</strong> - Write down key concepts and strategies</li>
        <li style="margin-bottom: 8px;"><strong>Practice</strong> - Apply what you learn in a demo account</li>
        <li style="margin-bottom: 8px;"><strong>Ask questions</strong> - Use the community to clarify doubts</li>
        <li style="margin-bottom: 8px;"><strong>Be patient</strong> - Trading mastery takes time and dedication</li>
    </ol>

    <div class="info-box">
        <p><strong>Exclusive Bonus:</strong> As a course student, you get access to our private Discord channel where you can interact with instructors and fellow students.</p>
    </div>

    <div style="background: linear-gradient(135deg, #1e3a5f 0%, #2563eb 100%); border-radius: 8px; padding: 24px; margin: 32px 0; text-align: center;">
        <p style="color: #ffffff; font-size: 18px; margin: 0 0 16px 0; font-weight: 600;">Join the Community</p>
        <a href="{{ config('app.url') }}/discord" class="btn-secondary" style="background: #ffffff; color: #1e3a5f !important;">Join Discord Server</a>
    </div>

    <p>We're thrilled to have you as a student and can't wait to see your trading journey unfold!</p>

    <p style="margin-top: 24px;">
        <strong>The Revolution Trading Pros Team</strong>
    </p>
@endsection
