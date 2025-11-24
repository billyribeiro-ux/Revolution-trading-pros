/**
 * RevolutionBehavior-L8-System - Utility Functions
 */

export function generateSessionId(): string {
	return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

export function generateVisitorId(): string {
	const stored = localStorage.getItem('behavior_visitor_id');
	if (stored) return stored;

	const visitorId = `visitor_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
	localStorage.setItem('behavior_visitor_id', visitorId);
	return visitorId;
}

export function getDeviceType(): 'mobile' | 'tablet' | 'desktop' {
	const width = window.innerWidth;
	if (width < 768) return 'mobile';
	if (width < 1024) return 'tablet';
	return 'desktop';
}

export function getBrowser(): string {
	const ua = navigator.userAgent;
	if (ua.includes('Firefox')) return 'Firefox';
	if (ua.includes('Chrome')) return 'Chrome';
	if (ua.includes('Safari')) return 'Safari';
	if (ua.includes('Edge')) return 'Edge';
	return 'Unknown';
}

export function calculateEngagementScore(data: {
	scrollDepth: number;
	timeOnPage: number;
	interactionCount: number;
	contentConsumption: number;
}): number {
	const weights = {
		scrollDepth: 0.25,
		timeOnPage: 0.30,
		interaction: 0.30,
		content: 0.15
	};

	const scrollScore = data.scrollDepth;
	const timeScore = Math.min((data.timeOnPage / 120000) * 100, 100); // 2 min = 100%
	const interactionScore = Math.min(data.interactionCount * 10, 100);
	const contentScore = Math.min(data.contentConsumption * 20, 100);

	return (
		scrollScore * weights.scrollDepth +
		timeScore * weights.timeOnPage +
		interactionScore * weights.interaction +
		contentScore * weights.content
	);
}

export function calculateIntentScore(data: {
	ctaInteractions: number;
	hoverIntents: number;
	formEngagements: number;
	goalOrientedActions: number;
}): number {
	const weights = {
		cta: 0.40,
		hover: 0.25,
		form: 0.20,
		navigation: 0.15
	};

	const ctaScore = Math.min(data.ctaInteractions * 25, 100);
	const hoverScore = Math.min(data.hoverIntents * 20, 100);
	const formScore = Math.min(data.formEngagements * 15, 100);
	const navScore = Math.min(data.goalOrientedActions * 10, 100);

	return (
		ctaScore * weights.cta +
		hoverScore * weights.hover +
		formScore * weights.form +
		navScore * weights.navigation
	);
}

export function calculateFrictionScore(data: {
	rageClicks: number;
	formAbandonments: number;
	deadClicks: number;
	speedScrolls: number;
	backtracks: number;
	errors: number;
}): number {
	const penalties = {
		rageClick: 30,
		formAbandon: 25,
		deadClick: 15,
		speedScroll: 10,
		backtrack: 10,
		error: 10
	};

	const rageScore = Math.min(data.rageClicks * 10, 100) * (penalties.rageClick / 100);
	const abandonScore = Math.min(data.formAbandonments * 25, 100) * (penalties.formAbandon / 100);
	const deadScore = Math.min(data.deadClicks * 15, 100) * (penalties.deadClick / 100);
	const speedScore = Math.min(data.speedScrolls * 5, 100) * (penalties.speedScroll / 100);
	const backtrackScore = Math.min(data.backtracks * 3, 100) * (penalties.backtrack / 100);
	const errorScore = Math.min(data.errors * 5, 100) * (penalties.error / 100);

	return Math.min(
		rageScore + abandonScore + deadScore + speedScore + backtrackScore + errorScore,
		100
	);
}

export function calculateChurnRisk(data: {
	exitIntentDetected: boolean;
	engagementScore: number;
	frictionScore: number;
	abandonmentSignals: number;
	idleTimeRatio: number;
}): number {
	const weights = {
		exitIntent: 0.30,
		lowEngagement: 0.25,
		highFriction: 0.20,
		abandonment: 0.15,
		idle: 0.10
	};

	const exitScore = data.exitIntentDetected ? 100 : 0;
	const engagementScore = 100 - data.engagementScore;
	const frictionScore = data.frictionScore;
	const abandonmentScore = Math.min(data.abandonmentSignals * 20, 100);
	const idleScore = data.idleTimeRatio * 100;

	return (
		exitScore * weights.exitIntent +
		engagementScore * weights.lowEngagement +
		frictionScore * weights.highFriction +
		abandonmentScore * weights.abandonment +
		idleScore * weights.idle
	);
}
