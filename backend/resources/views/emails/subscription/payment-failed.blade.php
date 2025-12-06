<x-mail::message>
# Payment Failed

Hi {{ $user->name }},

We were unable to process your payment of **{{ $currency }} {{ $amount }}**.

@if($attemptCount === 1)
This is the first failed attempt. Please update your payment method to avoid any service interruption.
@elseif($attemptCount === 2)
This is your second payment attempt. Please update your payment information as soon as possible to maintain access to your account.
@elseif($attemptCount >= 3)
**This is urgent.** Your account may be suspended if we cannot process your payment. Please update your payment method immediately.
@endif

<x-mail::button :url="$updatePaymentUrl" color="primary">
Update Payment Method
</x-mail::button>

## What happens next?

@if($attemptCount < 3)
- We will automatically retry the payment in a few days
- You will continue to have access to your account
- Update your payment method to avoid further issues
@else
- **Your account will be suspended** if payment is not received
- You may lose access to premium features
- Update your payment method now to prevent suspension
@endif

If you have any questions or need assistance, please contact our support team.

Thanks,<br>
{{ config('app.name') }}

---

<small>
If you believe this is an error or you've already updated your payment information, please ignore this email. Your next payment attempt is scheduled automatically.
</small>
</x-mail::message>
