<?php

namespace App\Services;

use Illuminate\Support\Facades\Config;
use Illuminate\Support\Facades\Mail;

class SmtpTestService
{
    /**
     * Test SMTP connection with given settings
     */
    public function testConnection(array $settings): array
    {
        try {
            // Temporarily configure mail settings
            Config::set('mail.mailers.smtp', [
                'transport' => 'smtp',
                'host' => $settings['host'],
                'port' => $settings['port'],
                'encryption' => $settings['encryption'] ?? 'tls',
                'username' => $settings['username'],
                'password' => $settings['password'],
                'timeout' => 10,
            ]);

            Config::set('mail.from', [
                'address' => $settings['from_address'],
                'name' => $settings['from_name'],
            ]);

            // Send test email
            Mail::raw('This is a test email from Revolution Trading Pros email system.', function ($message) use ($settings) {
                $message->to($settings['from_address'])
                    ->subject('SMTP Test Email - Revolution Trading Pros');
            });

            return [
                'success' => true,
                'message' => 'Test email sent successfully! Check your inbox.',
            ];
        } catch (\Exception $e) {
            return [
                'success' => false,
                'message' => 'SMTP connection failed: '.$e->getMessage(),
            ];
        }
    }
}
