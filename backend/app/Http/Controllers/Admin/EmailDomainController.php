<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailDomain;
use App\Services\Email\AuditService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;

/**
 * EmailDomainController
 *
 * Enterprise domain management for email deliverability.
 *
 * Features:
 * - Domain verification
 * - DNS record management
 * - SPF, DKIM, DMARC configuration
 * - Domain health monitoring
 * - Reputation tracking
 *
 * @version 1.0.0
 */
class EmailDomainController extends Controller
{
    public function __construct(
        private readonly AuditService $auditService
    ) {}

    /**
     * List all domains
     */
    public function index(): JsonResponse
    {
        $domains = EmailDomain::latest()->get();

        return response()->json([
            'success' => true,
            'data' => $domains,
        ]);
    }

    /**
     * Get domain statistics
     */
    public function stats(): JsonResponse
    {
        $stats = [
            'total' => EmailDomain::count(),
            'verified' => EmailDomain::where('verified', true)->count(),
            'pending' => EmailDomain::where('verified', false)->count(),
            'with_dkim' => EmailDomain::where('dkim_verified', true)->count(),
            'with_spf' => EmailDomain::where('spf_verified', true)->count(),
            'with_dmarc' => EmailDomain::where('dmarc_verified', true)->count(),
        ];

        // Calculate overall health score
        $domains = EmailDomain::where('verified', true)->get();
        if ($domains->count() > 0) {
            $healthScores = $domains->map(fn($d) => $this->calculateHealthScore($d));
            $stats['avg_health_score'] = round($healthScores->avg(), 1);
        } else {
            $stats['avg_health_score'] = 0;
        }

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get single domain
     */
    public function show(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        $data = $domain->toArray();
        $data['health_score'] = $this->calculateHealthScore($domain);
        $data['dns_records'] = $this->getDnsRecords($domain);

        return response()->json([
            'success' => true,
            'data' => $data,
        ]);
    }

    /**
     * Add new domain
     */
    public function store(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'domain' => 'required|string|max:255|unique:email_domains,domain',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'success' => false,
                'errors' => $validator->errors(),
            ], 422);
        }

        $domainName = strtolower(trim($request->get('domain')));

        // Validate domain format
        if (!preg_match('/^[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,}$/', $domainName)) {
            return response()->json([
                'success' => false,
                'message' => 'Invalid domain format',
            ], 422);
        }

        $this->auditService->startTiming();

        // Generate verification token and DKIM keys
        $verificationToken = 'rtp-verify-' . Str::random(32);
        $dkimSelector = 'rtp' . date('Ymd');

        $domain = EmailDomain::create([
            'domain' => $domainName,
            'verification_token' => $verificationToken,
            'dkim_selector' => $dkimSelector,
            'verified' => false,
            'created_by' => auth()->id(),
        ]);

        $this->auditService->log(
            'create',
            'domain',
            $domain->id,
            $domain->domain,
            null,
            $domain->toArray()
        );

        // Return with setup instructions
        return response()->json([
            'success' => true,
            'data' => $domain,
            'dns_records' => $this->getRequiredDnsRecords($domain),
            'message' => 'Domain added. Please add the DNS records to verify ownership.',
        ], 201);
    }

    /**
     * Delete domain
     */
    public function destroy(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        $this->auditService->startTiming();
        $oldData = $domain->toArray();

        $domain->delete();

        $this->auditService->log(
            'delete',
            'domain',
            (int) $id,
            $oldData['domain'],
            $oldData,
            null
        );

        return response()->json([
            'success' => true,
            'message' => 'Domain deleted successfully',
        ]);
    }

    /**
     * Verify domain ownership
     */
    public function verify(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        if ($domain->verified) {
            return response()->json([
                'success' => true,
                'message' => 'Domain already verified',
            ]);
        }

        // Check TXT record
        $verified = $this->verifyDomainOwnership($domain);

        if ($verified) {
            $domain->update([
                'verified' => true,
                'verified_at' => now(),
            ]);

            $this->auditService->log(
                'verify',
                'domain',
                $domain->id,
                $domain->domain,
                ['verified' => false],
                ['verified' => true]
            );

            return response()->json([
                'success' => true,
                'data' => $domain->fresh(),
                'message' => 'Domain verified successfully',
            ]);
        }

        return response()->json([
            'success' => false,
            'message' => 'Domain verification failed. Please ensure the TXT record is properly configured.',
            'expected_record' => [
                'type' => 'TXT',
                'name' => '_rtp-verify.' . $domain->domain,
                'value' => $domain->verification_token,
            ],
        ], 422);
    }

    /**
     * Check DNS configuration
     */
    public function checkDns(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        $results = [
            'spf' => $this->checkSpf($domain),
            'dkim' => $this->checkDkim($domain),
            'dmarc' => $this->checkDmarc($domain),
            'mx' => $this->checkMx($domain),
        ];

        // Update domain record
        $domain->update([
            'spf_verified' => $results['spf']['valid'],
            'dkim_verified' => $results['dkim']['valid'],
            'dmarc_verified' => $results['dmarc']['valid'],
            'last_checked_at' => now(),
        ]);

        $results['health_score'] = $this->calculateHealthScore($domain->fresh());

        return response()->json([
            'success' => true,
            'data' => $results,
        ]);
    }

    /**
     * Get required DNS records for setup
     */
    public function dnsRecords(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        return response()->json([
            'success' => true,
            'data' => $this->getRequiredDnsRecords($domain),
        ]);
    }

    /**
     * Get deliverability report
     */
    public function deliverability(string|int $id): JsonResponse
    {
        $domain = EmailDomain::findOrFail((int) $id);

        // Get sending stats from last 30 days
        $stats = \DB::table('email_logs')
            ->where('from_email', 'like', '%@' . $domain->domain)
            ->where('created_at', '>=', now()->subDays(30))
            ->selectRaw('
                COUNT(*) as total_sent,
                SUM(CASE WHEN event_type = "delivered" THEN 1 ELSE 0 END) as delivered,
                SUM(CASE WHEN event_type = "bounced" THEN 1 ELSE 0 END) as bounced,
                SUM(CASE WHEN event_type = "complained" THEN 1 ELSE 0 END) as complaints
            ')
            ->first();

        $totalSent = $stats->total_sent ?? 0;
        $delivered = $stats->delivered ?? 0;
        $bounced = $stats->bounced ?? 0;
        $complaints = $stats->complaints ?? 0;

        $report = [
            'domain' => $domain->domain,
            'period' => '30 days',
            'metrics' => [
                'total_sent' => $totalSent,
                'delivered' => $delivered,
                'bounced' => $bounced,
                'complaints' => $complaints,
                'delivery_rate' => $totalSent > 0 ? round(($delivered / $totalSent) * 100, 2) : 100,
                'bounce_rate' => $totalSent > 0 ? round(($bounced / $totalSent) * 100, 2) : 0,
                'complaint_rate' => $totalSent > 0 ? round(($complaints / $totalSent) * 100, 4) : 0,
            ],
            'authentication' => [
                'spf' => $domain->spf_verified,
                'dkim' => $domain->dkim_verified,
                'dmarc' => $domain->dmarc_verified,
            ],
            'health_score' => $this->calculateHealthScore($domain),
            'recommendations' => $this->getRecommendations($domain, $stats),
        ];

        return response()->json([
            'success' => true,
            'data' => $report,
        ]);
    }

    /**
     * Verify domain ownership via DNS
     */
    private function verifyDomainOwnership(EmailDomain $domain): bool
    {
        try {
            $records = dns_get_record('_rtp-verify.' . $domain->domain, DNS_TXT);

            foreach ($records as $record) {
                if (isset($record['txt']) && $record['txt'] === $domain->verification_token) {
                    return true;
                }
            }
        } catch (\Exception $e) {
            // DNS lookup failed
        }

        return false;
    }

    /**
     * Check SPF record
     */
    private function checkSpf(EmailDomain $domain): array
    {
        try {
            $records = dns_get_record($domain->domain, DNS_TXT);

            foreach ($records as $record) {
                if (isset($record['txt']) && str_starts_with($record['txt'], 'v=spf1')) {
                    return [
                        'valid' => true,
                        'record' => $record['txt'],
                        'message' => 'SPF record found',
                    ];
                }
            }
        } catch (\Exception $e) {
            // DNS lookup failed
        }

        return [
            'valid' => false,
            'record' => null,
            'message' => 'No SPF record found',
        ];
    }

    /**
     * Check DKIM record
     */
    private function checkDkim(EmailDomain $domain): array
    {
        try {
            $selector = $domain->dkim_selector ?? 'default';
            $records = dns_get_record($selector . '._domainkey.' . $domain->domain, DNS_TXT);

            foreach ($records as $record) {
                if (isset($record['txt']) && str_contains($record['txt'], 'v=DKIM1')) {
                    return [
                        'valid' => true,
                        'record' => $record['txt'],
                        'selector' => $selector,
                        'message' => 'DKIM record found',
                    ];
                }
            }
        } catch (\Exception $e) {
            // DNS lookup failed
        }

        return [
            'valid' => false,
            'record' => null,
            'selector' => $domain->dkim_selector,
            'message' => 'No DKIM record found',
        ];
    }

    /**
     * Check DMARC record
     */
    private function checkDmarc(EmailDomain $domain): array
    {
        try {
            $records = dns_get_record('_dmarc.' . $domain->domain, DNS_TXT);

            foreach ($records as $record) {
                if (isset($record['txt']) && str_starts_with($record['txt'], 'v=DMARC1')) {
                    return [
                        'valid' => true,
                        'record' => $record['txt'],
                        'message' => 'DMARC record found',
                    ];
                }
            }
        } catch (\Exception $e) {
            // DNS lookup failed
        }

        return [
            'valid' => false,
            'record' => null,
            'message' => 'No DMARC record found',
        ];
    }

    /**
     * Check MX records
     */
    private function checkMx(EmailDomain $domain): array
    {
        try {
            $records = dns_get_record($domain->domain, DNS_MX);

            if (!empty($records)) {
                return [
                    'valid' => true,
                    'records' => array_map(fn($r) => [
                        'priority' => $r['pri'] ?? 0,
                        'host' => $r['target'] ?? '',
                    ], $records),
                    'message' => 'MX records found',
                ];
            }
        } catch (\Exception $e) {
            // DNS lookup failed
        }

        return [
            'valid' => false,
            'records' => [],
            'message' => 'No MX records found',
        ];
    }

    /**
     * Calculate domain health score
     */
    private function calculateHealthScore(EmailDomain $domain): int
    {
        $score = 0;

        // Verification (20 points)
        if ($domain->verified) {
            $score += 20;
        }

        // SPF (25 points)
        if ($domain->spf_verified) {
            $score += 25;
        }

        // DKIM (30 points)
        if ($domain->dkim_verified) {
            $score += 30;
        }

        // DMARC (25 points)
        if ($domain->dmarc_verified) {
            $score += 25;
        }

        return $score;
    }

    /**
     * Get required DNS records for setup
     */
    private function getRequiredDnsRecords(EmailDomain $domain): array
    {
        return [
            'verification' => [
                'type' => 'TXT',
                'name' => '_rtp-verify.' . $domain->domain,
                'value' => $domain->verification_token,
                'purpose' => 'Domain ownership verification',
            ],
            'spf' => [
                'type' => 'TXT',
                'name' => $domain->domain,
                'value' => 'v=spf1 include:_spf.revolutiontrading.com ~all',
                'purpose' => 'Sender Policy Framework - authorizes our servers to send on your behalf',
            ],
            'dkim' => [
                'type' => 'TXT',
                'name' => ($domain->dkim_selector ?? 'rtp') . '._domainkey.' . $domain->domain,
                'value' => 'v=DKIM1; k=rsa; p=YOUR_PUBLIC_KEY',
                'purpose' => 'DomainKeys Identified Mail - cryptographically signs emails',
                'note' => 'Contact support for your DKIM public key',
            ],
            'dmarc' => [
                'type' => 'TXT',
                'name' => '_dmarc.' . $domain->domain,
                'value' => 'v=DMARC1; p=quarantine; rua=mailto:dmarc@' . $domain->domain,
                'purpose' => 'Domain-based Message Authentication - policy for handling authentication failures',
            ],
        ];
    }

    /**
     * Get current DNS records
     */
    private function getDnsRecords(EmailDomain $domain): array
    {
        return [
            'spf' => $this->checkSpf($domain),
            'dkim' => $this->checkDkim($domain),
            'dmarc' => $this->checkDmarc($domain),
            'mx' => $this->checkMx($domain),
        ];
    }

    /**
     * Get recommendations for improving deliverability
     */
    private function getRecommendations(EmailDomain $domain, $stats): array
    {
        $recommendations = [];

        if (!$domain->spf_verified) {
            $recommendations[] = [
                'priority' => 'high',
                'type' => 'spf',
                'message' => 'Add SPF record to authorize sending servers',
            ];
        }

        if (!$domain->dkim_verified) {
            $recommendations[] = [
                'priority' => 'high',
                'type' => 'dkim',
                'message' => 'Configure DKIM for email authentication',
            ];
        }

        if (!$domain->dmarc_verified) {
            $recommendations[] = [
                'priority' => 'medium',
                'type' => 'dmarc',
                'message' => 'Add DMARC policy for improved deliverability',
            ];
        }

        $bounceRate = $stats->total_sent > 0
            ? ($stats->bounced / $stats->total_sent) * 100
            : 0;

        if ($bounceRate > 5) {
            $recommendations[] = [
                'priority' => 'high',
                'type' => 'list_hygiene',
                'message' => 'High bounce rate detected. Clean your email list.',
            ];
        }

        $complaintRate = $stats->total_sent > 0
            ? ($stats->complaints / $stats->total_sent) * 100
            : 0;

        if ($complaintRate > 0.1) {
            $recommendations[] = [
                'priority' => 'critical',
                'type' => 'complaints',
                'message' => 'Complaint rate above industry threshold. Review sending practices.',
            ];
        }

        return $recommendations;
    }
}
