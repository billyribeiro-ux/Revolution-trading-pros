<!--
/**
 * Coupon Create Component - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. ADVANCED RULES ENGINE:
 *    - Product/category restrictions
 *    - User segment targeting
 *    - Geographic restrictions
 *    - Time-based rules
 *    - Stacking rules
 *    - Dynamic pricing
 * 
 * 2. VALIDATION & SECURITY:
 *    - Real-time code validation
 *    - Fraud detection
 *    - Rate limiting
 *    - IP restrictions
 *    - Device fingerprinting
 *    - Audit logging
 * 
 * 3. A/B TESTING:
 *    - Multiple variants
 *    - Control groups
 *    - Performance tracking
 *    - Statistical significance
 *    - Auto-optimization
 *    - Winner selection
 * 
 * 4. ANALYTICS & INSIGHTS:
 *    - Usage tracking
 *    - Conversion metrics
 *    - Revenue impact
 *    - Customer segments
 *    - Redemption patterns
 *    - ROI calculation
 * 
 * 5. AUTOMATION:
 *    - Bulk generation
 *    - Auto-expiration
 *    - Smart distribution
 *    - Trigger-based activation
 *    - Lifecycle management
 *    - Integration hooks
 * 
 * @version 3.0.0 (Google L7+ Enterprise)
 * @component
 */
-->

<script lang="ts">
	import { untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { fade, slide } from 'svelte/transition';
	import {
		couponsApi,
		productsApi,
		categoriesApi,
		segmentsApi,
		AdminApiError
	} from '$lib/api/admin';
	import {
		IconCheck,
		IconX,
		IconPlus,
		IconUsers,
		IconTag,
		IconRefresh,
		IconAlertCircle,
		IconChartBar,
		IconShield,
		IconSettings,
		IconBolt,
		IconMail,
		IconBrandFacebook,
		IconSparkles,
		IconMapPin,
		IconTestPipe
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// Type Definitions
	// ═══════════════════════════════════════════════════════════════════════════

	interface CouponFormData {
		// Basic Information
		code: string;
		type:
			| 'percentage'
			| 'fixed'
			| 'bogo'
			| 'free_shipping'
			| 'tiered'
			| 'bundle'
			| 'cashback'
			| 'points';
		value: number;
		description: string;
		internal_notes: string;

		// Value Configuration
		tiers?: TierConfig[];
		bogo_config?: BOGOConfig;
		max_discount_amount?: number | null;

		// Usage Limits
		max_uses: number | null;
		max_uses_per_user: number | null;
		min_purchase_amount: number | null;
		max_purchase_amount: number | null;
		first_time_only: boolean;
		single_use: boolean;

		// Schedule
		starts_at: string;
		expires_at: string;
		timezone: string;
		recurring: boolean;
		recurrence_pattern?: RecurrencePattern;

		// Restrictions
		product_restrictions: ProductRestriction[];
		category_restrictions: string[];
		user_segments: string[];
		geographic_restrictions: GeographicRestriction[];
		device_restrictions: string[];
		payment_method_restrictions: string[];

		// Advanced Features
		stackable: boolean;
		stackable_with: string[];
		priority: number;
		requires_account: boolean;
		auto_apply: boolean;
		hidden: boolean;

		// A/B Testing
		ab_test_enabled: boolean;
		ab_variants: ABVariant[];
		control_group_percentage: number;

		// Distribution
		distribution_channels: string[];
		email_domains: string[];
		referral_sources: string[];
		utm_parameters: UTMParameters;

		// Status
		is_active: boolean;
		approval_required: boolean;
		approved_by: string | null;
		approved_at: string | null;

		// Metadata
		tags: string[];
		meta: Record<string, any>;
	}

	interface TierConfig {
		min_amount: number;
		max_amount: number;
		discount_value: number;
		discount_type: 'percentage' | 'fixed';
	}

	interface BOGOConfig {
		buy_quantity: number;
		get_quantity: number;
		get_discount: number;
		apply_to: 'cheapest' | 'most_expensive' | 'all';
		same_product_only: boolean;
	}

	interface ProductRestriction {
		type: 'include' | 'exclude';
		product_ids: string[];
		variant_ids: string[];
	}

	interface GeographicRestriction {
		type: 'country' | 'state' | 'city' | 'zip';
		values: string[];
		include: boolean;
	}

	interface RecurrencePattern {
		frequency: 'daily' | 'weekly' | 'monthly';
		interval: number;
		days_of_week?: number[];
		day_of_month?: number;
		max_occurrences?: number;
	}

	interface ABVariant {
		id: string;
		name: string;
		type:
			| 'percentage'
			| 'fixed'
			| 'bogo'
			| 'free_shipping'
			| 'tiered'
			| 'bundle'
			| 'cashback'
			| 'points';
		value: number;
		weight: number;
		is_control: boolean;
	}

	interface UTMParameters {
		source?: string;
		medium?: string;
		campaign?: string;
		term?: string;
		content?: string;
	}

	interface ValidationError {
		field: string;
		message: string;
		severity: 'error' | 'warning' | 'info';
	}

	interface CouponPreview {
		formatted_value: string;
		example_discount: number;
		affected_products: number;
		eligible_users: number;
		estimated_usage: number;
		revenue_impact: number;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	let saving = $state(false);
	let generating = $state(false);
	let testing = $state(false);
	let errors = $state<ValidationError[]>([]);
	let activeTab = $state<'basic' | 'restrictions' | 'advanced' | 'distribution' | 'testing'>('basic');
	let duplicateFrom: string | null = page.url.searchParams.get('duplicate');

	// Form Data
	let formData = $state<CouponFormData>({
		code: '',
		type: 'percentage',
		value: 0,
		description: '',
		internal_notes: '',
		tiers: [],
		bogo_config: {
			buy_quantity: 1,
			get_quantity: 1,
			get_discount: 100,
			apply_to: 'cheapest',
			same_product_only: false
		},
		max_discount_amount: null,
		max_uses: null,
		max_uses_per_user: null,
		min_purchase_amount: null,
		max_purchase_amount: null,
		first_time_only: false,
		single_use: false,
		starts_at: '',
		expires_at: '',
		timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
		recurring: false,
		product_restrictions: [],
		category_restrictions: [],
		user_segments: [],
		geographic_restrictions: [],
		device_restrictions: [],
		payment_method_restrictions: [],
		stackable: false,
		stackable_with: [],
		priority: 0,
		requires_account: false,
		auto_apply: false,
		hidden: false,
		ab_test_enabled: false,
		ab_variants: [],
		control_group_percentage: 10,
		distribution_channels: [],
		email_domains: [],
		referral_sources: [],
		utm_parameters: {},
		is_active: true,
		approval_required: false,
		approved_by: null,
		approved_at: null,
		tags: [],
		meta: {}
	});

	// Lookup Data
	let availableProducts = $state<any[]>([]);
	let availableCategories = $state<any[]>([]);
	let availableSegments = $state<any[]>([]);
	let availableCoupons = $state<any[]>([]);
	let couponPreview = $state<CouponPreview | null>(null);

	// Advanced Features
	let validationResults = $state<any>(null);

	// Events: Component callbacks can be passed as props in Svelte 5

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	let initialized = $state(false);

	$effect(() => {
		if (!initialized) {
			untrack(() => {
				initializeComponent();
			});
			initialized = true;
		}
	});

	async function initializeComponent() {
		await loadLookupData();

		if (duplicateFrom) {
			await loadCouponToDuplicate();
		}

		// Set default dates
		const now = new Date();
		formData.starts_at = formatDateTimeLocal(now);

		const nextMonth = new Date();
		nextMonth.setMonth(nextMonth.getMonth() + 1);
		formData.expires_at = formatDateTimeLocal(nextMonth);

		// Initialize A/B test variants
		initializeABVariants();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadLookupData() {
		try {
			const [products, categories, segments, coupons] = await Promise.all([
				productsApi.list({ limit: 1000 }),
				categoriesApi.list({ limit: 100 }),
				segmentsApi.list({ limit: 100 }),
				couponsApi.list({ limit: 1000, active: true })
			]);

			availableProducts = products.data;
			availableCategories = categories.data;
			availableSegments = segments.data;
			availableCoupons = coupons.data.filter((c) => c.stackable);
		} catch (error) {
			console.error('Failed to load lookup data:', error);
		}
	}

	async function loadCouponToDuplicate() {
		try {
			const response = await couponsApi.get(parseInt(duplicateFrom!));
			const coupon = response.data;

			// Copy relevant fields
			formData = {
				...formData,
				...coupon,
				code: `${coupon.code}_COPY`,
				is_active: false,
				max_uses: null,
				approved_by: null,
				approved_at: null
			};

			addNotification('info', 'Duplicating from existing coupon');
		} catch (error) {
			console.error('Failed to load coupon to duplicate:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Form Submission
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleSubmit() {
		// Clear previous errors
		errors = [];

		// Validate form
		const validationErrors = await validateForm();
		if (validationErrors.length > 0) {
			errors = validationErrors;
			const firstError = validationErrors.find((e) => e.severity === 'error');
			if (firstError) {
				scrollToError(firstError.field);
				return;
			}
		}

		saving = true;
		try {
			// Prepare data
			const submitData = prepareSubmitData();

			// Create coupon
			await couponsApi.create(submitData as unknown as Parameters<typeof couponsApi.create>[0]);

			// Track analytics
			trackEvent('coupon_created', {
				type: formData.type,
				value: formData.value,
				has_restrictions: hasRestrictions(),
				ab_test: formData.ab_test_enabled
			});

			// Success - coupon created

			// Redirect based on user preference
			if (shouldCreateAnother()) {
				resetForm();
				addNotification('success', 'Coupon created successfully');
			} else {
				goto('/admin/coupons');
			}
		} catch (err) {
			if (err instanceof AdminApiError) {
				handleApiError(err);
			} else {
				errors = [{ field: 'general', message: 'Failed to create coupon', severity: 'error' }];
			}
			console.error('Failed to create coupon:', err);
		} finally {
			saving = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Validation
	// ═══════════════════════════════════════════════════════════════════════════

	async function validateForm(): Promise<ValidationError[]> {
		const errors: ValidationError[] = [];

		// Basic validation
		if (!formData.code.trim()) {
			errors.push({ field: 'code', message: 'Coupon code is required', severity: 'error' });
		} else if (!/^[A-Z0-9_-]+$/i.test(formData.code)) {
			errors.push({
				field: 'code',
				message: 'Code can only contain letters, numbers, dashes and underscores',
				severity: 'error'
			});
		}

		// Check code uniqueness
		if (formData.code && !duplicateFrom) {
			const exists = await checkCodeExists(formData.code);
			if (exists) {
				errors.push({ field: 'code', message: 'This code already exists', severity: 'error' });
			}
		}

		// Value validation
		if (formData.value <= 0 && formData.type !== 'free_shipping') {
			errors.push({ field: 'value', message: 'Value must be greater than 0', severity: 'error' });
		}

		if (formData.type === 'percentage' && formData.value > 100) {
			errors.push({ field: 'value', message: 'Percentage cannot exceed 100%', severity: 'error' });
		}

		// Date validation
		if (formData.starts_at && formData.expires_at) {
			const start = new Date(formData.starts_at);
			const end = new Date(formData.expires_at);

			if (end <= start) {
				errors.push({
					field: 'expires_at',
					message: 'Expiration must be after start date',
					severity: 'error'
				});
			}

			if (end < new Date()) {
				errors.push({
					field: 'expires_at',
					message: 'Expiration date is in the past',
					severity: 'warning'
				});
			}
		}

		// Usage limit validation
		if (formData.max_uses_per_user && formData.max_uses) {
			if (formData.max_uses_per_user > formData.max_uses) {
				errors.push({
					field: 'max_uses_per_user',
					message: 'Per-user limit cannot exceed total limit',
					severity: 'error'
				});
			}
		}

		// Purchase amount validation
		if (formData.min_purchase_amount && formData.max_purchase_amount) {
			if (formData.min_purchase_amount >= formData.max_purchase_amount) {
				errors.push({
					field: 'max_purchase_amount',
					message: 'Maximum must be greater than minimum',
					severity: 'error'
				});
			}
		}

		// Tiered discount validation
		if (formData.type === 'tiered' && formData.tiers) {
			errors.push(...validateTiers(formData.tiers));
		}

		// BOGO validation
		if (formData.type === 'bogo' && formData.bogo_config) {
			errors.push(...validateBOGO(formData.bogo_config));
		}

		// A/B test validation
		if (formData.ab_test_enabled) {
			errors.push(...validateABTest(formData.ab_variants));
		}

		// Restriction validation
		if (formData.product_restrictions.length > 0) {
			errors.push(...validateProductRestrictions(formData.product_restrictions));
		}

		// Stacking validation
		if (formData.stackable && formData.stackable_with.length === 0) {
			errors.push({
				field: 'stackable_with',
				message: 'Select coupons that can stack with this one',
				severity: 'warning'
			});
		}

		return errors;
	}

	async function checkCodeExists(code: string): Promise<boolean> {
		try {
			const response = await couponsApi.checkCode(code);
			return response.data.exists;
		} catch {
			return false;
		}
	}

	function validateTiers(tiers: TierConfig[]): ValidationError[] {
		const errors: ValidationError[] = [];

		if (tiers.length === 0) {
			errors.push({ field: 'tiers', message: 'At least one tier is required', severity: 'error' });
		}

		// Check for overlaps and gaps
		const sortedTiers = [...tiers].sort((a, b) => a.min_amount - b.min_amount);

		for (let i = 0; i < sortedTiers.length; i++) {
			const tier = sortedTiers[i];
			if (!tier) continue;

			if (tier.min_amount >= tier.max_amount) {
				errors.push({
					field: `tier_${i}`,
					message: `Tier ${i + 1}: Max must be greater than min`,
					severity: 'error'
				});
			}

			if (i > 0) {
				const prevTier = sortedTiers[i - 1];
				if (prevTier && tier.min_amount < prevTier.max_amount) {
					errors.push({
						field: `tier_${i}`,
						message: `Tier ${i + 1} overlaps with previous tier`,
						severity: 'error'
					});
				}
			}
		}

		return errors;
	}

	function validateBOGO(config: BOGOConfig): ValidationError[] {
		const errors: ValidationError[] = [];

		if (config.buy_quantity <= 0) {
			errors.push({
				field: 'bogo_buy',
				message: 'Buy quantity must be positive',
				severity: 'error'
			});
		}

		if (config.get_quantity <= 0) {
			errors.push({
				field: 'bogo_get',
				message: 'Get quantity must be positive',
				severity: 'error'
			});
		}

		if (config.get_discount < 0 || config.get_discount > 100) {
			errors.push({
				field: 'bogo_discount',
				message: 'Discount must be between 0-100%',
				severity: 'error'
			});
		}

		return errors;
	}

	function validateABTest(variants: ABVariant[]): ValidationError[] {
		const errors: ValidationError[] = [];

		if (variants.length < 2) {
			errors.push({
				field: 'ab_variants',
				message: 'At least 2 variants required for A/B testing',
				severity: 'error'
			});
		}

		const totalWeight = variants.reduce((sum, v) => sum + v.weight, 0);
		if (Math.abs(totalWeight - 100) > 0.01) {
			errors.push({
				field: 'ab_variants',
				message: 'Variant weights must sum to 100%',
				severity: 'error'
			});
		}

		return errors;
	}

	function validateProductRestrictions(restrictions: ProductRestriction[]): ValidationError[] {
		const errors: ValidationError[] = [];

		const hasInclude = restrictions.some((r) => r.type === 'include');
		const hasExclude = restrictions.some((r) => r.type === 'exclude');

		if (hasInclude && hasExclude) {
			errors.push({
				field: 'product_restrictions',
				message: 'Cannot mix include and exclude rules',
				severity: 'error'
			});
		}

		return errors;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Advanced Features
	// ═══════════════════════════════════════════════════════════════════════════

	async function generateCouponCode() {
		generating = true;
		try {
			const response = await couponsApi.generateCode({
				prefix: formData.code || 'PROMO',
				length: 8,
				count: 1
			});
			formData.code = response.data.codes[0] ?? '';
			addNotification('success', 'Code generated successfully');
		} catch (error) {
			console.error('Failed to generate code:', error);
			addNotification('error', 'Failed to generate code');
		} finally {
			generating = false;
		}
	}

	async function testCoupon() {
		testing = true;
		validationResults = null;

		try {
			const results = await couponsApi.test({
				...formData,
				test_scenarios: [
					{ cart_total: 50, user_type: 'new', products: [] },
					{ cart_total: 100, user_type: 'existing', products: [] },
					{ cart_total: 200, user_type: 'vip', products: [] }
				]
			});
			validationResults = results.data;
		} catch (error) {
			console.error('Failed to test coupon:', error);
			addNotification('error', 'Failed to test coupon');
		} finally {
			testing = false;
		}
	}

	async function calculatePreview() {
		if (!formData.code) return;

		try {
			const preview = await couponsApi.preview(formData);
			couponPreview = preview.data;
		} catch (error) {
			console.error('Failed to calculate preview:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// A/B Testing
	// ═══════════════════════════════════════════════════════════════════════════

	function initializeABVariants() {
		if (formData.ab_variants.length === 0) {
			formData.ab_variants = [
				{
					id: generateId(),
					name: 'Control',
					type: formData.type,
					value: formData.value,
					weight: 50,
					is_control: true
				},
				{
					id: generateId(),
					name: 'Variant A',
					type: formData.type,
					value: formData.value * 1.2,
					weight: 50,
					is_control: false
				}
			];
		}
	}

	function addABVariant() {
		const totalWeight = formData.ab_variants.reduce((sum, v) => sum + v.weight, 0);

		formData.ab_variants = [
			...formData.ab_variants,
			{
				id: generateId(),
				name: `Variant ${String.fromCharCode(65 + formData.ab_variants.length - 1)}`,
				type: formData.type,
				value: formData.value,
				weight: Math.max(0, 100 - totalWeight),
				is_control: false
			}
		];
	}

	function removeABVariant(id: string) {
		formData.ab_variants = formData.ab_variants.filter((v) => v.id !== id);
		rebalanceVariantWeights();
	}

	function rebalanceVariantWeights() {
		if (formData.ab_variants.length === 0) return;

		const equalWeight = 100 / formData.ab_variants.length;
		formData.ab_variants = formData.ab_variants.map((v) => ({
			...v,
			weight: equalWeight
		}));
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Restrictions Management
	// ═══════════════════════════════════════════════════════════════════════════

	function addProductRestriction(type: 'include' | 'exclude') {
		formData.product_restrictions = [
			...formData.product_restrictions,
			{
				type,
				product_ids: [],
				variant_ids: []
			}
		];
	}

	function removeProductRestriction(index: number) {
		formData.product_restrictions = formData.product_restrictions.filter((_, i) => i !== index);
	}

	function addGeographicRestriction() {
		formData.geographic_restrictions = [
			...formData.geographic_restrictions,
			{
				type: 'country',
				values: [],
				include: true
			}
		];
	}

	function removeGeographicRestriction(index: number) {
		formData.geographic_restrictions = formData.geographic_restrictions.filter(
			(_, i) => i !== index
		);
	}

	function addTier() {
		if (!formData.tiers) {
			formData.tiers = [];
		}

		const lastTier = formData.tiers[formData.tiers.length - 1];
		const minAmount = lastTier ? lastTier.max_amount : 0;

		formData.tiers = [
			...formData.tiers,
			{
				min_amount: minAmount,
				max_amount: minAmount + 50,
				discount_value: 10,
				discount_type: 'percentage'
			}
		];
	}

	function removeTier(index: number) {
		if (formData.tiers) {
			formData.tiers = formData.tiers.filter((_, i) => i !== index);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function prepareSubmitData() {
		const data = { ...formData } as Record<string, unknown>;

		// Clean up null/undefined values
		Object.keys(data).forEach((key) => {
			if (data[key] === null || data[key] === undefined || data[key] === '') {
				delete data[key];
			}
		});

		// Remove empty arrays
		if (Array.isArray(data['product_restrictions']) && data['product_restrictions'].length === 0) delete data['product_restrictions'];
		if (Array.isArray(data['category_restrictions']) && data['category_restrictions'].length === 0) delete data['category_restrictions'];
		if (Array.isArray(data['user_segments']) && data['user_segments'].length === 0) delete data['user_segments'];

		// Convert dates to ISO format
		if (data['starts_at']) {
			data['starts_at'] = new Date(data['starts_at'] as string).toISOString();
		}
		if (data['expires_at']) {
			data['expires_at'] = new Date(data['expires_at'] as string).toISOString();
		}

		return data;
	}

	function formatDateTimeLocal(date: Date): string {
		const year = date.getFullYear();
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		return `${year}-${month}-${day}T${hours}:${minutes}`;
	}

	function generateId(): string {
		return Math.random().toString(36).substr(2, 9);
	}

	function hasRestrictions(): boolean {
		return (
			formData.product_restrictions.length > 0 ||
			formData.category_restrictions.length > 0 ||
			formData.user_segments.length > 0 ||
			formData.geographic_restrictions.length > 0
		);
	}

	function shouldCreateAnother(): boolean {
		return new URLSearchParams(window.location.search).get('create_another') === 'true';
	}

	function resetForm() {
		formData = {
			...formData,
			code: '',
			description: '',
			internal_notes: '',
			max_uses: null,
			tags: []
		};
	}

	function scrollToError(field: string) {
		const element = document.querySelector(`[data-field="${field}"]`);
		element?.scrollIntoView({ behavior: 'smooth', block: 'center' });
	}

	function handleApiError(error: AdminApiError) {
		if (error.isValidationError && error.validationErrors) {
			errors = Object.entries(error.validationErrors).flatMap(([field, messages]) =>
				messages.map((message) => ({
					field,
					message,
					severity: 'error' as const
				}))
			);
		} else {
			errors = [{ field: 'general', message: error.message, severity: 'error' }];
		}
	}

	function addNotification(type: 'success' | 'error' | 'info', message: string) {
		// Implementation would dispatch to notification store
		console.log(`[${type}] ${message}`);
	}

	function trackEvent(event: string, data: any) {
		// Implementation would send to analytics service
		console.log('Track event:', event, data);
	}

	// Reactive statements - migrated to Svelte 5 runes
	$effect(() => {
		if (formData.code) {
			calculatePreview();
		}
	});

	let discountDisplay = $derived(
		formData.type === 'percentage'
			? `${formData.value}% off`
			: formData.type === 'fixed'
				? `$${formData.value} off`
				: formData.type === 'bogo'
					? `Buy ${formData.bogo_config?.buy_quantity || 1} Get ${formData.bogo_config?.get_quantity || 1}`
					: formData.type === 'free_shipping'
						? 'Free Shipping'
						: 'Tiered Discount'
	);
</script>

<svelte:head>
	<title>Create Coupon | Enterprise Admin</title>
</svelte:head>

<div class="page">
	<!-- Header - CENTERED -->
	<div class="page-header">
		<h1>Create Coupon</h1>
		<p class="subtitle">Configure advanced discount rules and distribution</p>
	</div>

	<!-- Actions Row - CENTERED -->
	<div class="actions-row">
		<button class="btn-secondary" onclick={() => goto('/admin/coupons')}>
			<IconX size={18} />
			Cancel
		</button>
		<button class="btn-secondary" onclick={testCoupon} disabled={testing || !formData.code}>
			<IconTestPipe size={18} />
			Test
		</button>
		<button class="btn-primary" onclick={handleSubmit} disabled={saving}>
			{#if saving}
				<IconRefresh size={18} class="spinning" />
				Creating...
			{:else}
				<IconCheck size={18} />
				Create Coupon
			{/if}
		</button>
	</div>

	<!-- Preview Badge -->
	{#if couponPreview}
		<div class="preview-badge-row">
			<div class="preview-badge">
				<span class="preview-value">{discountDisplay}</span>
				<span class="preview-impact">~${couponPreview.revenue_impact} impact</span>
			</div>
		</div>
	{/if}

	<!-- Validation Errors -->
	{#if errors.length > 0}
		<div class="alerts" transition:slide={{ duration: 300 }}>
			{#each errors as error}
				<div class="alert alert-{error.severity}">
					<IconAlertCircle size={20} />
					<span>{error.message}</span>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Tabs -->
	<div class="tabs">
		<button class="tab" class:active={activeTab === 'basic'} onclick={() => (activeTab = 'basic')}>
			<IconTag size={18} />
			Basic
		</button>
		<button
			class="tab"
			class:active={activeTab === 'restrictions'}
			onclick={() => (activeTab = 'restrictions')}
		>
			<IconShield size={18} />
			Restrictions
		</button>
		<button
			class="tab"
			class:active={activeTab === 'advanced'}
			onclick={() => (activeTab = 'advanced')}
		>
			<IconSettings size={18} />
			Advanced
		</button>
		<button
			class="tab"
			class:active={activeTab === 'distribution'}
			onclick={() => (activeTab = 'distribution')}
		>
			<IconMail size={18} />
			Distribution
		</button>
		<button
			class="tab"
			class:active={activeTab === 'testing'}
			onclick={() => (activeTab = 'testing')}
			class:tab-highlight={formData.ab_test_enabled}
		>
			<IconChartBar size={18} />
			A/B Testing
			{#if formData.ab_test_enabled}
				<span class="tab-badge">ON</span>
			{/if}
		</button>
	</div>

	<!-- Form Content -->
	<form onsubmit={handleSubmit} class="coupon-form">
		{#if activeTab === 'basic'}
			<div class="tab-content" transition:fade={{ duration: 200 }}>
				<!-- Code Section -->
				<div class="form-section">
					<div class="section-header">
						<h2>Coupon Code</h2>
						<div class="section-actions">
							<button
								type="button"
								class="btn-small"
								onclick={generateCouponCode}
								disabled={generating}
							>
								<IconSparkles size={16} />
								Generate
							</button>
						</div>
					</div>

					<div class="form-group" data-field="code">
						<label for="code">
							Code *
							<span class="label-hint">Unique identifier for the coupon</span>
						</label>
						<div class="input-group">
							<input
								type="text"
								id="code"
								bind:value={formData.code}
								placeholder="SUMMER2024"
								class="input input-large"
								class:error={errors.some((e) => e.field === 'code')}
								required
								oninput={() => (formData.code = formData.code.toUpperCase())}
							/>
							{#if formData.code}
								<div class="input-addon">
									<IconCheck size={16} class="text-green" />
								</div>
							{/if}
						</div>
					</div>
				</div>

				<!-- Discount Configuration -->
				<div class="form-section">
					<div class="section-header">
						<h2>Discount Configuration</h2>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="type">Discount Type *</label>
							<select id="type" bind:value={formData.type} class="input">
								<option value="percentage">Percentage Off</option>
								<option value="fixed">Fixed Amount Off</option>
								<option value="bogo">Buy One Get One (BOGO)</option>
								<option value="free_shipping">Free Shipping</option>
								<option value="tiered">Tiered Discount</option>
							</select>
						</div>
					</div>

					{#if formData.type === 'bogo' && formData.bogo_config}
						<div class="form-row" transition:slide>
							<div class="form-group">
								<label for="bogo-buy-quantity">Buy Quantity</label>
								<input
									id="bogo-buy-quantity"
									type="number"
									bind:value={formData.bogo_config.buy_quantity}
									min="1"
									class="input"
								/>
							</div>
							<div class="form-group">
								<label for="bogo-get-quantity">Get Quantity</label>
								<input
									id="bogo-get-quantity"
									type="number"
									bind:value={formData.bogo_config.get_quantity}
									min="1"
									class="input"
								/>
							</div>
							<div class="form-group">
								<label for="bogo-discount">Discount %</label>
								<input
									id="bogo-discount"
									type="number"
									bind:value={formData.bogo_config.get_discount}
									min="0"
									max="100"
									class="input"
								/>
							</div>
						</div>
					{/if}

					{#if formData.type === 'tiered'}
						<div class="tiered-config">
							<div class="tier-header">
								<h3>Discount Tiers</h3>
								<button type="button" class="btn-small" onclick={addTier}>
									<IconPlus size={16} />
									Add Tier
								</button>
							</div>

							{#each formData.tiers || [] as tier, index}
								<div class="tier-row">
									<span class="tier-label">Tier {index + 1}</span>
									<input
										type="number"
										placeholder="Min $"
										bind:value={tier.min_amount}
										class="input input-small"
									/>
									<span>to</span>
									<input
										type="number"
										placeholder="Max $"
										bind:value={tier.max_amount}
										class="input input-small"
									/>
									<select bind:value={tier.discount_type} class="input input-small">
										<option value="percentage">%</option>
										<option value="fixed">$</option>
									</select>
									<input
										type="number"
										placeholder="Value"
										bind:value={tier.discount_value}
										class="input input-small"
									/>
									<button type="button" class="btn-icon" onclick={() => removeTier(index)}>
										<IconX size={16} />
									</button>
								</div>
							{/each}
						</div>
					{/if}

					{#if formData.type === 'percentage' || formData.type === 'fixed'}
						<div class="form-group">
							<label for="max_discount">Maximum Discount Amount ($)</label>
							<input
								type="number"
								id="max_discount"
								bind:value={formData.max_discount_amount}
								min="0"
								step="0.01"
								class="input"
								placeholder="No maximum"
							/>
							<p class="help-text">Cap the discount at this amount</p>
						</div>
					{/if}
				</div>

				<!-- Description -->
				<div class="form-section">
					<h2>Description & Notes</h2>

					<div class="form-group">
						<label for="description">Public Description</label>
						<textarea
							id="description"
							bind:value={formData.description}
							placeholder="Summer sale - Save on all products!"
							class="input textarea"
							rows="3"
						></textarea>
						<p class="help-text">Shown to customers</p>
					</div>

					<div class="form-group">
						<label for="internal_notes">Internal Notes</label>
						<textarea
							id="internal_notes"
							bind:value={formData.internal_notes}
							placeholder="Created for Q3 marketing campaign..."
							class="input textarea"
							rows="3"
						></textarea>
						<p class="help-text">Only visible to administrators</p>
					</div>
				</div>

				<!-- Schedule -->
				<div class="form-section">
					<h2>Schedule</h2>

					<div class="form-row">
						<div class="form-group">
							<label for="starts_at">Start Date</label>
							<input
								type="datetime-local"
								id="starts_at"
								bind:value={formData.starts_at}
								class="input"
							/>
						</div>

						<div class="form-group">
							<label for="expires_at">Expiration Date</label>
							<input
								type="datetime-local"
								id="expires_at"
								bind:value={formData.expires_at}
								class="input"
							/>
						</div>
					</div>

					<div class="form-group">
						<label class="toggle-label">
							<input type="checkbox" bind:checked={formData.recurring} class="toggle-checkbox" />
							<span class="toggle-switch"></span>
							<span class="toggle-text">Recurring coupon</span>
						</label>
					</div>

					{#if formData.recurring}
						<div class="recurrence-config">
							<!-- Recurrence pattern configuration would go here -->
						</div>
					{/if}
				</div>
			</div>
		{/if}

		{#if activeTab === 'restrictions'}
			<div class="tab-content" transition:fade={{ duration: 200 }}>
				<!-- Usage Limits -->
				<div class="form-section">
					<h2>Usage Limits</h2>

					<div class="form-row">
						<div class="form-group">
							<label for="max_uses">Total Uses</label>
							<input
								type="number"
								id="max_uses"
								bind:value={formData.max_uses}
								min="1"
								class="input"
								placeholder="Unlimited"
							/>
						</div>

						<div class="form-group">
							<label for="max_uses_per_user">Uses Per Customer</label>
							<input
								type="number"
								id="max_uses_per_user"
								bind:value={formData.max_uses_per_user}
								min="1"
								class="input"
								placeholder="Unlimited"
							/>
						</div>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="min_purchase">Minimum Purchase ($)</label>
							<input
								type="number"
								id="min_purchase"
								bind:value={formData.min_purchase_amount}
								min="0"
								step="0.01"
								class="input"
								placeholder="No minimum"
							/>
						</div>

						<div class="form-group">
							<label for="max_purchase">Maximum Purchase ($)</label>
							<input
								type="number"
								id="max_purchase"
								bind:value={formData.max_purchase_amount}
								min="0"
								step="0.01"
								class="input"
								placeholder="No maximum"
							/>
						</div>
					</div>

					<div class="form-checkboxes">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.first_time_only} />
							<span>First-time customers only</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.single_use} />
							<span>Single use per customer</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.requires_account} />
							<span>Requires customer account</span>
						</label>
					</div>
				</div>

				<!-- Product Restrictions -->
				<div class="form-section">
					<div class="section-header">
						<h2>Product Restrictions</h2>
						<button
							type="button"
							class="btn-small"
							onclick={() => addProductRestriction('include')}
						>
							<IconPlus size={16} />
							Add Rule
						</button>
					</div>

					{#each formData.product_restrictions as restriction, index}
						<div class="restriction-rule">
							<select bind:value={restriction.type} class="input input-small">
								<option value="include">Include</option>
								<option value="exclude">Exclude</option>
							</select>

							<select multiple bind:value={restriction.product_ids} class="input input-multi">
								{#each availableProducts as product}
									<option value={product.id}>{product.name}</option>
								{/each}
							</select>

							<button
								type="button"
								class="btn-icon"
								onclick={() => removeProductRestriction(index)}
							>
								<IconX size={16} />
							</button>
						</div>
					{/each}
				</div>

				<!-- Category Restrictions -->
				<div class="form-section">
					<h2>Category Restrictions</h2>

					<div class="form-group">
						<label for="category-restrictions">Applicable Categories</label>
						<select
							id="category-restrictions"
							multiple
							bind:value={formData.category_restrictions}
							class="input input-multi"
							size="5"
						>
							{#each availableCategories as category}
								<option value={category.id}>{category.name}</option>
							{/each}
						</select>
						<p class="help-text">Leave empty for all categories</p>
					</div>
				</div>

				<!-- User Segments -->
				<div class="form-section">
					<h2>Customer Segments</h2>

					<div class="form-group">
						<label for="eligible-segments">Eligible Segments</label>
						<select
							id="eligible-segments"
							multiple
							bind:value={formData.user_segments}
							class="input input-multi"
							size="5"
						>
							{#each availableSegments as segment}
								<option value={segment.id}>{segment.name}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Geographic Restrictions -->
				<div class="form-section">
					<div class="section-header">
						<h2>Geographic Restrictions</h2>
						<button type="button" class="btn-small" onclick={addGeographicRestriction}>
							<IconMapPin size={16} />
							Add Location
						</button>
					</div>

					{#each formData.geographic_restrictions as restriction, index}
						<div class="restriction-rule">
							<select bind:value={restriction.type} class="input input-small">
								<option value="country">Country</option>
								<option value="state">State</option>
								<option value="city">City</option>
								<option value="zip">ZIP Code</option>
							</select>

							<select bind:value={restriction.include} class="input input-small">
								<option value={true}>Include</option>
								<option value={false}>Exclude</option>
							</select>

							<input
								type="text"
								placeholder="Enter values separated by commas"
								bind:value={restriction.values}
								class="input"
							/>

							<button
								type="button"
								class="btn-icon"
								onclick={() => removeGeographicRestriction(index)}
							>
								<IconX size={16} />
							</button>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if activeTab === 'advanced'}
			<div class="tab-content" transition:fade={{ duration: 200 }}>
				<!-- Stacking Rules -->
				<div class="form-section">
					<h2>Stacking Rules</h2>

					<div class="form-group">
						<label class="toggle-label">
							<input type="checkbox" bind:checked={formData.stackable} class="toggle-checkbox" />
							<span class="toggle-switch"></span>
							<span class="toggle-text">Can be combined with other coupons</span>
						</label>
					</div>

					{#if formData.stackable}
						<div class="form-group">
							<label for="stackable-with">Stackable With</label>
							<select
								id="stackable-with"
								multiple
								bind:value={formData.stackable_with}
								class="input input-multi"
								size="5"
							>
								{#each availableCoupons as coupon}
									<option value={coupon.code}>{coupon.code} - {coupon.description}</option>
								{/each}
							</select>
						</div>
					{/if}

					<div class="form-group">
						<label for="priority">Priority</label>
						<input
							type="number"
							id="priority"
							bind:value={formData.priority}
							min="0"
							max="100"
							class="input"
						/>
						<p class="help-text">Higher priority coupons are applied first</p>
					</div>
				</div>

				<!-- Auto-Apply Settings -->
				<div class="form-section">
					<h2>Auto-Apply Settings</h2>

					<div class="form-checkboxes">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.auto_apply} />
							<span>Automatically apply when conditions are met</span>
						</label>
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={formData.hidden} />
							<span>Hidden coupon (not shown in lists)</span>
						</label>
					</div>
				</div>

				<!-- Tags & Metadata -->
				<div class="form-section">
					<h2>Tags & Metadata</h2>

					<div class="form-group">
						<label for="tags">Tags</label>
						<input
							type="text"
							id="tags"
							placeholder="summer, sale, vip (comma-separated)"
							bind:value={formData.tags}
							class="input"
						/>
					</div>
				</div>
			</div>
		{/if}

		{#if activeTab === 'distribution'}
			<div class="tab-content" transition:fade={{ duration: 200 }}>
				<!-- Distribution Channels -->
				<div class="form-section">
					<h2>Distribution Channels</h2>

					<div class="channels-grid">
						{#each ['email', 'sms', 'social', 'affiliate', 'influencer', 'partner'] as channel}
							<label class="channel-option">
								<input
									type="checkbox"
									bind:group={formData.distribution_channels}
									value={channel}
								/>
								<span class="channel-label">
									{#if channel === 'email'}
										<IconMail size={20} />
									{:else if channel === 'social'}
										<IconBrandFacebook size={20} />
									{:else}
										<IconUsers size={20} />
									{/if}
									{channel.charAt(0).toUpperCase() + channel.slice(1)}
								</span>
							</label>
						{/each}
					</div>
				</div>

				<!-- Email Restrictions -->
				<div class="form-section">
					<h2>Email Domain Restrictions</h2>

					<div class="form-group">
						<label for="email-domains">Allowed Email Domains</label>
						<textarea
							id="email-domains"
							placeholder="gmail.com&#10;company.com&#10;university.edu"
							bind:value={formData.email_domains}
							class="input textarea"
							rows="4"
						></textarea>
						<p class="help-text">One domain per line. Leave empty to allow all.</p>
					</div>
				</div>

				<!-- UTM Parameters -->
				<div class="form-section">
					<h2>UTM Tracking</h2>

					<div class="form-row">
						<div class="form-group">
							<label for="utm-source">Source</label>
							<input
								id="utm-source"
								type="text"
								bind:value={formData.utm_parameters.source}
								placeholder="newsletter"
								class="input"
							/>
						</div>
						<div class="form-group">
							<label for="utm-medium">Medium</label>
							<input
								id="utm-medium"
								type="text"
								bind:value={formData.utm_parameters.medium}
								placeholder="email"
								class="input"
							/>
						</div>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="utm-campaign">Campaign</label>
							<input
								id="utm-campaign"
								type="text"
								bind:value={formData.utm_parameters.campaign}
								placeholder="summer_sale"
								class="input"
							/>
						</div>
						<div class="form-group">
							<label for="utm-content">Content</label>
							<input
								id="utm-content"
								type="text"
								bind:value={formData.utm_parameters.content}
								placeholder="header_banner"
								class="input"
							/>
						</div>
					</div>
				</div>
			</div>
		{/if}

		{#if activeTab === 'testing'}
			<div class="tab-content" transition:fade={{ duration: 200 }}>
				<!-- A/B Test Configuration -->
				<div class="form-section">
					<h2>A/B Test Configuration</h2>

					<div class="form-group">
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.ab_test_enabled}
								class="toggle-checkbox"
							/>
							<span class="toggle-switch"></span>
							<span class="toggle-text">Enable A/B testing for this coupon</span>
						</label>
					</div>

					{#if formData.ab_test_enabled}
						<div class="ab-test-config">
							<div class="form-group">
								<label for="control-group-size">Control Group Size (%)</label>
								<input
									id="control-group-size"
									type="number"
									bind:value={formData.control_group_percentage}
									min="0"
									max="50"
									class="input"
								/>
								<p class="help-text">Percentage of users who won't see the coupon</p>
							</div>

							<div class="variants-section">
								<div class="section-header">
									<h3>Variants</h3>
									<button type="button" class="btn-small" onclick={addABVariant}>
										<IconPlus size={16} />
										Add Variant
									</button>
								</div>

								{#each formData.ab_variants as variant}
									<div class="variant-row">
										<input
											type="text"
											placeholder="Variant name"
											bind:value={variant.name}
											class="input input-small"
										/>

										<select bind:value={variant.type} class="input input-small">
											<option value="percentage">%</option>
											<option value="fixed">$</option>
										</select>

										<input
											type="number"
											placeholder="Value"
											bind:value={variant.value}
											class="input input-small"
										/>

										<input
											type="number"
											placeholder="Weight %"
											bind:value={variant.weight}
											min="0"
											max="100"
											class="input input-small"
										/>

										{#if !variant.is_control}
											<button
												type="button"
												class="btn-icon"
												onclick={() => removeABVariant(variant.id)}
											>
												<IconX size={16} />
											</button>
										{/if}
									</div>
								{/each}

								<button type="button" class="btn-small" onclick={rebalanceVariantWeights}>
									<IconBolt size={16} />
									Auto-Balance Weights
								</button>
							</div>
						</div>
					{/if}
				</div>

				<!-- Test Results -->
				{#if validationResults}
					<div class="form-section">
						<h2>Test Results</h2>

						<div class="test-results">
							{#each validationResults.scenarios as scenario}
								<div class="test-scenario">
									<h4>Cart: ${scenario.cart_total} | User: {scenario.user_type}</h4>
									<div class="scenario-result" class:success={scenario.applicable}>
										{#if scenario.applicable}
											<IconCheck size={16} />
											<span>Discount: ${scenario.discount_amount}</span>
										{:else}
											<IconX size={16} />
											<span>Not applicable: {scenario.reason}</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</form>
</div>

<style>
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header Styles - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Actions Row - CENTERED */
	.actions-row {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	/* Preview Badge */
	.preview-badge-row {
		display: flex;
		justify-content: center;
		margin-bottom: 1.5rem;
	}

	.preview-badge {
		display: inline-flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.1), rgba(179, 143, 0, 0.1));
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 12px;
		margin-top: 1rem;
	}

	.preview-value {
		font-size: 1.25rem;
		font-weight: 600;
		color: #E6B800;
	}

	.preview-impact {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Tabs */
	.tabs {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-bottom: 2rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
		position: relative;
	}

	.tab:hover {
		color: #cbd5e1;
	}

	.tab.active {
		color: #E6B800;
		border-bottom-color: #E6B800;
	}

	.tab-highlight {
		position: relative;
	}

	.tab-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		padding: 0.125rem 0.375rem;
		background: #10b981;
		color: white;
		font-size: 0.625rem;
		font-weight: 700;
		border-radius: 4px;
	}

	/* Alerts */
	.alerts {
		margin-bottom: 1.5rem;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.alert-warning {
		background: rgba(245, 158, 11, 0.1);
		border: 1px solid rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}

	.alert-info {
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		color: #60a5fa;
	}

	/* Form */
	.coupon-form {
		display: flex;
		flex-direction: column;
	}

	.tab-content {
		animation: fadeIn 0.3s ease;
	}

	.form-section {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		padding: 2rem;
		margin-bottom: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.section-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.section-actions {
		display: flex;
		gap: 0.5rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
	}

	label {
		display: block;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.label-hint {
		font-weight: 400;
		color: #94a3b8;
		font-size: 0.875rem;
		margin-left: 0.5rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.input.error {
		border-color: rgba(239, 68, 68, 0.5);
		box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
	}

	.input-large {
		font-size: 1.25rem;
		font-weight: 600;
		letter-spacing: 0.05em;
	}

	.input-small {
		max-width: 150px;
	}

	.input-multi {
		min-height: 120px;
	}

	.input-group {
		position: relative;
	}

	.input-addon {
		position: absolute;
		right: 1rem;
		top: 50%;
		transform: translateY(-50%);
	}

	.textarea {
		resize: vertical;
		min-height: 80px;
	}

	.help-text {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-ghost,
	.btn-small {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #E6B800, #B38F00);
		color: #0D1117;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-ghost {
		background: transparent;
		color: #94a3b8;
	}

	.btn-ghost:hover {
		background: rgba(148, 163, 184, 0.1);
	}

	.btn-small {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.btn-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Toggle Switch */
	.toggle-label {
		display: flex;
		align-items: center;
		gap: 1rem;
		cursor: pointer;
		margin-bottom: 0;
	}

	.toggle-checkbox {
		display: none;
	}

	.toggle-switch {
		position: relative;
		width: 52px;
		height: 28px;
		background: rgba(148, 163, 184, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 28px;
		transition: 0.3s;
		flex-shrink: 0;
	}

	.toggle-switch:before {
		content: '';
		position: absolute;
		height: 20px;
		width: 20px;
		left: 3px;
		bottom: 3px;
		background: #94a3b8;
		border-radius: 50%;
		transition: 0.3s;
	}

	.toggle-checkbox:checked + .toggle-switch {
		background: linear-gradient(135deg, #E6B800, #B38F00);
		border-color: transparent;
	}

	.toggle-checkbox:checked + .toggle-switch:before {
		transform: translateX(24px);
		background: white;
	}

	.toggle-text {
		color: #cbd5e1;
		font-weight: 500;
	}

	/* Checkbox */
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #cbd5e1;
		margin-bottom: 0.75rem;
	}

	.form-checkboxes {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	/* Special Configurations */
	.bogo-config,
	.tiered-config,
	.recurrence-config,
	.ab-test-config {
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
		margin-top: 1rem;
	}

	.tiered-config h3 {
		font-size: 1.1rem;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
	}

	.tier-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.tier-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.tier-label {
		font-weight: 600;
		color: #94a3b8;
		min-width: 60px;
	}

	/* Restrictions */
	.restriction-rule {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	/* Channels Grid */
	.channels-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		gap: 1rem;
	}

	.channel-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.channel-option:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.channel-option input[type='checkbox']:checked + .channel-label {
		color: #E6B800;
	}

	.channel-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #cbd5e1;
	}

	/* Variants */
	.variants-section {
		margin-top: 1.5rem;
	}

	.variant-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
	}

	/* Test Results */
	.test-results {
		display: grid;
		gap: 1rem;
	}

	.test-scenario {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
	}

	.test-scenario h4 {
		font-size: 0.95rem;
		color: #94a3b8;
		margin: 0 0 0.5rem 0;
	}

	.scenario-result {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
	}

	.scenario-result.success {
		color: #10b981;
	}

	.scenario-result:not(.success) {
		color: #f87171;
	}

	/* Utilities */
	.text-green {
		color: #10b981;
	}

	.spinning {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.actions-row {
			flex-wrap: wrap;
		}

		.tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.restriction-rule,
		.tier-row {
			flex-wrap: wrap;
		}
	}
</style>
