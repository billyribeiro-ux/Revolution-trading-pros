<x-mail::message>
# Your Trial Has Expired

Hi {{ $user->name }},

Your free trial of **{{ $planName }}** has ended.

We hope you enjoyed exploring our platform! To continue using all the features you experienced during your trial, upgrade to a paid plan today.

<x-mail::button :url="$upgradeUrl" color="primary">
Upgrade Now
</x-mail::button>

## What You're Missing

Without an active subscription, you no longer have access to:

- Premium features
- Priority support
- Advanced analytics
- And much more...

## Special Offer

As a thank you for trying us out, here's a special offer:

<x-mail::panel>
🎉 **Get 20% off your first month!**

Use code **WELCOME20** at checkout.
</x-mail::panel>

We'd love to have you as a member. If you have any questions about our plans or need help choosing the right one for you, just reply to this email.

Thanks for trying us out,<br>
{{ config('app.name') }}
</x-mail::message>
