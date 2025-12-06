<x-mail::message>
# ⚠️ Final Warning: Account Suspension Imminent

Hi {{ $user->name }},

**This is your final warning.** We have been unable to process payment for your {{ $planName }} subscription after multiple attempts.

Your account will be **suspended in {{ $daysUntilSuspension }} days** if we cannot successfully process your payment.

<x-mail::button :url="$updatePaymentUrl" color="error">
Update Payment Method Now
</x-mail::button>

## What happens when your account is suspended?

- You will lose access to all premium features
- Your data will be preserved for 14 days
- After 14 days, your account may be permanently closed

## How to prevent suspension

1. Click the button above to update your payment method
2. Ensure your card has sufficient funds
3. Check that your billing information is correct

We don't want to lose you as a customer. If you're experiencing any issues, please reach out to our support team immediately.

<x-mail::panel>
**Need help?** Reply to this email or contact our support team at {{ config('mail.from.address') }}
</x-mail::panel>

Thanks,<br>
{{ config('app.name') }}
</x-mail::message>
