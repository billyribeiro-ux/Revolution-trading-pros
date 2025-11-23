<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailSetting;
use App\Services\SmtpTestService;
use Illuminate\Http\Request;

class EmailSettingsController extends Controller
{
    public function __construct(
        protected SmtpTestService $smtpTestService
    ) {}

    /**
     * Get current SMTP settings
     */
    public function index()
    {
        $settings = EmailSetting::where('is_active', true)->first();

        return response()->json($settings ?? [
            'provider' => 'smtp',
            'host' => '',
            'port' => 587,
            'username' => '',
            'encryption' => 'tls',
            'from_address' => '',
            'from_name' => 'Revolution Trading Pros',
            'is_active' => false,
        ]);
    }

    /**
     * Update SMTP settings
     */
    public function update(Request $request)
    {
        $validated = $request->validate([
            'provider' => 'required|string',
            'host' => 'required|string',
            'port' => 'required|integer',
            'username' => 'nullable|string',
            'password' => 'nullable|string',
            'encryption' => 'required|in:tls,ssl,null',
            'from_address' => 'required|email',
            'from_name' => 'required|string',
        ]);

        // Deactivate all existing settings
        EmailSetting::query()->update(['is_active' => false]);

        // Create or update settings
        $settings = EmailSetting::updateOrCreate(
            ['id' => $request->id],
            array_merge($validated, ['is_active' => true])
        );

        return response()->json([
            'message' => 'SMTP settings saved successfully',
            'settings' => $settings,
        ]);
    }

    /**
     * Test SMTP connection
     */
    public function test(Request $request)
    {
        $validated = $request->validate([
            'host' => 'required|string',
            'port' => 'required|integer',
            'username' => 'nullable|string',
            'password' => 'nullable|string',
            'encryption' => 'required|in:tls,ssl,null',
            'from_address' => 'required|email',
            'from_name' => 'required|string',
        ]);

        $result = $this->smtpTestService->testConnection($validated);

        return response()->json($result, $result['success'] ? 200 : 500);
    }
}
