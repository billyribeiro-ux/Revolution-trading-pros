<x-mail::message>
# {{ $isUpcoming ? 'Payment Retry Scheduled' : 'Payment Retry Notice' }}

Hi {{ $user->name }},

@if($isUpcoming)
We wanted to let you know that we will be retrying your payment **tomorrow** for your {{ $planName }} subscription.

Please make sure your payment method is up to date to avoid any service interruption.
@else
We just attempted to process your payment for your {{ $planName }} subscription, but it was unsuccessful.

This was attempt #{{ $attemptNumber }}.
@endif

<x-mail::button :url="$updatePaymentUrl" color="primary">
Update Payment Method
</x-mail::button>

## Subscription Details

- **Plan:** {{ $planName }}
- **Status:** Payment pending
- **Attempt:** {{ $attemptNumber }} of 4

@if($attemptNumber >= 3)
<x-mail::panel>
⚠️ **Important:** After 4 failed attempts, your account will be suspended. Please update your payment method to avoid losing access.
</x-mail::panel>
@endif

If you need help or have questions about your subscription, please reach out to our support team.

Thanks,<br>
{{ config('app.name') }}
</x-mail::message>
