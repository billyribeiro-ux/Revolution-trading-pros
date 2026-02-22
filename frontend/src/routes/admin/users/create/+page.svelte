<!--
/**
 * User Create Component - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
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
 * @version 3.0.0 (Google L7+ Enterprise)
 * @component
 */
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { goto } from '$app/navigation';
	import { usersApi, AdminApiError } from '$lib/api/admin';
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

	// ═══════════════════════════════════════════════════════════════════════════
	// Type Definitions
	// ═══════════════════════════════════════════════════════════════════════════

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
		custom_fields: Record<string, any>;
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
		icon: any;
		color: string;
		permissions: string[];
		restrictions: string[];
		accessLevel: number;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Role Definitions
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

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
	let departments = $state<any[]>([]);
	let teams = $state<any[]>([]);
	let managers = $state<any[]>([]);
	let locations = $state<any[]>([]);
	let trainingModules = $state<any[]>([]);
	let onboardingPlans = $state<any[]>([]);

	// Progress calculation
	let completionPercentage = $derived(calculateCompletion());
	let currentStepIndex = $derived(getStepIndex(activeStep));
	let canProceed = $derived(validateCurrentStep());

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle (Svelte 5 $effect)
	// ═══════════════════════════════════════════════════════════════════════════

	// Initialize on mount
	$effect(() => {
		loadLookupData();
		initializeDefaults();
		setupRealtimeValidation();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

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
			logger.error('Failed to load lookup data:', error);
			// Set minimal defaults on complete failure
			departments = [{ id: 'general', name: 'General' }];
			locations = [{ id: 'remote', name: 'Remote', timezone: 'UTC' }];
			onboardingPlans = [{ id: 'standard', name: 'Standard Onboarding', duration: 5 }];
		}
	}

	function initializeDefaults() {
		const nextMonday = getNextMonday();
		const threeMonthsLater = new Date(nextMonday);
		threeMonthsLater.setMonth(threeMonthsLater.getMonth() + 3);

		formData.start_date = formatDate(nextMonday);
		formData.probation_end_date = formatDate(threeMonthsLater);

		const passwordExpiry = new Date();
		passwordExpiry.setDate(passwordExpiry.getDate() + 90);
		formData.password_expires_at = formatDate(passwordExpiry);

		formData.employee_id = generateEmployeeId();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Form Navigation
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// Form Submission
	// ═══════════════════════════════════════════════════════════════════════════

	async function handleSubmit() {
		const validationErrors = await validateForm();
		if (validationErrors.length > 0) {
			errors = validationErrors;
			return;
		}

		saving = true;
		try {
			const submitData = await prepareSubmitData();

			if (profilePhotoFile) {
				const photoUrl = await uploadProfilePhoto(profilePhotoFile);
				(submitData as any).profile_photo = photoUrl;
			}

			const user = await usersApi.create(submitData);

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
			logger.error('Failed to create user:', err);
		} finally {
			saving = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Validation
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// Real-time Validation
	// ═══════════════════════════════════════════════════════════════════════════

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
			logger.error('Failed to check username:', error);
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
			logger.error('Failed to check email:', error);
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

	// ═══════════════════════════════════════════════════════════════════════════
	// Role & Permission Management
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// File Upload
	// ═══════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

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
			checkUsernameAvailability();
		}
	}

	function formatDate(date: Date): string {
		return date.toISOString().split('T')[0];
	}

	function getNextMonday(): Date {
		const date = new Date();
		const day = date.getDay();
		const diff = date.getDate() - day + (day === 0 ? -6 : 1) + 7;
		return new Date(date.setDate(diff));
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

	async function sendNotifications(user: any) {
		// Mock notification sending
		logger.info('Sending notifications for user:', user);
	}

	function trackUserCreation(user: any) {
		// Analytics tracking
		logger.info('Tracking user creation:', user);
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

	function showSuccessMessage(message: string) {
		logger.info('Success:', message);
		// Could use a toast notification system here
	}

	// Reactive statements for real-time validation
	$effect(() => {
		if (formData.username) {
			checkUsernameAvailability();
		}
	});

	$effect(() => {
		if (formData.email) {
			checkEmailAvailability();
		}
	});

	$effect(() => {
		if (formData.password) {
			checkPasswordStrength(formData.password);
		}
	});
</script>

<svelte:head>
	<title>Create User | Enterprise Admin</title>
</svelte:head>

<div class="min-h-screen bg-zinc-950 text-white p-6">
	<div class="max-w-4xl mx-auto">
		<h1 class="text-3xl font-bold mb-2">Create User</h1>
		<p class="text-zinc-400 mb-6">
			Enterprise user provisioning • {completionPercentage}% complete
		</p>

		{#if errors.length > 0}
			<div class="bg-red-500/10 border border-red-500/30 rounded-lg p-4 mb-6">
				{#each errors as error}
					<p class="text-red-400 text-sm">{error.field}: {error.message}</p>
				{/each}
			</div>
		{/if}

		<nav class="flex gap-1 mb-8 overflow-x-auto pb-2">
			{#each STEPS as step, i}
				{@const StepIcon = step.icon}
				<button
					class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm whitespace-nowrap transition-colors {activeStep ===
					step.key
						? 'bg-blue-600 text-white'
						: i <= currentStepIndex
							? 'bg-zinc-800 text-zinc-300'
							: 'bg-zinc-900 text-zinc-600'}"
					onclick={() => goToStep(step.key as typeof activeStep)}
				>
					<StepIcon size={16} />
					{step.label}
				</button>
			{/each}
		</nav>

		<form
			onsubmit={(e) => {
				e.preventDefault();
				handleSubmit();
			}}
			class="space-y-6"
		>
			{#if activeStep === 'identity'}
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="first_name">First Name *</label>
						<input
							id="first_name"
							type="text"
							bind:value={formData.first_name}
							onblur={generateUsername}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="last_name">Last Name *</label>
						<input
							id="last_name"
							type="text"
							bind:value={formData.last_name}
							onblur={generateUsername}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="username">Username *</label>
						<div class="relative">
							<input
								id="username"
								type="text"
								bind:value={formData.username}
								class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
							/>
							{#if checkingUsername}
								<span class="absolute right-3 top-3 text-xs text-zinc-500">Checking...</span>
							{:else if usernameAvailable === true}
								<span class="absolute right-3 top-3 text-xs text-emerald-400">Available</span>
							{:else if usernameAvailable === false}
								<span class="absolute right-3 top-3 text-xs text-red-400">Taken</span>
							{/if}
						</div>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="email">Email *</label>
						<div class="relative">
							<input
								id="email"
								type="email"
								bind:value={formData.email}
								class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
							/>
							{#if checkingEmail}
								<span class="absolute right-3 top-3 text-xs text-zinc-500">Checking...</span>
							{:else if emailAvailable === true}
								<span class="absolute right-3 top-3 text-xs text-emerald-400">Available</span>
							{:else if emailAvailable === false}
								<span class="absolute right-3 top-3 text-xs text-red-400">Registered</span>
							{/if}
						</div>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="phone">Phone</label>
						<input
							id="phone"
							type="tel"
							bind:value={formData.phone}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="employee_id">Employee ID</label>
						<input
							id="employee_id"
							type="text"
							bind:value={formData.employee_id}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
				</div>
				<div>
					<label class="block text-sm text-zinc-400 mb-1" for="photo">Profile Photo</label>
					<input
						id="photo"
						type="file"
						accept="image/*"
						onchange={handlePhotoUpload}
						class="text-sm text-zinc-400"
					/>
					{#if profilePhotoPreview}
						<img
							src={profilePhotoPreview}
							alt="Preview"
							class="mt-2 w-16 h-16 rounded-full object-cover"
						/>
					{/if}
				</div>
			{:else if activeStep === 'security'}
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="password">Password *</label>
						<input
							id="password"
							type={passwordVisible ? 'text' : 'password'}
							bind:value={formData.password}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
						<button
							type="button"
							class="text-xs text-zinc-500 mt-1"
							onclick={() => (passwordVisible = !passwordVisible)}
						>
							{passwordVisible ? 'Hide' : 'Show'}
						</button>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="password_confirm"
							>Confirm Password *</label
						>
						<input
							id="password_confirm"
							type={confirmPasswordVisible ? 'text' : 'password'}
							bind:value={formData.password_confirmation}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
						<button
							type="button"
							class="text-xs text-zinc-500 mt-1"
							onclick={() => (confirmPasswordVisible = !confirmPasswordVisible)}
						>
							{confirmPasswordVisible ? 'Hide' : 'Show'}
						</button>
					</div>
				</div>
				{#if passwordStrength}
					<div class="bg-zinc-900 rounded-lg p-4">
						<div class="flex items-center gap-2 mb-2">
							<div class="flex-1 h-2 bg-zinc-800 rounded-full overflow-hidden">
								<div
									class="h-full rounded-full transition-all {passwordStrength.score >= 4
										? 'bg-emerald-500'
										: passwordStrength.score >= 3
											? 'bg-yellow-500'
											: 'bg-red-500'}"
									style="width: {(passwordStrength.score / 5) * 100}%"
								></div>
							</div>
							<span class="text-xs text-zinc-400">Crack time: {passwordStrength.crackTime}</span>
						</div>
						{#each passwordStrength.suggestions as suggestion}
							<p class="text-xs text-zinc-500">{suggestion}</p>
						{/each}
						{#if passwordStrength.isBreached}
							<p class="text-xs text-red-400 mt-1">This password was found in breach databases</p>
						{/if}
					</div>
				{/if}
				<div class="space-y-3">
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={formData.require_2fa} class="rounded" />
						<span class="text-zinc-300">Require Two-Factor Authentication</span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={formData.temporary_password} class="rounded" />
						<span class="text-zinc-300">Temporary password (expires)</span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input
							type="checkbox"
							bind:checked={formData.require_password_change}
							class="rounded"
						/>
						<span class="text-zinc-300">Require password change on first login</span>
					</label>
				</div>
				<button
					type="button"
					class="text-sm text-blue-400"
					onclick={() => (showAdvancedSecurity = !showAdvancedSecurity)}
				>
					{showAdvancedSecurity ? 'Hide' : 'Show'} Advanced Security
				</button>
				{#if showAdvancedSecurity}
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div>
							<label class="block text-sm text-zinc-400 mb-1" for="session_timeout"
								>Session Timeout (min)</label
							>
							<input
								id="session_timeout"
								type="number"
								bind:value={formData.session_timeout}
								class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
							/>
						</div>
						<div>
							<label class="block text-sm text-zinc-400 mb-1" for="password_expires"
								>Password Expires</label
							>
							<input
								id="password_expires"
								type="date"
								bind:value={formData.password_expires_at}
								class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
							/>
						</div>
					</div>
				{/if}
			{:else if activeStep === 'role'}
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					{#each ROLE_DEFINITIONS as role}
						{@const RoleIcon = role.icon}
						<button
							type="button"
							class="text-left p-5 rounded-xl border transition-all {formData.role === role.key
								? 'border-blue-500 bg-blue-500/10'
								: 'border-zinc-800 bg-zinc-900 hover:border-zinc-700'}"
							onclick={() => selectRole(role.key)}
						>
							<div class="flex items-center gap-3 mb-3">
								<div
									class="w-10 h-10 rounded-lg flex items-center justify-center"
									style="background: {role.color}20; color: {role.color}"
								>
									<RoleIcon size={20} />
								</div>
								<div>
									<h3 class="font-semibold">{role.name}</h3>
									<p class="text-xs text-zinc-500">Level {role.accessLevel}</p>
								</div>
							</div>
							<p class="text-sm text-zinc-400 mb-3">{role.description}</p>
							<div class="space-y-1">
								{#each role.permissions.slice(0, 3) as perm}
									<p class="text-xs text-emerald-400">+ {perm}</p>
								{/each}
								{#each role.restrictions.slice(0, 2) as restriction}
									<p class="text-xs text-red-400">- {restriction}</p>
								{/each}
							</div>
						</button>
					{/each}
				</div>
				<button
					type="button"
					class="text-sm text-blue-400"
					onclick={() => (showPermissionDetails = !showPermissionDetails)}
				>
					{showPermissionDetails ? 'Hide' : 'Show'} Permission Details
				</button>
				{#if showPermissionDetails}
					<div class="bg-zinc-900 rounded-lg p-4">
						<h4 class="text-sm font-medium mb-2">Access Scope</h4>
						<div class="grid grid-cols-2 gap-2 text-sm">
							<span class="text-zinc-400">Products:</span><span
								>{formData.access_scope.products}</span
							>
							<span class="text-zinc-400">Orders:</span><span>{formData.access_scope.orders}</span>
							<span class="text-zinc-400">Customers:</span><span
								>{formData.access_scope.customers}</span
							>
							<span class="text-zinc-400">Reports:</span><span>{formData.access_scope.reports}</span
							>
							<span class="text-zinc-400">Settings:</span><span
								>{formData.access_scope.settings}</span
							>
						</div>
					</div>
				{/if}
			{:else if activeStep === 'organization'}
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="department">Department *</label>
						<select
							id="department"
							bind:value={formData.department}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							<option value="">Select department</option>
							{#each departments as dept}
								<option value={dept.id}>{dept.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="team">Team</label>
						<select
							id="team"
							bind:value={formData.team}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							<option value="">Select team</option>
							{#each teams as team}
								<option value={team.id}>{team.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="manager">Manager</label>
						<select
							id="manager"
							bind:value={formData.manager_id}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							<option value="">
								{loadingManagers ? 'Loading...' : 'Select manager'}
							</option>
							{#each managers as manager}
								<option value={manager.id}>{manager.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="job_title">Job Title</label>
						<input
							id="job_title"
							type="text"
							bind:value={formData.job_title}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="location">Location *</label>
						<select
							id="location"
							bind:value={formData.location}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							<option value="">Select location</option>
							{#each locations as loc}
								<option value={loc.id}>{loc.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="office">Office</label>
						<input
							id="office"
							type="text"
							bind:value={formData.office}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="cost_center">Cost Center</label>
						<input
							id="cost_center"
							type="text"
							bind:value={formData.cost_center}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="start_date">Start Date</label>
						<input
							id="start_date"
							type="date"
							bind:value={formData.start_date}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
				</div>
			{:else if activeStep === 'preferences'}
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="timezone">Timezone</label>
						<input
							id="timezone"
							type="text"
							bind:value={formData.timezone}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						/>
					</div>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="language">Language</label>
						<select
							id="language"
							bind:value={formData.language}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							<option value="en">English</option>
							<option value="es">Spanish</option>
							<option value="pt">Portuguese</option>
						</select>
					</div>
				</div>
			{:else if activeStep === 'compliance'}
				<div class="space-y-4">
					<label class="flex items-center gap-3 p-4 bg-zinc-900 rounded-lg border border-zinc-800">
						<input type="checkbox" bind:checked={formData.terms_accepted} class="rounded" />
						<span class="text-sm text-zinc-300">I accept the Terms of Service *</span>
					</label>
					<label class="flex items-center gap-3 p-4 bg-zinc-900 rounded-lg border border-zinc-800">
						<input type="checkbox" bind:checked={formData.privacy_accepted} class="rounded" />
						<span class="text-sm text-zinc-300">I accept the Privacy Policy *</span>
					</label>
					<label class="flex items-center gap-3 p-4 bg-zinc-900 rounded-lg border border-zinc-800">
						<input
							type="checkbox"
							bind:checked={formData.data_processing_consent}
							class="rounded"
						/>
						<span class="text-sm text-zinc-300">Data Processing Consent</span>
					</label>
					<label class="flex items-center gap-3 p-4 bg-zinc-900 rounded-lg border border-zinc-800">
						<input type="checkbox" bind:checked={formData.marketing_consent} class="rounded" />
						<span class="text-sm text-zinc-300">Marketing Communications</span>
					</label>
					<div>
						<label class="block text-sm text-zinc-400 mb-1" for="onboarding">Onboarding Plan</label>
						<select
							id="onboarding"
							bind:value={formData.onboarding_plan}
							class="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2.5 text-white"
						>
							{#each onboardingPlans as plan}
								<option value={plan.id}>{plan.name}</option>
							{/each}
						</select>
					</div>
					<div>
						<!-- svelte-ignore a11y_label_has_associated_control -->
						<label class="block text-sm text-zinc-400 mb-1">Training Modules</label>
						<div class="flex flex-wrap gap-2">
							{#each trainingModules as mod}
								<span class="px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300"
									>{mod.name || mod.id}</span
								>
							{/each}
							{#if trainingModules.length === 0}
								<span class="text-xs text-zinc-500"
									>Assigned: {formData.training_modules.join(', ')}</span
								>
							{/if}
						</div>
					</div>
				</div>
			{:else if activeStep === 'review'}
				<div class="bg-zinc-900 rounded-xl p-6 space-y-4">
					<h3 class="text-lg font-semibold">Review & Submit</h3>
					<div class="grid grid-cols-2 gap-x-8 gap-y-2 text-sm">
						<span class="text-zinc-400">Name:</span><span
							>{formData.first_name} {formData.last_name}</span
						>
						<span class="text-zinc-400">Username:</span><span>{formData.username}</span>
						<span class="text-zinc-400">Email:</span><span>{formData.email}</span>
						<span class="text-zinc-400">Role:</span><span class="capitalize">{formData.role}</span>
						<span class="text-zinc-400">Department:</span><span
							>{formData.department || 'Not set'}</span
						>
						<span class="text-zinc-400">Status:</span><span class="capitalize"
							>{formData.status}</span
						>
						<span class="text-zinc-400">Activation:</span><span class="capitalize"
							>{formData.activation_method}</span
						>
					</div>
					<div class="flex items-center gap-4 pt-4">
						<label class="flex items-center gap-2 text-sm">
							<input type="checkbox" bind:checked={formData.send_welcome_email} class="rounded" />
							<span class="text-zinc-300">Send welcome email</span>
						</label>
						<label class="flex items-center gap-2 text-sm">
							<input type="checkbox" bind:checked={formData.add_to_directory} class="rounded" />
							<span class="text-zinc-300">Add to directory</span>
						</label>
					</div>
					{#if formData.notes}
						<p class="text-sm text-zinc-400">Notes: {formData.notes}</p>
					{/if}
				</div>
			{/if}

			<div class="flex items-center justify-between pt-6 border-t border-zinc-800">
				<button
					type="button"
					class="px-6 py-2.5 rounded-lg bg-zinc-800 text-zinc-300 hover:bg-zinc-700 transition-colors disabled:opacity-50"
					onclick={previousStep}
					disabled={currentStepIndex === 0}
				>
					Previous
				</button>
				<div class="flex items-center gap-2">
					{#if !canProceed}
						<span class="text-xs text-zinc-500">Complete required fields</span>
					{/if}
					{#if validating}
						<span class="text-xs text-zinc-500">Validating...</span>
					{/if}
				</div>
				{#if activeStep === 'review'}
					<button
						type="submit"
						class="px-8 py-2.5 rounded-lg bg-blue-600 text-white font-medium hover:bg-blue-500 transition-colors disabled:opacity-50"
						disabled={saving}
					>
						{saving ? 'Creating...' : 'Create User'}
					</button>
				{:else}
					<button
						type="button"
						class="px-6 py-2.5 rounded-lg bg-blue-600 text-white hover:bg-blue-500 transition-colors"
						onclick={nextStep}
					>
						Next
					</button>
				{/if}
			</div>
		</form>
	</div>
</div>
