/**
 * FluentForm Pro Integrations Configuration
 *
 * Complete list of all 60+ integrations, field types, and features
 * from FluentForm Pro v6.1.8
 *
 * @version 2.0.0
 */

// ============================================================================
// INTEGRATION TYPES
// ============================================================================

export type IntegrationType =
	| 'email_marketing'
	| 'crm'
	| 'messaging'
	| 'automation'
	| 'storage'
	| 'payment'
	| 'user'
	| 'notification';

export interface Integration {
	id: string;
	name: string;
	type: IntegrationType;
	icon: string;
	description: string;
	docsUrl?: string;
	requiredFields: string[];
	features: string[];
	premium?: boolean;
}

// ============================================================================
// EMAIL MARKETING INTEGRATIONS
// ============================================================================

export const EMAIL_MARKETING_INTEGRATIONS: Integration[] = [
	{
		id: 'activecampaign',
		name: 'ActiveCampaign',
		type: 'email_marketing',
		icon: 'activecampaign',
		description: 'Powerful email marketing, automation, and CRM platform',
		requiredFields: ['api_url', 'api_key', 'list_id'],
		features: ['contacts', 'tags', 'automations', 'deals']
	},
	{
		id: 'mailchimp',
		name: 'Mailchimp',
		type: 'email_marketing',
		icon: 'mailchimp',
		description: 'All-in-one marketing platform for growing businesses',
		requiredFields: ['api_key', 'list_id'],
		features: ['contacts', 'tags', 'groups', 'double_optin']
	},
	{
		id: 'convertkit',
		name: 'ConvertKit',
		type: 'email_marketing',
		icon: 'convertkit',
		description: 'Email marketing for creators',
		requiredFields: ['api_key', 'api_secret', 'form_id'],
		features: ['subscribers', 'tags', 'sequences', 'forms']
	},
	{
		id: 'mailerlite',
		name: 'MailerLite',
		type: 'email_marketing',
		icon: 'mailerlite',
		description: 'Email marketing software with landing pages',
		requiredFields: ['api_key', 'group_id'],
		features: ['subscribers', 'groups', 'segments', 'automations']
	},
	{
		id: 'getresponse',
		name: 'GetResponse',
		type: 'email_marketing',
		icon: 'getresponse',
		description: 'Marketing software with email, automation, and webinars',
		requiredFields: ['api_key', 'list_id'],
		features: ['contacts', 'tags', 'automations', 'webinars']
	},
	{
		id: 'drip',
		name: 'Drip',
		type: 'email_marketing',
		icon: 'drip',
		description: 'Ecommerce email marketing automation',
		requiredFields: ['api_key', 'account_id'],
		features: ['subscribers', 'tags', 'workflows', 'events']
	},
	{
		id: 'sendinblue',
		name: 'Brevo (Sendinblue)',
		type: 'email_marketing',
		icon: 'sendinblue',
		description: 'All-in-one marketing platform',
		requiredFields: ['api_key', 'list_id'],
		features: ['contacts', 'lists', 'automations', 'sms']
	},
	{
		id: 'campaignmonitor',
		name: 'Campaign Monitor',
		type: 'email_marketing',
		icon: 'campaignmonitor',
		description: 'Email marketing and automation',
		requiredFields: ['api_key', 'list_id'],
		features: ['subscribers', 'segments', 'journeys']
	},
	{
		id: 'constantcontact',
		name: 'Constant Contact',
		type: 'email_marketing',
		icon: 'constantcontact',
		description: 'Email marketing made simple',
		requiredFields: ['api_key', 'access_token', 'list_id'],
		features: ['contacts', 'lists', 'events']
	},
	{
		id: 'moosend',
		name: 'Moosend',
		type: 'email_marketing',
		icon: 'moosend',
		description: 'Affordable email marketing automation',
		requiredFields: ['api_key', 'list_id'],
		features: ['subscribers', 'segments', 'automations']
	},
	{
		id: 'sendfox',
		name: 'SendFox',
		type: 'email_marketing',
		icon: 'sendfox',
		description: 'Simple email marketing for creators',
		requiredFields: ['api_key', 'list_id'],
		features: ['contacts', 'lists', 'automations']
	},
	{
		id: 'automizy',
		name: 'Automizy',
		type: 'email_marketing',
		icon: 'automizy',
		description: 'AI-powered email marketing',
		requiredFields: ['api_token', 'list_id'],
		features: ['contacts', 'segments', 'automations', 'ai_suggestions']
	},
	{
		id: 'cleverreach',
		name: 'CleverReach',
		type: 'email_marketing',
		icon: 'cleverreach',
		description: 'European email marketing solution',
		requiredFields: ['api_key', 'list_id'],
		features: ['receivers', 'groups', 'automations']
	},
	{
		id: 'mailjet',
		name: 'Mailjet',
		type: 'email_marketing',
		icon: 'mailjet',
		description: 'Email delivery and marketing',
		requiredFields: ['api_key', 'api_secret', 'list_id'],
		features: ['contacts', 'segments', 'transactional']
	},
	{
		id: 'mailster',
		name: 'Mailster',
		type: 'email_marketing',
		icon: 'mailster',
		description: 'WordPress newsletter plugin',
		requiredFields: ['list_id'],
		features: ['subscribers', 'lists', 'campaigns']
	},
	{
		id: 'getgist',
		name: 'GetGist',
		type: 'email_marketing',
		icon: 'getgist',
		description: 'Marketing, support, and sales automation',
		requiredFields: ['api_key'],
		features: ['contacts', 'conversations', 'automations']
	},
	{
		id: 'platformly',
		name: 'Platformly',
		type: 'email_marketing',
		icon: 'platformly',
		description: 'Marketing automation platform',
		requiredFields: ['api_key', 'project_id'],
		features: ['contacts', 'segments', 'automations']
	},
	{
		id: 'icontact',
		name: 'iContact',
		type: 'email_marketing',
		icon: 'icontact',
		description: 'Email marketing for small business',
		requiredFields: ['api_key', 'api_secret', 'list_id'],
		features: ['contacts', 'lists', 'campaigns']
	}
];

// ============================================================================
// CRM INTEGRATIONS
// ============================================================================

export const CRM_INTEGRATIONS: Integration[] = [
	{
		id: 'hubspot',
		name: 'HubSpot',
		type: 'crm',
		icon: 'hubspot',
		description: 'Inbound marketing, sales, and service software',
		requiredFields: ['api_key', 'portal_id'],
		features: ['contacts', 'companies', 'deals', 'tickets', 'forms']
	},
	{
		id: 'salesforce',
		name: 'Salesforce',
		type: 'crm',
		icon: 'salesforce',
		description: 'World\'s #1 CRM platform',
		requiredFields: ['client_id', 'client_secret', 'refresh_token'],
		features: ['leads', 'contacts', 'accounts', 'opportunities']
	},
	{
		id: 'pipedrive',
		name: 'Pipedrive',
		type: 'crm',
		icon: 'pipedrive',
		description: 'Sales CRM & pipeline management',
		requiredFields: ['api_token', 'domain'],
		features: ['persons', 'organizations', 'deals', 'activities']
	},
	{
		id: 'zohocrm',
		name: 'Zoho CRM',
		type: 'crm',
		icon: 'zoho',
		description: 'Cloud-based CRM software',
		requiredFields: ['client_id', 'client_secret', 'refresh_token'],
		features: ['leads', 'contacts', 'accounts', 'deals']
	},
	{
		id: 'amocrm',
		name: 'amoCRM',
		type: 'crm',
		icon: 'amocrm',
		description: 'Easy to use CRM for sales teams',
		requiredFields: ['subdomain', 'client_id', 'client_secret'],
		features: ['leads', 'contacts', 'companies', 'tasks']
	},
	{
		id: 'onepagecrm',
		name: 'OnePageCRM',
		type: 'crm',
		icon: 'onepagecrm',
		description: 'Action-focused CRM',
		requiredFields: ['user_id', 'api_key'],
		features: ['contacts', 'organizations', 'deals', 'actions']
	},
	{
		id: 'salesflare',
		name: 'Salesflare',
		type: 'crm',
		icon: 'salesflare',
		description: 'Intelligent CRM for SMBs',
		requiredFields: ['api_key'],
		features: ['contacts', 'accounts', 'opportunities', 'tasks']
	},
	{
		id: 'insightly',
		name: 'Insightly',
		type: 'crm',
		icon: 'insightly',
		description: 'CRM & project management',
		requiredFields: ['api_key'],
		features: ['leads', 'contacts', 'organizations', 'projects']
	}
];

// ============================================================================
// MESSAGING INTEGRATIONS
// ============================================================================

export const MESSAGING_INTEGRATIONS: Integration[] = [
	{
		id: 'slack',
		name: 'Slack',
		type: 'messaging',
		icon: 'slack',
		description: 'Business messaging platform',
		requiredFields: ['webhook_url'],
		features: ['messages', 'channels', 'attachments', 'mentions']
	},
	{
		id: 'discord',
		name: 'Discord',
		type: 'messaging',
		icon: 'discord',
		description: 'Community chat platform',
		requiredFields: ['webhook_url'],
		features: ['messages', 'embeds', 'mentions']
	},
	{
		id: 'telegram',
		name: 'Telegram',
		type: 'messaging',
		icon: 'telegram',
		description: 'Cloud-based messaging app',
		requiredFields: ['bot_token', 'chat_id'],
		features: ['messages', 'formatting', 'files']
	},
	{
		id: 'sms',
		name: 'SMS Notification',
		type: 'messaging',
		icon: 'sms',
		description: 'Send SMS notifications',
		requiredFields: ['provider', 'api_key', 'phone_number'],
		features: ['sms', 'templates', 'scheduling']
	},
	{
		id: 'clicksend',
		name: 'ClickSend',
		type: 'messaging',
		icon: 'clicksend',
		description: 'SMS, Email, Voice, and more',
		requiredFields: ['username', 'api_key'],
		features: ['sms', 'voice', 'email', 'mms']
	}
];

// ============================================================================
// AUTOMATION INTEGRATIONS
// ============================================================================

export const AUTOMATION_INTEGRATIONS: Integration[] = [
	{
		id: 'zapier',
		name: 'Zapier',
		type: 'automation',
		icon: 'zapier',
		description: 'Connect apps and automate workflows',
		requiredFields: ['webhook_url'],
		features: ['webhooks', 'triggers', 'actions', '5000+ apps']
	},
	{
		id: 'webhook',
		name: 'Custom Webhook',
		type: 'automation',
		icon: 'webhook',
		description: 'Send data to any webhook endpoint',
		requiredFields: ['url', 'method'],
		features: ['post', 'get', 'headers', 'authentication']
	},
	{
		id: 'make',
		name: 'Make (Integromat)',
		type: 'automation',
		icon: 'make',
		description: 'Visual automation platform',
		requiredFields: ['webhook_url'],
		features: ['scenarios', 'modules', 'scheduling']
	},
	{
		id: 'pabbly',
		name: 'Pabbly Connect',
		type: 'automation',
		icon: 'pabbly',
		description: 'Automation with unlimited workflows',
		requiredFields: ['webhook_url'],
		features: ['workflows', 'triggers', 'actions']
	}
];

// ============================================================================
// STORAGE INTEGRATIONS
// ============================================================================

export const STORAGE_INTEGRATIONS: Integration[] = [
	{
		id: 'googlesheets',
		name: 'Google Sheets',
		type: 'storage',
		icon: 'googlesheets',
		description: 'Save submissions to spreadsheets',
		requiredFields: ['spreadsheet_id', 'sheet_name', 'credentials'],
		features: ['append_row', 'update_row', 'formatting']
	},
	{
		id: 'airtable',
		name: 'Airtable',
		type: 'storage',
		icon: 'airtable',
		description: 'Spreadsheet-database hybrid',
		requiredFields: ['api_key', 'base_id', 'table_name'],
		features: ['records', 'attachments', 'linked_records']
	},
	{
		id: 'notion',
		name: 'Notion',
		type: 'storage',
		icon: 'notion',
		description: 'All-in-one workspace',
		requiredFields: ['api_key', 'database_id'],
		features: ['pages', 'databases', 'properties']
	},
	{
		id: 'trello',
		name: 'Trello',
		type: 'storage',
		icon: 'trello',
		description: 'Visual project management',
		requiredFields: ['api_key', 'token', 'board_id', 'list_id'],
		features: ['cards', 'labels', 'checklists', 'attachments']
	}
];

// ============================================================================
// PAYMENT INTEGRATIONS
// ============================================================================

export const PAYMENT_INTEGRATIONS: Integration[] = [
	{
		id: 'stripe',
		name: 'Stripe',
		type: 'payment',
		icon: 'stripe',
		description: 'Online payment processing',
		requiredFields: ['secret_key', 'publishable_key'],
		features: ['one_time', 'subscriptions', 'invoices', 'connect'],
		premium: true
	},
	{
		id: 'paypal',
		name: 'PayPal',
		type: 'payment',
		icon: 'paypal',
		description: 'Online payment system',
		requiredFields: ['client_id', 'client_secret', 'mode'],
		features: ['one_time', 'subscriptions', 'invoices'],
		premium: true
	},
	{
		id: 'square',
		name: 'Square',
		type: 'payment',
		icon: 'square',
		description: 'Payment and business tools',
		requiredFields: ['access_token', 'location_id'],
		features: ['one_time', 'catalog', 'invoices'],
		premium: true
	},
	{
		id: 'razorpay',
		name: 'Razorpay',
		type: 'payment',
		icon: 'razorpay',
		description: 'Payment gateway for India',
		requiredFields: ['key_id', 'key_secret'],
		features: ['one_time', 'subscriptions', 'invoices'],
		premium: true
	},
	{
		id: 'mollie',
		name: 'Mollie',
		type: 'payment',
		icon: 'mollie',
		description: 'European payment gateway',
		requiredFields: ['api_key'],
		features: ['one_time', 'subscriptions', 'ideal', 'bancontact'],
		premium: true
	}
];

// ============================================================================
// USER MANAGEMENT INTEGRATIONS
// ============================================================================

export const USER_INTEGRATIONS: Integration[] = [
	{
		id: 'user_registration',
		name: 'User Registration',
		type: 'user',
		icon: 'user-plus',
		description: 'Create new user accounts from form submissions',
		requiredFields: ['username_field', 'email_field', 'password_field'],
		features: ['registration', 'roles', 'meta_fields', 'email_verification']
	},
	{
		id: 'user_update',
		name: 'User Update',
		type: 'user',
		icon: 'user-edit',
		description: 'Update existing user profiles',
		requiredFields: ['user_id_field'],
		features: ['profile_update', 'meta_fields', 'password_change']
	}
];

// ============================================================================
// ALL INTEGRATIONS
// ============================================================================

export const ALL_INTEGRATIONS: Integration[] = [
	...EMAIL_MARKETING_INTEGRATIONS,
	...CRM_INTEGRATIONS,
	...MESSAGING_INTEGRATIONS,
	...AUTOMATION_INTEGRATIONS,
	...STORAGE_INTEGRATIONS,
	...PAYMENT_INTEGRATIONS,
	...USER_INTEGRATIONS
];

export const INTEGRATIONS_BY_TYPE: Record<IntegrationType, Integration[]> = {
	email_marketing: EMAIL_MARKETING_INTEGRATIONS,
	crm: CRM_INTEGRATIONS,
	messaging: MESSAGING_INTEGRATIONS,
	automation: AUTOMATION_INTEGRATIONS,
	storage: STORAGE_INTEGRATIONS,
	payment: PAYMENT_INTEGRATIONS,
	user: USER_INTEGRATIONS,
	notification: [] // Internal notifications
};

// ============================================================================
// FIELD TYPES (FluentForm Pro v6.1.8)
// ============================================================================

export interface FieldType {
	id: string;
	label: string;
	icon: string;
	category: string;
	description: string;
	premium?: boolean;
	settings?: string[];
}

export const FIELD_CATEGORIES = {
	general: 'General Fields',
	advanced: 'Advanced Fields',
	payment: 'Payment Fields',
	container: 'Container Fields',
	layout: 'Layout Elements',
	pro: 'Pro Fields'
} as const;

export const FIELD_TYPES: FieldType[] = [
	// General Fields
	{ id: 'text', label: 'Text Input', icon: 'text', category: 'general', description: 'Single line text input' },
	{ id: 'email', label: 'Email', icon: 'mail', category: 'general', description: 'Email address with validation' },
	{ id: 'textarea', label: 'Text Area', icon: 'align-left', category: 'general', description: 'Multi-line text input' },
	{ id: 'number', label: 'Number', icon: 'hash', category: 'general', description: 'Numeric input with min/max' },
	{ id: 'select', label: 'Dropdown', icon: 'chevron-down', category: 'general', description: 'Single selection dropdown' },
	{ id: 'radio', label: 'Radio Button', icon: 'circle-dot', category: 'general', description: 'Single choice from options' },
	{ id: 'checkbox', label: 'Checkbox', icon: 'square-check', category: 'general', description: 'Multiple choice selection' },
	{ id: 'multiselect', label: 'Multi Select', icon: 'list-check', category: 'general', description: 'Multiple selection dropdown' },
	{ id: 'date', label: 'Date Picker', icon: 'calendar', category: 'general', description: 'Date selection with calendar' },
	{ id: 'time', label: 'Time Picker', icon: 'clock', category: 'general', description: 'Time selection' },
	{ id: 'datetime', label: 'Date & Time', icon: 'calendar-clock', category: 'general', description: 'Combined date and time' },
	{ id: 'file', label: 'File Upload', icon: 'upload', category: 'general', description: 'File attachment upload' },
	{ id: 'image', label: 'Image Upload', icon: 'image', category: 'general', description: 'Image file upload with preview' },
	{ id: 'hidden', label: 'Hidden Field', icon: 'eye-off', category: 'general', description: 'Hidden value field' },
	{ id: 'password', label: 'Password', icon: 'lock', category: 'general', description: 'Password input with masking' },
	{ id: 'url', label: 'Website URL', icon: 'link', category: 'general', description: 'URL input with validation' },

	// Advanced Fields
	{ id: 'phone', label: 'Phone Number', icon: 'phone', category: 'advanced', description: 'Phone with formatting' },
	{ id: 'phone_intl', label: 'International Phone', icon: 'globe', category: 'advanced', description: 'Phone with country codes', premium: true },
	{ id: 'address', label: 'Address', icon: 'map-pin', category: 'advanced', description: 'Address with autocomplete', premium: true },
	{ id: 'rating', label: 'Star Rating', icon: 'star', category: 'advanced', description: 'Star-based rating input' },
	{ id: 'nps', label: 'Net Promoter Score', icon: 'gauge', category: 'advanced', description: 'NPS survey (0-10)', premium: true },
	{ id: 'range', label: 'Range Slider', icon: 'sliders', category: 'advanced', description: 'Slider with min/max' },
	{ id: 'range_slider', label: 'Advanced Slider', icon: 'sliders-horizontal', category: 'advanced', description: 'Multi-handle slider', premium: true },
	{ id: 'color', label: 'Color Picker', icon: 'palette', category: 'advanced', description: 'Color selection input' },
	{ id: 'signature', label: 'Signature Pad', icon: 'pen-tool', category: 'advanced', description: 'Digital signature capture' },
	{ id: 'wysiwyg', label: 'Rich Text Editor', icon: 'type', category: 'advanced', description: 'Full WYSIWYG editor' },
	{ id: 'rich_text_input', label: 'Rich Text Input', icon: 'pilcrow', category: 'advanced', description: 'Lightweight rich text', premium: true },
	{ id: 'code', label: 'Code Editor', icon: 'code', category: 'advanced', description: 'Code input with syntax' },
	{ id: 'calculator', label: 'Calculator', icon: 'calculator', category: 'advanced', description: 'Computed field', premium: true },
	{ id: 'chained_select', label: 'Chained Select', icon: 'git-branch', category: 'advanced', description: 'Dependent dropdowns', premium: true },
	{ id: 'dynamic_field', label: 'Dynamic Field', icon: 'database', category: 'advanced', description: 'Dynamic data population', premium: true },
	{ id: 'post_selection', label: 'Post Selection', icon: 'file-text', category: 'advanced', description: 'Select posts/products', premium: true },

	// Payment Fields
	{ id: 'payment', label: 'Payment Amount', icon: 'dollar-sign', category: 'payment', description: 'Payment total field', premium: true },
	{ id: 'payment_method', label: 'Payment Method', icon: 'credit-card', category: 'payment', description: 'Payment method selector', premium: true },
	{ id: 'subscription', label: 'Subscription', icon: 'repeat', category: 'payment', description: 'Subscription plan selector', premium: true },
	{ id: 'item_quantity', label: 'Item Quantity', icon: 'shopping-cart', category: 'payment', description: 'Product quantity', premium: true },
	{ id: 'coupon', label: 'Coupon Code', icon: 'tag', category: 'payment', description: 'Discount coupon field', premium: true },
	{ id: 'payment_summary', label: 'Order Summary', icon: 'receipt', category: 'payment', description: 'Payment summary display', premium: true },

	// Container Fields
	{ id: 'form_step', label: 'Form Step', icon: 'layers', category: 'container', description: 'Multi-step form marker', premium: true },
	{ id: 'repeater', label: 'Repeater', icon: 'copy-plus', category: 'container', description: 'Repeatable field group', premium: true },
	{ id: 'accordion', label: 'Accordion', icon: 'chevrons-down', category: 'container', description: 'Collapsible section', premium: true },

	// Layout Elements
	{ id: 'heading', label: 'Heading', icon: 'heading', category: 'layout', description: 'Section heading text' },
	{ id: 'paragraph', label: 'Paragraph', icon: 'text', category: 'layout', description: 'Descriptive text block' },
	{ id: 'divider', label: 'Divider', icon: 'minus', category: 'layout', description: 'Visual separator line' },
	{ id: 'spacer', label: 'Spacer', icon: 'move-vertical', category: 'layout', description: 'Vertical spacing' },
	{ id: 'shortcode', label: 'Shortcode', icon: 'code-2', category: 'layout', description: 'Embed shortcode', premium: true },

	// Pro Fields
	{ id: 'toggle', label: 'Toggle Switch', icon: 'toggle-right', category: 'pro', description: 'On/off toggle switch', premium: true },
	{ id: 'gdpr', label: 'GDPR Consent', icon: 'shield-check', category: 'pro', description: 'GDPR compliance checkbox', premium: true },
	{ id: 'terms', label: 'Terms & Conditions', icon: 'file-check', category: 'pro', description: 'Terms acceptance', premium: true },
	{ id: 'quiz', label: 'Quiz Question', icon: 'help-circle', category: 'pro', description: 'Quiz/assessment field', premium: true },
	{ id: 'save_progress', label: 'Save Progress', icon: 'save', category: 'pro', description: 'Save and resume button', premium: true },
	{ id: 'chat', label: 'Conversational', icon: 'message-circle', category: 'pro', description: 'Chat-style input', premium: true }
];

// ============================================================================
// FEATURES LIST
// ============================================================================

export interface Feature {
	id: string;
	name: string;
	description: string;
	category: string;
	icon: string;
	premium?: boolean;
}

export const FEATURES: Feature[] = [
	// Form Building
	{ id: 'drag_drop', name: 'Drag & Drop Builder', description: 'Effortlessly create forms with drag and drop', category: 'building', icon: 'move' },
	{ id: 'templates', name: 'Pre-built Templates', description: 'Start from dozens of ready templates', category: 'building', icon: 'layout' },
	{ id: 'multi_step', name: 'Multi-Step Forms', description: 'Break forms into multiple steps', category: 'building', icon: 'layers', premium: true },
	{ id: 'conversational', name: 'Conversational Forms', description: 'Interactive chat-style forms', category: 'building', icon: 'message-circle', premium: true },
	{ id: 'conditional_logic', name: 'Conditional Logic', description: 'Show/hide fields based on input', category: 'building', icon: 'git-branch' },
	{ id: 'calculations', name: 'Numeric Calculations', description: 'Real-time calculations in forms', category: 'building', icon: 'calculator', premium: true },
	{ id: 'file_upload', name: 'File & Image Upload', description: 'Accept file attachments', category: 'building', icon: 'upload', premium: true },

	// Customization
	{ id: 'form_styler', name: 'Advanced Styler', description: 'Customize form appearance', category: 'customization', icon: 'palette', premium: true },
	{ id: 'custom_css', name: 'Custom CSS & JS', description: 'Add custom styling code', category: 'customization', icon: 'code' },
	{ id: 'landing_pages', name: 'Landing Pages', description: 'Turn forms into landing pages', category: 'customization', icon: 'layout', premium: true },
	{ id: 'multi_column', name: 'Multi-Column Layout', description: 'Create multi-column forms', category: 'customization', icon: 'columns' },

	// Data & Entries
	{ id: 'export_entries', name: 'Export Entries', description: 'Export to CSV/Excel', category: 'data', icon: 'download' },
	{ id: 'pdf_generation', name: 'PDF Generation', description: 'Convert entries to PDF', category: 'data', icon: 'file-text', premium: true },
	{ id: 'visual_reporting', name: 'Visual Reports', description: 'Charts and analytics', category: 'data', icon: 'bar-chart' },
	{ id: 'partial_entries', name: 'Partial Entries', description: 'Capture incomplete submissions', category: 'data', icon: 'edit-3', premium: true },
	{ id: 'entry_filter', name: 'Filter Entries', description: 'Advanced entry filtering', category: 'data', icon: 'filter', premium: true },
	{ id: 'print_entries', name: 'Print Entries', description: 'Generate printable records', category: 'data', icon: 'printer' },

	// Advanced
	{ id: 'quiz_survey', name: 'Quiz & Survey', description: 'Build quizzes with scoring', category: 'advanced', icon: 'help-circle', premium: true },
	{ id: 'payment', name: 'Payment Integration', description: 'Accept payments via Stripe/PayPal', category: 'advanced', icon: 'credit-card', premium: true },
	{ id: 'user_registration', name: 'User Registration', description: 'Create user accounts from forms', category: 'advanced', icon: 'user-plus', premium: true },
	{ id: 'save_resume', name: 'Save & Resume', description: 'Let users save progress', category: 'advanced', icon: 'save', premium: true },
	{ id: 'double_optin', name: 'Double Opt-in', description: 'Email confirmation for submissions', category: 'advanced', icon: 'mail-check', premium: true },
	{ id: 'admin_approval', name: 'Admin Approval', description: 'Approve/reject submissions', category: 'advanced', icon: 'check-circle', premium: true },
	{ id: 'scheduling', name: 'Form Scheduling', description: 'Schedule form availability', category: 'advanced', icon: 'calendar', premium: true },
	{ id: 'inventory', name: 'Inventory Management', description: 'Track product availability', category: 'advanced', icon: 'package', premium: true },
	{ id: 'coupons', name: 'Coupon Codes', description: 'Offer discounts via coupons', category: 'advanced', icon: 'tag', premium: true },

	// Security
	{ id: 'spam_protection', name: 'Spam Protection', description: 'reCAPTCHA, hCaptcha, Turnstile', category: 'security', icon: 'shield' },
	{ id: 'gdpr', name: 'GDPR Compliance', description: 'GDPR agreement fields', category: 'security', icon: 'shield-check' },
	{ id: 'advanced_validation', name: 'Advanced Validation', description: 'Custom validation rules', category: 'security', icon: 'check-square' }
];

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

export function getIntegrationById(id: string): Integration | undefined {
	return ALL_INTEGRATIONS.find((i) => i.id === id);
}

export function getFieldTypeById(id: string): FieldType | undefined {
	return FIELD_TYPES.find((f) => f.id === id);
}

export function getIntegrationsByType(type: IntegrationType): Integration[] {
	return INTEGRATIONS_BY_TYPE[type] || [];
}

export function getFieldsByCategory(category: string): FieldType[] {
	return FIELD_TYPES.filter((f) => f.category === category);
}

export function getPremiumFields(): FieldType[] {
	return FIELD_TYPES.filter((f) => f.premium);
}

export function getPremiumIntegrations(): Integration[] {
	return ALL_INTEGRATIONS.filter((i) => i.premium);
}

export function getPremiumFeatures(): Feature[] {
	return FEATURES.filter((f) => f.premium);
}
