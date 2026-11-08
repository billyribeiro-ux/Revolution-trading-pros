import type { EnhancedPopup } from './popups.js';

type PopupRow = Record<string, unknown>;

function record(value: unknown): PopupRow {
	if (typeof value !== 'object' || value === null || Array.isArray(value)) {
		throw new Error('Invalid popup response');
	}
	return value as PopupRow;
}

function string(value: unknown, fallback = ''): string {
	return typeof value === 'string' ? value : fallback;
}

function number(value: unknown, fallback = 0): number {
	return typeof value === 'number' && Number.isFinite(value) ? value : fallback;
}

function displayFrequency(value: unknown): EnhancedPopup['displayRules']['frequency'] {
	switch (value) {
		case 'always':
			return 'always';
		case 'daily':
			return 'once-per-day';
		case 'weekly':
			return 'once-per-week';
		case 'once':
			return 'once-ever';
		default:
			return 'once-per-session';
	}
}

function toLocalInput(value: unknown): string {
	if (typeof value !== 'string' || !value) return '';
	const instant = new Date(value);
	if (Number.isNaN(instant.getTime())) return '';
	const local = new Date(instant.getTime() - instant.getTimezoneOffset() * 60_000);
	return local.toISOString().slice(0, 16);
}

function toWireDate(value: string | undefined): string | undefined {
	if (!value?.trim()) return undefined;
	const instant = new Date(value);
	if (Number.isNaN(instant.getTime())) throw new Error('Invalid popup schedule date');
	return instant.toISOString();
}

export function normalizePopup(value: unknown): EnhancedPopup {
	const row = record(value);
	const rawId = row['id'];
	if ((typeof rawId !== 'number' && typeof rawId !== 'string') || !row['name']) {
		throw new Error('Invalid popup response');
	}
	const display =
		typeof row['display_rules'] === 'object' && row['display_rules'] !== null
			? record(row['display_rules'])
			: typeof row['displayRules'] === 'object' && row['displayRules'] !== null
				? record(row['displayRules'])
				: {};
	const frequency =
		typeof row['frequency_rules'] === 'object' && row['frequency_rules'] !== null
			? record(row['frequency_rules'])
			: {};
	const trigger =
		typeof row['trigger_rules'] === 'object' && row['trigger_rules'] !== null
			? record(row['trigger_rules'])
			: {};
	const design =
		typeof row['design'] === 'object' && row['design'] !== null ? record(row['design']) : {};
	const active = row['isActive'] === true || row['status'] === 'published';
	const devices = Array.isArray(display['devices']) ? display['devices'] : [];
	const deviceTargeting = devices.length === 1 ? string(devices[0], 'all') : 'all';

	return {
		id: String(rawId),
		name: string(row['name']),
		isActive: active,
		title: string(row['title']),
		content: string(row['content']),
		buttons: [],
		closeButton: row['show_close_button'] !== false,
		closeOnOverlayClick: row['close_on_overlay_click'] !== false,
		closeOnEscape: row['close_on_escape'] !== false,
		design: {
			...design,
			backgroundColor: string(design['background_color'], '#ffffff'),
			textColor: string(design['text_color'], '#1f2937'),
			titleColor: string(design['title_color'], '#1f2937'),
			buttonColor: string(design['button_color'], '#3b82f6'),
			buttonTextColor: string(design['button_text_color'], '#ffffff'),
			buttonBorderRadius: number(design['button_border_radius'], 8),
			overlayColor: string(design['overlay_color'], '#000000'),
			overlayOpacity: number(design['overlay_opacity'], 50)
		},
		animation: string(row['animation'], 'zoom'),
		overlayColor: string(design['overlay_color'], '#000000'),
		overlayOpacity: number(design['overlay_opacity'], 50),
		displayRules: {
			frequency: displayFrequency(frequency['frequency']),
			delaySeconds: number(trigger['delay']) / 1000,
			showOnScroll: row['type'] === 'scroll' || typeof trigger['scroll_depth'] === 'number',
			scrollPercentage: number(trigger['scroll_depth'], 50),
			showOnExit: row['type'] === 'exit_intent' || trigger['exit_intent'] === true,
			pages: Array.isArray(display['include_pages']) ? display['include_pages'] : [],
			excludePages: Array.isArray(display['exclude_pages']) ? display['exclude_pages'] : [],
			deviceTargeting: deviceTargeting as 'all' | 'desktop' | 'tablet' | 'mobile',
			newVisitorsOnly: display['new_visitors_only'] === true,
			returningVisitorsOnly: display['returning_visitors_only'] === true,
			minPageViews: number(display['min_page_views'])
		},
		impressions: number(row['impressions'] ?? row['total_views']),
		conversions: number(row['conversions'] ?? row['total_conversions']),
		createdAt: string(row['created_at']),
		updatedAt: string(row['updated_at']),
		priority: number(row['priority'], 10),
		trigger_rules: trigger,
		frequency_rules: frequency,
		display_rules: display,
		show_close_button: row['show_close_button'] !== false,
		close_on_overlay_click: row['close_on_overlay_click'] !== false,
		auto_close_after: number(row['auto_close_after']),
		cta_text: string(row['cta_text']),
		cta_url: string(row['cta_url']),
		cta_new_tab: row['cta_new_tab'] === true,
		has_form: row['has_form'] === true,
		form_id:
			typeof row['form_id'] === 'number' || typeof row['form_id'] === 'string'
				? String(row['form_id'])
				: undefined,
		type: string(row['type'], 'timed'),
		status: string(row['status'], 'draft'),
		position: string(row['position'], 'center'),
		size: string(row['size'], 'md'),
		performance_status: string(row['performance_status']),
		total_views: number(row['total_views']),
		total_conversions: number(row['total_conversions']),
		conversion_rate: number(row['conversion_rate']),
		start_date: toLocalInput(row['start_date']),
		end_date: toLocalInput(row['end_date'])
	};
}

export function popupWritePayload(popup: Partial<EnhancedPopup>): PopupRow {
	const design =
		typeof popup.design === 'object' && popup.design !== null ? (popup.design as PopupRow) : {};
	const payload: PopupRow = {
		name: popup.name,
		type: popup.type,
		status: popup.status,
		priority: popup.priority,
		title: popup.title,
		content: popup.content,
		cta_text: popup.cta_text,
		cta_url: popup.cta_url,
		cta_new_tab: popup.cta_new_tab,
		position: popup.position,
		size: popup.size,
		animation: typeof popup.animation === 'string' ? popup.animation : undefined,
		show_close_button: popup.show_close_button,
		close_on_overlay_click: popup.close_on_overlay_click,
		close_on_escape: popup.closeOnEscape,
		auto_close_after: popup.auto_close_after || null,
		has_form: popup.has_form,
		form_id: popup.form_id ? Number(popup.form_id) : undefined,
		trigger_rules: popup.trigger_rules,
		frequency_rules: popup.frequency_rules,
		display_rules: popup.display_rules,
		design:
			popup.design === undefined
				? undefined
				: {
						...design,
						background_color: design['backgroundColor'] ?? design['background_color'],
						title_color: design['titleColor'] ?? design['title_color'],
						text_color: design['textColor'] ?? design['text_color'],
						button_color: design['buttonColor'] ?? design['button_color'],
						button_text_color: design['buttonTextColor'] ?? design['button_text_color'],
						button_border_radius: design['buttonBorderRadius'] ?? design['button_border_radius'],
						button_shadow: design['buttonShadow'] ?? design['button_shadow'],
						button_padding: design['buttonPadding'] ?? design['button_padding'],
						overlay_color: design['overlayColor'] ?? design['overlay_color'],
						overlay_opacity: design['overlayOpacity'] ?? design['overlay_opacity'],
						overlay_blur: design['overlayBlur'] ?? design['overlay_blur']
					},
		start_date: toWireDate(popup.start_date),
		end_date: toWireDate(popup.end_date),
		isActive: popup.isActive
	};
	return Object.fromEntries(Object.entries(payload).filter(([, value]) => value !== undefined));
}
