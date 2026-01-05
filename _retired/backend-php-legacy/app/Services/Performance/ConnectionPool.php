<?php

declare(strict_types=1);

namespace App\Services\Performance;

use Illuminate\Support\Facades\Http;
use Illuminate\Http\Client\PendingRequest;
use Illuminate\Http\Client\Pool;
use Illuminate\Support\Facades\Cache;
use GuzzleHttp\Client;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use GuzzleHttp\Psr7\Response;

/**
 * Connection Pool Service (ICT9+ Enterprise Grade)
 *
 * Manages HTTP connection pooling for external services:
 * - Persistent connections with keep-alive
 * - Connection reuse
 * - Concurrent request handling
 * - Circuit breaker per service
 * - Request retry with backoff
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class ConnectionPool
{
    /**
     * Guzzle clients per service
     */
    private static array $clients = [];

    /**
     * Circuit breaker states
     */
    private static array $circuitBreakers = [];

    /**
     * Configuration
     */
    private const DEFAULT_TIMEOUT = 30;
    private const DEFAULT_CONNECT_TIMEOUT = 5;
    private const MAX_CONNECTIONS = 50;
    private const CIRCUIT_FAILURE_THRESHOLD = 5;
    private const CIRCUIT_RESET_TIMEOUT = 60;

    /**
     * Get or create a pooled HTTP client for a service
     */
    public function getClient(string $service, array $options = []): Client
    {
        if (!isset(self::$clients[$service])) {
            self::$clients[$service] = $this->createClient($service, $options);
        }

        return self::$clients[$service];
    }

    /**
     * Create optimized Guzzle client
     */
    private function createClient(string $service, array $options): Client
    {
        $stack = HandlerStack::create();

        // Add retry middleware
        $stack->push(Middleware::retry(
            function ($retries, $request, $response, $exception) {
                // Retry on connection errors or 5xx responses
                if ($retries >= 3) {
                    return false;
                }

                if ($exception instanceof \GuzzleHttp\Exception\ConnectException) {
                    return true;
                }

                if ($response && $response->getStatusCode() >= 500) {
                    return true;
                }

                return false;
            },
            function ($retries) {
                // Exponential backoff: 100ms, 200ms, 400ms
                return 100 * pow(2, $retries);
            }
        ));

        $config = array_merge([
            'handler' => $stack,
            'timeout' => self::DEFAULT_TIMEOUT,
            'connect_timeout' => self::DEFAULT_CONNECT_TIMEOUT,
            'http_errors' => false,
            'headers' => [
                'Connection' => 'keep-alive',
                'Accept-Encoding' => 'gzip, deflate, br',
            ],
            'curl' => [
                CURLOPT_TCP_KEEPALIVE => 1,
                CURLOPT_TCP_KEEPIDLE => 60,
                CURLOPT_TCP_KEEPINTVL => 30,
            ],
        ], $options);

        return new Client($config);
    }

    /**
     * Make a request with circuit breaker protection
     */
    public function request(
        string $service,
        string $method,
        string $url,
        array $options = []
    ): ?Response {
        // Check circuit breaker
        if ($this->isCircuitOpen($service)) {
            throw new \RuntimeException("Circuit breaker open for service: {$service}");
        }

        try {
            $client = $this->getClient($service);
            $response = $client->request($method, $url, $options);

            // Reset failure count on success
            $this->recordSuccess($service);

            return $response;
        } catch (\Throwable $e) {
            $this->recordFailure($service, $e);
            throw $e;
        }
    }

    /**
     * Execute concurrent requests
     */
    public function concurrent(array $requests): array
    {
        return Http::pool(function (Pool $pool) use ($requests) {
            $poolRequests = [];

            foreach ($requests as $key => $request) {
                $method = strtolower($request['method'] ?? 'get');
                $url = $request['url'];
                $options = $request['options'] ?? [];

                $poolRequests[$key] = $pool->withOptions([
                    'timeout' => $options['timeout'] ?? self::DEFAULT_TIMEOUT,
                    'connect_timeout' => self::DEFAULT_CONNECT_TIMEOUT,
                ])->{$method}($url, $options['data'] ?? []);
            }

            return $poolRequests;
        });
    }

    /**
     * Check if circuit breaker is open for service
     */
    private function isCircuitOpen(string $service): bool
    {
        $state = self::$circuitBreakers[$service] ?? null;

        if (!$state) {
            return false;
        }

        if ($state['open'] && (time() - $state['opened_at']) > self::CIRCUIT_RESET_TIMEOUT) {
            // Half-open: allow one request to test
            self::$circuitBreakers[$service]['open'] = false;
            return false;
        }

        return $state['open'] ?? false;
    }

    /**
     * Record successful request
     */
    private function recordSuccess(string $service): void
    {
        self::$circuitBreakers[$service] = [
            'failures' => 0,
            'open' => false,
            'opened_at' => null,
        ];
    }

    /**
     * Record failed request
     */
    private function recordFailure(string $service, \Throwable $e): void
    {
        if (!isset(self::$circuitBreakers[$service])) {
            self::$circuitBreakers[$service] = ['failures' => 0, 'open' => false];
        }

        self::$circuitBreakers[$service]['failures']++;

        if (self::$circuitBreakers[$service]['failures'] >= self::CIRCUIT_FAILURE_THRESHOLD) {
            self::$circuitBreakers[$service]['open'] = true;
            self::$circuitBreakers[$service]['opened_at'] = time();

            \Log::error("Circuit breaker opened for {$service}", [
                'failures' => self::$circuitBreakers[$service]['failures'],
                'last_error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Get connection pool statistics
     */
    public function getStats(): array
    {
        return [
            'active_clients' => count(self::$clients),
            'circuit_breakers' => self::$circuitBreakers,
        ];
    }

    /**
     * Close all connections
     */
    public function closeAll(): void
    {
        self::$clients = [];
    }
}
