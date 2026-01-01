<x-mail::message>
# Your Trial Ends in {{ $daysRemaining }} Days

Hi {{ $user->name }},

Your free trial of **{{ $planName }}** is ending soon!

**Trial ends:** {{ $trialEndsAt->format('F j, Y') }}

@if($daysRemaining <= 1)
⏰ **Less than 24 hours remaining!**
@endif

<x-mail::button :url="$addPaymentUrl" color="primary">
Add Payment Method
</x-mail::button>

## Keep Your Access

To continue enjoying all the features you've been using during your trial, simply add a payment method before your trial ends.

**What happens when your trial ends?**

@if($subscription->plan)
- If you've added a payment method: Your subscription automatically continues
- If no payment method: You'll lose access to premium features
@endif

## What You'll Lose Without a Subscription

<x-mail::panel>
Don't miss out on all the great features you've been using!

Add a payment method now to ensure uninterrupted access.
</x-mail::panel>

Questions about pricing or features? We're here to help.

Thanks,<br>
{{ config('app.name') }}
</x-mail::message>
