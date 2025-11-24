<?php

namespace App\Services\Workflow\Actions;

use Illuminate\Support\Facades\Http;

class HttpRequestAction implements ActionInterface
{
    public function execute(array $config, array $context): array
    {
        $url = $config['url'] ?? null;
        $method = strtoupper($config['method'] ?? 'GET');
        $headers = $config['headers'] ?? [];
        $body = $config['body'] ?? [];
        $timeout = $config['timeout'] ?? 30;

        if (!$url) {
            throw new \Exception('URL is required');
        }

        $http = Http::withHeaders($headers)->timeout($timeout);

        $response = match($method) {
            'GET' => $http->get($url, $body),
            'POST' => $http->post($url, $body),
            'PUT' => $http->put($url, $body),
            'PATCH' => $http->patch($url, $body),
            'DELETE' => $http->delete($url, $body),
            default => throw new \Exception("Unsupported HTTP method: {$method}"),
        };

        return array_merge($context, [
            '_action_result' => [
                'success' => $response->successful(),
                'status_code' => $response->status(),
                'headers' => $response->headers(),
                'body' => $response->json() ?? $response->body(),
            ],
        ]);
    }

    public function getMetadata(): array
    {
        return [
            'name' => 'HTTP Request',
            'description' => 'Make HTTP request to any URL',
            'required_fields' => ['url', 'method'],
            'optional_fields' => ['headers', 'body', 'timeout'],
        ];
    }
}
