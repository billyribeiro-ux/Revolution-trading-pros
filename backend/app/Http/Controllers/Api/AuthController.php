<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\AuditLog;
use App\Models\User;
use App\Models\UserSession;
use App\Models\SecurityEvent;
use App\Services\MFAService;
use App\Services\SessionService;
use Illuminate\Auth\Events\Registered;
use Illuminate\Auth\Events\Verified;
use Illuminate\Auth\Events\PasswordReset;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Password;
use Illuminate\Support\Str;
use Illuminate\Validation\ValidationException;
use Laravel\Sanctum\PersonalAccessToken;
use App\Rules\StrongPassword;

/**
 * Revolution Trading Pros - AuthController
 * Enterprise-grade authentication with single-session enforcement
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */
class AuthController extends Controller
{
    /**
     * Session service instance
     */
    private SessionService $sessionService;

    public function __construct(SessionService $sessionService)
    {
        $this->sessionService = $sessionService;
    }

    /**
     * Create access and refresh tokens for the given user and session.
     */
    protected function createAuthTokens(User $user, UserSession $session): array
    {
        $accessExpiresIn = 60 * 60; // 1 hour in seconds
        $refreshExpiresIn = 60 * 60 * 24 * 30; // 30 days in seconds

        $accessToken = $user->createToken(
            'auth_token',
            ['*'],
            now()->addSeconds($accessExpiresIn)
        );

        $refreshToken = $user->createToken(
            'refresh_token',
            ['refresh'],
            now()->addSeconds($refreshExpiresIn)
        );

        // Link tokens to session
        PersonalAccessToken::where('id', $accessToken->accessToken->id)
            ->update([
                'user_session_id' => $session->id,
                'token_type' => 'access',
            ]);

        PersonalAccessToken::where('id', $refreshToken->accessToken->id)
            ->update([
                'user_session_id' => $session->id,
                'token_type' => 'refresh',
            ]);

        return [
            'token' => $accessToken->plainTextToken,
            'refresh_token' => $refreshToken->plainTextToken,
            'expires_in' => $accessExpiresIn,
            'session_id' => $session->session_id,
        ];
    }

    public function register(Request $request)
    {
        $request->validate([
            'name' => 'required|string|max:255',
            'email' => 'required|string|email|max:255|unique:users',
            'password' => ['required', 'string', 'confirmed', new StrongPassword],
        ]);

        $user = User::create([
            'name' => $request->name,
            'email' => $request->email,
            'password' => Hash::make($request->password),
        ]);

        event(new Registered($user));

        // Create session (no previous sessions to revoke for new user)
        $session = $this->sessionService->createSession($user, $request, false);
        $tokens = $this->createAuthTokens($user, $session);

        return response()->json([
            'user' => $user,
            'token' => $tokens['token'],
            'refresh_token' => $tokens['refresh_token'],
            'session_id' => $tokens['session_id'],
            'expires_in' => $tokens['expires_in'],
            'message' => 'Registration successful. Please verify your email.',
        ], 201);
    }

    public function login(Request $request)
    {
        $request->validate([
            'email' => 'required|email',
            'password' => 'required',
        ]);

        $user = User::where('email', $request->email)->first();

        if (! $user || ! Hash::check($request->password, $user->password)) {
            $this->logSecurityEvent(null, SecurityEvent::TYPE_LOGIN_FAILED, $request);
            throw ValidationException::withMessages([
                'email' => ['The provided credentials are incorrect.'],
            ]);
        }

        // Check if MFA is enabled
        if ($user->mfa_enabled) {
            $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_VERIFIED, $request);
            return response()->json([
                'user' => $user,
                'mfa_required' => true,
            ]);
        }

        // Create new session (revokes all previous sessions - single-session enforcement)
        $session = $this->sessionService->createSession($user, $request, true);
        $tokens = $this->createAuthTokens($user, $session);

        $this->logSecurityEvent($user->id, SecurityEvent::TYPE_LOGIN_SUCCESS, $request);

        return response()->json([
            'user' => array_merge($user->toArray(), [
                'roles' => $user->getRoleNames()->toArray(),
                'is_admin' => $user->hasRole(['admin', 'super_admin', 'super-admin']),
            ]),
            'token' => $tokens['token'],
            'refresh_token' => $tokens['refresh_token'],
            'session_id' => $tokens['session_id'],
            'expires_in' => $tokens['expires_in'],
        ]);
    }

    public function logout(Request $request)
    {
        $user = $request->user();
        $sessionId = $request->header('X-Session-ID');

        // Revoke the session if provided
        if ($sessionId) {
            $this->sessionService->revokeSession($sessionId, 'manual');
        }

        // Also delete the current access token
        $user->currentAccessToken()->delete();

        AuditLog::logAuth('logout', $user, ['session_id' => $sessionId]);

        return response()->json(['message' => 'Logged out successfully']);
    }

    /**
     * Logout from all devices (revoke all sessions).
     */
    public function logoutAllDevices(Request $request)
    {
        $user = $request->user();
        $currentSessionId = $request->header('X-Session-ID');

        // Revoke all sessions except current (optional)
        $keepCurrent = $request->input('keep_current', false);
        $exceptSession = $keepCurrent ? $currentSessionId : null;

        $count = $this->sessionService->revokeAllUserSessions($user, 'revoked_by_user', $exceptSession);

        // If not keeping current, also delete current token
        if (!$keepCurrent) {
            $user->currentAccessToken()->delete();
        }

        return response()->json([
            'message' => "Logged out from {$count} device(s)",
            'revoked_count' => $count,
        ]);
    }

    /**
     * Get active sessions for current user.
     */
    public function getSessions(Request $request)
    {
        $user = $request->user();
        $currentSessionId = $request->header('X-Session-ID');

        $sessions = $this->sessionService->getUserActiveSessions($user);

        // Mark the current session
        $sessions = array_map(function ($session) use ($currentSessionId) {
            $session['is_current'] = $session['session_id'] === $currentSessionId;
            return $session;
        }, $sessions);

        return response()->json([
            'sessions' => $sessions,
            'count' => count($sessions),
        ]);
    }

    /**
     * Revoke a specific session.
     */
    public function revokeSession(Request $request, string $sessionId)
    {
        $user = $request->user();

        // Verify session belongs to user
        $session = UserSession::where('session_id', $sessionId)
            ->where('user_id', $user->id)
            ->first();

        if (!$session) {
            return response()->json([
                'message' => 'Session not found',
            ], 404);
        }

        $this->sessionService->revokeSession($sessionId, 'revoked_by_user');

        return response()->json([
            'message' => 'Session revoked successfully',
        ]);
    }

    /**
     * Check if the current token is valid.
     */
    public function check(Request $request)
    {
        return response()->json(['valid' => true]);
    }

    /**
     * Refresh access token using a refresh token.
     */
    public function refreshToken(Request $request)
    {
        $request->validate([
            'refresh_token' => 'required|string',
        ]);

        $tokenModel = PersonalAccessToken::findToken($request->input('refresh_token'));

        if (! $tokenModel || ! in_array('refresh', $tokenModel->abilities ?? [])) {
            return response()->json([
                'message' => 'Invalid refresh token.',
            ], 401);
        }

        $user = $tokenModel->tokenable;

        // Get the session associated with this token
        $session = $tokenModel->user_session_id
            ? UserSession::find($tokenModel->user_session_id)
            : null;

        // If session exists, verify it's still valid
        if ($session && !$session->isValid()) {
            return response()->json([
                'message' => 'Session has been invalidated. Please login again.',
                'code' => 'SESSION_INVALIDATED',
            ], 401);
        }

        // Delete old tokens for this session
        PersonalAccessToken::where('user_session_id', $tokenModel->user_session_id)->delete();

        // Create new session if needed, or reuse existing
        if (!$session) {
            $session = $this->sessionService->createSession($user, $request, false);
        } else {
            // Extend session expiry
            $this->sessionService->extendSession($session->session_id);
        }

        $tokens = $this->createAuthTokens($user, $session);

        return response()->json([
            'token' => $tokens['token'],
            'refresh_token' => $tokens['refresh_token'],
            'session_id' => $tokens['session_id'],
            'expires_in' => $tokens['expires_in'],
        ]);
    }

    public function verify(Request $request)
    {
        $user = User::findOrFail($request->route('id'));

        if (! hash_equals((string) $request->route('hash'), sha1($user->getEmailForVerification()))) {
            return response()->json(['message' => 'Invalid verification link'], 403);
        }

        if ($user->hasVerifiedEmail()) {
            return response()->json(['message' => 'Email already verified']);
        }

        $user->markEmailAsVerified();
        event(new Verified($user));

        return response()->json(['message' => 'Email verified successfully']);
    }

    public function sendVerification(Request $request)
    {
        if ($request->user()->hasVerifiedEmail()) {
            return response()->json(['message' => 'Email already verified']);
        }

        $request->user()->sendEmailVerificationNotification();

        return response()->json(['message' => 'Verification link sent']);
    }

    public function forgotPassword(Request $request)
    {
        $request->validate([
            'email' => 'required|email',
        ]);

        $status = Password::sendResetLink(
            $request->only('email')
        );

        if ($status === Password::RESET_LINK_SENT) {
            return response()->json([
                'message' => __($status),
            ]);
        }

        throw ValidationException::withMessages([
            'email' => [__($status)],
        ]);
    }

    public function resetPassword(Request $request)
    {
        $request->validate([
            'token' => 'required',
            'email' => 'required|email',
            'password' => ['required', 'string', 'confirmed', new StrongPassword],
        ]);

        $status = Password::reset(
            $request->only('email', 'password', 'password_confirmation', 'token'),
            function ($user, $password) {
                $user->forceFill([
                    'password' => Hash::make($password),
                ])->setRememberToken(Str::random(60));

                $user->save();

                event(new PasswordReset($user));
            }
        );

        if ($status === Password::PASSWORD_RESET) {
            return response()->json([
                'message' => __($status),
            ]);
        }

        throw ValidationException::withMessages([
            'email' => [__($status)],
        ]);
    }

    public function changePassword(Request $request)
    {
        $user = $request->user();

        $request->validate([
            'current_password' => 'required|string',
            'password' => ['required', 'string', 'confirmed', new StrongPassword],
        ]);

        if (! Hash::check($request->current_password, $user->password)) {
            return response()->json([
                'message' => 'Current password is incorrect.',
            ], 422);
        }

        $user->password = Hash::make($request->password);
        $user->must_change_password = false;
        $user->save();

        return response()->json([
            'message' => 'Password updated successfully.',
        ]);
    }

    /**
     * Login with MFA code.
     */
    public function loginWithMFA(Request $request)
    {
        $request->validate([
            'email' => 'required|email',
            'password' => 'required',
            'mfa_code' => 'required_without:backup_code|string|size:6',
            'backup_code' => 'required_without:mfa_code|string',
        ]);

        $user = User::where('email', $request->email)->first();

        if (! $user || ! Hash::check($request->password, $user->password)) {
            $this->logSecurityEvent(null, SecurityEvent::TYPE_MFA_FAILED, $request);
            throw ValidationException::withMessages([
                'email' => ['The provided credentials are incorrect.'],
            ]);
        }

        if (! $user->mfa_enabled) {
            throw ValidationException::withMessages([
                'mfa_code' => ['MFA is not enabled for this account.'],
            ]);
        }

        $mfaService = new MFAService();

        // Check backup code first
        if ($request->backup_code) {
            $hashedBackupCodes = $user->mfa_backup_codes ?? [];

            // SECURITY: Use proper hashed backup code verification
            $codeIndex = $mfaService->verifyBackupCode($request->backup_code, $hashedBackupCodes);

            if ($codeIndex === false) {
                $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_FAILED, $request);
                throw ValidationException::withMessages([
                    'backup_code' => ['Invalid backup code.'],
                ]);
            }

            // Remove used backup code
            unset($hashedBackupCodes[$codeIndex]);
            $user->mfa_backup_codes = array_values($hashedBackupCodes);
            $user->save();
        } else {
            // Verify TOTP code
            if (! $mfaService->verifyCode($user->mfa_secret, $request->mfa_code)) {
                $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_FAILED, $request);
                throw ValidationException::withMessages([
                    'mfa_code' => ['Invalid MFA code.'],
                ]);
            }
        }

        // Create new session (revokes all previous sessions - single-session enforcement)
        $session = $this->sessionService->createSession($user, $request, true);
        $tokens = $this->createAuthTokens($user, $session);

        $this->logSecurityEvent($user->id, SecurityEvent::TYPE_LOGIN_SUCCESS, $request);

        return response()->json([
            'user' => $user,
            'token' => $tokens['token'],
            'refresh_token' => $tokens['refresh_token'],
            'session_id' => $tokens['session_id'],
            'expires_in' => $tokens['expires_in'],
        ]);
    }

    /**
     * Login with biometric (WebAuthn).
     * Implements FIDO2/WebAuthn passwordless authentication
     */
    public function loginWithBiometric(Request $request)
    {
        $request->validate([
            'credential' => 'required|string',
            'device_id' => 'required|string',
            'challenge' => 'required|string',
        ]);

        try {
            // Decode the credential
            $credentialData = json_decode(base64_decode($request->credential), true);

            if (!$credentialData || !isset($credentialData['id']) || !isset($credentialData['response'])) {
                return response()->json([
                    'message' => 'Invalid credential format.',
                ], 400);
            }

            // Find the user by their stored WebAuthn credential ID
            $user = \App\Models\User::whereJsonContains('webauthn_credentials', ['credential_id' => $credentialData['id']])
                ->first();

            if (!$user) {
                return response()->json([
                    'message' => 'No account found for this credential.',
                ], 401);
            }

            // Find the matching credential
            $storedCredential = collect($user->webauthn_credentials ?? [])
                ->firstWhere('credential_id', $credentialData['id']);

            if (!$storedCredential) {
                return response()->json([
                    'message' => 'Credential not found.',
                ], 401);
            }

            // Verify the challenge
            $expectedChallenge = Cache::get("webauthn:challenge:{$request->device_id}");
            if (!$expectedChallenge || $expectedChallenge !== $request->challenge) {
                return response()->json([
                    'message' => 'Challenge verification failed.',
                ], 401);
            }

            // Verify the authenticator response
            $verified = $this->verifyWebAuthnAssertion(
                $credentialData,
                $storedCredential,
                $expectedChallenge
            );

            if (!$verified) {
                $this->logSecurityEvent($user->id, 'webauthn_failed', $request);
                return response()->json([
                    'message' => 'Biometric verification failed.',
                ], 401);
            }

            // Clear the challenge
            Cache::forget("webauthn:challenge:{$request->device_id}");

            // Update credential sign count to prevent replay attacks
            $this->updateCredentialSignCount($user, $credentialData['id'], $credentialData['response']['authenticatorData'] ?? null);

            // Create session and tokens
            $session = $this->sessionService->createSession($user, $request, true);
            $tokens = $this->createAuthTokens($user, $session);

            $this->logSecurityEvent($user->id, 'webauthn_login', $request);

            return response()->json([
                'user' => $user,
                'token' => $tokens['token'],
                'refresh_token' => $tokens['refresh_token'],
                'session_id' => $tokens['session_id'],
                'expires_in' => $tokens['expires_in'],
            ]);

        } catch (\Exception $e) {
            Log::error('WebAuthn login failed', [
                'error' => $e->getMessage(),
                'device_id' => $request->device_id,
            ]);

            return response()->json([
                'message' => 'Biometric authentication failed.',
            ], 401);
        }
    }

    /**
     * Generate WebAuthn challenge for authentication
     */
    public function getWebAuthnChallenge(Request $request)
    {
        $request->validate([
            'device_id' => 'required|string',
        ]);

        // Generate a random challenge
        $challenge = base64_encode(random_bytes(32));

        // Store challenge with 5-minute expiry
        Cache::put("webauthn:challenge:{$request->device_id}", $challenge, 300);

        return response()->json([
            'challenge' => $challenge,
            'timeout' => 300000, // 5 minutes in milliseconds
            'rpId' => config('app.domain', parse_url(config('app.url'), PHP_URL_HOST)),
        ]);
    }

    /**
     * Register a new WebAuthn credential
     */
    public function registerWebAuthnCredential(Request $request)
    {
        $request->validate([
            'credential' => 'required|string',
            'device_name' => 'required|string|max:100',
        ]);

        $user = $request->user();

        try {
            $credentialData = json_decode(base64_decode($request->credential), true);

            if (!$credentialData || !isset($credentialData['id']) || !isset($credentialData['publicKey'])) {
                return response()->json([
                    'message' => 'Invalid credential format.',
                ], 400);
            }

            // Store the credential
            $credentials = $user->webauthn_credentials ?? [];
            $credentials[] = [
                'credential_id' => $credentialData['id'],
                'public_key' => $credentialData['publicKey'],
                'device_name' => $request->device_name,
                'sign_count' => 0,
                'created_at' => now()->toIso8601String(),
            ];

            $user->webauthn_credentials = $credentials;
            $user->save();

            $this->logSecurityEvent($user->id, 'webauthn_registered', $request);

            return response()->json([
                'message' => 'Biometric credential registered successfully.',
                'credential_count' => count($credentials),
            ]);

        } catch (\Exception $e) {
            Log::error('WebAuthn registration failed', [
                'user_id' => $user->id,
                'error' => $e->getMessage(),
            ]);

            return response()->json([
                'message' => 'Failed to register biometric credential.',
            ], 500);
        }
    }

    /**
     * Remove a WebAuthn credential
     */
    public function removeWebAuthnCredential(Request $request)
    {
        $request->validate([
            'credential_id' => 'required|string',
            'password' => 'required|string',
        ]);

        $user = $request->user();

        if (!Hash::check($request->password, $user->password)) {
            return response()->json([
                'message' => 'Invalid password.',
            ], 422);
        }

        $credentials = collect($user->webauthn_credentials ?? [])
            ->filter(fn($c) => $c['credential_id'] !== $request->credential_id)
            ->values()
            ->toArray();

        $user->webauthn_credentials = $credentials;
        $user->save();

        $this->logSecurityEvent($user->id, 'webauthn_removed', $request);

        return response()->json([
            'message' => 'Biometric credential removed.',
            'credential_count' => count($credentials),
        ]);
    }

    /**
     * Verify WebAuthn assertion
     * ICT11+ Principal Engineer: Full FIDO2/WebAuthn signature verification
     */
    private function verifyWebAuthnAssertion(array $credentialData, array $storedCredential, string $challenge): bool
    {
        try {
            // 1. Decode and verify client data
            $clientDataJSON = base64_decode($credentialData['response']['clientDataJSON'] ?? '');
            if (!$clientDataJSON) {
                Log::warning('WebAuthn: Invalid clientDataJSON encoding');
                return false;
            }
            
            $clientData = json_decode($clientDataJSON, true);
            if (!$clientData) {
                Log::warning('WebAuthn: Invalid clientDataJSON format');
                return false;
            }

            // Verify challenge matches
            if (($clientData['challenge'] ?? '') !== $challenge) {
                Log::warning('WebAuthn: Challenge mismatch');
                return false;
            }

            // Verify type is webauthn.get
            if (($clientData['type'] ?? '') !== 'webauthn.get') {
                Log::warning('WebAuthn: Invalid type', ['type' => $clientData['type'] ?? 'null']);
                return false;
            }

            // Verify origin
            $expectedOrigin = config('app.url');
            if (!str_starts_with($clientData['origin'] ?? '', $expectedOrigin)) {
                Log::warning('WebAuthn: Origin mismatch', [
                    'expected' => $expectedOrigin,
                    'received' => $clientData['origin'] ?? 'null'
                ]);
                return false;
            }

            // 2. Decode authenticator data
            $authenticatorData = base64_decode($credentialData['response']['authenticatorData'] ?? '');
            if (!$authenticatorData || strlen($authenticatorData) < 37) {
                Log::warning('WebAuthn: Invalid authenticatorData');
                return false;
            }

            // Verify RP ID hash (first 32 bytes)
            $rpIdHash = substr($authenticatorData, 0, 32);
            $expectedRpId = config('app.domain', parse_url(config('app.url'), PHP_URL_HOST));
            $expectedRpIdHash = hash('sha256', $expectedRpId, true);
            
            if (!hash_equals($expectedRpIdHash, $rpIdHash)) {
                Log::warning('WebAuthn: RP ID hash mismatch');
                return false;
            }

            // Verify flags (byte 32)
            $flags = ord($authenticatorData[32]);
            $userPresent = ($flags & 0x01) !== 0;
            
            if (!$userPresent) {
                Log::warning('WebAuthn: User presence flag not set');
                return false;
            }

            // 3. Verify sign count to prevent replay attacks (bytes 33-36)
            $signCount = unpack('N', substr($authenticatorData, 33, 4))[1];
            $storedSignCount = $storedCredential['sign_count'] ?? 0;
            
            if ($signCount !== 0 && $signCount <= $storedSignCount) {
                Log::warning('WebAuthn: Sign count regression detected (possible cloned authenticator)', [
                    'stored' => $storedSignCount,
                    'received' => $signCount
                ]);
                return false;
            }

            // 4. Verify signature using stored public key
            $signature = base64_decode($credentialData['response']['signature'] ?? '');
            if (!$signature) {
                Log::warning('WebAuthn: Missing signature');
                return false;
            }

            $publicKeyPem = $storedCredential['public_key'] ?? null;
            if (!$publicKeyPem) {
                Log::warning('WebAuthn: Missing stored public key');
                return false;
            }

            // Construct the signed data: authenticatorData + SHA256(clientDataJSON)
            $clientDataHash = hash('sha256', $clientDataJSON, true);
            $signedData = $authenticatorData . $clientDataHash;

            // Verify signature with public key
            $publicKey = openssl_pkey_get_public($publicKeyPem);
            if (!$publicKey) {
                Log::warning('WebAuthn: Invalid public key format');
                return false;
            }

            // WebAuthn uses ECDSA with SHA-256 for ES256 algorithm
            $verified = openssl_verify($signedData, $signature, $publicKey, OPENSSL_ALGO_SHA256);
            
            if ($verified !== 1) {
                Log::warning('WebAuthn: Signature verification failed', [
                    'openssl_error' => openssl_error_string()
                ]);
                return false;
            }

            Log::info('WebAuthn: Assertion verified successfully');
            return true;

        } catch (\Exception $e) {
            Log::warning('WebAuthn verification error', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Update credential sign count to prevent replay attacks
     */
    private function updateCredentialSignCount(\App\Models\User $user, string $credentialId, ?string $authenticatorData): void
    {
        if (!$authenticatorData) {
            return;
        }

        try {
            $data = base64_decode($authenticatorData);
            // Sign count is a 32-bit big-endian unsigned integer starting at byte 33
            if (strlen($data) >= 37) {
                $signCount = unpack('N', substr($data, 33, 4))[1];

                $credentials = collect($user->webauthn_credentials ?? [])
                    ->map(function ($c) use ($credentialId, $signCount) {
                        if ($c['credential_id'] === $credentialId) {
                            $c['sign_count'] = $signCount;
                            $c['last_used_at'] = now()->toIso8601String();
                        }
                        return $c;
                    })
                    ->toArray();

                $user->webauthn_credentials = $credentials;
                $user->save();
            }
        } catch (\Exception $e) {
            Log::warning('Failed to update WebAuthn sign count', ['error' => $e->getMessage()]);
        }
    }

    public function listSecurityEvents(Request $request)
    {
        $events = SecurityEvent::where('user_id', $request->user()->id)
            ->orderBy('created_at', 'desc')
            ->limit(50)
            ->get()
            ->map(function ($event) {
                return [
                    'type' => $event->type,
                    'ip_address' => $event->ip_address,
                    'user_agent' => $event->user_agent,
                    'location' => $event->location,
                    'timestamp' => $event->created_at->toISOString(),
                ];
            });

        return response()->json($events);
    }

    /**
     * Enable MFA for the authenticated user.
     */
    public function enableMFA(Request $request)
    {
        $user = $request->user();

        if ($user->mfa_enabled) {
            return response()->json([
                'message' => 'MFA is already enabled.',
            ], 422);
        }

        $mfaService = new MFAService();

        // Generate secret and backup codes
        $secret = $mfaService->generateSecret();
        $backupCodesResult = $mfaService->generateBackupCodes();
        $qrCode = $mfaService->generateQRCode($user->email, $secret);

        // Store in user model (not enabled yet until verified)
        // SECURITY: Only store hashed backup codes
        $user->mfa_secret = $secret;
        $user->mfa_backup_codes = $backupCodesResult['hashed'];
        $user->save();

        $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_ENABLED, $request);

        // SECURITY: Return plain codes ONLY ONCE - user must save them
        return response()->json([
            'qr_code' => $qrCode,
            'secret' => $secret,
            'backup_codes' => $backupCodesResult['plain'],
        ]);
    }

    /**
     * Verify and enable MFA.
     */
    public function verifyMFA(Request $request)
    {
        $request->validate([
            'code' => 'required|string|size:6',
        ]);

        $user = $request->user();

        if ($user->mfa_enabled) {
            return response()->json([
                'message' => 'MFA is already enabled.',
            ], 422);
        }

        if (! $user->mfa_secret) {
            return response()->json([
                'message' => 'MFA setup not initiated.',
            ], 422);
        }

        $mfaService = new MFAService();

        if (! $mfaService->verifyCode($user->mfa_secret, $request->code)) {
            return response()->json([
                'message' => 'Invalid MFA code.',
            ], 422);
        }

        // Enable MFA
        $user->mfa_enabled = true;
        $user->mfa_enabled_at = now();
        $user->save();

        $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_ENABLED, $request);

        return response()->json([
            'message' => 'MFA enabled successfully.',
        ]);
    }

    /**
     * Disable MFA for the authenticated user.
     */
    public function disableMFA(Request $request)
    {
        $request->validate([
            'password' => 'required|string',
        ]);

        $user = $request->user();

        if (! Hash::check($request->password, $user->password)) {
            return response()->json([
                'message' => 'Invalid password.',
            ], 422);
        }

        if (! $user->mfa_enabled) {
            return response()->json([
                'message' => 'MFA is not enabled.',
            ], 422);
        }

        // Disable MFA
        $user->mfa_enabled = false;
        $user->mfa_secret = null;
        $user->mfa_backup_codes = null;
        $user->mfa_enabled_at = null;
        $user->save();

        $this->logSecurityEvent($user->id, SecurityEvent::TYPE_MFA_DISABLED, $request);

        return response()->json([
            'message' => 'MFA disabled successfully.',
        ]);
    }

    /**
     * Track security events from the frontend.
     */
    public function securityEvent(Request $request)
    {
        $data = $request->all();
        
        $this->logSecurityEvent(
            optional($request->user())->id,
            $data['type'] ?? 'unknown',
            $request,
            $data
        );

        return response()->json(['message' => 'Event logged']);
    }

    /**
     * Helper to log security events.
     */
    private function logSecurityEvent(?int $userId, string $type, Request $request, array $metadata = [])
    {
        SecurityEvent::create([
            'user_id' => $userId,
            'type' => $type,
            'ip_address' => $request->ip(),
            'user_agent' => $request->userAgent(),
            'location' => null, // Could integrate with IP geolocation service
            'metadata' => $metadata,
        ]);
    }
}
