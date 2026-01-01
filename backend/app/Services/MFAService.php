<?php

namespace App\Services;

use Illuminate\Support\Str;
use Illuminate\Support\Facades\Hash;

class MFAService
{
    /**
     * Base32 alphabet for TOTP secrets (RFC 4648)
     */
    private const BASE32_ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';

    /**
     * Generate a random TOTP secret in valid base32 format.
     * Generates a 160-bit (20 byte) secret encoded as 32 base32 characters.
     */
    public function generateSecret(): string
    {
        // Generate 20 random bytes (160 bits) - standard for TOTP
        $randomBytes = random_bytes(20);

        // Encode to base32 (RFC 4648)
        return $this->base32Encode($randomBytes);
    }

    /**
     * Encode binary data to base32.
     */
    private function base32Encode(string $data): string
    {
        $binary = '';
        foreach (str_split($data) as $char) {
            $binary .= str_pad(decbin(ord($char)), 8, '0', STR_PAD_LEFT);
        }

        $base32 = '';
        $chunks = str_split($binary, 5);

        foreach ($chunks as $chunk) {
            // Pad last chunk if necessary
            $chunk = str_pad($chunk, 5, '0', STR_PAD_RIGHT);
            $index = bindec($chunk);
            $base32 .= self::BASE32_ALPHABET[$index];
        }

        return $base32;
    }

    /**
     * Generate a QR code URL for Google Authenticator.
     */
    public function generateQRCode(string $email, string $secret): string
    {
        $appName = config('app.name', 'Revolution Trading Pros');
        $otpauthUrl = sprintf(
            'otpauth://totp/%s:%s?secret=%s&issuer=%s',
            urlencode($appName),
            urlencode($email),
            $secret,
            urlencode($appName)
        );

        // Return a Google Charts QR code URL
        return sprintf(
            'https://chart.googleapis.com/chart?chs=200x200&cht=qr&chl=%s',
            urlencode($otpauthUrl)
        );
    }

    /**
     * Generate backup codes.
     * Returns both plain codes (to show user once) and hashed codes (to store).
     */
    public function generateBackupCodes(int $count = 8): array
    {
        $plainCodes = [];
        $hashedCodes = [];

        for ($i = 0; $i < $count; $i++) {
            // Generate cryptographically secure code
            $code = strtoupper(bin2hex(random_bytes(2)) . '-' . bin2hex(random_bytes(2)));
            $plainCodes[] = $code;
            $hashedCodes[] = Hash::make($code);
        }

        return [
            'plain' => $plainCodes,      // Show to user ONCE
            'hashed' => $hashedCodes,    // Store in database
        ];
    }

    /**
     * Verify a backup code against stored hashed codes.
     */
    public function verifyBackupCode(string $code, array $hashedCodes): int|false
    {
        $code = strtoupper(trim($code));

        foreach ($hashedCodes as $index => $hashedCode) {
            if (Hash::check($code, $hashedCode)) {
                return $index; // Return index so it can be removed after use
            }
        }

        return false;
    }

    /**
     * Verify a TOTP code.
     * Simple implementation - in production, use a library like pragmarx/google2fa
     */
    public function verifyCode(string $secret, string $code): bool
    {
        // Simple time-based verification
        // In production, use: (new Google2FA())->verifyKey($secret, $code)
        
        $timeSlice = floor(time() / 30);
        
        // Check current time slice and ±1 for clock drift
        for ($i = -1; $i <= 1; $i++) {
            if ($this->generateCode($secret, $timeSlice + $i) === $code) {
                return true;
            }
        }
        
        return false;
    }

    /**
     * Generate a TOTP code for a given time slice.
     * Simplified implementation.
     */
    private function generateCode(string $secret, int $timeSlice): string
    {
        // This is a simplified version
        // In production, use a proper TOTP library
        $hash = hash_hmac('sha1', pack('N*', 0, $timeSlice), $this->base32Decode($secret), true);
        $offset = ord($hash[19]) & 0xf;
        $code = (
            ((ord($hash[$offset]) & 0x7f) << 24) |
            ((ord($hash[$offset + 1]) & 0xff) << 16) |
            ((ord($hash[$offset + 2]) & 0xff) << 8) |
            (ord($hash[$offset + 3]) & 0xff)
        ) % 1000000;
        
        return str_pad((string)$code, 6, '0', STR_PAD_LEFT);
    }

    /**
     * Decode base32 string.
     */
    private function base32Decode(string $secret): string
    {
        $base32chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
        $base32charsFlipped = array_flip(str_split($base32chars));
        
        $paddingCharCount = substr_count($secret, '=');
        $allowedValues = [6, 4, 3, 1, 0];
        
        if (!in_array($paddingCharCount, $allowedValues)) {
            return '';
        }
        
        for ($i = 0; $i < 4; $i++) {
            if ($paddingCharCount == $allowedValues[$i] &&
                substr($secret, -($allowedValues[$i])) != str_repeat('=', $allowedValues[$i])) {
                return '';
            }
        }
        
        $secret = str_replace('=', '', $secret);
        $secret = str_split($secret);
        $binaryString = '';
        
        for ($i = 0; $i < count($secret); $i = $i + 8) {
            $x = '';
            if (!in_array($secret[$i], $base32charsFlipped)) {
                return '';
            }
            
            for ($j = 0; $j < 8; $j++) {
                $x .= str_pad(base_convert(@$base32charsFlipped[@$secret[$i + $j]], 10, 2), 5, '0', STR_PAD_LEFT);
            }
            
            $eightBits = str_split($x, 8);
            
            for ($z = 0; $z < count($eightBits); $z++) {
                $binaryString .= (($y = chr(base_convert($eightBits[$z], 2, 10))) || ord($y) == 48) ? $y : '';
            }
        }
        
        return $binaryString;
    }
}
