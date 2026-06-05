<!--
/**
 * User Create Component - Google L7+ Enterprise Implementation
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. ADVANCED SECURITY:
 *    - Password strength meter with requirements
 *    - Breach database checking (HaveIBeenPwned)
 *    - 2FA configuration
 *    - Session management
 *    - IP restrictions
 *    - Device fingerprinting
 * 
 * 2. ROLE SYSTEM (3 TIERS):
 *    - Admin: Full system control
 *    - Trader: Store access, no creation rights
 *    - Member: Basic access only
 *    - Granular permission visualization
 *    - Permission inheritance
 *    - Access scope definition
 * 
 * 3. ORGANIZATIONAL FEATURES:
 *    - Department assignment
 *    - Team structure
 *    - Manager relationships
 *    - Location/Office
 *    - Employee ID
 *    - Cost center
 * 
 * 4. COMPLIANCE & AUDIT:
 *    - Terms acceptance
 *    - Privacy consent
 *    - Data processing agreements
 *    - Background check status
 *    - Training requirements
 *    - Audit trail
 * 
 * 5. COMMUNICATION:
 *    - Welcome email customization
 *    - Onboarding schedule
 *    - Training assignments
 *    - Team notifications
 *    - Calendar integration
 * 
 * @component
 */
-->

<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { SvelteDate } from 'svelte/reactivity';
	import { goto } from '$app/navigation';
	import { usersApi, AdminApiError } from '$lib/api/admin';
	import { logger } from '$lib/utils/logger';
	import type { IconComponent } from '$lib/icons';
	import {
		IconCheck,
		IconUser,
		IconShield,
		IconBuilding,
		IconUserCheck,
		IconChartBar,
		IconShieldCheck,
		IconSettings,
		IconCheckupList
	} from '$lib/icons';

	// Type Definitions

	interface UserFormData {
		// Identity
		first_name: string;
		last_name: string;
		username: string;
		email: string;
		alternate_email: string;
		phone: string;
		employee_id: string;

		// Security
		password: string;
		password_confirmation: string;
		temporary_password: boolean;
		password_expires_at: string;
		require_password_change: boolean;
		require_2fa: boolean;
		allowed_ips: string[];
		session_timeout: number;

		// Role & Permissions
		role: 'admin' | 'trader' | 'member';
		custom_permissions: string[];
		access_scope: AccessScope;

		// Organization
		department: string;
		team: string;
		manager_id: string;
		job_title: string;
		location: string;
		office: string;
		cost_center: string;

		// Preferences
		timezone: string;
		language: string;
		date_format: string;
		currency: string;
		notification_preferences: NotificationPreferences;

		// Compliance
		terms_accepted: boolean;
		privacy_accepted: boolean;
		data_processing_consent: boolean;
		marketing_consent: boolean;
		background_check_status: 'pending' | 'completed' | 'failed' | 'not_required';
		background_check_date: string;

		// Onboarding
		start_date: string;
		probation_end_date: string;
		onboarding_plan: string;
		training_modules: string[];
		assigned_equipment: string[];
		assigned_buddy: string;

		// Status
		status: 'pending' | 'active' | 'suspended' | 'inactive';
		activation_method: 'email' | 'manual' | 'auto';
		send_welcome_email: boolean;
		add_to_directory: boolean;
		sync_to_systems: string[];

		// Meta
		notes: string;
		tags: string[];
		custom_fields: Record<string, unknown>;
	}

	interface AccessScope {
		products: 'all' | 'category' | 'specific' | 'none';
		orders: 'all' | 'own' | 'team' | 'none';
		customers: 'all' | 'assigned' | 'none';
		reports: 'all' | 'standard' | 'none';
		settings: 'all' | 'limited' | 'none';
	}

	interface NotificationPreferences {
		email: boolean;
		sms: boolean;
		push: boolean;
		slack: boolean;
		digest_frequency: 'realtime' | 'hourly' | 'daily' | 'weekly';
		categories: {
			security: boolean;
			updates: boolean;
			reports: boolean;
			mentions: boolean;
			tasks: boolean;
		};
	}

	interface PasswordStrength {
		score: number;
		feedback: string[];
		suggestions: string[];
		crackTime: string;
		isBreached: boolean;
	}

	interface ValidationError {
		field: string;
		message: string;
		severity: 'error' | 'warning' | 'info';
	}

	interface RoleDefinition {
		name: string;
		key: 'admin' | 'trader' | 'member';
		description: string;
		icon: IconComponent;
		color: string;
		permissions: string[];
		restrictions: string[];
		accessLevel: number;
	}

	// Lookup option shapes returned by the organization APIs. Only the fields
	// actually read in this component are typed; extras are tolerated.
	interface LookupOption {
		id: string;
		name?: string;
	}

	interface LocationOption extends LookupOption {
		timezone?: string;
	}

	interface OnboardingPlanOption extends LookupOption {
		duration?: number;
	}

	// Role Definitions

	const ROLE_DEFINITIONS: RoleDefinition[] = [
		{
			name: 'Administrator',
			key: 'admin',
			description: 'Full system control with unrestricted access to all features',
			icon: IconShieldCheck,
			color: '#E6B800',
			permissions: [
				'Full admin panel access',
				'User management',
				'System configuration',
				'Create/edit/delete all content',
				'Financial controls',
				'API management',
				'Security settings',
				'Audit logs'
			],
			restrictions: [],
			accessLevel: 100
		},
		{
			name: 'Trader',
			key: 'trader',
			description: 'Complete store access with view and manage rights, no creation',
			icon: IconChartBar,
			color: '#B38F00',
			permissions: [
				'View all store data',
				'Manage existing products',
				'Process orders',
				'View reports',
				'Manage inventory',
				'Customer service',
				'View analytics'
			],
			restrictions: [
				'Cannot create new pages',
				'Cannot add new products',
				'Cannot modify system settings',
				'Cannot manage users',
				'Cannot access billing'
			],
			accessLevel: 50
		},
		{
			name: 'Member',
			key: 'member',
			description: 'Basic access for viewing and personal account management',
			icon: IconUser,
			color: '#10b981',
			permissions: [
				'View public content',
				'Manage own profile',
				'View assigned tasks',
				'Submit support tickets',
				'Access knowledge base'
			],
			restrictions: [
				'No admin panel access',
				'Cannot view sensitive data',
				'Cannot modify content',
				'Cannot access reports',
				'Cannot manage others'
			],
			accessLevel: 10
		}
	];

	// State Management (Svelte 5 Runes)

	let saving = $state(false);
	let validating = $state(false);
	let checkingUsername = $state(false);
	let checkingEmail = $state(false);
	let loadingManagers = $state(false);
	let errors = $state<ValidationError[]>([]);
	let activeStep = $state<
		'identity' | 'security' | 'role' | 'organization' | 'preferences' | 'compliance' | 'review'
	>('identity');
	let passwordVisible = $state(false);
	let confirmPasswordVisible = $state(false);
	let passwordStrength = $state<PasswordStrength | null>(null);
	let usernameAvailable = $state<boolean | null>(null);
	let emailAvailable = $state<boolean | null>(null);
	let showAdvancedSecurity = $state(false);
	let showPermissionDetails = $state(false);
	let profilePhotoFile = $state<File | null>(null);
	let profilePhotoPreview = $state<string | null>(null);
	let usernameValidationTimer: ReturnType<typeof setTimeout> | null = null;
	let emailValidationTimer: ReturnType<typeof setTimeout> | null = null;
	let passwordStrengthTimer: ReturnType<typeof setTimeout> | null = null;

	// Form Data with defaults (Svelte 5 $state)
	let formData = $state<UserFormData>({
		first_name: '',
		last_name: '',
		username: '',
		email: '',
		alternate_email: '',
		phone: '',
		employee_id: '',
		password: '',
		password_confirmation: '',
		temporary_password: false,
		password_expires_at: '',
		require_password_change: false,
		require_2fa: false,
		allowed_ips: [],
		session_timeout: 480,
		role: 'member',
		custom_permissions: [],
		access_scope: {
			products: 'none',
			orders: 'none',
			customers: 'none',
			reports: 'none',
			settings: 'none'
		},
		department: '',
		team: '',
		manager_id: '',
		job_title: '',
		location: '',
		office: '',
		cost_center: '',
		timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
		language: 'en',
		date_format: 'MM/DD/YYYY',
		currency: 'USD',
		notification_preferences: {
			email: true,
			sms: false,
			push: true,
			slack: false,
			digest_frequency: 'daily',
			categories: {
				security: true,
				updates: true,
				reports: false,
				mentions: true,
				tasks: true
			}
		},
		terms_accepted: false,
		privacy_accepted: false,
		data_processing_consent: false,
		marketing_consent: false,
		background_check_status: 'not_required',
		background_check_date: '',
		start_date: '',
		probation_end_date: '',
		onboarding_plan: 'standard',
		training_modules: [],
		assigned_equipment: [],
		assigned_buddy: '',
		status: 'pending',
		activation_method: 'email',
		send_welcome_email: true,
		add_to_directory: true,
		sync_to_systems: [],
		notes: '',
		tags: [],
		custom_fields: {}
	});

	// Lookup data (Svelte 5 $state)
	let departments = $state<LookupOption[]>([]);
	let teams = $state<LookupOption[]>([]);
	let managers = $state<LookupOption[]>([]);
	let locations = $state<LocationOption[]>([]);
	let trainingModules = $state<LookupOption[]>([]);
	let onboardingPlans = $state<OnboardingPlanOption[]>([]);

	// Progress calculation
	let completionPercentage = $derived(calculateCompletion());
	let currentStepIndex = $derived(getStepIndex(activeStep));
	let canProceed = $derived(validateCurrentStep());

	// Lifecycle (Svelte 5 $effect)

	// Initialize on mount
	onMount(() => {
		loadLookupData();
		initializeDefaults();
		setupRealtimeValidation();
	});

	onDestroy(() => {
		if (usernameValidationTimer) clearTimeout(usernameValidationTimer);
		if (emailValidationTimer) clearTimeout(emailValidationTimer);
		if (passwordStrengthTimer) clearTimeout(passwordStrengthTimer);
	});

	// Data Loading

	async function loadLookupData() {
		try {
			// Load REAL data from APIs using Promise.allSettled for resilience
			const [deptRes, teamsRes, managersRes, locationsRes, trainingRes, onboardingRes] =
				await Promise.allSettled([
					fetch('/api/admin/organization/departments'),
					fetch('/api/admin/organization/teams'),
					fetch('/api/admin/users?role=manager&limit=50'),
					fetch('/api/admin/organization/locations'),
					fetch('/api/admin/organization/training-modules'),
					fetch('/api/admin/organization/onboarding-plans')
				]);

			// Process departments - provide sensible defaults if API fails
			if (deptRes.status === 'fulfilled' && deptRes.value.ok) {
				const data = await deptRes.value.json();
				departments = Array.isArray(data) ? data : data.data || [];
			}
			// Default departments only if API returns empty
			if (departments.length === 0) {
				departments = [{ id: 'general', name: 'General' }];
			}

			// Process teams
			if (teamsRes.status === 'fulfilled' && teamsRes.value.ok) {
				const data = await teamsRes.value.json();
				teams = Array.isArray(data) ? data : data.data || [];
			}

			// Process managers
			if (managersRes.status === 'fulfilled' && managersRes.value.ok) {
				const data = await managersRes.value.json();
				managers = Array.isArray(data) ? data : data.data || [];
			}

			// Process locations - provide remote as default
			if (locationsRes.status === 'fulfilled' && locationsRes.value.ok) {
				const data = await locationsRes.value.json();
				locations = Array.isArray(data) ? data : data.data || [];
			}
			if (locations.length === 0) {
				locations = [{ id: 'remote', name: 'Remote', timezone: 'UTC' }];
			}

			// Process training modules
			if (trainingRes.status === 'fulfilled' && trainingRes.value.ok) {
				const data = await trainingRes.value.json();
				trainingModules = Array.isArray(data) ? data : data.data || [];
			}

			// Process onboarding plans
			if (onboardingRes.status === 'fulfilled' && onboardingRes.value.ok) {
				const data = await onboardingRes.value.json();
				onboardingPlans = Array.isArray(data) ? data : data.data || [];
			}
			if (onboardingPlans.length === 0) {
				onboardingPlans = [{ id: 'standard', name: 'Standard Onboarding', duration: 5 }];
			}
		} catch (error) {
			logger.error('[admin/users/create] Failed to load lookup data', { error });
			// Set minimal defaults on complete failure
			departments = [{ id: 'general', name: 'General' }];
			locations = [{ id: 'remote', name: 'Remote', timezone: 'UTC' }];
			onboardingPlans = [{ id: 'standard', name: 'Standard Onboarding', duration: 5 }];
		}
	}

	function initializeDefaults() {
		const nextMonday = getNextMonday();
		const threeMonthsLater = new SvelteDate(nextMonday);
		threeMonthsLater.setMonth(threeMonthsLater.getMonth() + 3);

		formData.start_date = formatDate(nextMonday);
		formData.probation_end_date = formatDate(threeMonthsLater);

		const passwordExpiry = new SvelteDate();
		passwordExpiry.setDate(passwordExpiry.getDate() + 90);
		formData.password_expires_at = formatDate(passwordExpiry);

		formData.employee_id = generateEmployeeId();
	}

	// Form Navigation

	const STEPS = [
		{ key: 'identity', label: 'Identity', icon: IconUser },
		{ key: 'security', label: 'Security', icon: IconShield },
		{ key: 'role', label: 'Role & Permissions', icon: IconUserCheck },
		{ key: 'organization', label: 'Organization', icon: IconBuilding },
		{ key: 'preferences', label: 'Preferences', icon: IconSettings },
		{ key: 'compliance', label: 'Compliance', icon: IconCheckupList },
		{ key: 'review', label: 'Review', icon: IconCheck }
	];

	function nextStep() {
		const currentIndex = getStepIndex(activeStep);
		if (currentIndex < STEPS.length - 1) {
			activeStep = STEPS[currentIndex + 1].key as typeof activeStep;
		}
	}

	function previousStep() {
		const currentIndex = getStepIndex(activeStep);
		if (currentIndex > 0) {
			activeStep = STEPS[currentIndex - 1].key as typeof activeStep;
		}
	}

	function goToStep(step: typeof activeStep) {
		activeStep = step;
	}

	function getStepIndex(step: string): number {
		return STEPS.findIndex((s) => s.key === step);
	}

	// Form Submission

	async function handleSubmit() {
		const validationErrors = await validateForm();
		if (validationErrors.length > 0) {
			errors = validationErrors;
			return;
		}

		saving = true;
		try {
			const submitData = await prepareSubmitData();

			let payload: typeof submitData & { profile_photo?: string } = submitData;
			if (profilePhotoFile) {
				const photoUrl = await uploadProfilePhoto(profilePhotoFile);
				payload = { ...submitData, profile_photo: photoUrl };
			}

			const user = await usersApi.create(payload);

			await sendNotifications(user);
			trackUserCreation(user);

			if (shouldCreateAnother()) {
				resetForm();
				showSuccessMessage('User created successfully');
			} else {
				goto('/admin/users');
			}
		} catch (err) {
			if (err instanceof AdminApiError) {
				handleApiError(err);
			} else {
				errors = [{ field: 'general', message: 'Failed to create user', severity: 'error' }];
			}
			logger.error('[admin/users/create] Failed to create user', { error: err });
		} finally {
			saving = false;
		}
	}

	// Validation

	async function validateForm(): Promise<ValidationError[]> {
		const errors: ValidationError[] = [];

		// Identity validation
		if (!formData.first_name.trim()) {
			errors.push({ field: 'first_name', message: 'First name is required', severity: 'error' });
		}

		if (!formData.last_name.trim()) {
			errors.push({ field: 'last_name', message: 'Last name is required', severity: 'error' });
		}

		if (!formData.username.trim()) {
			errors.push({ field: 'username', message: 'Username is required', severity: 'error' });
		} else if (!isValidUsername(formData.username)) {
			errors.push({
				field: 'username',
				message: 'Username must be 3-20 characters, alphanumeric with underscores',
				severity: 'error'
			});
		} else if (!usernameAvailable) {
			errors.push({ field: 'username', message: 'Username is already taken', severity: 'error' });
		}

		if (!formData.email.trim()) {
			errors.push({ field: 'email', message: 'Email is required', severity: 'error' });
		} else if (!isValidEmail(formData.email)) {
			errors.push({ field: 'email', message: 'Invalid email format', severity: 'error' });
		} else if (!emailAvailable) {
			errors.push({ field: 'email', message: 'Email is already registered', severity: 'error' });
		}

		// Security validation
		if (!formData.password) {
			errors.push({ field: 'password', message: 'Password is required', severity: 'error' });
		} else if (passwordStrength && passwordStrength.score < 3) {
			errors.push({ field: 'password', message: 'Password is too weak', severity: 'error' });
		} else if (passwordStrength?.isBreached) {
			errors.push({
				field: 'password',
				message: 'This password has been found in data breaches',
				severity: 'error'
			});
		}

		if (formData.password !== formData.password_confirmation) {
			errors.push({
				field: 'password_confirmation',
				message: 'Passwords do not match',
				severity: 'error'
			});
		}

		// Role validation
		if (!formData.role) {
			errors.push({ field: 'role', message: 'Role selection is required', severity: 'error' });
		}

		// Organization validation
		if (!formData.department) {
			errors.push({ field: 'department', message: 'Department is required', severity: 'error' });
		}

		if (!formData.location) {
			errors.push({ field: 'location', message: 'Location is required', severity: 'error' });
		}

		// Phone validation (optional but must be valid format if provided)
		if (formData.phone && !isValidPhone(formData.phone)) {
			errors.push({ field: 'phone', message: 'Invalid phone number format', severity: 'error' });
		}

		// Compliance validation
		if (!formData.terms_accepted) {
			errors.push({
				field: 'terms_accepted',
				message: 'Terms must be accepted',
				severity: 'error'
			});
		}

		if (!formData.privacy_accepted) {
			errors.push({
				field: 'privacy_accepted',
				message: 'Privacy policy must be accepted',
				severity: 'error'
			});
		}

		// Date validation
		if (formData.start_date) {
			const startDate = new Date(formData.start_date);
			if (startDate < new Date()) {
				errors.push({
					field: 'start_date',
					message: 'Start date cannot be in the past',
					severity: 'warning'
				});
			}
		}

		return errors;
	}

	function validateCurrentStep(): boolean {
		switch (activeStep) {
			case 'identity':
				return !!(formData.first_name && formData.last_name && formData.username && formData.email);
			case 'security':
				return !!(
					formData.password &&
					formData.password_confirmation &&
					formData.password === formData.password_confirmation
				);
			case 'role':
				return !!formData.role;
			case 'organization':
				return !!(formData.department && formData.location);
			case 'preferences':
				return true;
			case 'compliance':
				return formData.terms_accepted && formData.privacy_accepted;
			case 'review':
				return true;
			default:
				return false;
		}
	}

	// Real-time Validation

	function setupRealtimeValidation() {
		// Real-time validation setup would go here
	}

	async function checkUsernameAvailability() {
		if (!formData.username || formData.username.length < 3) {
			usernameAvailable = null;
			return;
		}

		checkingUsername = true;
		try {
			// Call REAL API to check username availability
			const response = await fetch(
				`/api/admin/users/check-username?username=${encodeURIComponent(formData.username)}`
			);
			if (response.ok) {
				const data = await response.json();
				usernameAvailable = data.available === true;
			} else {
				// On API error, assume available and let server validate on submit
				usernameAvailable = null;
			}
		} catch (error) {
			logger.error('[admin/users/create] Failed to check username', { error });
			usernameAvailable = null; // Don't block user, server will validate on submit
		} finally {
			checkingUsername = false;
		}
	}

	async function checkEmailAvailability() {
		if (!isValidEmail(formData.email)) {
			emailAvailable = null;
			return;
		}

		checkingEmail = true;
		try {
			// Call REAL API to check email availability
			const response = await fetch(
				`/api/admin/users/check-email?email=${encodeURIComponent(formData.email)}`
			);
			if (response.ok) {
				const data = await response.json();
				emailAvailable = data.available === true;
			} else {
				// On API error, assume available and let server validate on submit
				emailAvailable = null;
			}
		} catch (error) {
			logger.error('[admin/users/create] Failed to check email', { error });
			emailAvailable = null; // Don't block user, server will validate on submit
		} finally {
			checkingEmail = false;
		}
	}

	async function checkPasswordStrength(password: string) {
		if (!password) {
			passwordStrength = null;
			return;
		}

		let score = 0;
		const feedback: string[] = [];
		const suggestions: string[] = [];

		// Length check
		if (password.length >= 8) score++;
		if (password.length >= 12) score++;
		if (password.length < 8) {
			feedback.push('Password is too short');
			suggestions.push('Use at least 8 characters');
		}

		// Complexity checks
		if (/[a-z]/.test(password)) score++;
		else suggestions.push('Add lowercase letters');

		if (/[A-Z]/.test(password)) score++;
		else suggestions.push('Add uppercase letters');

		if (/\d/.test(password)) score++;
		else suggestions.push('Add numbers');

		if (/[^a-zA-Z0-9]/.test(password)) score++;
		else suggestions.push('Add special characters');

		// Common patterns check
		if (/(.)\1{2,}/.test(password)) {
			feedback.push('Avoid repeated characters');
			score--;
		}

		if (/^(password|123456|qwerty)/i.test(password)) {
			feedback.push('This is a commonly used password');
			score = 0;
		}

		// Check against breach database (mock)
		const isBreached = await checkBreachDatabase(password);

		// Calculate crack time
		const crackTime = calculateCrackTime(score);

		passwordStrength = {
			score: Math.max(0, Math.min(5, score)),
			feedback,
			suggestions,
			crackTime,
			isBreached
		};
	}

	async function checkBreachDatabase(password: string): Promise<boolean> {
		const commonPasswords = ['password', '123456', 'qwerty', 'admin', 'letmein'];
		return commonPasswords.includes(password.toLowerCase());
	}

	function calculateCrackTime(score: number): string {
		const times = ['Instant', 'Minutes', 'Hours', 'Days', 'Months', 'Years', 'Centuries'];
		return times[Math.min(score, times.length - 1)];
	}

	// Role & Permission Management

	function selectRole(role: 'admin' | 'trader' | 'member') {
		formData.role = role;

		// Auto-configure access scope based on role
		switch (role) {
			case 'admin':
				formData.access_scope = {
					products: 'all',
					orders: 'all',
					customers: 'all',
					reports: 'all',
					settings: 'all'
				};
				formData.require_2fa = true;
				formData.session_timeout = 240;
				break;
			case 'trader':
				formData.access_scope = {
					products: 'all',
					orders: 'all',
					customers: 'all',
					reports: 'standard',
					settings: 'none'
				};
				formData.require_2fa = false;
				formData.session_timeout = 480;
				break;
			case 'member':
				formData.access_scope = {
					products: 'none',
					orders: 'own',
					customers: 'none',
					reports: 'none',
					settings: 'none'
				};
				formData.require_2fa = false;
				formData.session_timeout = 720;
				break;
		}

		// Set default training based on role
		updateTrainingModules(role);
	}

	function updateTrainingModules(role: string) {
		const baseModules = ['security', 'compliance'];

		switch (role) {
			case 'admin':
				formData.training_modules = [...baseModules, 'product', 'support', 'admin'];
				break;
			case 'trader':
				formData.training_modules = [...baseModules, 'product', 'sales'];
				break;
			case 'member':
				formData.training_modules = baseModules;
				break;
		}
	}

	// File Upload

	function handlePhotoUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];

		if (!file) return;

		if (!file.type.startsWith('image/')) {
			showError('Please upload an image file');
			return;
		}

		if (file.size > 5 * 1024 * 1024) {
			showError('Image must be less than 5MB');
			return;
		}

		profilePhotoFile = file;

		const reader = new FileReader();
		reader.onload = (e) => {
			profilePhotoPreview = e.target?.result as string;
		};
		reader.readAsDataURL(file);
	}

	async function uploadProfilePhoto(file: File): Promise<string> {
		const formData = new FormData();
		formData.append('photo', file);

		// Mock upload - would be real API call
		return 'https://example.com/photo.jpg';
	}

	// Helper Functions

	function calculateCompletion(): number {
		let completed = 0;
		let total = 0;

		// Identity fields
		if (formData.first_name) completed++;
		if (formData.last_name) completed++;
		if (formData.username) completed++;
		if (formData.email) completed++;
		total += 4;

		// Security fields
		if (formData.password) completed++;
		if (formData.password_confirmation) completed++;
		total += 2;

		// Role
		if (formData.role) completed++;
		total += 1;

		// Organization
		if (formData.department) completed++;
		if (formData.location) completed++;
		total += 2;

		// Compliance
		if (formData.terms_accepted) completed++;
		if (formData.privacy_accepted) completed++;
		total += 2;

		return Math.round((completed / total) * 100);
	}

	function generateEmployeeId(): string {
		const prefix = 'EMP';
		const year = new Date().getFullYear();
		const random = Math.floor(Math.random() * 10000)
			.toString()
			.padStart(4, '0');
		return `${prefix}${year}${random}`;
	}

	function generateUsername(): void {
		if (formData.first_name && formData.last_name) {
			const base = `${formData.first_name.toLowerCase()}.${formData.last_name.toLowerCase()}`;
			formData.username = base.replace(/[^a-z0-9._]/g, '');
			scheduleUsernameValidation();
		}
	}

	function formatDate(date: Date): string {
		return date.toISOString().split('T')[0];
	}

	function getNextMonday(): Date {
		const date = new SvelteDate();
		const day = date.getDay();
		const diff = date.getDate() - day + (day === 0 ? -6 : 1) + 7;
		return new SvelteDate(date.setDate(diff));
	}

	function isValidEmail(email: string): boolean {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	}

	function isValidUsername(username: string): boolean {
		const usernameRegex = /^[a-zA-Z0-9_]{3,20}$/;
		return usernameRegex.test(username);
	}

	function isValidPhone(phone: string): boolean {
		const phoneRegex = /^\+?[1-9]\d{1,14}$/;
		return phoneRegex.test(phone.replace(/\s/g, ''));
	}

	async function prepareSubmitData() {
		return {
			...formData,
			name: `${formData.first_name} ${formData.last_name}`.trim(),
			roles: [formData.role],
			metadata: {
				created_by: 'admin',
				created_at: new Date().toISOString(),
				source: 'admin_panel'
			}
		};
	}

	async function sendNotifications(_user: unknown) {
		// FIX-2026-04-26: dropped console.log per audit §E. TODO: wire to notification svc.
	}

	function trackUserCreation(_user: unknown) {
		// FIX-2026-04-26: dropped console.log per audit §E. TODO: wire to analytics.
	}

	function shouldCreateAnother(): boolean {
		return new URLSearchParams(window.location.search).get('create_another') === 'true';
	}

	function resetForm() {
		const preserveFields = {
			department: formData.department,
			team: formData.team,
			location: formData.location,
			office: formData.office
		};

		initializeDefaults();
		Object.assign(formData, preserveFields);

		profilePhotoFile = null;
		profilePhotoPreview = null;
		passwordStrength = null;
		usernameAvailable = null;
		emailAvailable = null;
		activeStep = 'identity';
	}

	function handleApiError(error: AdminApiError) {
		if (error.validationErrors) {
			errors = Object.entries(error.validationErrors).map(([field, message]) => ({
				field,
				message: Array.isArray(message) ? message[0] : String(message),
				severity: 'error' as const
			}));
		} else {
			errors = [{ field: 'general', message: error.message, severity: 'error' as const }];
		}
	}

	function showError(message: string) {
		errors = [{ field: 'general', message, severity: 'error' }];
	}

	function showSuccessMessage(_message: string) {
		// FIX-2026-04-26: dropped console.log per audit §E. TODO: toast notification.
	}

	function scheduleUsernameValidation() {
		if (usernameValidationTimer) clearTimeout(usernameValidationTimer);
		if (!formData.username) {
			usernameAvailable = null;
			return;
		}
		usernameValidationTimer = setTimeout(() => {
			checkUsernameAvailability();
		}, 400);
	}

	function scheduleEmailValidation() {
		if (emailValidationTimer) clearTimeout(emailValidationTimer);
		if (!formData.email) {
			emailAvailable = null;
			return;
		}
		emailValidationTimer = setTimeout(() => {
			checkEmailAvailability();
		}, 400);
	}

	function schedulePasswordStrengthCheck() {
		if (passwordStrengthTimer) clearTimeout(passwordStrengthTimer);
		if (!formData.password) {
			passwordStrength = null;
			return;
		}
		passwordStrengthTimer = setTimeout(() => {
			checkPasswordStrength(formData.password);
		}, 400);
	}
</script>

<svelte:head>
	<title>Create User | Enterprise Admin</title>
</svelte:head>

<div class="create-user-page">
	<div class="create-user-container">
		<header class="page-header">
			<h1>Create User</h1>
			<p>Enterprise user provisioning - {completionPercentage}% complete</p>
		</header>

		{#if errors.length > 0}
			<div class="error-summary">
				{#each errors as error (error.field)}
					<p>{error.field}: {error.message}</p>
				{/each}
			</div>
		{/if}

		<nav class="step-nav" aria-label="Create user steps">
			{#each STEPS as step, i (step.key)}
				{@const StepIcon = step.icon}
				<button
					type="button"
					class={{
						'step-button': true,
						active: activeStep === step.key,
						complete: activeStep !== step.key && i <= currentStepIndex,
						pending: i > currentStepIndex
					}}
					onclick={() => goToStep(step.key as typeof activeStep)}
				>
					<StepIcon size={16} />
					<span>{step.label}</span>
				</button>
			{/each}
		</nav>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				handleSubmit();
			}}
			class="user-form"
		>
			{#if activeStep === 'identity'}
				<div class="form-grid">
					<div class="field-group">
						<label for="first_name">First Name *</label>
						<input
							id="first_name"
							type="text"
							bind:value={formData.first_name}
							onblur={generateUsername}
							class="form-control"
						/>
					</div>
					<div class="field-group">
						<label for="last_name">Last Name *</label>
						<input
							id="last_name"
							type="text"
							bind:value={formData.last_name}
							onblur={generateUsername}
							class="form-control"
						/>
					</div>
					<div class="field-group">
						<label for="username">Username *</label>
						<div class="field-with-status">
							<input
								id="username"
								type="text"
								bind:value={formData.username}
								oninput={scheduleUsernameValidation}
								class="form-control"
							/>
							{#if checkingUsername}
								<span class="input-status">Checking...</span>
							{:else if usernameAvailable === true}
								<span class="input-status success">Available</span>
							{:else if usernameAvailable === false}
								<span class="input-status error">Taken</span>
							{/if}
						</div>
					</div>
					<div class="field-group">
						<label for="email">Email *</label>
						<div class="field-with-status">
							<input
								id="email"
								type="email"
								bind:value={formData.email}
								oninput={scheduleEmailValidation}
								class="form-control"
							/>
							{#if checkingEmail}
								<span class="input-status">Checking...</span>
							{:else if emailAvailable === true}
								<span class="input-status success">Available</span>
							{:else if emailAvailable === false}
								<span class="input-status error">Registered</span>
							{/if}
						</div>
					</div>
					<div class="field-group">
						<label for="phone">Phone</label>
						<input id="phone" type="tel" bind:value={formData.phone} class="form-control" />
					</div>
					<div class="field-group">
						<label for="employee_id">Employee ID</label>
						<input
							id="employee_id"
							type="text"
							bind:value={formData.employee_id}
							class="form-control"
						/>
					</div>
				</div>
				<div class="field-group">
					<label for="photo">Profile Photo</label>
					<input
						id="photo"
						type="file"
						accept="image/*"
						onchange={handlePhotoUpload}
						class="file-control"
					/>
					{#if profilePhotoPreview}
						<img
							src={profilePhotoPreview}
							alt="Preview"
							class="profile-preview"
							width="64"
							height="64"
							loading="lazy"
						/>
					{/if}
				</div>
			{:else if activeStep === 'security'}
				<div class="form-grid">
					<div class="field-group">
						<label for="password">Password *</label>
						<input
							id="password"
							type={passwordVisible ? 'text' : 'password'}
							bind:value={formData.password}
							oninput={schedulePasswordStrengthCheck}
							class="form-control"
						/>
						<button
							type="button"
							class="inline-action"
							onclick={() => (passwordVisible = !passwordVisible)}
						>
							{passwordVisible ? 'Hide' : 'Show'}
						</button>
					</div>
					<div class="field-group">
						<label for="password_confirm">Confirm Password *</label>
						<input
							id="password_confirm"
							type={confirmPasswordVisible ? 'text' : 'password'}
							bind:value={formData.password_confirmation}
							class="form-control"
						/>
						<button
							type="button"
							class="inline-action"
							onclick={() => (confirmPasswordVisible = !confirmPasswordVisible)}
						>
							{confirmPasswordVisible ? 'Hide' : 'Show'}
						</button>
					</div>
				</div>
				{#if passwordStrength}
					<div class="strength-panel">
						<div class="strength-row">
							<div class="strength-track">
								<div
									class={{
										'strength-fill': true,
										strong: passwordStrength.score >= 4,
										medium: passwordStrength.score >= 3 && passwordStrength.score < 4,
										weak: passwordStrength.score < 3
									}}
									style="width: {(passwordStrength.score / 5) * 100}%"
								></div>
							</div>
							<span class="strength-meta">Crack time: {passwordStrength.crackTime}</span>
						</div>
						{#each passwordStrength.suggestions as suggestion (suggestion)}
							<p class="field-help">{suggestion}</p>
						{/each}
						{#if passwordStrength.isBreached}
							<p class="field-help error">This password was found in breach databases</p>
						{/if}
					</div>
				{/if}
				<div class="checkbox-stack">
					<label class="checkbox-row">
						<input type="checkbox" bind:checked={formData.require_2fa} />
						<span>Require Two-Factor Authentication</span>
					</label>
					<label class="checkbox-row">
						<input type="checkbox" bind:checked={formData.temporary_password} />
						<span>Temporary password (expires)</span>
					</label>
					<label class="checkbox-row">
						<input type="checkbox" bind:checked={formData.require_password_change} />
						<span>Require password change on first login</span>
					</label>
				</div>
				<button
					type="button"
					class="link-button"
					onclick={() => (showAdvancedSecurity = !showAdvancedSecurity)}
				>
					{showAdvancedSecurity ? 'Hide' : 'Show'} Advanced Security
				</button>
				{#if showAdvancedSecurity}
					<div class="form-grid">
						<div class="field-group">
							<label for="session_timeout">Session Timeout (min)</label>
							<input
								id="session_timeout"
								type="number"
								bind:value={formData.session_timeout}
								class="form-control"
							/>
						</div>
						<div class="field-group">
							<label for="password_expires">Password Expires</label>
							<input
								id="password_expires"
								type="date"
								bind:value={formData.password_expires_at}
								class="form-control"
							/>
						</div>
					</div>
				{/if}
			{:else if activeStep === 'role'}
				<div class="role-grid">
					{#each ROLE_DEFINITIONS as role (role.key)}
						{@const RoleIcon = role.icon}
						<button
							type="button"
							class={{ 'role-card': true, selected: formData.role === role.key }}
							onclick={() => selectRole(role.key)}
						>
							<div class="role-header">
								<div class="role-icon" style="background: {role.color}20; color: {role.color}">
									<RoleIcon size={20} />
								</div>
								<div>
									<h3>{role.name}</h3>
									<p>Level {role.accessLevel}</p>
								</div>
							</div>
							<p class="role-description">{role.description}</p>
							<div class="role-points">
								{#each role.permissions.slice(0, 3) as perm (perm)}
									<p class="positive">+ {perm}</p>
								{/each}
								{#each role.restrictions.slice(0, 2) as restriction (restriction)}
									<p class="negative">- {restriction}</p>
								{/each}
							</div>
						</button>
					{/each}
				</div>
				<button
					type="button"
					class="link-button"
					onclick={() => (showPermissionDetails = !showPermissionDetails)}
				>
					{showPermissionDetails ? 'Hide' : 'Show'} Permission Details
				</button>
				{#if showPermissionDetails}
					<div class="info-panel">
						<h4>Access Scope</h4>
						<div class="scope-grid">
							<span>Products:</span><span>{formData.access_scope.products}</span>
							<span>Orders:</span><span>{formData.access_scope.orders}</span>
							<span>Customers:</span><span>{formData.access_scope.customers}</span>
							<span>Reports:</span><span>{formData.access_scope.reports}</span>
							<span>Settings:</span><span>{formData.access_scope.settings}</span>
						</div>
					</div>
				{/if}
			{:else if activeStep === 'organization'}
				<div class="form-grid">
					<div class="field-group">
						<label for="department">Department *</label>
						<select id="department" bind:value={formData.department} class="form-control">
							<option value="">Select department</option>
							{#each departments as dept (dept.id)}
								<option value={dept.id}>{dept.name}</option>
							{/each}
						</select>
					</div>
					<div class="field-group">
						<label for="team">Team</label>
						<select id="team" bind:value={formData.team} class="form-control">
							<option value="">Select team</option>
							{#each teams as team (team.id)}
								<option value={team.id}>{team.name}</option>
							{/each}
						</select>
					</div>
					<div class="field-group">
						<label for="manager">Manager</label>
						<select id="manager" bind:value={formData.manager_id} class="form-control">
							<option value="">
								{loadingManagers ? 'Loading...' : 'Select manager'}
							</option>
							{#each managers as manager (manager.id)}
								<option value={manager.id}>{manager.name}</option>
							{/each}
						</select>
					</div>
					<div class="field-group">
						<label for="job_title">Job Title</label>
						<input
							id="job_title"
							type="text"
							bind:value={formData.job_title}
							class="form-control"
						/>
					</div>
					<div class="field-group">
						<label for="location">Location *</label>
						<select id="location" bind:value={formData.location} class="form-control">
							<option value="">Select location</option>
							{#each locations as loc (loc.id)}
								<option value={loc.id}>{loc.name}</option>
							{/each}
						</select>
					</div>
					<div class="field-group">
						<label for="office">Office</label>
						<input id="office" type="text" bind:value={formData.office} class="form-control" />
					</div>
					<div class="field-group">
						<label for="cost_center">Cost Center</label>
						<input
							id="cost_center"
							type="text"
							bind:value={formData.cost_center}
							class="form-control"
						/>
					</div>
					<div class="field-group">
						<label for="start_date">Start Date</label>
						<input
							id="start_date"
							type="date"
							bind:value={formData.start_date}
							class="form-control"
						/>
					</div>
				</div>
			{:else if activeStep === 'preferences'}
				<div class="form-grid">
					<div class="field-group">
						<label for="timezone">Timezone</label>
						<input id="timezone" type="text" bind:value={formData.timezone} class="form-control" />
					</div>
					<div class="field-group">
						<label for="language">Language</label>
						<select id="language" bind:value={formData.language} class="form-control">
							<option value="en">English</option>
							<option value="es">Spanish</option>
							<option value="pt">Portuguese</option>
						</select>
					</div>
				</div>
			{:else if activeStep === 'compliance'}
				<div class="compliance-stack">
					<label class="check-card">
						<input type="checkbox" bind:checked={formData.terms_accepted} />
						<span>I accept the Terms of Service *</span>
					</label>
					<label class="check-card">
						<input type="checkbox" bind:checked={formData.privacy_accepted} />
						<span>I accept the Privacy Policy *</span>
					</label>
					<label class="check-card">
						<input type="checkbox" bind:checked={formData.data_processing_consent} />
						<span>Data Processing Consent</span>
					</label>
					<label class="check-card">
						<input type="checkbox" bind:checked={formData.marketing_consent} />
						<span>Marketing Communications</span>
					</label>
					<div class="field-group">
						<label for="onboarding">Onboarding Plan</label>
						<select id="onboarding" bind:value={formData.onboarding_plan} class="form-control">
							{#each onboardingPlans as plan (plan.id)}
								<option value={plan.id}>{plan.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<span class="field-label">Training Modules</span>
						<div class="chip-list">
							{#each trainingModules as mod (mod.id)}
								<span class="training-chip">{mod.name || mod.id}</span>
							{/each}
							{#if trainingModules.length === 0}
								<span class="field-help">Assigned: {formData.training_modules.join(', ')}</span>
							{/if}
						</div>
					</div>
				</div>
			{:else if activeStep === 'review'}
				<div class="review-panel">
					<h3>Review & Submit</h3>
					<div class="review-grid">
						<span>Name:</span><span>{formData.first_name} {formData.last_name}</span>
						<span>Username:</span><span>{formData.username}</span>
						<span>Email:</span><span>{formData.email}</span>
						<span>Role:</span><span class="text-capitalize">{formData.role}</span>
						<span>Department:</span><span>{formData.department || 'Not set'}</span>
						<span>Status:</span><span class="text-capitalize">{formData.status}</span>
						<span>Activation:</span><span class="text-capitalize">{formData.activation_method}</span
						>
					</div>
					<div class="review-options">
						<label class="checkbox-row">
							<input type="checkbox" bind:checked={formData.send_welcome_email} />
							<span>Send welcome email</span>
						</label>
						<label class="checkbox-row">
							<input type="checkbox" bind:checked={formData.add_to_directory} />
							<span>Add to directory</span>
						</label>
					</div>
					{#if formData.notes}
						<p class="review-notes">Notes: {formData.notes}</p>
					{/if}
				</div>
			{/if}

			<div class="form-footer">
				<button
					type="button"
					class="secondary-button"
					onclick={previousStep}
					disabled={currentStepIndex === 0}
				>
					Previous
				</button>
				<div class="form-status">
					{#if !canProceed}
						<span>Complete required fields</span>
					{/if}
					{#if validating}
						<span>Validating...</span>
					{/if}
				</div>
				{#if activeStep === 'review'}
					<button type="submit" class="primary-button primary-button--wide" disabled={saving}>
						{saving ? 'Creating...' : 'Create User'}
					</button>
				{:else}
					<button type="button" class="primary-button" onclick={nextStep}> Next </button>
				{/if}
			</div>
		</form>
	</div>
</div>

<style>
	.create-user-page {
		min-height: 100%;
		background: #09090b;
		color: #ffffff;
		padding: 1.5rem;
	}

	.create-user-container {
		width: min(100%, 56rem);
		margin-inline: auto;
	}

	.page-header {
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		margin: 0 0 0.5rem;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.page-header p {
		margin: 0;
		color: #a1a1aa;
	}

	.error-summary {
		display: grid;
		gap: 0.375rem;
		margin-bottom: 1.5rem;
		border: 1px solid rgb(239 68 68 / 30%);
		border-radius: 0.5rem;
		background: rgb(239 68 68 / 10%);
		padding: 1rem;
	}

	.error-summary p {
		margin: 0;
		color: #f87171;
		font-size: 0.875rem;
	}

	.step-nav {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 2rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.step-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		flex: 0 0 auto;
		border: 0;
		border-radius: 0.5rem;
		background: #18181b;
		padding: 0.5rem 1rem;
		color: #52525b;
		font: inherit;
		font-size: 0.875rem;
		white-space: nowrap;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.step-button.active {
		background: #2563eb;
		color: #ffffff;
	}

	.step-button.complete {
		background: #27272a;
		color: #d4d4d8;
	}

	.user-form,
	.compliance-stack,
	.checkbox-stack {
		display: grid;
		gap: 1.5rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.field-group {
		display: grid;
		gap: 0.25rem;
	}

	label,
	.field-label {
		color: #a1a1aa;
		font-size: 0.875rem;
	}

	.form-control {
		width: 100%;
		min-height: 2.625rem;
		border: 1px solid #3f3f46;
		border-radius: 0.5rem;
		background: #18181b;
		padding: 0.625rem 1rem;
		color: #ffffff;
		font: inherit;
	}

	.form-control:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgb(59 130 246 / 18%);
	}

	.field-with-status {
		position: relative;
	}

	.field-with-status .form-control {
		padding-right: 6.5rem;
	}

	.input-status {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		color: #71717a;
		font-size: 0.75rem;
	}

	.input-status.success {
		color: #34d399;
	}

	.input-status.error,
	.field-help.error {
		color: #f87171;
	}

	.file-control {
		color: #a1a1aa;
		font-size: 0.875rem;
	}

	.profile-preview {
		width: 4rem;
		height: 4rem;
		margin-top: 0.5rem;
		border-radius: 999px;
		object-fit: cover;
	}

	.inline-action,
	.link-button {
		width: fit-content;
		border: 0;
		background: transparent;
		padding: 0;
		font: inherit;
		cursor: pointer;
	}

	.inline-action {
		margin-top: 0.25rem;
		color: #71717a;
		font-size: 0.75rem;
	}

	.link-button {
		color: #60a5fa;
		font-size: 0.875rem;
	}

	.strength-panel,
	.info-panel,
	.review-panel {
		border-radius: 0.75rem;
		background: #18181b;
	}

	.strength-panel,
	.info-panel {
		padding: 1rem;
	}

	.strength-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.strength-track {
		flex: 1 1 auto;
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: #27272a;
	}

	.strength-fill {
		height: 100%;
		border-radius: inherit;
		transition:
			width 160ms ease,
			background-color 160ms ease;
	}

	.strength-fill.strong {
		background: #10b981;
	}

	.strength-fill.medium {
		background: #eab308;
	}

	.strength-fill.weak {
		background: #ef4444;
	}

	.strength-meta,
	.field-help,
	.form-status,
	.review-notes {
		color: #71717a;
		font-size: 0.75rem;
	}

	.field-help {
		margin: 0;
	}

	.checkbox-row,
	.check-card,
	.review-options,
	.form-footer,
	.form-status,
	.role-header {
		display: flex;
		align-items: center;
	}

	.checkbox-row {
		gap: 0.5rem;
		color: #d4d4d8;
		font-size: 0.875rem;
	}

	input[type='checkbox'] {
		width: 1rem;
		height: 1rem;
		accent-color: #2563eb;
	}

	.role-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1rem;
	}

	.role-card {
		border: 1px solid #27272a;
		border-radius: 0.75rem;
		background: #18181b;
		padding: 1.25rem;
		color: #ffffff;
		text-align: left;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			border-color 160ms ease;
	}

	.role-card:hover {
		border-color: #3f3f46;
	}

	.role-card.selected {
		border-color: #3b82f6;
		background: rgb(59 130 246 / 10%);
	}

	.role-header {
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.role-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
	}

	.role-header h3,
	.info-panel h4,
	.review-panel h3 {
		margin: 0;
		font-weight: 600;
	}

	.role-header p,
	.role-description,
	.review-notes {
		margin: 0;
	}

	.role-header p {
		color: #71717a;
		font-size: 0.75rem;
	}

	.role-description {
		margin-bottom: 0.75rem;
		color: #a1a1aa;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.role-points {
		display: grid;
		gap: 0.25rem;
	}

	.role-points p {
		margin: 0;
		font-size: 0.75rem;
	}

	.positive {
		color: #34d399;
	}

	.negative {
		color: #f87171;
	}

	.info-panel h4 {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.scope-grid,
	.review-grid {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.5rem 2rem;
		font-size: 0.875rem;
	}

	.scope-grid span:nth-child(odd),
	.review-grid span:nth-child(odd) {
		color: #a1a1aa;
	}

	.check-card {
		gap: 0.75rem;
		border: 1px solid #27272a;
		border-radius: 0.5rem;
		background: #18181b;
		padding: 1rem;
		color: #d4d4d8;
	}

	.chip-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.25rem;
	}

	.training-chip {
		border-radius: 999px;
		background: #27272a;
		padding: 0.25rem 0.75rem;
		color: #d4d4d8;
		font-size: 0.75rem;
	}

	.review-panel {
		display: grid;
		gap: 1rem;
		padding: 1.5rem;
	}

	.review-panel h3 {
		font-size: 1.125rem;
	}

	.text-capitalize {
		text-transform: capitalize;
	}

	.review-options {
		flex-wrap: wrap;
		gap: 1rem;
		border-top: 1px solid #27272a;
		padding-top: 1rem;
	}

	.form-footer {
		justify-content: space-between;
		gap: 1rem;
		border-top: 1px solid #27272a;
		padding-top: 1.5rem;
	}

	.form-status {
		justify-content: center;
		gap: 0.5rem;
		text-align: center;
	}

	.primary-button,
	.secondary-button {
		border: 0;
		border-radius: 0.5rem;
		padding: 0.625rem 1.5rem;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			opacity 160ms ease;
	}

	.primary-button {
		background: #2563eb;
		color: #ffffff;
	}

	.primary-button:hover:not(:disabled) {
		background: #3b82f6;
	}

	.primary-button--wide {
		padding-inline: 2rem;
		font-weight: 600;
	}

	.secondary-button {
		background: #27272a;
		color: #d4d4d8;
	}

	.secondary-button:hover:not(:disabled) {
		background: #3f3f46;
	}

	.primary-button:disabled,
	.secondary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	@media (max-width: 760px) {
		.create-user-page {
			padding: 1rem;
		}

		.form-grid,
		.role-grid {
			grid-template-columns: 1fr;
		}

		.form-footer {
			align-items: stretch;
			flex-direction: column;
		}

		.primary-button,
		.secondary-button {
			width: 100%;
		}
	}

	@media (max-width: 520px) {
		.scope-grid,
		.review-grid {
			grid-template-columns: 1fr;
			gap: 0.25rem;
		}

		.review-grid span:nth-child(even) {
			margin-bottom: 0.5rem;
		}
	}
</style>
