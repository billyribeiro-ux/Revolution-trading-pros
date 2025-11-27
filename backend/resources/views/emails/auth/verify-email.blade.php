@extends('emails.layouts.base')

@section('title', 'Verify Your Email - Revolution Trading Pros')
@section('header-subtitle', 'Welcome to the Trading Revolution')

@section('content')
    <h2>Welcome, {{ $user->name }}!</h2>

    <p>Thank you for joining Revolution Trading Pros! We're excited to have you on board.</p>

    <p>Please click the button below to verify your email address and unlock full access to your account:</p>

    <div style="text-align: center; margin: 32px 0;">
        <a href="{{ $verificationUrl }}" class="btn-primary">Verify Email Address</a>
    </div>

    <div class="info-box">
        <p><strong>Why verify?</strong> Email verification helps us keep your account secure and ensures you receive important updates about your trading journey.</p>
    </div>

    <p>This verification link will expire in <strong>60 minutes</strong>.</p>

    <p style="font-size: 14px; color: #6b7280;">
        If the button doesn't work, copy and paste this link into your browser:<br>
        <a href="{{ $verificationUrl }}" style="word-break: break-all; color: #2563eb;">{{ $verificationUrl }}</a>
    </p>

    <hr style="border: none; border-top: 1px solid #e5e7eb; margin: 32px 0;">

    <p style="font-size: 14px; color: #6b7280;">
        If you didn't create an account with Revolution Trading Pros, please ignore this email or <a href="{{ config('app.url') }}/contact">contact support</a> if you have concerns.
    </p>
@endsection
