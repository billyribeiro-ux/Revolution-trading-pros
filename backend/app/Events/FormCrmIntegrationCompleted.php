<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Contact;
use App\Models\FormSubmission;
use Illuminate\Broadcasting\InteractsWithSockets;
use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

class FormCrmIntegrationCompleted implements ShouldBroadcast
{
    use Dispatchable, InteractsWithSockets, SerializesModels;

    public function __construct(
        public readonly FormSubmission $submission,
        public readonly Contact $contact,
        public readonly array $result
    ) {}

    public function broadcastOn(): array
    {
        return [
            new PrivateChannel('contact.' . $this->contact->id),
            new PrivateChannel('form.' . $this->submission->form_id),
        ];
    }

    public function broadcastAs(): string
    {
        return 'crm.integration.completed';
    }

    public function broadcastWith(): array
    {
        return [
            'submission_id' => $this->submission->id,
            'contact_id' => $this->contact->id,
            'contact_created' => $this->result['contact_created'],
            'tags_applied' => $this->result['tags_applied'],
            'automations_triggered' => $this->result['automations_triggered'],
        ];
    }
}
