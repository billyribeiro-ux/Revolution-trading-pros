<?php

namespace App\Services\Workflow\Actions;

use Illuminate\Support\Facades\Http;

class SendWebhookAction implements ActionInterface
{
    public function execute(array $config, array $context): array
    {
        $url = $config['url'] ?? null;
        $method = $config['method'] ?? 'POST';
        $headers = $config['headers'] ?? [];
        $body = $config['body'] ?? $context;

        if (!$url) {
            throw new \Exception('Webhook URL is required');
        }

        $response = Http::withHeaders($headers)
            ->send($method, $url, ['json' => $body]);

        return array_merge($context, [
            '_action_result' => [
                'success' => $response->successful(),
                'status_code' => $response->status(),
                'response' => $response->json(),
            ],
        ]);
    }

    public function getMetadata(): array
    {
        return [
            'name' => 'Send Webhook',
            'description' => 'Send HTTP webhook to external service',
            'required_fields' => ['url'],
            'optional_fields' => ['method', 'headers', 'body'],
        ];
    }
}
