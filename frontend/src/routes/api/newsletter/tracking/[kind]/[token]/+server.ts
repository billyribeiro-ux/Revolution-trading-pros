import type { RequestHandler } from './$types';
import { campaignTracking } from '#lib/server/campaign-tracking.js';

export const GET: RequestHandler = campaignTracking;
export const HEAD: RequestHandler = campaignTracking;
