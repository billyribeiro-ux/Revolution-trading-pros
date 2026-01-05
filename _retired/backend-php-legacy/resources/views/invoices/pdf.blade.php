<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title>Invoice #{{ $invoice->invoice_number ?? $invoice->id }}</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
            font-size: 12px;
            line-height: 1.5;
            color: #333;
        }
        .invoice {
            padding: 40px;
            max-width: 800px;
            margin: 0 auto;
        }
        .header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 40px;
            border-bottom: 2px solid #2563eb;
            padding-bottom: 20px;
        }
        .logo-section {
            float: left;
            width: 50%;
        }
        .logo {
            max-height: 60px;
            max-width: 200px;
        }
        .company-name {
            font-size: 24px;
            font-weight: bold;
            color: #2563eb;
            margin-bottom: 5px;
        }
        .invoice-title-section {
            float: right;
            width: 50%;
            text-align: right;
        }
        .invoice-title {
            font-size: 28px;
            font-weight: bold;
            color: #1f2937;
            text-transform: uppercase;
        }
        .invoice-number {
            font-size: 14px;
            color: #6b7280;
            margin-top: 5px;
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
            color: #6b7280;
            margin-bottom: 5px;
            font-weight: bold;
        }
        .address-name {
            font-weight: bold;
            font-size: 14px;
            margin-bottom: 5px;
        }
        .address-line {
            color: #4b5563;
        }
        .invoice-details {
            margin-bottom: 30px;
            background: #f9fafb;
            padding: 15px;
            border-radius: 5px;
        }
        .detail-row {
            margin-bottom: 5px;
        }
        .detail-label {
            font-weight: bold;
            display: inline-block;
            width: 120px;
        }
        .items-table {
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 30px;
        }
        .items-table th {
            background: #2563eb;
            color: white;
            padding: 12px 15px;
            text-align: left;
            font-size: 11px;
            text-transform: uppercase;
        }
        .items-table th:last-child,
        .items-table td:last-child {
            text-align: right;
        }
        .items-table td {
            padding: 12px 15px;
            border-bottom: 1px solid #e5e7eb;
        }
        .items-table tr:nth-child(even) {
            background: #f9fafb;
        }
        .totals-section {
            float: right;
            width: 300px;
        }
        .totals-table {
            width: 100%;
        }
        .totals-table td {
            padding: 8px 0;
        }
        .totals-table .label {
            text-align: right;
            padding-right: 20px;
            color: #6b7280;
        }
        .totals-table .value {
            text-align: right;
            font-weight: bold;
        }
        .totals-table .total-row {
            border-top: 2px solid #2563eb;
            font-size: 16px;
        }
        .totals-table .total-row .label,
        .totals-table .total-row .value {
            padding-top: 15px;
            color: #1f2937;
        }
        .payment-status {
            display: inline-block;
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 11px;
            font-weight: bold;
            text-transform: uppercase;
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
            border-top: 1px solid #e5e7eb;
            text-align: center;
            color: #6b7280;
            font-size: 10px;
        }
        .notes {
            margin-top: 30px;
            padding: 15px;
            background: #f9fafb;
            border-left: 3px solid #2563eb;
        }
        .notes-title {
            font-weight: bold;
            margin-bottom: 5px;
        }
    </style>
</head>
<body>
    <div class="invoice">
        <div class="header clearfix">
            <div class="logo-section">
                @if($company['logo'])
                    <img src="{{ $company['logo'] }}" alt="Logo" class="logo">
                @else
                    <div class="company-name">{{ $company['name'] }}</div>
                @endif
            </div>
            <div class="invoice-title-section">
                <div class="invoice-title">Invoice</div>
                <div class="invoice-number">#{{ $invoice->invoice_number ?? $invoice->id }}</div>
            </div>
        </div>

        <div class="addresses clearfix">
            <div class="from-address">
                <div class="address-label">From</div>
                <div class="address-name">{{ $company['name'] }}</div>
                @if($company['address'])
                    <div class="address-line">{{ $company['address'] }}</div>
                @endif
                @if($company['city'])
                    <div class="address-line">{{ $company['city'] }}</div>
                @endif
                @if($company['vat_id'])
                    <div class="address-line">VAT: {{ $company['vat_id'] }}</div>
                @endif
                <div class="address-line">{{ $company['email'] }}</div>
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
                <span>{{ $invoice->created_at->format('F j, Y') }}</span>
            </div>
            @if($invoice->due_date)
            <div class="detail-row">
                <span class="detail-label">Due Date:</span>
                <span>{{ $invoice->due_date->format('F j, Y') }}</span>
            </div>
            @endif
            <div class="detail-row">
                <span class="detail-label">Status:</span>
                <span class="payment-status status-{{ $payment['status'] }}">{{ ucfirst($payment['status']) }}</span>
            </div>
            @if($payment['paid_at'])
            <div class="detail-row">
                <span class="detail-label">Paid On:</span>
                <span>{{ $payment['paid_at']->format('F j, Y') }}</span>
            </div>
            @endif
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
                            @if($tax['rate'] > 0)
                                ({{ $tax['rate'] }}%)
                            @endif
                            :
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

        @if($tax['reverse_charge'])
        <div class="notes">
            <div class="notes-title">VAT Note</div>
            <p>Reverse charge - VAT to be accounted for by the recipient as per Article 196 of Council Directive 2006/112/EC.</p>
        </div>
        @endif

        @if($payment['card_last_four'])
        <div class="notes">
            <div class="notes-title">Payment Method</div>
            <p>{{ ucfirst($payment['card_brand'] ?? 'Card') }} ending in {{ $payment['card_last_four'] }}</p>
        </div>
        @endif

        <div class="footer">
            <p>Thank you for your business!</p>
            <p>{{ $company['name'] }} • {{ $company['email'] }}</p>
            @if($company['vat_id'])
                <p>VAT Registration: {{ $company['vat_id'] }}</p>
            @endif
        </div>
    </div>
</body>
</html>
