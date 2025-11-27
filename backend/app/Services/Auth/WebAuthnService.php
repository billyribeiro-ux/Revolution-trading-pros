<?php

declare(strict_types=1);

namespace App\Services\Auth;

use App\Models\User;
use App\Models\WebAuthnCredential;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Throwable;
use Webauthn\AttestationStatement\AttestationStatementSupportManager;
use Webauthn\AttestationStatement\NoneAttestationStatementSupport;
use Webauthn\AuthenticatorAttestationResponse;
use Webauthn\AuthenticatorAssertionResponse;
use Webauthn\AuthenticatorSelectionCriteria;
use Webauthn\PublicKeyCredentialCreationOptions;
use Webauthn\PublicKeyCredentialRequestOptions;
use Webauthn\PublicKeyCredentialRpEntity;
use Webauthn\PublicKeyCredentialUserEntity;
use Webauthn\PublicKeyCredentialParameters;
use Webauthn\PublicKeyCredentialDescriptor;
use Webauthn\AuthenticatorAttestationResponseValidator;
use Webauthn\AuthenticatorAssertionResponseValidator;

/**
 * WebAuthn Service - Biometric/Passkey Authentication
 *
 * Enterprise-grade WebAuthn implementation supporting:
 * - Passkeys (platform authenticators)
 * - Security keys (roaming authenticators)
 * - Cross-platform registration and authentication
 *
 * @version 1.0.0
 * @level L11 Principal Engineer
 */
class WebAuthnService
{
    private string $rpId;
    private string $rpName;
    private string $rpOrigin;

    public function __construct()
    {
        $this->rpId = config('webauthn.rp_id', parse_url(config('app.url'), PHP_URL_HOST));
        $this->rpName = config('webauthn.rp_name', config('app.name'));
        $this->rpOrigin = config('app.url');
    }

    /**
     * Generate registration options for a new credential
     */
    public function generateRegistrationOptions(User $user, string $credentialName = 'Passkey'): array
    {
        $challenge = random_bytes(32);
        $challengeB64 = $this->base64UrlEncode($challenge);

        // Store challenge for verification
        $this->storeChallenge($user->id, 'registration', $challengeB64);

        // Get existing credentials to exclude
        $excludeCredentials = $this->getExcludeCredentials($user);

        // Supported algorithms (ES256, RS256)
        $pubKeyCredParams = [
            ['type' => 'public-key', 'alg' => -7],   // ES256
            ['type' => 'public-key', 'alg' => -257], // RS256
        ];

        $options = [
            'challenge' => $challengeB64,
            'rp' => [
                'id' => $this->rpId,
                'name' => $this->rpName,
            ],
            'user' => [
                'id' => $this->base64UrlEncode($user->id . '-' . $user->email),
                'name' => $user->email,
                'displayName' => $user->name ?? $user->email,
            ],
            'pubKeyCredParams' => $pubKeyCredParams,
            'authenticatorSelection' => [
                'authenticatorAttachment' => 'platform', // Prefer platform (biometric)
                'requireResidentKey' => true,           // Discoverable credential
                'residentKey' => 'required',
                'userVerification' => 'required',       // Require biometric/PIN
            ],
            'attestation' => 'none', // Don't require attestation for better compatibility
            'timeout' => 60000,      // 60 seconds
            'excludeCredentials' => $excludeCredentials,
            'extensions' => [
                'credProps' => true, // Request credential properties
            ],
        ];

        Log::info('WebAuthn registration options generated', [
            'user_id' => $user->id,
            'rp_id' => $this->rpId,
        ]);

        return $options;
    }

    /**
     * Verify and store a new credential registration
     */
    public function verifyRegistration(
        User $user,
        array $credential,
        string $credentialName = 'Passkey'
    ): WebAuthnCredential {
        // Retrieve and verify challenge
        $storedChallenge = $this->getStoredChallenge($user->id, 'registration');
        if (!$storedChallenge) {
            throw new \RuntimeException('Registration challenge not found or expired');
        }

        // Validate the response
        $clientDataJSON = $this->base64UrlDecode($credential['response']['clientDataJSON']);
        $attestationObject = $this->base64UrlDecode($credential['response']['attestationObject']);

        $clientData = json_decode($clientDataJSON, true);

        // Verify challenge
        if ($clientData['challenge'] !== $storedChallenge) {
            throw new \RuntimeException('Challenge mismatch');
        }

        // Verify origin
        if ($clientData['origin'] !== $this->rpOrigin) {
            throw new \RuntimeException('Origin mismatch');
        }

        // Verify type
        if ($clientData['type'] !== 'webauthn.create') {
            throw new \RuntimeException('Invalid type');
        }

        // Parse attestation object
        $attestation = $this->parseAttestationObject($attestationObject);
        $authData = $attestation['authData'];
        $credentialId = $this->base64UrlEncode($authData['credentialId']);
        $publicKey = $authData['publicKey'];

        // Store credential
        $webAuthnCredential = WebAuthnCredential::create([
            'user_id' => $user->id,
            'credential_id' => $credentialId,
            'public_key' => json_encode($publicKey),
            'name' => $credentialName,
            'counter' => $authData['signCount'],
            'transports' => $credential['response']['transports'] ?? ['internal'],
            'aaguid' => $authData['aaguid'] ?? null,
            'last_used_at' => null,
        ]);

        // Clear challenge
        $this->clearChallenge($user->id, 'registration');

        Log::info('WebAuthn credential registered', [
            'user_id' => $user->id,
            'credential_id' => $credentialId,
        ]);

        return $webAuthnCredential;
    }

    /**
     * Generate authentication options for login
     */
    public function generateAuthenticationOptions(?User $user = null): array
    {
        $challenge = random_bytes(32);
        $challengeB64 = $this->base64UrlEncode($challenge);

        $userId = $user?->id ?? 'anonymous';
        $this->storeChallenge($userId, 'authentication', $challengeB64);

        $options = [
            'challenge' => $challengeB64,
            'rpId' => $this->rpId,
            'timeout' => 60000,
            'userVerification' => 'required',
        ];

        // If user is known, include their credentials
        if ($user) {
            $credentials = $user->webAuthnCredentials()->get();
            $options['allowCredentials'] = $credentials->map(function ($cred) {
                return [
                    'type' => 'public-key',
                    'id' => $cred->credential_id,
                    'transports' => $cred->transports ?? ['internal'],
                ];
            })->toArray();
        }

        Log::info('WebAuthn authentication options generated', [
            'user_id' => $userId,
        ]);

        return $options;
    }

    /**
     * Verify authentication and return user
     */
    public function verifyAuthentication(array $credential): User
    {
        $credentialId = $credential['id'];

        // Find credential
        $webAuthnCredential = WebAuthnCredential::where('credential_id', $credentialId)->first();
        if (!$webAuthnCredential) {
            throw new \RuntimeException('Credential not found');
        }

        $user = $webAuthnCredential->user;
        $storedChallenge = $this->getStoredChallenge($user->id, 'authentication')
            ?? $this->getStoredChallenge('anonymous', 'authentication');

        if (!$storedChallenge) {
            throw new \RuntimeException('Authentication challenge not found or expired');
        }

        // Parse response
        $clientDataJSON = $this->base64UrlDecode($credential['response']['clientDataJSON']);
        $authenticatorData = $this->base64UrlDecode($credential['response']['authenticatorData']);
        $signature = $this->base64UrlDecode($credential['response']['signature']);

        $clientData = json_decode($clientDataJSON, true);

        // Verify challenge
        if ($clientData['challenge'] !== $storedChallenge) {
            throw new \RuntimeException('Challenge mismatch');
        }

        // Verify origin
        if ($clientData['origin'] !== $this->rpOrigin) {
            throw new \RuntimeException('Origin mismatch');
        }

        // Verify type
        if ($clientData['type'] !== 'webauthn.get') {
            throw new \RuntimeException('Invalid type');
        }

        // Verify signature
        $publicKey = json_decode($webAuthnCredential->public_key, true);
        $hash = hash('sha256', $clientDataJSON, true);
        $signedData = $authenticatorData . $hash;

        if (!$this->verifySignature($publicKey, $signedData, $signature)) {
            throw new \RuntimeException('Invalid signature');
        }

        // Verify counter (replay attack prevention)
        $authDataParsed = $this->parseAuthenticatorData($authenticatorData);
        $newCounter = $authDataParsed['signCount'];

        if ($newCounter > 0 && $newCounter <= $webAuthnCredential->counter) {
            Log::warning('WebAuthn counter replay detected', [
                'user_id' => $user->id,
                'credential_id' => $credentialId,
                'stored_counter' => $webAuthnCredential->counter,
                'received_counter' => $newCounter,
            ]);
            throw new \RuntimeException('Potential cloned authenticator detected');
        }

        // Update credential
        $webAuthnCredential->update([
            'counter' => $newCounter,
            'last_used_at' => now(),
        ]);

        // Clear challenge
        $this->clearChallenge($user->id, 'authentication');
        $this->clearChallenge('anonymous', 'authentication');

        Log::info('WebAuthn authentication successful', [
            'user_id' => $user->id,
            'credential_id' => $credentialId,
        ]);

        return $user;
    }

    /**
     * Get user's registered credentials
     */
    public function getCredentials(User $user): array
    {
        return $user->webAuthnCredentials()
            ->select(['id', 'name', 'created_at', 'last_used_at'])
            ->get()
            ->toArray();
    }

    /**
     * Delete a credential
     */
    public function deleteCredential(User $user, string $credentialId): bool
    {
        $deleted = $user->webAuthnCredentials()
            ->where('id', $credentialId)
            ->delete();

        if ($deleted) {
            Log::info('WebAuthn credential deleted', [
                'user_id' => $user->id,
                'credential_id' => $credentialId,
            ]);
        }

        return $deleted > 0;
    }

    /**
     * Check if user has any WebAuthn credentials
     */
    public function hasCredentials(User $user): bool
    {
        return $user->webAuthnCredentials()->exists();
    }

    // Private helper methods

    private function storeChallenge(mixed $userId, string $type, string $challenge): void
    {
        Cache::put(
            "webauthn:{$type}:{$userId}",
            $challenge,
            now()->addMinutes(5)
        );
    }

    private function getStoredChallenge(mixed $userId, string $type): ?string
    {
        return Cache::get("webauthn:{$type}:{$userId}");
    }

    private function clearChallenge(mixed $userId, string $type): void
    {
        Cache::forget("webauthn:{$type}:{$userId}");
    }

    private function getExcludeCredentials(User $user): array
    {
        return $user->webAuthnCredentials()
            ->get()
            ->map(function ($cred) {
                return [
                    'type' => 'public-key',
                    'id' => $cred->credential_id,
                    'transports' => $cred->transports ?? ['internal'],
                ];
            })
            ->toArray();
    }

    private function base64UrlEncode(string $data): string
    {
        return rtrim(strtr(base64_encode($data), '+/', '-_'), '=');
    }

    private function base64UrlDecode(string $data): string
    {
        return base64_decode(strtr($data, '-_', '+/') . str_repeat('=', 3 - (3 + strlen($data)) % 4));
    }

    private function parseAttestationObject(string $attestationObject): array
    {
        // CBOR decode (simplified - in production use a proper CBOR library)
        // This is a simplified parser for demonstration
        $decoded = $this->cborDecode($attestationObject);

        $authData = $this->parseAuthenticatorData($decoded['authData']);

        return [
            'fmt' => $decoded['fmt'],
            'authData' => $authData,
            'attStmt' => $decoded['attStmt'] ?? [],
        ];
    }

    private function parseAuthenticatorData(string $authData): array
    {
        $rpIdHash = substr($authData, 0, 32);
        $flags = ord($authData[32]);
        $signCount = unpack('N', substr($authData, 33, 4))[1];

        $result = [
            'rpIdHash' => bin2hex($rpIdHash),
            'flags' => $flags,
            'signCount' => $signCount,
            'userPresent' => (bool)($flags & 0x01),
            'userVerified' => (bool)($flags & 0x04),
            'attestedCredentialData' => (bool)($flags & 0x40),
        ];

        // Parse attested credential data if present
        if ($result['attestedCredentialData'] && strlen($authData) > 37) {
            $offset = 37;
            $aaguid = substr($authData, $offset, 16);
            $offset += 16;

            $credIdLength = unpack('n', substr($authData, $offset, 2))[1];
            $offset += 2;

            $credentialId = substr($authData, $offset, $credIdLength);
            $offset += $credIdLength;

            // Public key is CBOR encoded
            $publicKeyData = substr($authData, $offset);
            $publicKey = $this->cborDecode($publicKeyData);

            $result['aaguid'] = bin2hex($aaguid);
            $result['credentialId'] = $credentialId;
            $result['publicKey'] = $publicKey;
        }

        return $result;
    }

    private function cborDecode(string $data): mixed
    {
        // Simplified CBOR decoder
        // In production, use a proper CBOR library like spomky-labs/cbor-php
        // This handles the basic cases needed for WebAuthn

        $offset = 0;
        return $this->cborDecodeItem($data, $offset);
    }

    private function cborDecodeItem(string $data, int &$offset): mixed
    {
        if ($offset >= strlen($data)) {
            return null;
        }

        $byte = ord($data[$offset]);
        $majorType = $byte >> 5;
        $additionalInfo = $byte & 0x1f;
        $offset++;

        $value = $this->cborDecodeLength($data, $offset, $additionalInfo);

        switch ($majorType) {
            case 0: // Unsigned integer
                return $value;
            case 1: // Negative integer
                return -1 - $value;
            case 2: // Byte string
                $result = substr($data, $offset, $value);
                $offset += $value;
                return $result;
            case 3: // Text string
                $result = substr($data, $offset, $value);
                $offset += $value;
                return $result;
            case 4: // Array
                $array = [];
                for ($i = 0; $i < $value; $i++) {
                    $array[] = $this->cborDecodeItem($data, $offset);
                }
                return $array;
            case 5: // Map
                $map = [];
                for ($i = 0; $i < $value; $i++) {
                    $key = $this->cborDecodeItem($data, $offset);
                    $map[$key] = $this->cborDecodeItem($data, $offset);
                }
                return $map;
            default:
                return null;
        }
    }

    private function cborDecodeLength(string $data, int &$offset, int $additionalInfo): int
    {
        if ($additionalInfo < 24) {
            return $additionalInfo;
        }
        if ($additionalInfo === 24) {
            return ord($data[$offset++]);
        }
        if ($additionalInfo === 25) {
            $result = unpack('n', substr($data, $offset, 2))[1];
            $offset += 2;
            return $result;
        }
        if ($additionalInfo === 26) {
            $result = unpack('N', substr($data, $offset, 4))[1];
            $offset += 4;
            return $result;
        }
        return 0;
    }

    private function verifySignature(array $publicKey, string $data, string $signature): bool
    {
        // COSE key type -7 = ES256 (ECDSA with P-256)
        if (($publicKey[3] ?? null) === -7) {
            // ES256 verification
            $x = $publicKey[-2];
            $y = $publicKey[-3];

            // Build PEM public key
            $der = "\x30\x59\x30\x13\x06\x07\x2a\x86\x48\xce\x3d\x02\x01\x06\x08\x2a\x86\x48\xce\x3d\x03\x01\x07\x03\x42\x00\x04" . $x . $y;
            $pem = "-----BEGIN PUBLIC KEY-----\n" . chunk_split(base64_encode($der), 64) . "-----END PUBLIC KEY-----";

            // Convert DER signature to raw format for openssl
            $rawSignature = $this->derToRaw($signature);

            return openssl_verify($data, $rawSignature, $pem, OPENSSL_ALGO_SHA256) === 1;
        }

        // RS256 verification
        if (($publicKey[3] ?? null) === -257) {
            $n = $publicKey[-1];
            $e = $publicKey[-2];

            // Build RSA public key
            $pem = $this->buildRsaPublicKey($n, $e);

            return openssl_verify($data, $signature, $pem, OPENSSL_ALGO_SHA256) === 1;
        }

        return false;
    }

    private function derToRaw(string $der): string
    {
        // WebAuthn signatures are in DER format, convert to raw R|S format
        $offset = 2;
        if (ord($der[1]) & 0x80) {
            $offset += ord($der[1]) & 0x7f;
        }

        $rLength = ord($der[$offset + 1]);
        $r = substr($der, $offset + 2, $rLength);
        $offset += 2 + $rLength;

        $sLength = ord($der[$offset + 1]);
        $s = substr($der, $offset + 2, $sLength);

        // Pad to 32 bytes each
        $r = str_pad(ltrim($r, "\x00"), 32, "\x00", STR_PAD_LEFT);
        $s = str_pad(ltrim($s, "\x00"), 32, "\x00", STR_PAD_LEFT);

        return $r . $s;
    }

    private function buildRsaPublicKey(string $n, string $e): string
    {
        $modulus = "\x00" . $n;
        $exponent = $e;

        $modulusLen = strlen($modulus);
        $exponentLen = strlen($exponent);

        $sequence = "\x02" . chr($modulusLen) . $modulus . "\x02" . chr($exponentLen) . $exponent;
        $sequenceLen = strlen($sequence);

        $bitString = "\x00" . $sequence;
        $bitStringLen = strlen($bitString);

        $rsaOid = "\x30\x0d\x06\x09\x2a\x86\x48\x86\xf7\x0d\x01\x01\x01\x05\x00";

        $pubKeySequence = $rsaOid . "\x03" . chr($bitStringLen) . $bitString;
        $totalLen = strlen($pubKeySequence);

        $der = "\x30" . chr($totalLen) . $pubKeySequence;

        return "-----BEGIN PUBLIC KEY-----\n" . chunk_split(base64_encode($der), 64) . "-----END PUBLIC KEY-----";
    }
}
