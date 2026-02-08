/**
 * Axum Server Adapter — Barrel Export
 *
 * Re-exports all domain adapters for convenient imports.
 * All modules are server-only ($lib/server/).
 *
 * @version 1.0.0
 */

export { axum, axumFetch, AxumError, type AxumRequestOptions } from './client';
export * as axumAlerts from './alerts';
export * as axumTrades from './trades';
export * as axumTradePlans from './trade-plans';
export * as axumStats from './stats';
export * as axumVideos from './videos';
export * as axumAuth from './auth';
