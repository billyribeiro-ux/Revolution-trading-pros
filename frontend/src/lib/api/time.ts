import { API_ENDPOINTS, apiFetch } from './config';

export interface ServerTimeResponse {
	server_time: string;
	timezone: string;
}

/**
 * Fetch canonical server time for enterprise-grade countdown sync.
 */
export async function getServerTime(): Promise<ServerTimeResponse> {
	return apiFetch<ServerTimeResponse>(API_ENDPOINTS.time.now);
}
