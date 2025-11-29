/**
 * OpenTelemetry Instrumentation for SvelteKit
 * November 2025 - Performance monitoring and distributed tracing
 *
 * This file is automatically loaded by SvelteKit for server-side instrumentation.
 * It provides observability into request handling, database queries, and external calls.
 *
 * Features:
 * - Request tracing with span context
 * - Performance metrics collection
 * - Error tracking and reporting
 * - Custom span attributes for SEO/Core Web Vitals correlation
 *
 * @version 1.0.0 - November 2025
 */

import { trace, SpanStatusCode, type Span, type Tracer } from '@opentelemetry/api';

// Tracer for Revolution Trading Pros
const tracer: Tracer = trace.getTracer('revolution-trading-pros', '2.0.0');

/**
 * Track a server-side operation with OpenTelemetry
 */
export function trackOperation<T>(
	name: string,
	operation: (span: Span) => Promise<T>,
	attributes?: Record<string, string | number | boolean>
): Promise<T> {
	return tracer.startActiveSpan(name, async (span) => {
		try {
			// Add custom attributes
			if (attributes) {
				Object.entries(attributes).forEach(([key, value]) => {
					span.setAttribute(key, value);
				});
			}

			const result = await operation(span);

			span.setStatus({ code: SpanStatusCode.OK });
			return result;
		} catch (error) {
			span.setStatus({
				code: SpanStatusCode.ERROR,
				message: error instanceof Error ? error.message : 'Unknown error'
			});
			span.recordException(error as Error);
			throw error;
		} finally {
			span.end();
		}
	});
}

/**
 * Track page render performance
 */
export function trackPageRender(
	route: string,
	renderTime: number,
	statusCode: number
): void {
	const span = tracer.startSpan('page.render');

	span.setAttributes({
		'http.route': route,
		'http.status_code': statusCode,
		'page.render_time_ms': renderTime,
		'service.name': 'revolution-trading-pros',
		'deployment.environment': process.env.NODE_ENV || 'development'
	});

	span.setStatus({ code: SpanStatusCode.OK });
	span.end();
}

/**
 * Track API call performance
 */
export function trackApiCall(
	endpoint: string,
	method: string,
	duration: number,
	statusCode: number
): void {
	const span = tracer.startSpan('api.call');

	span.setAttributes({
		'http.method': method,
		'http.url': endpoint,
		'http.status_code': statusCode,
		'http.duration_ms': duration,
		'service.name': 'revolution-trading-pros'
	});

	span.setStatus({
		code: statusCode >= 400 ? SpanStatusCode.ERROR : SpanStatusCode.OK
	});
	span.end();
}

/**
 * Track Core Web Vitals metrics (server-side collection)
 */
export function trackCoreWebVitals(metrics: {
	lcp?: number;
	inp?: number;
	cls?: number;
	fcp?: number;
	ttfb?: number;
	route: string;
}): void {
	const span = tracer.startSpan('core.web.vitals');

	span.setAttributes({
		'cwv.route': metrics.route,
		...(metrics.lcp !== undefined && { 'cwv.lcp_ms': metrics.lcp }),
		...(metrics.inp !== undefined && { 'cwv.inp_ms': metrics.inp }),
		...(metrics.cls !== undefined && { 'cwv.cls': metrics.cls }),
		...(metrics.fcp !== undefined && { 'cwv.fcp_ms': metrics.fcp }),
		...(metrics.ttfb !== undefined && { 'cwv.ttfb_ms': metrics.ttfb }),
		'service.name': 'revolution-trading-pros'
	});

	span.setStatus({ code: SpanStatusCode.OK });
	span.end();
}

/**
 * Track SEO-relevant events for correlation
 */
export function trackSeoEvent(event: {
	type: 'crawl' | 'index' | 'render';
	route: string;
	userAgent?: string;
	statusCode: number;
}): void {
	const span = tracer.startSpan('seo.event');

	span.setAttributes({
		'seo.event_type': event.type,
		'seo.route': event.route,
		'seo.status_code': event.statusCode,
		...(event.userAgent && { 'seo.user_agent': event.userAgent }),
		'service.name': 'revolution-trading-pros'
	});

	span.setStatus({ code: SpanStatusCode.OK });
	span.end();
}

/**
 * Initialize OpenTelemetry (called automatically by SvelteKit)
 */
export async function init(): Promise<void> {
	// OpenTelemetry is initialized via environment configuration
	// This function can be extended to set up custom exporters

	console.log('[OpenTelemetry] Instrumentation initialized for Revolution Trading Pros');

	// In production, you would configure exporters here:
	// - OTLP exporter for backends like Jaeger, Zipkin, Honeycomb, etc.
	// - Console exporter for development
	// - Custom exporters for specific monitoring services
}
