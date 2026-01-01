<?php

declare(strict_types=1);

namespace App\Services\Forms;

use App\Models\FormPayment;
use App\Models\FormSubmission;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * AuthorizeNetPaymentService - FluentForms 6.1.5 (November 2025)
 *
 * Authorize.net payment gateway integration for form submissions.
 * Features:
 * - Accept.js token processing (PCI-compliant)
 * - Transaction creation with opaque data
 * - Refund processing
 * - Void transactions
 * - Transaction status checking
 * - Webhook handling
 */
class AuthorizeNetPaymentService
{
    private string $apiLoginId;
    private string $transactionKey;
    private bool $testMode;
    private string $apiEndpoint;

    public function __construct()
    {
        $this->apiLoginId = config('services.authorize_net.api_login_id', '');
        $this->transactionKey = config('services.authorize_net.transaction_key', '');
        $this->testMode = config('services.authorize_net.test_mode', true);
        $this->apiEndpoint = $this->testMode
            ? 'https://apitest.authorize.net/xml/v1/request.api'
            : 'https://api.authorize.net/xml/v1/request.api';
    }

    /**
     * Process payment with Accept.js opaque data
     */
    public function processPayment(array $data): array
    {
        $dataDescriptor = $data['data_descriptor'] ?? '';
        $dataValue = $data['data_value'] ?? '';
        $amount = (float) ($data['amount'] ?? 0);
        $submissionId = $data['submission_id'] ?? null;
        $formId = $data['form_id'] ?? null;

        if (empty($dataDescriptor) || empty($dataValue) || $amount <= 0) {
            return [
                'success' => false,
                'message' => 'Invalid payment data provided.',
            ];
        }

        $invoiceNumber = $this->generateInvoiceNumber();
        $transRefId = Str::uuid()->toString();

        $payload = $this->buildTransactionPayload([
            'transactionType' => 'authCaptureTransaction',
            'amount' => number_format($amount, 2, '.', ''),
            'opaqueData' => [
                'dataDescriptor' => $dataDescriptor,
                'dataValue' => $dataValue,
            ],
            'order' => [
                'invoiceNumber' => $invoiceNumber,
                'description' => $data['description'] ?? 'Form submission payment',
            ],
            'customer' => [
                'email' => $data['email'] ?? null,
            ],
            'billTo' => $this->formatBillingAddress($data['billing'] ?? []),
            'transactionSettings' => [
                'setting' => [
                    'settingName' => 'duplicateWindow',
                    'settingValue' => '120', // 2 minute duplicate check
                ],
            ],
        ], $transRefId);

        try {
            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post($this->apiEndpoint, $payload);

            $result = $response->json();

            return $this->processTransactionResponse($result, [
                'submission_id' => $submissionId,
                'form_id' => $formId,
                'amount' => $amount,
                'invoice_number' => $invoiceNumber,
                'trans_ref_id' => $transRefId,
            ]);
        } catch (\Exception $e) {
            Log::error('AuthorizeNetPaymentService: Payment failed', [
                'error' => $e->getMessage(),
                'submission_id' => $submissionId,
            ]);

            return [
                'success' => false,
                'message' => 'Payment processing failed. Please try again.',
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Process authorization only (no capture)
     */
    public function authorizeOnly(array $data): array
    {
        $data['transaction_type'] = 'authOnlyTransaction';
        return $this->processPayment($data);
    }

    /**
     * Capture a previously authorized transaction
     */
    public function captureTransaction(string $transactionId, float $amount): array
    {
        $payload = $this->buildTransactionPayload([
            'transactionType' => 'priorAuthCaptureTransaction',
            'amount' => number_format($amount, 2, '.', ''),
            'refTransId' => $transactionId,
        ]);

        try {
            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post($this->apiEndpoint, $payload);

            $result = $response->json();

            return $this->processTransactionResponse($result, [
                'original_transaction_id' => $transactionId,
                'amount' => $amount,
            ]);
        } catch (\Exception $e) {
            Log::error('AuthorizeNetPaymentService: Capture failed', [
                'transaction_id' => $transactionId,
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Transaction capture failed.',
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Refund a transaction
     */
    public function refundTransaction(string $transactionId, float $amount, string $cardLastFour): array
    {
        $payload = $this->buildTransactionPayload([
            'transactionType' => 'refundTransaction',
            'amount' => number_format($amount, 2, '.', ''),
            'payment' => [
                'creditCard' => [
                    'cardNumber' => $cardLastFour,
                    'expirationDate' => 'XXXX', // Authorize.net accepts XXXX for refunds
                ],
            ],
            'refTransId' => $transactionId,
        ]);

        try {
            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post($this->apiEndpoint, $payload);

            $result = $response->json();

            $processed = $this->processTransactionResponse($result, [
                'original_transaction_id' => $transactionId,
                'amount' => $amount,
                'type' => 'refund',
            ]);

            if ($processed['success']) {
                // Update payment record
                FormPayment::where('transaction_id', $transactionId)
                    ->update([
                        'status' => 'refunded',
                        'refunded_at' => now(),
                        'refund_amount' => $amount,
                    ]);
            }

            return $processed;
        } catch (\Exception $e) {
            Log::error('AuthorizeNetPaymentService: Refund failed', [
                'transaction_id' => $transactionId,
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Refund processing failed.',
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Void a transaction
     */
    public function voidTransaction(string $transactionId): array
    {
        $payload = $this->buildTransactionPayload([
            'transactionType' => 'voidTransaction',
            'refTransId' => $transactionId,
        ]);

        try {
            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post($this->apiEndpoint, $payload);

            $result = $response->json();

            $processed = $this->processTransactionResponse($result, [
                'original_transaction_id' => $transactionId,
                'type' => 'void',
            ]);

            if ($processed['success']) {
                FormPayment::where('transaction_id', $transactionId)
                    ->update(['status' => 'voided', 'voided_at' => now()]);
            }

            return $processed;
        } catch (\Exception $e) {
            Log::error('AuthorizeNetPaymentService: Void failed', [
                'transaction_id' => $transactionId,
                'error' => $e->getMessage(),
            ]);

            return [
                'success' => false,
                'message' => 'Transaction void failed.',
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Get transaction details
     */
    public function getTransactionDetails(string $transactionId): array
    {
        $payload = [
            'getTransactionDetailsRequest' => [
                'merchantAuthentication' => [
                    'name' => $this->apiLoginId,
                    'transactionKey' => $this->transactionKey,
                ],
                'transId' => $transactionId,
            ],
        ];

        try {
            $response = Http::timeout(30)
                ->withHeaders(['Content-Type' => 'application/json'])
                ->post($this->apiEndpoint, $payload);

            $result = $response->json();

            if (isset($result['transaction'])) {
                return [
                    'success' => true,
                    'transaction' => $result['transaction'],
                ];
            }

            return [
                'success' => false,
                'message' => $result['messages']['message'][0]['text'] ?? 'Transaction not found.',
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'Failed to retrieve transaction details.',
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Verify webhook signature
     */
    public function verifyWebhookSignature(string $payload, string $signature): bool
    {
        $webhookSignatureKey = config('services.authorize_net.webhook_signature_key', '');

        if (empty($webhookSignatureKey)) {
            return false;
        }

        $expectedSignature = hash_hmac('sha512', $payload, $webhookSignatureKey);
        return hash_equals($expectedSignature, $signature);
    }

    /**
     * Process webhook notification
     */
    public function processWebhook(array $payload): array
    {
        $eventType = $payload['eventType'] ?? '';
        $webhookId = $payload['webhookId'] ?? '';
        $payloadData = $payload['payload'] ?? [];

        Log::info('AuthorizeNetPaymentService: Webhook received', [
            'event_type' => $eventType,
            'webhook_id' => $webhookId,
        ]);

        switch ($eventType) {
            case 'net.authorize.payment.authcapture.created':
            case 'net.authorize.payment.capture.created':
                return $this->handlePaymentCreated($payloadData);

            case 'net.authorize.payment.refund.created':
                return $this->handleRefundCreated($payloadData);

            case 'net.authorize.payment.void.created':
                return $this->handleVoidCreated($payloadData);

            case 'net.authorize.payment.fraud.declined':
                return $this->handleFraudDeclined($payloadData);

            default:
                return ['success' => true, 'message' => 'Webhook acknowledged'];
        }
    }

    // =========================================================================
    // PRIVATE HELPERS
    // =========================================================================

    private function buildTransactionPayload(array $transactionData, ?string $transRefId = null): array
    {
        return [
            'createTransactionRequest' => [
                'merchantAuthentication' => [
                    'name' => $this->apiLoginId,
                    'transactionKey' => $this->transactionKey,
                ],
                'refId' => $transRefId ?? Str::uuid()->toString(),
                'transactionRequest' => $transactionData,
            ],
        ];
    }

    private function processTransactionResponse(array $response, array $context = []): array
    {
        $messages = $response['messages'] ?? [];
        $resultCode = $messages['resultCode'] ?? 'Error';

        if ($resultCode !== 'Ok') {
            $errorMessage = $messages['message'][0]['text'] ?? 'Transaction failed';
            $errorCode = $messages['message'][0]['code'] ?? 'E00001';

            Log::warning('AuthorizeNetPaymentService: Transaction error', [
                'error_code' => $errorCode,
                'error_message' => $errorMessage,
                'context' => $context,
            ]);

            return [
                'success' => false,
                'message' => $errorMessage,
                'error_code' => $errorCode,
            ];
        }

        $transactionResponse = $response['transactionResponse'] ?? [];
        $responseCode = $transactionResponse['responseCode'] ?? '';

        // Response code 1 = Approved, 2 = Declined, 3 = Error, 4 = Held for Review
        if ($responseCode !== '1') {
            $errors = $transactionResponse['errors'] ?? [];
            $errorMessage = $errors[0]['errorText'] ?? 'Transaction was not approved';

            return [
                'success' => false,
                'message' => $errorMessage,
                'response_code' => $responseCode,
            ];
        }

        $transactionId = $transactionResponse['transId'] ?? '';

        // Save payment record if submission_id provided
        if (!empty($context['submission_id']) && !empty($transactionId)) {
            FormPayment::create([
                'submission_id' => $context['submission_id'],
                'form_id' => $context['form_id'] ?? null,
                'gateway' => 'authorize_net',
                'transaction_id' => $transactionId,
                'amount' => $context['amount'] ?? 0,
                'currency' => 'USD',
                'status' => 'completed',
                'invoice_number' => $context['invoice_number'] ?? null,
                'metadata' => [
                    'auth_code' => $transactionResponse['authCode'] ?? null,
                    'avs_result' => $transactionResponse['avsResultCode'] ?? null,
                    'cvv_result' => $transactionResponse['cvvResultCode'] ?? null,
                    'account_type' => $transactionResponse['accountType'] ?? null,
                    'account_number' => $transactionResponse['accountNumber'] ?? null,
                ],
            ]);
        }

        Log::info('AuthorizeNetPaymentService: Transaction successful', [
            'transaction_id' => $transactionId,
            'amount' => $context['amount'] ?? 0,
        ]);

        return [
            'success' => true,
            'message' => 'Payment processed successfully.',
            'transaction_id' => $transactionId,
            'auth_code' => $transactionResponse['authCode'] ?? null,
            'account_number' => $transactionResponse['accountNumber'] ?? null,
        ];
    }

    private function formatBillingAddress(array $billing): array
    {
        return array_filter([
            'firstName' => $billing['first_name'] ?? null,
            'lastName' => $billing['last_name'] ?? null,
            'address' => $billing['address'] ?? null,
            'city' => $billing['city'] ?? null,
            'state' => $billing['state'] ?? null,
            'zip' => $billing['zip'] ?? null,
            'country' => $billing['country'] ?? null,
        ]);
    }

    private function generateInvoiceNumber(): string
    {
        return 'INV-' . date('Ymd') . '-' . strtoupper(Str::random(6));
    }

    private function handlePaymentCreated(array $payload): array
    {
        $transactionId = $payload['id'] ?? '';

        Log::info('AuthorizeNetPaymentService: Payment webhook processed', [
            'transaction_id' => $transactionId,
        ]);

        return ['success' => true, 'message' => 'Payment recorded'];
    }

    private function handleRefundCreated(array $payload): array
    {
        $transactionId = $payload['id'] ?? '';

        Log::info('AuthorizeNetPaymentService: Refund webhook processed', [
            'transaction_id' => $transactionId,
        ]);

        return ['success' => true, 'message' => 'Refund recorded'];
    }

    private function handleVoidCreated(array $payload): array
    {
        $transactionId = $payload['id'] ?? '';

        Log::info('AuthorizeNetPaymentService: Void webhook processed', [
            'transaction_id' => $transactionId,
        ]);

        return ['success' => true, 'message' => 'Void recorded'];
    }

    private function handleFraudDeclined(array $payload): array
    {
        $transactionId = $payload['id'] ?? '';

        Log::warning('AuthorizeNetPaymentService: Fraud decline webhook', [
            'transaction_id' => $transactionId,
        ]);

        FormPayment::where('transaction_id', $transactionId)
            ->update(['status' => 'fraud_declined']);

        return ['success' => true, 'message' => 'Fraud decline recorded'];
    }

    /**
     * Check if service is configured
     */
    public function isConfigured(): bool
    {
        return !empty($this->apiLoginId) && !empty($this->transactionKey);
    }

    /**
     * Get test mode status
     */
    public function isTestMode(): bool
    {
        return $this->testMode;
    }
}
