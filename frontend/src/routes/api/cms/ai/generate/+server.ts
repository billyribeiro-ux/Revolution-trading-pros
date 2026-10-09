import type { RequestHandler } from './$types';
import Anthropic from '@anthropic-ai/sdk';
import { ANTHROPIC_API_KEY } from '$app/env/private';
import { isAdminRole } from '#lib/server/auth.js';

/**
 * Model for CMS content generation.
 *
 * `claude-sonnet-4-20250514` (used until 2026-10-09) was retired by Anthropic on
 * 2026-06-15 and returns 404, so this endpoint was failing on every call. Its
 * documented drop-in replacement is `claude-sonnet-5-5` — same Sonnet tier the
 * route was built on.
 */
const MODEL = 'claude-sonnet-5-5';

/**
 * Claude Sonnet 5.5 thinks adaptively by default and thinking tokens count
 * toward `max_tokens`. `low` effort is Anthropic's documented starting point
 * for content generation and skips thinking on most simple requests; the
 * headroom keeps any thinking it does from truncating the requested reply.
 */
const EFFORT = 'low';
const MAX_REPLY_TOKENS = 4096;
const THINKING_HEADROOM_TOKENS = 4096;

function getAnthropicClient() {
	const apiKey = ANTHROPIC_API_KEY;
	if (!apiKey) {
		throw new Error('ANTHROPIC_API_KEY is not configured');
	}
	return new Anthropic({ apiKey });
}

interface GenerateRequest {
	prompt: string;
	context?: string;
	blockType?: string;
	maxTokens?: number;
}

export const POST: RequestHandler = async ({ request, locals }) => {
	try {
		// This endpoint calls a paid third-party LLM (Anthropic). Gate it behind
		// an authenticated, privileged CMS session. `locals.user` is populated by
		// hooks.server.ts:authHandler for any request carrying a valid token
		// (this route is not in PROTECTED_ROUTES, but the hook still hydrates the
		// user when a token is present).
		const user = locals.user;
		if (!user) {
			return Response.json({ error: 'Unauthorized' }, { status: 401 });
		}
		if (!isAdminRole(user.role) && (user.role ?? '').toLowerCase() !== 'editor') {
			return Response.json({ error: 'Forbidden' }, { status: 403 });
		}

		const body: GenerateRequest = await request.json();

		if (!body.prompt || typeof body.prompt !== 'string') {
			return Response.json({ error: 'Prompt is required' }, { status: 400 });
		}

		const systemPrompt = `You are a helpful content generation assistant for a CMS block editor.
Generate content that is:
- Clear and well-structured
- Appropriate for the block type: ${body.blockType || 'paragraph'}
- Professional and engaging

${body.context ? `Context: ${body.context}` : ''}`;

		const replyTokens = Math.min(body.maxTokens ?? 1024, MAX_REPLY_TOKENS);

		const anthropic = getAnthropicClient();
		// Server-side refusal fallback (`fallbacks: 'default'`) routes a declined
		// request by refusal category without maintaining a model list. It is a
		// beta on the first-party Claude API, which is what this client targets.
		const response = await anthropic.beta.messages.create({
			model: MODEL,
			max_tokens: replyTokens + THINKING_HEADROOM_TOKENS,
			output_config: { effort: EFFORT },
			betas: ['server-side-fallback-2026-07-01'],
			fallbacks: 'default',
			system: systemPrompt,
			messages: [{ role: 'user', content: body.prompt }]
		});

		// A decline is an HTTP 200 with `stop_reason: 'refusal'`; check it before
		// reading content.
		if (response.stop_reason === 'refusal') {
			return Response.json(
				{
					error: 'The AI model declined this request',
					category: response.stop_details?.category ?? null
				},
				{ status: 422 }
			);
		}

		// Read by block type, never by position: a response can lead with a
		// (possibly empty) `thinking` block, or carry `fallback` blocks.
		const text = response.content
			.filter((block): block is Anthropic.Beta.BetaTextBlock => block.type === 'text')
			.map((block) => block.text)
			.join('');

		if (!text) {
			return Response.json({ error: 'The AI model returned no text' }, { status: 502 });
		}

		return Response.json({
			success: true,
			content: text,
			truncated: response.stop_reason === 'max_tokens',
			usage: {
				inputTokens: response.usage.input_tokens,
				outputTokens: response.usage.output_tokens
			}
		});
	} catch (error) {
		console.error('[AI Generate] Error:', error);

		// Most specific first. Upstream failures are reported as 5xx/429 so a
		// misconfigured server key (Anthropic 401/403) is never surfaced to the
		// editor as if *they* were unauthenticated.
		if (error instanceof Anthropic.RateLimitError) {
			return Response.json(
				{ error: 'AI service is busy, please retry shortly', details: error.message },
				{ status: 429 }
			);
		}
		if (error instanceof Anthropic.APIConnectionError) {
			return Response.json(
				{ error: 'AI service unreachable', details: error.message },
				{ status: 503 }
			);
		}
		if (error instanceof Anthropic.APIError) {
			return Response.json({ error: 'AI service error', details: error.message }, { status: 502 });
		}

		return Response.json({ error: 'Internal server error' }, { status: 500 });
	}
};
