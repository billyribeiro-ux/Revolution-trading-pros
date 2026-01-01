<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\InvoiceSettings;
use App\Models\Invoice;
use App\Services\Subscription\InvoicePdfGenerator;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;

/**
 * Invoice Settings Controller
 *
 * Visual invoice customization API:
 * - Logo upload
 * - Color customization
 * - Template settings
 * - Live preview
 */
class InvoiceSettingsController extends Controller
{
    public function __construct(
        private InvoicePdfGenerator $pdfGenerator
    ) {}

    /**
     * Get current invoice settings
     */
    public function index(): JsonResponse
    {
        $settings = InvoiceSettings::get();

        return response()->json([
            'settings' => $settings,
            'logo_url' => $settings->logo_url,
            'defaults' => InvoiceSettings::getDefaults(),
            'fonts' => $this->getAvailableFonts(),
            'countries' => $this->getCountries(),
        ]);
    }

    /**
     * Update invoice settings
     */
    public function update(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'company_name' => 'sometimes|string|max:255',
            'company_email' => 'sometimes|email|max:255',
            'company_phone' => 'sometimes|string|max:50',
            'company_address' => 'sometimes|string|max:500',
            'company_city' => 'sometimes|string|max:100',
            'company_state' => 'sometimes|string|max:100',
            'company_zip' => 'sometimes|string|max:20',
            'company_country' => 'sometimes|string|size:2',
            'tax_id' => 'sometimes|string|max:100',
            'primary_color' => 'sometimes|string|regex:/^#[0-9A-Fa-f]{6}$/',
            'secondary_color' => 'sometimes|string|regex:/^#[0-9A-Fa-f]{6}$/',
            'accent_color' => 'sometimes|string|regex:/^#[0-9A-Fa-f]{6}$/',
            'font_family' => 'sometimes|string|max:100',
            'header_text' => 'sometimes|string|max:100',
            'footer_text' => 'sometimes|string|max:500',
            'payment_terms' => 'sometimes|string|max:255',
            'notes_template' => 'sometimes|string|max:1000',
            'show_logo' => 'sometimes|boolean',
            'show_tax_id' => 'sometimes|boolean',
            'show_payment_method' => 'sometimes|boolean',
            'show_due_date' => 'sometimes|boolean',
            'invoice_prefix' => 'sometimes|string|max:20',
            'custom_css' => 'sometimes|string|max:5000',
        ]);

        $settings = InvoiceSettings::updateSettings($validated);

        return response()->json([
            'message' => 'Invoice settings updated successfully',
            'settings' => $settings,
        ]);
    }

    /**
     * Upload logo
     */
    public function uploadLogo(Request $request): JsonResponse
    {
        $request->validate([
            'logo' => 'required|image|mimes:png,jpg,jpeg,svg,webp|max:2048',
        ]);

        $settings = InvoiceSettings::get();

        // Delete old logo if exists
        if ($settings->logo_path) {
            Storage::disk('public')->delete($settings->logo_path);
        }

        // Store new logo
        $file = $request->file('logo');
        $filename = 'invoice-logo-' . time() . '.' . $file->getClientOriginalExtension();
        $path = $file->storeAs('branding', $filename, 'public');

        $settings->update(['logo_path' => $path]);

        return response()->json([
            'message' => 'Logo uploaded successfully',
            'logo_url' => Storage::disk('public')->url($path),
            'logo_path' => $path,
        ]);
    }

    /**
     * Remove logo
     */
    public function removeLogo(): JsonResponse
    {
        $settings = InvoiceSettings::get();

        if ($settings->logo_path) {
            Storage::disk('public')->delete($settings->logo_path);
            $settings->update(['logo_path' => null]);
        }

        return response()->json([
            'message' => 'Logo removed successfully',
        ]);
    }

    /**
     * Generate preview PDF
     */
    public function preview(Request $request): mixed
    {
        // Create a mock invoice for preview
        $mockInvoice = $this->createMockInvoice();

        // Generate preview PDF
        return $this->pdfGenerator->streamPreview($mockInvoice);
    }

    /**
     * Get preview HTML (for live preview in editor)
     */
    public function previewHtml(Request $request): JsonResponse
    {
        $settings = InvoiceSettings::get();
        $mockInvoice = $this->createMockInvoice();

        $html = view('invoices.pdf-customizable', [
            'invoice' => $mockInvoice,
            'user' => $this->createMockUser(),
            'settings' => $settings,
            'items' => $this->getMockLineItems(),
            'tax' => $this->getMockTax(),
            'totals' => $this->getMockTotals(),
            'payment' => $this->getMockPayment(),
            'isPreview' => true,
        ])->render();

        return response()->json([
            'html' => $html,
        ]);
    }

    /**
     * Reset to defaults
     */
    public function resetToDefaults(): JsonResponse
    {
        $settings = InvoiceSettings::get();

        // Keep logo if exists
        $logoPath = $settings->logo_path;

        $defaults = InvoiceSettings::getDefaults();
        $defaults['logo_path'] = $logoPath;

        $settings->update($defaults);

        return response()->json([
            'message' => 'Settings reset to defaults',
            'settings' => $settings->fresh(),
        ]);
    }

    /**
     * Export settings
     */
    public function export(): JsonResponse
    {
        $settings = InvoiceSettings::get()->toArray();
        unset($settings['id'], $settings['created_at'], $settings['updated_at']);

        return response()->json([
            'settings' => $settings,
            'exported_at' => now()->toIso8601String(),
        ]);
    }

    /**
     * Import settings
     */
    public function import(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'settings' => 'required|array',
        ]);

        // Filter out non-fillable fields
        $allowedFields = (new InvoiceSettings())->getFillable();
        $importData = array_intersect_key($validated['settings'], array_flip($allowedFields));

        // Don't overwrite logo
        unset($importData['logo_path']);

        InvoiceSettings::updateSettings($importData);

        return response()->json([
            'message' => 'Settings imported successfully',
            'settings' => InvoiceSettings::get(),
        ]);
    }

    /**
     * Create mock invoice for preview
     */
    private function createMockInvoice(): object
    {
        return (object) [
            'id' => 1,
            'invoice_number' => 'INV-2024-001',
            'created_at' => now(),
            'due_date' => now()->addDays(30),
            'status' => 'paid',
            'amount' => 9900,
            'subtotal' => 9900,
            'tax_amount' => 0,
            'currency' => 'USD',
        ];
    }

    /**
     * Create mock user for preview
     */
    private function createMockUser(): object
    {
        return (object) [
            'name' => 'John Doe',
            'email' => 'john@example.com',
            'company_name' => 'Acme Corporation',
            'billing_address' => '123 Main Street',
            'billing_city' => 'San Francisco',
            'billing_state' => 'CA',
            'billing_zip' => '94102',
            'billing_country' => 'US',
            'tax_id' => null,
        ];
    }

    /**
     * Get mock line items
     */
    private function getMockLineItems(): array
    {
        return [
            [
                'description' => 'Professional Plan - Monthly Subscription',
                'quantity' => 1,
                'unit_price' => 9900,
                'total' => 9900,
            ],
        ];
    }

    /**
     * Get mock tax
     */
    private function getMockTax(): array
    {
        return [
            'rate' => 0,
            'amount' => 0,
            'type' => 'tax',
            'description' => '',
            'reverse_charge' => false,
        ];
    }

    /**
     * Get mock totals
     */
    private function getMockTotals(): array
    {
        return [
            'subtotal' => 9900,
            'tax' => 0,
            'discount' => 0,
            'total' => 9900,
            'currency' => 'USD',
            'formatted' => [
                'subtotal' => '$99.00',
                'tax' => '$0.00',
                'discount' => '$0.00',
                'total' => '$99.00',
            ],
        ];
    }

    /**
     * Get mock payment
     */
    private function getMockPayment(): array
    {
        return [
            'status' => 'paid',
            'paid_at' => now(),
            'payment_method' => 'card',
            'card_last_four' => '4242',
            'card_brand' => 'visa',
        ];
    }

    /**
     * Get available fonts
     */
    private function getAvailableFonts(): array
    {
        return [
            ['value' => 'Inter, sans-serif', 'label' => 'Inter'],
            ['value' => 'Helvetica, Arial, sans-serif', 'label' => 'Helvetica'],
            ['value' => 'Georgia, serif', 'label' => 'Georgia'],
            ['value' => 'Times New Roman, serif', 'label' => 'Times New Roman'],
            ['value' => 'Courier New, monospace', 'label' => 'Courier New'],
            ['value' => 'Roboto, sans-serif', 'label' => 'Roboto'],
            ['value' => 'Open Sans, sans-serif', 'label' => 'Open Sans'],
            ['value' => 'Lato, sans-serif', 'label' => 'Lato'],
            ['value' => 'Montserrat, sans-serif', 'label' => 'Montserrat'],
            ['value' => 'Playfair Display, serif', 'label' => 'Playfair Display'],
        ];
    }

    /**
     * Get countries list
     */
    private function getCountries(): array
    {
        return [
            'US' => 'United States',
            'GB' => 'United Kingdom',
            'CA' => 'Canada',
            'AU' => 'Australia',
            'DE' => 'Germany',
            'FR' => 'France',
            'ES' => 'Spain',
            'IT' => 'Italy',
            'NL' => 'Netherlands',
            'BE' => 'Belgium',
            'AT' => 'Austria',
            'CH' => 'Switzerland',
            'SE' => 'Sweden',
            'NO' => 'Norway',
            'DK' => 'Denmark',
            'FI' => 'Finland',
            'IE' => 'Ireland',
            'PT' => 'Portugal',
            'PL' => 'Poland',
            'CZ' => 'Czech Republic',
            'BR' => 'Brazil',
            'MX' => 'Mexico',
            'IN' => 'India',
            'JP' => 'Japan',
            'SG' => 'Singapore',
            'NZ' => 'New Zealand',
        ];
    }
}
