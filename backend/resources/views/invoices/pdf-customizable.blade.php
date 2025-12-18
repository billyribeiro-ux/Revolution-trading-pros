<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title>Invoice #{{ $invoice->invoice_number ?? $invoice->id }}</title>
    <style>
        @if($settings->font_family && str_contains($settings->font_family, 'Inter'))
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
        @endif

        :root {
            --primary-color: {{ $settings->primary_color ?? '#2563eb' }};
            --secondary-color: {{ $settings->secondary_color ?? '#1f2937' }};
            --accent-color: {{ $settings->accent_color ?? '#10b981' }};
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: {{ $settings->font_family ?? 'Helvetica, Arial, sans-serif' }};
            font-size: 12px;
            line-height: 1.5;
            color: #333;
        }

        .invoice {
            padding: 40px;
            max-width: 800px;
            margin: 0 auto;
            @if(isset($isPreview) && $isPreview)
            background: white;
            box-shadow: 0 0 20px rgba(0,0,0,0.1);
            @endif
        }

        .header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 40px;
            border-bottom: 3px solid var(--primary-color);
            padding-bottom: 20px;
        }

        .logo-section {
            float: left;
            width: 50%;
        }

        .logo {
            max-height: 70px;
            max-width: 220px;
        }

        .company-name {
            font-size: 26px;
            font-weight: bold;
            color: var(--primary-color);
            margin-bottom: 5px;
        }

        .invoice-title-section {
            float: right;
            width: 50%;
            text-align: right;
        }

        .invoice-title {
            font-size: 32px;
            font-weight: bold;
            color: var(--secondary-color);
            text-transform: uppercase;
            letter-spacing: 2px;
        }

        .invoice-number {
            font-size: 14px;
            color: #6b7280;
            margin-top: 8px;
        }

        .clearfix::after {
            content: "";
            display: table;
            clear: both;
        }

        .addresses {
            margin-bottom: 30px;
        }

        .from-address, .to-address {
            width: 48%;
            float: left;
        }

        .to-address {
            float: right;
        }

        .address-label {
            font-size: 10px;
            text-transform: uppercase;
            color: var(--primary-color);
            margin-bottom: 8px;
            font-weight: bold;
            letter-spacing: 1px;
        }

        .address-name {
            font-weight: bold;
            font-size: 15px;
            margin-bottom: 5px;
            color: var(--secondary-color);
        }

        .address-line {
            color: #4b5563;
            margin-bottom: 2px;
        }

        .invoice-details {
            margin-bottom: 30px;
            background: linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%);
            padding: 20px;
            border-radius: 8px;
            border-left: 4px solid var(--primary-color);
        }

        .detail-row {
            margin-bottom: 8px;
            display: flex;
            align-items: center;
        }

        .detail-label {
            font-weight: 600;
            display: inline-block;
            width: 130px;
            color: #6b7280;
        }

        .detail-value {
            color: var(--secondary-color);
        }

        .items-table {
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 30px;
        }

        .items-table th {
            background: var(--primary-color);
            color: white;
            padding: 14px 15px;
            text-align: left;
            font-size: 11px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .items-table th:first-child {
            border-radius: 8px 0 0 0;
        }

        .items-table th:last-child {
            border-radius: 0 8px 0 0;
            text-align: right;
        }

        .items-table td {
            padding: 14px 15px;
            border-bottom: 1px solid #e5e7eb;
        }

        .items-table td:last-child {
            text-align: right;
        }

        .items-table tr:nth-child(even) {
            background: #f9fafb;
        }

        .items-table tr:hover {
            background: #f3f4f6;
        }

        .totals-section {
            float: right;
            width: 320px;
        }

        .totals-table {
            width: 100%;
            background: #f9fafb;
            border-radius: 8px;
            overflow: hidden;
        }

        .totals-table td {
            padding: 12px 15px;
        }

        .totals-table .label {
            text-align: right;
            padding-right: 20px;
            color: #6b7280;
        }

        .totals-table .value {
            text-align: right;
            font-weight: 600;
            color: var(--secondary-color);
        }

        .totals-table .total-row {
            background: var(--primary-color);
        }

        .totals-table .total-row td {
            padding-top: 15px;
            padding-bottom: 15px;
        }

        .totals-table .total-row .label,
        .totals-table .total-row .value {
            color: white;
            font-size: 16px;
            font-weight: bold;
        }

        .payment-status {
            display: inline-block;
            padding: 6px 16px;
            border-radius: 20px;
            font-size: 11px;
            font-weight: bold;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .status-paid {
            background: #d1fae5;
            color: #065f46;
        }

        .status-pending {
            background: #fef3c7;
            color: #92400e;
        }

        .status-failed {
            background: #fee2e2;
            color: #991b1b;
        }

        .footer {
            margin-top: 50px;
            padding-top: 20px;
            border-top: 2px solid #e5e7eb;
            text-align: center;
            color: #6b7280;
            font-size: 11px;
        }

        .footer-text {
            font-size: 14px;
            color: var(--primary-color);
            font-weight: 500;
            margin-bottom: 10px;
        }

        .notes {
            margin-top: 30px;
            padding: 20px;
            background: #f9fafb;
            border-radius: 8px;
            border-left: 4px solid var(--accent-color);
        }

        .notes-title {
            font-weight: bold;
            margin-bottom: 8px;
            color: var(--secondary-color);
        }

        .notes p {
            color: #4b5563;
            font-size: 11px;
        }

        .payment-info {
            background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
            border-left-color: var(--accent-color);
        }

        /* Custom CSS from settings */
        {!! $settings->custom_css ?? '' !!}
    </style>
</head>
<body>
    <div class="invoice">
        <div class="header clearfix">
            <div class="logo-section">
                @if($settings->show_logo && $settings->logo_path)
                    <img src="{{ Storage::disk('public')->url($settings->logo_path) }}" alt="Logo" class="logo">
                @else
                    <div class="company-name">{{ $settings->company_name }}</div>
                @endif
            </div>
            <div class="invoice-title-section">
                <div class="invoice-title">{{ $settings->header_text ?? 'INVOICE' }}</div>
                <div class="invoice-number">#{{ $settings->invoice_prefix }}{{ $invoice->invoice_number ?? $invoice->id }}</div>
            </div>
        </div>

        <div class="addresses clearfix">
            <div class="from-address">
                <div class="address-label">From</div>
                <div class="address-name">{{ $settings->company_name }}</div>
                @if($settings->company_address)
                    <div class="address-line">{{ $settings->company_address }}</div>
                @endif
                @if($settings->company_city || $settings->company_state || $settings->company_zip)
                    <div class="address-line">
                        {{ $settings->company_city }}{{ $settings->company_state ? ', ' . $settings->company_state : '' }} {{ $settings->company_zip }}
                    </div>
                @endif
                @if($settings->company_country)
                    <div class="address-line">{{ $settings->company_country }}</div>
                @endif
                @if($settings->show_tax_id && $settings->tax_id)
                    <div class="address-line">Tax ID: {{ $settings->tax_id }}</div>
                @endif
                <div class="address-line">{{ $settings->company_email }}</div>
                @if($settings->company_phone)
                    <div class="address-line">{{ $settings->company_phone }}</div>
                @endif
            </div>
            <div class="to-address">
                <div class="address-label">Bill To</div>
                <div class="address-name">{{ $user->name }}</div>
                @if($user->company_name)
                    <div class="address-line">{{ $user->company_name }}</div>
                @endif
                @if($user->billing_address)
                    <div class="address-line">{{ $user->billing_address }}</div>
                @endif
                @if($user->billing_city || $user->billing_state || $user->billing_zip)
                    <div class="address-line">
                        {{ $user->billing_city }}{{ $user->billing_state ? ', ' . $user->billing_state : '' }} {{ $user->billing_zip }}
                    </div>
                @endif
                @if($user->billing_country)
                    <div class="address-line">{{ $user->billing_country }}</div>
                @endif
                @if($user->tax_id)
                    <div class="address-line">Tax ID: {{ $user->tax_id }}</div>
                @endif
                <div class="address-line">{{ $user->email }}</div>
            </div>
        </div>

        <div class="invoice-details clearfix">
            <div class="detail-row">
                <span class="detail-label">Invoice Date:</span>
                <span class="detail-value">{{ $invoice->created_at->format('F j, Y') }}</span>
            </div>
            @if($settings->show_due_date && $invoice->due_date)
            <div class="detail-row">
                <span class="detail-label">Due Date:</span>
                <span class="detail-value">{{ $invoice->due_date->format('F j, Y') }}</span>
            </div>
            @endif
            <div class="detail-row">
                <span class="detail-label">Payment Terms:</span>
                <span class="detail-value">{{ $settings->payment_terms }}</span>
            </div>
            <div class="detail-row">
                <span class="detail-label">Status:</span>
                <span class="payment-status status-{{ $payment['status'] }}">{{ ucfirst($payment['status']) }}</span>
            </div>
        </div>

        <table class="items-table">
            <thead>
                <tr>
                    <th style="width: 50%">Description</th>
                    <th style="width: 15%">Qty</th>
                    <th style="width: 17%">Unit Price</th>
                    <th style="width: 18%">Amount</th>
                </tr>
            </thead>
            <tbody>
                @foreach($items as $item)
                <tr>
                    <td>{{ $item['description'] }}</td>
                    <td>{{ $item['quantity'] }}</td>
                    <td>{{ $totals['currency'] }} {{ number_format($item['unit_price'] / 100, 2) }}</td>
                    <td>{{ $totals['currency'] }} {{ number_format($item['total'] / 100, 2) }}</td>
                </tr>
                @endforeach
            </tbody>
        </table>

        <div class="clearfix">
            <div class="totals-section">
                <table class="totals-table">
                    <tr>
                        <td class="label">Subtotal:</td>
                        <td class="value">{{ $totals['formatted']['subtotal'] }}</td>
                    </tr>
                    @if($totals['discount'] > 0)
                    <tr>
                        <td class="label">Discount:</td>
                        <td class="value">-{{ $totals['formatted']['discount'] }}</td>
                    </tr>
                    @endif
                    @if($tax['amount'] > 0 || $tax['reverse_charge'])
                    <tr>
                        <td class="label">
                            {{ strtoupper($tax['type']) }}
                            @if($tax['rate'] > 0)({{ $tax['rate'] }}%)@endif:
                        </td>
                        <td class="value">
                            @if($tax['reverse_charge'])
                                Reverse Charge
                            @else
                                {{ $totals['formatted']['tax'] }}
                            @endif
                        </td>
                    </tr>
                    @endif
                    <tr class="total-row">
                        <td class="label">Total:</td>
                        <td class="value">{{ $totals['formatted']['total'] }}</td>
                    </tr>
                </table>
            </div>
        </div>

        @if($settings->show_payment_method && $payment['card_last_four'])
        <div class="notes payment-info" style="clear: both; margin-top: 40px;">
            <div class="notes-title">Payment Method</div>
            <p>{{ ucfirst($payment['card_brand'] ?? 'Card') }} ending in {{ $payment['card_last_four'] }}</p>
            @if($payment['paid_at'])
            <p>Paid on {{ $payment['paid_at']->format('F j, Y') }}</p>
            @endif
        </div>
        @endif

        @if($tax['reverse_charge'])
        <div class="notes">
            <div class="notes-title">VAT Note</div>
            <p>Reverse charge - VAT to be accounted for by the recipient as per Article 196 of Council Directive 2006/112/EC.</p>
        </div>
        @endif

        @if($settings->notes_template)
        <div class="notes">
            <div class="notes-title">Notes</div>
            <p>{{ $settings->notes_template }}</p>
        </div>
        @endif

        <div class="footer">
            <p class="footer-text">{{ $settings->footer_text }}</p>
            <p>{{ $settings->company_name }} &bull; {{ $settings->company_email }}</p>
            @if($settings->show_tax_id && $settings->tax_id)
                <p>Tax ID: {{ $settings->tax_id }}</p>
            @endif
        </div>
    </div>
</body>
</html>
