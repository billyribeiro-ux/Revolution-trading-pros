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
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Password;
use Illuminate\Support\Str;
use Illuminate\Validation\ValidationException;
use Laravel\Sanctum\PersonalAccessToken;

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
            'password' => 'required|string|min:8|confirmed',
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
            $this->logSecurityEvent(null, 'login_failed', $request);
            throw ValidationException::withMessages([
                'email' => ['The provided credentials are incorrect.'],
            ]);
        }

        // Check if MFA is enabled
        if ($user->mfa_enabled) {
            $this->logSecurityEvent($user->id, 'login_mfa_required', $request);
            return response()->json([
                'user' => $user,
                'mfa_required' => true,
            ]);
        }

        // Create new session (revokes all previous sessions - single-session enforcement)
        $session = $this->sessionService->createSession($user, $request, true);
        $tokens = $this->createAuthTokens($user, $session);

        $this->logSecurityEvent($user->id, 'login', $request);

        return response()->json([
            'user' => $user,
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
            'password' => 'required|string|min:8|confirmed',
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
            'password' => 'required|string|min:8|confirmed',
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
            $this->logSecurityEvent(null, 'mfa_login_failed', $request);
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
            $backupCodes = $user->mfa_backup_codes ?? [];
            $codeIndex = array_search($request->backup_code, $backupCodes);
            
            if ($codeIndex === false) {
                $this->logSecurityEvent($user->id, 'mfa_backup_code_failed', $request);
                throw ValidationException::withMessages([
                    'backup_code' => ['Invalid backup code.'],
                ]);
            }

            // Remove used backup code
            unset($backupCodes[$codeIndex]);
            $user->mfa_backup_codes = array_values($backupCodes);
            $user->save();
        } else {
            // Verify TOTP code
            if (! $mfaService->verifyCode($user->mfa_secret, $request->mfa_code)) {
                $this->logSecurityEvent($user->id, 'mfa_code_failed', $request);
                throw ValidationException::withMessages([
                    'mfa_code' => ['Invalid MFA code.'],
                ]);
            }
        }

        // Create new session (revokes all previous sessions - single-session enforcement)
        $session = $this->sessionService->createSession($user, $request, true);
        $tokens = $this->createAuthTokens($user, $session);

        $this->logSecurityEvent($user->id, 'mfa_login_success', $request);

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
     * TODO: Implement full WebAuthn flow
     */
    public function loginWithBiometric(Request $request)
    {
        $request->validate([
            'credential' => 'required|string',
            'device_id' => 'required|string',
        ]);

        // TODO: Implement WebAuthn verification
        // For now, return error indicating not implemented
        return response()->json([
            'message' => 'Biometric login not yet implemented. Please use email/password.',
        ], 501);
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
        $backupCodes = $mfaService->generateBackupCodes();
        $qrCode = $mfaService->generateQRCode($user->email, $secret);

        // Store in user model (not enabled yet until verified)
        $user->mfa_secret = $secret;
        $user->mfa_backup_codes = $backupCodes;
        $user->save();

        $this->logSecurityEvent($user->id, 'mfa_setup_initiated', $request);

        return response()->json([
            'qr_code' => $qrCode,
            'secret' => $secret,
            'backup_codes' => $backupCodes,
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

        $this->logSecurityEvent($user->id, 'mfa_enabled', $request);

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

        $this->logSecurityEvent($user->id, 'mfa_disabled', $request);

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
