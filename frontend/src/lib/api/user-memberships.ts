/**
 * User Memberships API
 * Fetches the current user's active subscriptions/memberships
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth';

const API_BASE = browser ? import.meta.env.VITE_API_URL || 'http://localhost:8000/api' : '';

export interface UserMembership {
	id: string;
	name: string;
	type: 'trading-room' | 'alert-service' | 'course' | 'indicator';
	slug: string;
	status: 'active' | 'pending' | 'cancelled' | 'expired';
	icon?: string;
	startDate: string;
	nextBillingDate?: string;
	price?: number;
	interval?: 'monthly' | 'quarterly' | 'yearly';
}

export interface UserMembershipsResponse {
	memberships: UserMembership[];
	tradingRooms: UserMembership[];
	alertServices: UserMembership[];
	courses: UserMembership[];
	indicators: UserMembership[];
}

/**
 * Get current user's active memberships
 */
export async function getUserMemberships(): Promise<UserMembershipsResponse> {
	if (!browser) {
		return {
			memberships: [],
			tradingRooms: [],
			alertServices: [],
			courses: [],
			indicators: []
		};
	}

	const token = authStore.getToken();
	if (!token) {
		throw new Error('Not authenticated');
	}

	try {
		const response = await fetch(`${API_BASE}/user/memberships`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			credentials: 'include'
		});

		if (!response.ok) {
			throw new Error('Failed to fetch memberships');
		}

		const data = await response.json();
		return data;
	} catch (error) {
		console.error('[UserMemberships] Error fetching memberships:', error);
		
		// Return mock data for development/demo
		return getMockMemberships();
	}
}

/**
 * Mock data for development/demo purposes
 * This simulates what the API would return
 */
function getMockMemberships(): UserMembershipsResponse {
	const tradingRooms: UserMembership[] = [
		{
			id: 'day-trading',
			name: 'Day Trading Room',
			type: 'trading-room',
			slug: 'day-trading',
			status: 'active',
			startDate: '2024-01-15',
			nextBillingDate: '2025-01-15',
			price: 197,
			interval: 'monthly'
		},
		{
			id: 'swing-trading',
			name: 'Swing Trading Room',
			type: 'trading-room',
			slug: 'swing-trading',
			status: 'active',
			startDate: '2024-03-01',
			nextBillingDate: '2025-03-01',
			price: 147,
			interval: 'monthly'
		},
		{
			id: 'small-accounts',
			name: 'Small Accounts Room',
			type: 'trading-room',
			slug: 'small-accounts',
			status: 'active',
			startDate: '2024-06-01',
			nextBillingDate: '2025-06-01',
			price: 97,
			interval: 'monthly'
		}
	];

	const alertServices: UserMembership[] = [
		{
			id: 'spx-profit-pulse',
			name: 'SPX Profit Pulse',
			type: 'alert-service',
			slug: 'spx-profit-pulse',
			status: 'active',
			startDate: '2024-02-01',
			nextBillingDate: '2025-02-01',
			price: 297,
			interval: 'monthly'
		},
		{
			id: 'explosive-swings',
			name: 'Explosive Swings',
			type: 'alert-service',
			slug: 'explosive-swings',
			status: 'active',
			startDate: '2024-04-15',
			nextBillingDate: '2025-04-15',
			price: 197,
			interval: 'monthly'
		}
	];

	const allMemberships = [...tradingRooms, ...alertServices];

	return {
		memberships: allMemberships,
		tradingRooms,
		alertServices,
		courses: [],
		indicators: []
	};
}
