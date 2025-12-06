<?php

declare(strict_types=1);

namespace App\Services\Subscription;

use App\Models\Invoice;
use App\Models\InvoiceSettings;
use App\Models\User;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\View;
use Barryvdh\DomPDF\Facade\Pdf;

/**
 * Invoice PDF Generator (ICT9+ Enterprise Grade)
 *
 * Generates professional PDF invoices:
 * - Visual customization via admin UI
 * - Logo upload support
 * - Color customization
 * - Tax details
 * - Line items
 * - Payment status
 *
 * @version 2.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class InvoicePdfGenerator
{
    /**
     * Get invoice settings (cached)
     */
    private function getSettings(): InvoiceSettings
    {
        return InvoiceSettings::get();
    }

    /**
     * Generate PDF for an invoice
     */
    public function generate(Invoice $invoice): string
    {
        $user = $invoice->user;
        $subscription = $invoice->subscription;
        $settings = $this->getSettings();

        $data = [
            'invoice' => $invoice,
            'user' => $user,
            'subscription' => $subscription,
            'settings' => $settings,
            'items' => $this->getLineItems($invoice),
            'tax' => $this->getTaxDetails($invoice),
            'totals' => $this->calculateTotals($invoice),
            'payment' => $this->getPaymentDetails($invoice),
        ];

        $pdf = Pdf::loadView('invoices.pdf-customizable', $data);

        // Configure PDF
        $pdf->setPaper('a4', 'portrait');
        $pdf->setOptions([
            'isHtml5ParserEnabled' => true,
            'isRemoteEnabled' => true,
            'defaultFont' => 'sans-serif',
        ]);

        // Generate filename
        $filename = $this->generateFilename($invoice);

        // Store PDF
        $path = "invoices/{$user->id}/{$filename}";
        Storage::disk('private')->put($path, $pdf->output());

        // Update invoice with PDF path
        $invoice->update(['pdf_path' => $path]);

        return $path;
    }

    /**
     * Get PDF download URL
     */
    public function getDownloadUrl(Invoice $invoice): string
    {
        if (!$invoice->pdf_path) {
            $this->generate($invoice);
            $invoice->refresh();
        }

        return route('invoices.download', $invoice->id);
    }

    /**
     * Stream PDF to browser
     */
    public function stream(Invoice $invoice)
    {
        $user = $invoice->user;
        $subscription = $invoice->subscription;
        $settings = $this->getSettings();

        $data = [
            'invoice' => $invoice,
            'user' => $user,
            'subscription' => $subscription,
            'settings' => $settings,
            'items' => $this->getLineItems($invoice),
            'tax' => $this->getTaxDetails($invoice),
            'totals' => $this->calculateTotals($invoice),
            'payment' => $this->getPaymentDetails($invoice),
        ];

        $pdf = Pdf::loadView('invoices.pdf-customizable', $data);
        $filename = $this->generateFilename($invoice);

        return $pdf->stream($filename);
    }

    /**
     * Stream preview PDF (for admin customization)
     */
    public function streamPreview(object $mockInvoice)
    {
        $settings = $this->getSettings();

        $data = [
            'invoice' => $mockInvoice,
            'user' => $this->createMockUser(),
            'settings' => $settings,
            'items' => $this->getMockLineItems(),
            'tax' => $this->getMockTax(),
            'totals' => $this->getMockTotals(),
            'payment' => $this->getMockPayment(),
            'isPreview' => true,
        ];

        $pdf = Pdf::loadView('invoices.pdf-customizable', $data);

        $pdf->setPaper('a4', 'portrait');
        $pdf->setOptions([
            'isHtml5ParserEnabled' => true,
            'isRemoteEnabled' => true,
        ]);

        return $pdf->stream('invoice-preview.pdf');
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
     * Download PDF
     */
    public function download(Invoice $invoice)
    {
        if ($invoice->pdf_path && Storage::disk('private')->exists($invoice->pdf_path)) {
            return Storage::disk('private')->download(
                $invoice->pdf_path,
                $this->generateFilename($invoice)
            );
        }

        // Generate on the fly
        $path = $this->generate($invoice);

        return Storage::disk('private')->download(
            $path,
            $this->generateFilename($invoice)
        );
    }

    /**
     * Get line items for invoice
     */
    private function getLineItems(Invoice $invoice): array
    {
        $items = [];

        // Main subscription item
        if ($invoice->subscription) {
            $plan = $invoice->subscription->plan;

            $items[] = [
                'description' => $plan?->name ?? 'Subscription',
                'quantity' => 1,
                'unit_price' => $invoice->subtotal ?? $invoice->amount,
                'total' => $invoice->subtotal ?? $invoice->amount,
            ];
        }

        // Additional items from metadata
        if ($invoice->line_items) {
            foreach ($invoice->line_items as $item) {
                $items[] = [
                    'description' => $item['description'] ?? 'Item',
                    'quantity' => $item['quantity'] ?? 1,
                    'unit_price' => $item['unit_price'] ?? 0,
                    'total' => $item['total'] ?? 0,
                ];
            }
        }

        return $items;
    }

    /**
     * Get tax details
     */
    private function getTaxDetails(Invoice $invoice): array
    {
        return [
            'rate' => $invoice->tax_rate ?? 0,
            'amount' => $invoice->tax_amount ?? 0,
            'type' => $invoice->tax_type ?? 'tax',
            'description' => $invoice->tax_description ?? '',
            'reverse_charge' => $invoice->reverse_charge ?? false,
        ];
    }

    /**
     * Calculate totals
     */
    private function calculateTotals(Invoice $invoice): array
    {
        $subtotal = $invoice->subtotal ?? ($invoice->amount - ($invoice->tax_amount ?? 0));
        $tax = $invoice->tax_amount ?? 0;
        $discount = $invoice->discount_amount ?? 0;
        $total = $invoice->amount;

        return [
            'subtotal' => $subtotal,
            'tax' => $tax,
            'discount' => $discount,
            'total' => $total,
            'currency' => strtoupper($invoice->currency ?? 'USD'),
            'formatted' => [
                'subtotal' => $this->formatCurrency($subtotal, $invoice->currency),
                'tax' => $this->formatCurrency($tax, $invoice->currency),
                'discount' => $this->formatCurrency($discount, $invoice->currency),
                'total' => $this->formatCurrency($total, $invoice->currency),
            ],
        ];
    }

    /**
     * Get payment details
     */
    private function getPaymentDetails(Invoice $invoice): array
    {
        return [
            'status' => $invoice->status,
            'paid_at' => $invoice->paid_at,
            'payment_method' => $invoice->payment_method ?? 'card',
            'card_last_four' => $invoice->card_last_four,
            'card_brand' => $invoice->card_brand,
        ];
    }

    /**
     * Generate filename
     */
    private function generateFilename(Invoice $invoice): string
    {
        $number = $invoice->invoice_number ?? $invoice->id;
        $date = $invoice->created_at->format('Y-m-d');

        return "invoice-{$number}-{$date}.pdf";
    }

    /**
     * Format currency
     */
    private function formatCurrency(int $amountInCents, ?string $currency = 'USD'): string
    {
        $amount = $amountInCents / 100;
        $currency = strtoupper($currency ?? 'USD');

        $symbols = [
            'USD' => '$',
            'EUR' => '€',
            'GBP' => '£',
            'JPY' => '¥',
            'AUD' => 'A$',
            'CAD' => 'C$',
        ];

        $symbol = $symbols[$currency] ?? $currency . ' ';

        // JPY doesn't have cents
        if ($currency === 'JPY') {
            return $symbol . number_format($amountInCents);
        }

        return $symbol . number_format($amount, 2);
    }

    /**
     * Bulk generate invoices
     */
    public function bulkGenerate(array $invoiceIds): array
    {
        $results = [];

        foreach ($invoiceIds as $id) {
            $invoice = Invoice::find($id);

            if ($invoice) {
                try {
                    $path = $this->generate($invoice);
                    $results[$id] = ['success' => true, 'path' => $path];
                } catch (\Throwable $e) {
                    $results[$id] = ['success' => false, 'error' => $e->getMessage()];
                }
            }
        }

        return $results;
    }
}
