<?php

namespace App\Services\Workflow\Actions;

class CreateDealAction implements ActionInterface
{
    public function execute(array $config, array $context): array
    {
        $title = $config['title'] ?? 'New Deal';
        $value = $config['value'] ?? 0;
        $contactId = $config['contact_id'] ?? $context['trigger']['contact_id'] ?? null;
        $stage = $config['stage'] ?? 'new';

        // Create deal (implementation depends on your CRM system)
        $dealId = rand(1000, 9999); // Placeholder

        return array_merge($context, [
            '_action_result' => [
                'success' => true,
                'deal_id' => $dealId,
                'title' => $title,
                'value' => $value,
            ],
        ]);
    }

    public function getMetadata(): array
    {
        return [
            'name' => 'Create Deal',
            'description' => 'Create a new CRM deal',
            'required_fields' => ['title', 'value'],
            'optional_fields' => ['contact_id', 'stage'],
        ];
    }
}
