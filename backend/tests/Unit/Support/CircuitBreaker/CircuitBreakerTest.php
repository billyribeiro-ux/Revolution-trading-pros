<?php

declare(strict_types=1);

namespace Tests\Unit\Support\CircuitBreaker;

use App\Support\CircuitBreaker\CircuitBreaker;
use App\Support\CircuitBreaker\CircuitBreakerOpenException;
use Exception;
use Illuminate\Support\Facades\Cache;
use Tests\TestCase;

/**
 * Unit tests for CircuitBreaker.
 *
 * @covers \App\Support\CircuitBreaker\CircuitBreaker
 */
class CircuitBreakerTest extends TestCase
{
    private CircuitBreaker $breaker;

    protected function setUp(): void
    {
        parent::setUp();
        Cache::flush();

        $this->breaker = new CircuitBreaker(
            cache: Cache::store(),
            name: 'test-circuit',
            failureThreshold: 3,
            successThreshold: 2,
            timeout: 10,
        );
    }

    /** @test */
    public function it_starts_in_closed_state(): void
    {
        $this->assertEquals(CircuitBreaker::STATE_CLOSED, $this->breaker->getState());
        $this->assertTrue($this->breaker->isAvailable());
    }

    /** @test */
    public function it_allows_calls_in_closed_state(): void
    {
        $result = $this->breaker->call(fn () => 'success');

        $this->assertEquals('success', $result);
        $this->assertEquals(CircuitBreaker::STATE_CLOSED, $this->breaker->getState());
    }

    /** @test */
    public function it_opens_after_failure_threshold(): void
    {
        // Fail 3 times (threshold)
        for ($i = 0; $i < 3; $i++) {
            try {
                $this->breaker->call(fn () => throw new Exception('fail'));
            } catch (Exception) {
                // Expected
            }
        }

        $this->assertEquals(CircuitBreaker::STATE_OPEN, $this->breaker->getState());
        $this->assertFalse($this->breaker->isAvailable());
    }

    /** @test */
    public function it_rejects_calls_when_open(): void
    {
        // Open the circuit
        $this->breaker->trip();

        $this->expectException(CircuitBreakerOpenException::class);

        $this->breaker->call(fn () => 'should not execute');
    }

    /** @test */
    public function it_uses_fallback_when_open(): void
    {
        $this->breaker->trip();

        $result = $this->breaker->call(
            fn () => 'should not execute',
            fn () => 'fallback value',
        );

        $this->assertEquals('fallback value', $result);
    }

    /** @test */
    public function it_resets_failure_count_on_success(): void
    {
        // Fail twice (below threshold)
        for ($i = 0; $i < 2; $i++) {
            try {
                $this->breaker->call(fn () => throw new Exception('fail'));
            } catch (Exception) {
                // Expected
            }
        }

        $this->assertEquals(2, $this->breaker->getFailureCount());

        // Success should reset
        $this->breaker->call(fn () => 'success');

        $this->assertEquals(0, $this->breaker->getFailureCount());
        $this->assertEquals(CircuitBreaker::STATE_CLOSED, $this->breaker->getState());
    }

    /** @test */
    public function it_transitions_to_half_open_after_timeout(): void
    {
        $this->breaker->trip();

        // Fast forward past timeout
        $this->travel(15)->seconds();

        $this->assertTrue($this->breaker->isAvailable());

        // Next call should transition to half-open
        try {
            $this->breaker->call(fn () => throw new Exception('still failing'));
        } catch (Exception) {
            // Expected
        }

        // Should be back to open after failure in half-open
        $this->assertEquals(CircuitBreaker::STATE_OPEN, $this->breaker->getState());
    }

    /** @test */
    public function it_closes_after_success_threshold_in_half_open(): void
    {
        $breaker = new CircuitBreaker(
            cache: Cache::store(),
            name: 'test-half-open',
            failureThreshold: 1,
            successThreshold: 2,
            timeout: 1,
        );

        // Open the circuit
        try {
            $breaker->call(fn () => throw new Exception('fail'));
        } catch (Exception) {
            // Expected
        }

        $this->assertEquals(CircuitBreaker::STATE_OPEN, $breaker->getState());

        // Wait for timeout
        $this->travel(2)->seconds();

        // Two successes should close it
        $breaker->call(fn () => 'success 1');
        $breaker->call(fn () => 'success 2');

        $this->assertEquals(CircuitBreaker::STATE_CLOSED, $breaker->getState());
    }

    /** @test */
    public function it_provides_retry_after_value(): void
    {
        $this->breaker->trip();

        $retryAfter = $this->breaker->getRetryAfter();

        $this->assertGreaterThan(0, $retryAfter);
        $this->assertLessThanOrEqual(10, $retryAfter);
    }

    /** @test */
    public function it_can_be_manually_reset(): void
    {
        $this->breaker->trip();
        $this->assertEquals(CircuitBreaker::STATE_OPEN, $this->breaker->getState());

        $this->breaker->reset();

        $this->assertEquals(CircuitBreaker::STATE_CLOSED, $this->breaker->getState());
        $this->assertEquals(0, $this->breaker->getFailureCount());
    }

    /** @test */
    public function it_provides_stats(): void
    {
        // Generate some activity
        $this->breaker->call(fn () => 'success');

        try {
            $this->breaker->call(fn () => throw new Exception('fail'));
        } catch (Exception) {
            // Expected
        }

        $stats = $this->breaker->getStats();

        $this->assertArrayHasKey('name', $stats);
        $this->assertArrayHasKey('state', $stats);
        $this->assertArrayHasKey('failures', $stats);
        $this->assertArrayHasKey('failure_threshold', $stats);
        $this->assertEquals('test-circuit', $stats['name']);
    }

    /** @test */
    public function exception_contains_circuit_info(): void
    {
        $this->breaker->trip();

        try {
            $this->breaker->call(fn () => 'fail');
            $this->fail('Expected CircuitBreakerOpenException');
        } catch (CircuitBreakerOpenException $e) {
            $this->assertEquals('test-circuit', $e->getCircuitName());
            $this->assertGreaterThan(0, $e->getRetryAfter());
        }
    }
}
