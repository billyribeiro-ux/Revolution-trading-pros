export type {
	ContactRecord as Contact,
	ContactTimelineEvent as TimelineEvent,
	ContactEmailActivity as EmailActivity,
	ContactNote as Note
} from '#lib/crm/contact-types.js';
export interface TagOption {
	id: string;
	name: string;
	color?: string;
}
export interface ListOption {
	id: string;
	name: string;
}
export interface EmailTemplateOption {
	id: string;
	name: string;
	subject: string;
	body_html: string;
	body_text?: string;
}
