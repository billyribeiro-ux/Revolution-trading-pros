<x-mail::message>
# Welcome to {{ $planName }}! 🎉

Hi {{ $user->name }},

Thank you for subscribing to **{{ $planName }}**! We're thrilled to have you on board.

<x-mail::button :url="$dashboardUrl" color="primary">
Go to Dashboard
</x-mail::button>

## You Now Have Access To:

@if(is_array($features) && count($features) > 0)
@foreach($features as $feature => $value)
@if($value === true || $value > 0)
✅ {{ ucwords(str_replace('_', ' ', $feature)) }}
@endif
@endforeach
@else
✅ All premium features included in your plan
@endif

## Getting Started

Here are some quick tips to help you get the most out of your subscription:

1. **Complete your profile** - Add your details to personalize your experience
2. **Explore the dashboard** - Familiarize yourself with all the features
3. **Check out our guides** - Visit our help center for tutorials and tips

## Need Help?

Our support team is here for you. Don't hesitate to reach out if you have any questions.

<x-mail::panel>
**Pro Tip:** Bookmark your dashboard for quick access!
</x-mail::panel>

Thanks for choosing us!<br>
{{ config('app.name') }}

---

<small>
You can manage your subscription at any time from your account settings.
</small>
</x-mail::message>
