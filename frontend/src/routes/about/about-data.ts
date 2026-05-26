/**
 * Shared static data for the About page.
 * Extracted so both +page.svelte and +page.ts (SEO loader) can consume.
 */

export interface AboutFAQ {
	q: string;
	a: string;
}

export const aboutFaqs: AboutFAQ[] = [
	{
		q: 'Is this suitable for beginners?',
		a: "Yes, but be prepared to work. We don't sell 'get rich quick' schemes. We provide a Foundation Course that brings you up to speed on our terminology and tools before you jump into the live room."
	},
	{
		q: 'Do you just give buy/sell alerts?',
		a: 'No. We are an educational community. While we call out our own live trades, the goal is for you to understand the *rationale* so you can eventually become self-sufficient. Blindly following alerts is a recipe for failure.'
	},
	{
		q: 'What makes this different from other Discords?',
		a: "Professionalism and care. We don't tolerate hype, rocket emojis, or pumping low-float stocks. This is a serious workspace for people who treat trading as a business, run by mentors who actually care if you succeed."
	},
	{
		q: 'Do I need a large account to start?',
		a: 'No. We teach risk management based on percentages, not dollar amounts. Many members start with small accounts or funded trader evaluations while they learn the system.'
	}
];
