<?php

namespace App\Services\Email;

use Illuminate\Support\Str;

/**
 * Variable Resolver Service
 * RevolutionEmailTemplates-L8-System
 * 
 * Resolves dynamic variables in email templates
 */
class VariableResolverService
{
    /**
     * Resolve all variables in text
     */
    public function resolve(string $text, array $context): string
    {
        // Find all {{variable}} patterns
        preg_match_all('/\{\{([^}]+)\}\}/', $text, $matches);
        
        foreach ($matches[1] as $index => $variable) {
            $variable = trim($variable);
            $value = $this->resolveVariable($variable, $context);
            $text = str_replace($matches[0][$index], $value, $text);
        }
        
        return $text;
    }

    /**
     * Resolve single variable
     */
    public function resolveVariable(string $variable, array $context)
    {
        // Handle dot notation (user.name, order.total, etc.)
        $parts = explode('.', $variable);
        $value = $context;
        
        foreach ($parts as $part) {
            if (is_array($value) && isset($value[$part])) {
                $value = $value[$part];
            } elseif (is_object($value) && isset($value->$part)) {
                $value = $value->$part;
            } else {
                return $this->getDefaultValue($variable);
            }
        }
        
        return $this->formatValue($value);
    }

    /**
     * Get default value for variable
     */
    private function getDefaultValue(string $variable)
    {
        return match($variable) {
            'site.name' => config('app.name'),
            'site.url' => config('app.url'),
            'site.support_email' => config('mail.from.address'),
            'current_year' => date('Y'),
            'unsubscribe_url' => config('app.url') . '/unsubscribe',
            'preferences_url' => config('app.url') . '/preferences',
            default => "{{$variable}}",
        };
    }

    /**
     * Format value for display
     */
    private function formatValue($value): string
    {
        if (is_bool($value)) {
            return $value ? 'Yes' : 'No';
        }
        
        if (is_array($value)) {
            return implode(', ', $value);
        }
        
        if (is_object($value) && method_exists($value, '__toString')) {
            return (string) $value;
        }
        
        return (string) $value;
    }

    /**
     * Get available variables for template
     */
    public function getAvailableVariables(string $category = null): array
    {
        $variables = [
            'user' => [
                'user.name' => 'User Full Name',
                'user.first_name' => 'User First Name',
                'user.last_name' => 'User Last Name',
                'user.email' => 'User Email',
                'user.username' => 'Username',
                'user.avatar' => 'User Avatar URL',
                'user.created_at' => 'Account Created Date',
            ],
            'order' => [
                'order.id' => 'Order ID',
                'order.number' => 'Order Number',
                'order.total' => 'Order Total',
                'order.subtotal' => 'Order Subtotal',
                'order.tax' => 'Order Tax',
                'order.shipping' => 'Shipping Cost',
                'order.status' => 'Order Status',
                'order.tracking_number' => 'Tracking Number',
                'order.estimated_delivery' => 'Estimated Delivery Date',
            ],
            'subscription' => [
                'subscription.plan' => 'Subscription Plan Name',
                'subscription.price' => 'Subscription Price',
                'subscription.billing_cycle' => 'Billing Cycle',
                'subscription.next_billing_date' => 'Next Billing Date',
                'subscription.status' => 'Subscription Status',
                'subscription.features' => 'Plan Features',
            ],
            'system' => [
                'site.name' => 'Site Name',
                'site.url' => 'Site URL',
                'site.logo' => 'Site Logo URL',
                'site.support_email' => 'Support Email',
                'site.phone' => 'Support Phone',
                'current_year' => 'Current Year',
                'unsubscribe_url' => 'Unsubscribe URL',
                'preferences_url' => 'Preferences URL',
            ],
        ];
        
        if ($category) {
            return $variables[$category] ?? [];
        }
        
        return $variables;
    }
}
