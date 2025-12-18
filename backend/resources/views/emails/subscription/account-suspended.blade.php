<x-mail::message>
# Your Account Has Been Suspended

Hi {{ $user->name }},

Due to continued payment failures, your {{ $planName }} subscription has been suspended.

**Your access to premium features has been restricted.**

<x-mail::button :url="$reactivateUrl" color="primary">
Reactivate My Account
</x-mail::button>

## What this means

- You no longer have access to premium features
- Your data is safe and preserved
- You have **{{ $gracePeriodDays }} days** to reactivate before permanent closure

## How to reactivate

1. Click the "Reactivate My Account" button above
2. Update your payment method
3. Your subscription will be restored immediately

<x-mail::panel>
**Grace Period:** Your data will be preserved for {{ $gracePeriodDays }} days. After this period, your account may be permanently deleted.
</x-mail::panel>

If you've decided to cancel your subscription, no action is needed. However, if this suspension was unexpected, please contact us immediately.

<x-mail::button :url="$updatePaymentUrl" color="secondary">
Update Payment Method
</x-mail::button>

We hope to see you back soon.

Thanks,<br>
{{ config('app.name') }}
</x-mail::message>
