<?php

declare(strict_types=1);

namespace App\Broadcasting;

use App\Models\User;
use Illuminate\Http\Request;

/**
 * SessionChannel - Authorization for session-based channels (guests)
 *
 * Handles authorization for channels like:
 * - session.{sessionId}.cart (guest cart updates)
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class SessionChannel
{
    /**
     * Create a new channel instance.
     */
    public function __construct(
        protected Request $request
    ) {}

    /**
     * Authenticate the user's access to the channel.
     */
    public function join(?User $user, string $sessionId): bool|array
    {
        // Get session ID from request (via socket connection or header)
        $requestSessionId = $this->request->header('X-Session-ID')
            ?? $this->request->session()->getId()
            ?? $this->request->cookie('session_id');

        // Match session ID
        if ($requestSessionId === $sessionId) {
            return [
                'session_id' => $sessionId,
                'user_id' => $user?->id,
            ];
        }

        return false;
    }
}
