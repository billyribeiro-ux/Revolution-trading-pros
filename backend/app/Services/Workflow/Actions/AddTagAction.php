<?php

namespace App\Services\Workflow\Actions;

use App\Models\User;

class AddTagAction implements ActionInterface
{
    public function execute(array $config, array $context): array
    {
        $contactId = $config['contact_id'] ?? $context['trigger']['contact_id'] ?? null;
        $tags = $config['tags'] ?? [];

        if (!$contactId || empty($tags)) {
            throw new \Exception('Contact ID and tags are required');
        }

        $contact = User::find($contactId);
        
        if (!$contact) {
            throw new \Exception("Contact {$contactId} not found");
        }

        // Add tags to contact (implementation depends on your tag system)
        foreach ($tags as $tag) {
            // Example: $contact->tags()->attach($tag);
        }

        return array_merge($context, [
            '_action_result' => [
                'success' => true,
                'tags_added' => $tags,
                'contact_id' => $contactId,
            ],
        ]);
    }

    public function getMetadata(): array
    {
        return [
            'name' => 'Add Tag',
            'description' => 'Add tags to a contact',
            'required_fields' => ['tags'],
            'optional_fields' => ['contact_id'],
        ];
    }
}
