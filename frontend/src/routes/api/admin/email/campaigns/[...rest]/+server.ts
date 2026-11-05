import type { RequestHandler } from '@sveltejs/kit';
import { proxyEmailCampaigns } from '#lib/server/email-campaign-proxy.js';
export const GET: RequestHandler = proxyEmailCampaigns;
export const POST: RequestHandler = proxyEmailCampaigns;
export const PUT: RequestHandler = proxyEmailCampaigns;
export const DELETE: RequestHandler = proxyEmailCampaigns;
