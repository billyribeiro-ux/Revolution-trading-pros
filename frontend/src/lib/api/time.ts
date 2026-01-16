import { apiClient } from './client.svelte';
import { API_ENDPOINTS } from './config';

export interface ServerTimeResponse {
	server_time: string;
	timezone: string;
}

/**
 * Fetch canonical server time for enterprise-grade countdown sync.
 */
export async function getServerTime(): Promise<ServerTimeResponse> {
	return apiClient.get<ServerTimeResponse>(API_ENDPOINTS.time.now);
}
