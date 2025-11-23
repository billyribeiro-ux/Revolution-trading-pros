<?php

namespace App\Services;

use Illuminate\Support\Str;

class MFAService
{
    /**
     * Generate a random TOTP secret.
     */
    public function generateSecret(): string
    {
        return strtoupper(Str::random(32));
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
     */
    public function generateBackupCodes(int $count = 8): array
    {
        $codes = [];
        for ($i = 0; $i < $count; $i++) {
            $codes[] = strtoupper(Str::random(4) . '-' . Str::random(4));
        }
        return $codes;
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
