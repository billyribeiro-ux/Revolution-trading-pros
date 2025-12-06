<?php

declare(strict_types=1);

namespace App\Services\Subscription;

use App\Models\User;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

/**
 * Tax Calculator Service (ICT9+ Enterprise Grade)
 *
 * Handles tax calculation for subscriptions:
 * - VAT for EU customers
 * - Sales tax for US customers
 * - GST for AU/NZ customers
 * - Tax exemptions
 * - Reverse charge mechanism
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class TaxCalculator
{
    /**
     * EU VAT rates by country (2024)
     */
    private const EU_VAT_RATES = [
        'AT' => 20, 'BE' => 21, 'BG' => 20, 'HR' => 25, 'CY' => 19,
        'CZ' => 21, 'DK' => 25, 'EE' => 22, 'FI' => 24, 'FR' => 20,
        'DE' => 19, 'GR' => 24, 'HU' => 27, 'IE' => 23, 'IT' => 22,
        'LV' => 21, 'LT' => 21, 'LU' => 17, 'MT' => 18, 'NL' => 21,
        'PL' => 23, 'PT' => 23, 'RO' => 19, 'SK' => 20, 'SI' => 22,
        'ES' => 21, 'SE' => 25,
    ];

    /**
     * US states with sales tax
     */
    private const US_SALES_TAX_STATES = [
        'CA' => 7.25, 'TX' => 6.25, 'NY' => 4.00, 'FL' => 6.00,
        'WA' => 6.50, 'PA' => 6.00, 'IL' => 6.25, 'OH' => 5.75,
        'GA' => 4.00, 'NC' => 4.75, 'MI' => 6.00, 'NJ' => 6.625,
        'VA' => 5.30, 'AZ' => 5.60, 'MA' => 6.25, 'TN' => 7.00,
        'IN' => 7.00, 'MO' => 4.225, 'MD' => 6.00, 'WI' => 5.00,
        'CO' => 2.90, 'MN' => 6.875, 'SC' => 6.00, 'AL' => 4.00,
        'LA' => 4.45, 'KY' => 6.00, 'OK' => 4.50, 'CT' => 6.35,
        'IA' => 6.00, 'UT' => 4.85, 'NV' => 6.85, 'AR' => 6.50,
        'MS' => 7.00, 'KS' => 6.50, 'NE' => 5.50, 'NM' => 5.125,
        'WV' => 6.00, 'ID' => 6.00, 'HI' => 4.00, 'ME' => 5.50,
        'RI' => 7.00, 'SD' => 4.50, 'ND' => 5.00, 'VT' => 6.00,
        'WY' => 4.00,
        // No sales tax states: DE, MT, NH, OR (handled as 0)
    ];

    /**
     * Countries with GST
     */
    private const GST_RATES = [
        'AU' => 10,
        'NZ' => 15,
        'SG' => 9,
        'IN' => 18,
        'CA' => 5, // Federal GST (HST varies by province)
    ];

    /**
     * Business tax ID (our company)
     */
    private string $businessCountry;
    private ?string $businessVatId;

    public function __construct()
    {
        $this->businessCountry = config('subscription.business_country', 'US');
        $this->businessVatId = config('subscription.business_vat_id');
    }

    /**
     * Calculate tax for an amount
     */
    public function calculate(
        int $amountInCents,
        string $country,
        ?string $state = null,
        ?string $taxId = null,
        string $productType = 'digital_service'
    ): TaxResult {
        $country = strtoupper($country);
        $state = $state ? strtoupper($state) : null;

        // Check for B2B reverse charge (EU)
        if ($this->isEuCountry($country) && $taxId && $this->validateVatId($taxId)) {
            return new TaxResult(
                amount: 0,
                rate: 0,
                type: 'reverse_charge',
                country: $country,
                description: 'B2B Reverse Charge - Customer responsible for VAT',
                taxId: $taxId,
            );
        }

        // Calculate applicable tax
        $rate = $this->getTaxRate($country, $state, $productType);
        $taxAmount = (int) round($amountInCents * ($rate / 100));

        return new TaxResult(
            amount: $taxAmount,
            rate: $rate,
            type: $this->getTaxType($country),
            country: $country,
            state: $state,
            description: $this->getTaxDescription($country, $rate),
        );
    }

    /**
     * Calculate tax for a subscription
     */
    public function calculateForSubscription(
        User $user,
        int $amountInCents,
        string $productType = 'digital_service'
    ): TaxResult {
        $country = $user->billing_country ?? $user->country ?? 'US';
        $state = $user->billing_state ?? $user->state;
        $taxId = $user->tax_id ?? $user->vat_id;

        return $this->calculate($amountInCents, $country, $state, $taxId, $productType);
    }

    /**
     * Get applicable tax rate
     */
    public function getTaxRate(
        string $country,
        ?string $state = null,
        string $productType = 'digital_service'
    ): float {
        // Digital services may have different tax treatment
        if ($productType === 'digital_service') {
            return $this->getDigitalServiceTaxRate($country, $state);
        }

        return $this->getStandardTaxRate($country, $state);
    }

    /**
     * Get tax rate for digital services
     */
    private function getDigitalServiceTaxRate(string $country, ?string $state = null): float
    {
        // EU: Digital services taxed at customer's location
        if (isset(self::EU_VAT_RATES[$country])) {
            return (float) self::EU_VAT_RATES[$country];
        }

        // US: Varies by state
        if ($country === 'US' && $state) {
            return (float) (self::US_SALES_TAX_STATES[$state] ?? 0);
        }

        // GST countries
        if (isset(self::GST_RATES[$country])) {
            return (float) self::GST_RATES[$country];
        }

        // UK VAT
        if ($country === 'GB') {
            return 20.0;
        }

        // Default: no tax
        return 0.0;
    }

    /**
     * Get standard tax rate
     */
    private function getStandardTaxRate(string $country, ?string $state = null): float
    {
        return $this->getDigitalServiceTaxRate($country, $state);
    }

    /**
     * Check if country is in EU
     */
    public function isEuCountry(string $country): bool
    {
        return isset(self::EU_VAT_RATES[$country]);
    }

    /**
     * Validate VAT ID
     */
    public function validateVatId(string $vatId): bool
    {
        $vatId = preg_replace('/\s+/', '', strtoupper($vatId));

        if (strlen($vatId) < 4) {
            return false;
        }

        $countryCode = substr($vatId, 0, 2);
        $number = substr($vatId, 2);

        // Check if it's a valid EU country code
        if (!$this->isEuCountry($countryCode)) {
            return false;
        }

        // Use VIES API for real validation
        return $this->validateVatIdWithVies($countryCode, $number);
    }

    /**
     * Validate VAT ID using EU VIES service
     */
    private function validateVatIdWithVies(string $countryCode, string $vatNumber): bool
    {
        $cacheKey = "vat:valid:{$countryCode}{$vatNumber}";

        return Cache::remember($cacheKey, 86400, function () use ($countryCode, $vatNumber) {
            try {
                $response = Http::timeout(10)->get('https://ec.europa.eu/taxation_customs/vies/rest-api/ms/{country}/vat/{number}', [
                    'country' => $countryCode,
                    'number' => $vatNumber,
                ]);

                if ($response->successful()) {
                    $data = $response->json();
                    return $data['isValid'] ?? false;
                }

                // Fallback: basic format validation
                return $this->validateVatIdFormat($countryCode, $vatNumber);
            } catch (\Throwable $e) {
                Log::warning('VIES VAT validation failed', [
                    'country' => $countryCode,
                    'error' => $e->getMessage(),
                ]);

                // Fallback to format validation
                return $this->validateVatIdFormat($countryCode, $vatNumber);
            }
        });
    }

    /**
     * Basic VAT ID format validation
     */
    private function validateVatIdFormat(string $countryCode, string $vatNumber): bool
    {
        $patterns = [
            'AT' => '/^U\d{8}$/',
            'BE' => '/^0\d{9}$/',
            'BG' => '/^\d{9,10}$/',
            'CY' => '/^\d{8}[A-Z]$/',
            'CZ' => '/^\d{8,10}$/',
            'DE' => '/^\d{9}$/',
            'DK' => '/^\d{8}$/',
            'EE' => '/^\d{9}$/',
            'GR' => '/^\d{9}$/',
            'ES' => '/^[A-Z0-9]\d{7}[A-Z0-9]$/',
            'FI' => '/^\d{8}$/',
            'FR' => '/^[A-Z0-9]{2}\d{9}$/',
            'HR' => '/^\d{11}$/',
            'HU' => '/^\d{8}$/',
            'IE' => '/^\d{7}[A-Z]{1,2}$/',
            'IT' => '/^\d{11}$/',
            'LT' => '/^(\d{9}|\d{12})$/',
            'LU' => '/^\d{8}$/',
            'LV' => '/^\d{11}$/',
            'MT' => '/^\d{8}$/',
            'NL' => '/^\d{9}B\d{2}$/',
            'PL' => '/^\d{10}$/',
            'PT' => '/^\d{9}$/',
            'RO' => '/^\d{2,10}$/',
            'SE' => '/^\d{12}$/',
            'SI' => '/^\d{8}$/',
            'SK' => '/^\d{10}$/',
        ];

        if (!isset($patterns[$countryCode])) {
            return false;
        }

        return (bool) preg_match($patterns[$countryCode], $vatNumber);
    }

    /**
     * Get tax type based on country
     */
    private function getTaxType(string $country): string
    {
        if ($this->isEuCountry($country) || $country === 'GB') {
            return 'vat';
        }

        if ($country === 'US') {
            return 'sales_tax';
        }

        if (isset(self::GST_RATES[$country])) {
            return 'gst';
        }

        return 'tax';
    }

    /**
     * Get tax description
     */
    private function getTaxDescription(string $country, float $rate): string
    {
        if ($rate === 0.0) {
            return 'No tax applicable';
        }

        $type = match ($this->getTaxType($country)) {
            'vat' => 'VAT',
            'sales_tax' => 'Sales Tax',
            'gst' => 'GST',
            default => 'Tax',
        };

        return "{$type} ({$rate}%)";
    }

    /**
     * Get all available tax rates for display
     */
    public function getAllTaxRates(): array
    {
        $rates = [];

        foreach (self::EU_VAT_RATES as $country => $rate) {
            $rates[$country] = [
                'rate' => $rate,
                'type' => 'vat',
                'name' => "VAT {$rate}%",
            ];
        }

        $rates['GB'] = ['rate' => 20, 'type' => 'vat', 'name' => 'VAT 20%'];
        $rates['US'] = ['rate' => 'varies', 'type' => 'sales_tax', 'name' => 'Sales Tax (varies by state)'];

        foreach (self::GST_RATES as $country => $rate) {
            $rates[$country] = [
                'rate' => $rate,
                'type' => 'gst',
                'name' => "GST {$rate}%",
            ];
        }

        return $rates;
    }
}

/**
 * Tax calculation result
 */
class TaxResult
{
    public function __construct(
        public int $amount,
        public float $rate,
        public string $type,
        public string $country,
        public ?string $state = null,
        public string $description = '',
        public ?string $taxId = null,
        public bool $reverseCharge = false,
    ) {
        $this->reverseCharge = $type === 'reverse_charge';
    }

    public function toArray(): array
    {
        return [
            'amount' => $this->amount,
            'rate' => $this->rate,
            'type' => $this->type,
            'country' => $this->country,
            'state' => $this->state,
            'description' => $this->description,
            'tax_id' => $this->taxId,
            'reverse_charge' => $this->reverseCharge,
        ];
    }
}
