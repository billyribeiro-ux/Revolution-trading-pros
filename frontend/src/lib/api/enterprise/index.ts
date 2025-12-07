/**
 * Enterprise API Infrastructure - Apple ICT9+ Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * This module provides a unified, production-hardened API infrastructure with:
 *
 * 1. CORRELATION TRACING: Every request gets a unique trace ID that flows through
 *    the entire system for debugging and monitoring
 *
 * 2. CSRF PROTECTION: Double-submit cookie pattern with automatic token management
 *
 * 3. AUTOMATIC TOKEN REFRESH: Transparent 401 handling with queue-based retry
 *
 * 4. REQUEST INTERCEPTORS: Pre/post processing hooks for cross-cutting concerns
 *
 * 5. ERROR NORMALIZATION: Consistent error shape across all API boundaries
 *
 * 6. SECURITY HARDENING: XSS prevention, timing attack mitigation, audit logging
 *
 * @version 1.0.0
 * @license MIT
 * @author Revolution Trading Pros - Enterprise Architecture
 */

export * from './types';
export * from './interceptor';
export * from './csrf';
export * from './tracing';
export * from './errors';
export * from './client';
