import type { Form, FormField, FieldValidation, FormSettings } from '$lib/api/forms';

export interface TemplateField {
	field_type: string;
	label: string;
	name: string;
	placeholder?: string;
	help_text?: string;
	default_value?: string;
	options?: string[];
	validation?: FieldValidation;
	required: boolean;
	width?: number;
}

export interface FormTemplate {
	id: string;
	name: string;
	description: string;
	category: 'contact' | 'survey' | 'registration' | 'feedback' | 'order' | 'application';
	icon: string;
	theme: FormTheme;
	fields: TemplateField[];
	settings?: FormSettings;
}

export interface FormTheme {
	id: string;
	name: string;
	colors: {
		primary: string;
		secondary: string;
		background: string;
		text: string;
		border: string;
	};
	fonts: {
		heading: string;
		body: string;
	};
	spacing: 'compact' | 'normal' | 'spacious';
	borderRadius: 'none' | 'small' | 'medium' | 'large';
}

export const themes: FormTheme[] = [
	{
		id: 'modern-blue',
		name: 'Modern Blue',
		colors: {
			primary: '#6366f1',
			secondary: '#8b5cf6',
			background: '#ffffff',
			text: '#1e293b',
			border: '#e2e8f0'
		},
		fonts: {
			heading: 'Inter, sans-serif',
			body: 'Inter, sans-serif'
		},
		spacing: 'normal',
		borderRadius: 'medium'
	},
	{
		id: 'elegant-purple',
		name: 'Elegant Purple',
		colors: {
			primary: '#8b5cf6',
			secondary: '#a78bfa',
			background: '#faf5ff',
			text: '#2e1065',
			border: '#e9d5ff'
		},
		fonts: {
			heading: 'Playfair Display, serif',
			body: 'Inter, sans-serif'
		},
		spacing: 'spacious',
		borderRadius: 'large'
	},
	{
		id: 'minimal-gray',
		name: 'Minimal Gray',
		colors: {
			primary: '#475569',
			secondary: '#64748b',
			background: '#f8fafc',
			text: '#0f172a',
			border: '#cbd5e1'
		},
		fonts: {
			heading: 'Inter, sans-serif',
			body: 'Inter, sans-serif'
		},
		spacing: 'compact',
		borderRadius: 'small'
	},
	{
		id: 'vibrant-green',
		name: 'Vibrant Green',
		colors: {
			primary: '#10b981',
			secondary: '#34d399',
			background: '#f0fdf4',
			text: '#064e3b',
			border: '#d1fae5'
		},
		fonts: {
			heading: 'Poppins, sans-serif',
			body: 'Inter, sans-serif'
		},
		spacing: 'normal',
		borderRadius: 'medium'
	},
	{
		id: 'dark-mode',
		name: 'Dark Mode',
		colors: {
			primary: '#818cf8',
			secondary: '#a5b4fc',
			background: '#1e293b',
			text: '#f1f5f9',
			border: '#334155'
		},
		fonts: {
			heading: 'Inter, sans-serif',
			body: 'Inter, sans-serif'
		},
		spacing: 'normal',
		borderRadius: 'medium'
	}
];

export const templates: FormTemplate[] = [
	{
		id: 'contact-basic',
		name: 'Basic Contact Form',
		description: 'Simple contact form with name, email, and message',
		category: 'contact',
		icon: '📧',
		theme: themes[0],
		fields: [
			{
				field_type: 'text',
				name: 'name',
				label: 'Full Name',
				required: true,
				placeholder: 'John Doe',
				validation: { min_length: 2, max_length: 100 },
				width: 100
			},
			{
				field_type: 'email',
				name: 'email',
				label: 'Email Address',
				required: true,
				placeholder: 'john@example.com',
				validation: { pattern: '^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$' },
				width: 100
			},
			{
				field_type: 'tel',
				name: 'phone',
				label: 'Phone Number',
				required: false,
				placeholder: '(555) 123-4567',
				width: 100
			},
			{
				field_type: 'textarea',
				name: 'message',
				label: 'Message',
				required: true,
				placeholder: 'How can we help you?',
				validation: { min_length: 10, max_length: 1000 },
				width: 100
			}
		],
		settings: {
			send_email: true,
			success_message: "Thank you for contacting us! We'll get back to you soon."
		}
	},
	{
		id: 'survey-satisfaction',
		name: 'Customer Satisfaction Survey',
		description: 'Gather feedback on customer experience',
		category: 'survey',
		icon: '📊',
		theme: themes[3],
		fields: [
			{
				field_type: 'radio',
				name: 'overall_satisfaction',
				label: 'How satisfied are you with our service?',
				required: true,
				options: ['Very Satisfied', 'Satisfied', 'Neutral', 'Dissatisfied', 'Very Dissatisfied'],
				width: 100
			},
			{
				field_type: 'select',
				name: 'recommendation',
				label: 'How likely are you to recommend us to a friend?',
				required: true,
				options: [
					'10 - Extremely Likely',
					'9',
					'8',
					'7',
					'6',
					'5',
					'4',
					'3',
					'2',
					'1',
					'0 - Not at all Likely'
				],
				width: 100
			},
			{
				field_type: 'checkbox',
				name: 'aspects',
				label: 'What did you like most? (Select all that apply)',
				required: false,
				options: [
					'Customer Service',
					'Product Quality',
					'Pricing',
					'Delivery Speed',
					'Website Experience'
				],
				width: 100
			},
			{
				field_type: 'textarea',
				name: 'improvements',
				label: 'What can we improve?',
				required: false,
				placeholder: 'Your suggestions...',
				validation: { max_length: 500 },
				width: 100
			},
			{
				field_type: 'email',
				name: 'contact_email',
				label: 'Email (optional for follow-up)',
				required: false,
				placeholder: 'your@email.com',
				width: 100
			}
		]
	},
	{
		id: 'registration-event',
		name: 'Event Registration',
		description: 'Register attendees for events, webinars, or workshops',
		category: 'registration',
		icon: '🎫',
		theme: themes[1],
		fields: [
			{
				name: 'first_name',
				label: 'First Name',
				field_type: 'text',
				required: true,
				validation: { min_length: 2, max_length: 50 }
			},
			{
				name: 'last_name',
				label: 'Last Name',
				field_type: 'text',
				required: true,
				validation: { min_length: 2, max_length: 50 }
			},
			{
				name: 'email',
				label: 'Email Address',
				field_type: 'email',
				required: true,
				validation: { pattern: '^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$' }
			},
			{
				name: 'phone',
				label: 'Phone Number',
				field_type: 'tel',
				required: true
			},
			{
				name: 'company',
				label: 'Company/Organization',
				field_type: 'text',
				required: false
			},
			{
				name: 'ticket_type',
				label: 'Ticket Type',
				field_type: 'radio',
				required: true,
				options: ['General Admission', 'VIP', 'Student', 'Group (3+)']
			},
			{
				name: 'dietary_restrictions',
				label: 'Dietary Restrictions',
				field_type: 'checkbox',
				required: false,
				options: ['Vegetarian', 'Vegan', 'Gluten-Free', 'Dairy-Free', 'Nut Allergy', 'None']
			},
			{
				name: 'special_requests',
				label: 'Special Requests or Questions',
				field_type: 'textarea',
				required: false,
				placeholder: 'Any special accommodations needed?'
			}
		],
		settings: {
			send_email: true,
			success_message:
				'Registration successful! Check your email for confirmation and event details.'
		}
	},
	{
		id: 'feedback-product',
		name: 'Product Feedback',
		description: 'Collect detailed product feedback from users',
		category: 'feedback',
		icon: '💭',
		theme: themes[0],
		fields: [
			{
				name: 'product',
				label: 'Which product are you providing feedback for?',
				field_type: 'select',
				required: true,
				options: ['Product A', 'Product B', 'Product C', 'Other']
			},
			{
				name: 'usage_duration',
				label: 'How long have you been using this product?',
				field_type: 'radio',
				required: true,
				options: ['Less than 1 month', '1-3 months', '3-6 months', '6-12 months', 'Over 1 year']
			},
			{
				name: 'rating',
				label: 'Overall Rating',
				field_type: 'select',
				required: true,
				options: ['5 - Excellent', '4 - Good', '3 - Average', '2 - Below Average', '1 - Poor']
			},
			{
				name: 'features_used',
				label: 'Which features do you use most?',
				field_type: 'checkbox',
				required: false,
				options: ['Feature 1', 'Feature 2', 'Feature 3', 'Feature 4', 'Feature 5']
			},
			{
				name: 'pros',
				label: 'What do you like about the product?',
				field_type: 'textarea',
				required: false,
				placeholder: 'Tell us what works well...'
			},
			{
				name: 'cons',
				label: 'What could be improved?',
				field_type: 'textarea',
				required: false,
				placeholder: 'Tell us what needs work...'
			},
			{
				name: 'email',
				label: 'Email (for follow-up)',
				field_type: 'email',
				required: false
			}
		]
	},
	{
		id: 'order-custom',
		name: 'Custom Order Form',
		description: 'Take custom product or service orders',
		category: 'order',
		icon: '🛒',
		theme: themes[3],
		fields: [
			{
				name: 'customer_name',
				label: 'Full Name',
				field_type: 'text',
				required: true,
				validation: { min_length: 2, max_length: 100 }
			},
			{
				name: 'email',
				label: 'Email Address',
				field_type: 'email',
				required: true
			},
			{
				name: 'phone',
				label: 'Phone Number',
				field_type: 'tel',
				required: true
			},
			{
				name: 'product_type',
				label: 'Product/Service Type',
				field_type: 'select',
				required: true,
				options: ['Custom Design', 'Standard Product', 'Bulk Order', 'Special Request']
			},
			{
				name: 'quantity',
				label: 'Quantity',
				field_type: 'number',
				required: true,
				validation: { min: 1, max: 1000 }
			},
			{
				name: 'specifications',
				label: 'Detailed Specifications',
				field_type: 'textarea',
				required: true,
				placeholder: 'Please describe your requirements in detail...',
				validation: { min_length: 20 }
			},
			{
				name: 'budget',
				label: 'Budget Range',
				field_type: 'select',
				required: false,
				options: ['Under $100', '$100-$500', '$500-$1000', '$1000-$5000', 'Over $5000']
			},
			{
				name: 'deadline',
				label: 'Preferred Completion Date',
				field_type: 'date',
				required: false
			},
			{
				name: 'attachments',
				label: 'Reference Files (optional)',
				field_type: 'file',
				required: false
			}
		],
		settings: {
			send_email: true,
			success_message: "Order received! We'll contact you within 24 hours to discuss details."
		}
	},
	{
		id: 'application-job',
		name: 'Job Application',
		description: 'Collect job applications with resume upload',
		category: 'application',
		icon: '💼',
		theme: themes[2],
		fields: [
			{
				name: 'first_name',
				label: 'First Name',
				field_type: 'text',
				required: true,
				validation: { min_length: 2, max_length: 50 }
			},
			{
				name: 'last_name',
				label: 'Last Name',
				field_type: 'text',
				required: true,
				validation: { min_length: 2, max_length: 50 }
			},
			{
				name: 'email',
				label: 'Email Address',
				field_type: 'email',
				required: true
			},
			{
				name: 'phone',
				label: 'Phone Number',
				field_type: 'tel',
				required: true
			},
			{
				name: 'position',
				label: 'Position Applied For',
				field_type: 'select',
				required: true,
				options: ['Software Engineer', 'Product Manager', 'Designer', 'Marketing', 'Sales', 'Other']
			},
			{
				name: 'experience',
				label: 'Years of Experience',
				field_type: 'select',
				required: true,
				options: ['0-1 years', '1-3 years', '3-5 years', '5-10 years', '10+ years']
			},
			{
				name: 'resume',
				label: 'Resume/CV',
				field_type: 'file',
				required: true
			},
			{
				name: 'cover_letter',
				label: 'Cover Letter',
				field_type: 'textarea',
				required: false,
				placeholder: "Tell us why you're a great fit...",
				validation: { max_length: 2000 }
			},
			{
				name: 'portfolio',
				label: 'Portfolio/LinkedIn URL',
				field_type: 'url',
				required: false,
				placeholder: 'https://...'
			},
			{
				name: 'start_date',
				label: 'Available Start Date',
				field_type: 'date',
				required: false
			},
			{
				name: 'salary_expectation',
				label: 'Salary Expectation',
				field_type: 'text',
				required: false,
				placeholder: 'e.g., $80,000 - $100,000'
			}
		],
		settings: {
			send_email: true,
			success_message:
				"Application submitted successfully! We'll review your application and be in touch soon."
		}
	}
];

export function getTemplatesByCategory(category: string): FormTemplate[] {
	return templates.filter((t) => t.category === category);
}

export function getTemplateById(id: string): FormTemplate | undefined {
	return templates.find((t) => t.id === id);
}

export function getThemeById(id: string): FormTheme | undefined {
	return themes.find((t) => t.id === id);
}
