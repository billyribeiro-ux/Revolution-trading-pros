import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import Anthropic from '@anthropic-ai/sdk';
import { env } from '$env/dynamic/private';
import { logger } from '$lib/utils/logger';

function getAnthropicClient() {
	const apiKey = env.ANTHROPIC_API_KEY;
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

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body: GenerateRequest = await request.json();

		if (!body.prompt || typeof body.prompt !== 'string') {
			return json({ error: 'Prompt is required' }, { status: 400 });
		}

		const systemPrompt = `You are a helpful content generation assistant for a CMS block editor.
Generate content that is:
- Clear and well-structured
- Appropriate for the block type: ${body.blockType || 'paragraph'}
- Professional and engaging

${body.context ? `Context: ${body.context}` : ''}`;

		const anthropic = getAnthropicClient();
		const response = await anthropic.messages.create({
			model: 'claude-sonnet-4-20250514',
			max_tokens: body.maxTokens || 1024,
			system: systemPrompt,
			messages: [{ role: 'user', content: body.prompt }]
		});

		const content = response.content[0];
		if (content.type !== 'text') {
			return json({ error: 'Unexpected response type' }, { status: 500 });
		}

		return json({
			success: true,
			content: content.text,
			usage: {
				inputTokens: response.usage.input_tokens,
				outputTokens: response.usage.output_tokens
			}
		});
	} catch (err: unknown) {
		logger.error('[AI Generate] Error:', err);

		if (err instanceof Anthropic.APIError) {
			return json(
				{ error: 'AI service error', details: err.message },
				{ status: err.status || 500 }
			);
		}

		return json({ error: 'Internal server error' }, { status: 500 });
	}
};
