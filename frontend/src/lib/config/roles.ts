/**
 * Role & Permission Configuration - Microsoft Enterprise Pattern
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Centralized role and permission management for the entire application.
 * 
 * ROLE HIERARCHY:
 * 1. SUPERADMIN - Full system access, cannot be restricted
 * 2. ADMIN - Administrative access, can be restricted by permissions
 * 3. MEMBER - Authenticated user with purchased access
 * 4. USER - Basic authenticated user
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// Superadmin Configuration
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Superadmin emails - these users have FULL unrestricted access to everything.
 * They bypass all permission checks and can access any feature.
 * 
 * @security This list should be kept minimal and secure
 */
export const SUPERADMIN_EMAILS: readonly string[] = Object.freeze([
	'welberribeirodrums@gmail.com'
]);

/**
 * Developer emails - these users bypass email verification and get all memberships unlocked.
 * They experience the platform as a regular member (not admin) with full access for testing.
 * 
 * @security This list should be kept minimal and secure
 */
export const DEVELOPER_EMAILS: readonly string[] = Object.freeze([
	'welberribeirodrums@gmail.com'
]);

// ═══════════════════════════════════════════════════════════════════════════
// Role Definitions
// ═══════════════════════════════════════════════════════════════════════════

export const ROLES = Object.freeze({
	SUPERADMIN: 'super-admin',
	DEVELOPER: 'developer',
	ADMIN: 'admin',
	MEMBER: 'member',
	USER: 'user'
} as const);

export type RoleType = typeof ROLES[keyof typeof ROLES];

/**
 * Role hierarchy - higher index = more permissions
 */
export const ROLE_HIERARCHY: readonly RoleType[] = Object.freeze([
	ROLES.USER,
	ROLES.MEMBER,
	ROLES.ADMIN,
	ROLES.SUPERADMIN
]);

// ═══════════════════════════════════════════════════════════════════════════
// Permission Definitions
// ═══════════════════════════════════════════════════════════════════════════

export const PERMISSIONS = Object.freeze({
	// Dashboard
	DASHBOARD_VIEW: 'dashboard.view',
	DASHBOARD_ANALYTICS: 'dashboard.analytics',
	
	// Users
	USERS_VIEW: 'users.view',
	USERS_CREATE: 'users.create',
	USERS_EDIT: 'users.edit',
	USERS_DELETE: 'users.delete',
	USERS_IMPERSONATE: 'users.impersonate',
	
	// Content
	POSTS_VIEW: 'posts.view',
	POSTS_CREATE: 'posts.create',
	POSTS_EDIT: 'posts.edit',
	POSTS_DELETE: 'posts.delete',
	POSTS_PUBLISH: 'posts.publish',
	
	// Products
	PRODUCTS_VIEW: 'products.view',
	PRODUCTS_CREATE: 'products.create',
	PRODUCTS_EDIT: 'products.edit',
	PRODUCTS_DELETE: 'products.delete',
	
	// Orders
	ORDERS_VIEW: 'orders.view',
	ORDERS_MANAGE: 'orders.manage',
	ORDERS_REFUND: 'orders.refund',
	
	// Coupons
	COUPONS_VIEW: 'coupons.view',
	COUPONS_CREATE: 'coupons.create',
	COUPONS_EDIT: 'coupons.edit',
	COUPONS_DELETE: 'coupons.delete',
	
	// Popups
	POPUPS_VIEW: 'popups.view',
	POPUPS_CREATE: 'popups.create',
	POPUPS_EDIT: 'popups.edit',
	POPUPS_DELETE: 'popups.delete',
	
	// Forms
	FORMS_VIEW: 'forms.view',
	FORMS_CREATE: 'forms.create',
	FORMS_EDIT: 'forms.edit',
	FORMS_DELETE: 'forms.delete',
	FORMS_SUBMISSIONS: 'forms.submissions',
	
	// Settings
	SETTINGS_VIEW: 'settings.view',
	SETTINGS_EDIT: 'settings.edit',
	SETTINGS_SYSTEM: 'settings.system',
	
	// Memberships
	MEMBERSHIPS_VIEW: 'memberships.view',
	MEMBERSHIPS_MANAGE: 'memberships.manage',
	
	// Trading Rooms
	TRADING_ROOMS_ACCESS: 'trading_rooms.access',
	TRADING_ROOMS_MODERATE: 'trading_rooms.moderate',
	TRADING_ROOMS_ADMIN: 'trading_rooms.admin',
	
	// Media
	MEDIA_VIEW: 'media.view',
	MEDIA_UPLOAD: 'media.upload',
	MEDIA_DELETE: 'media.delete',
	
	// System
	SYSTEM_LOGS: 'system.logs',
	SYSTEM_CACHE: 'system.cache',
	SYSTEM_MAINTENANCE: 'system.maintenance'
} as const);

export type PermissionType = typeof PERMISSIONS[keyof typeof PERMISSIONS];

// ═══════════════════════════════════════════════════════════════════════════
// Role-Permission Mapping
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Default permissions for each role
 * Superadmin has ALL permissions automatically (not listed here)
 */
export const ROLE_PERMISSIONS: Record<RoleType, readonly PermissionType[]> = Object.freeze({
	[ROLES.SUPERADMIN]: Object.values(PERMISSIONS), // All permissions
	
	[ROLES.DEVELOPER]: [
		PERMISSIONS.DASHBOARD_VIEW,
		PERMISSIONS.TRADING_ROOMS_ACCESS,
		PERMISSIONS.MEMBERSHIPS_VIEW,
		PERMISSIONS.MEDIA_VIEW
	],
	
	[ROLES.ADMIN]: [
		PERMISSIONS.DASHBOARD_VIEW,
		PERMISSIONS.DASHBOARD_ANALYTICS,
		PERMISSIONS.USERS_VIEW,
		PERMISSIONS.USERS_CREATE,
		PERMISSIONS.USERS_EDIT,
		PERMISSIONS.POSTS_VIEW,
		PERMISSIONS.POSTS_CREATE,
		PERMISSIONS.POSTS_EDIT,
		PERMISSIONS.POSTS_DELETE,
		PERMISSIONS.POSTS_PUBLISH,
		PERMISSIONS.PRODUCTS_VIEW,
		PERMISSIONS.PRODUCTS_CREATE,
		PERMISSIONS.PRODUCTS_EDIT,
		PERMISSIONS.ORDERS_VIEW,
		PERMISSIONS.ORDERS_MANAGE,
		PERMISSIONS.COUPONS_VIEW,
		PERMISSIONS.COUPONS_CREATE,
		PERMISSIONS.COUPONS_EDIT,
		PERMISSIONS.POPUPS_VIEW,
		PERMISSIONS.POPUPS_CREATE,
		PERMISSIONS.POPUPS_EDIT,
		PERMISSIONS.FORMS_VIEW,
		PERMISSIONS.FORMS_CREATE,
		PERMISSIONS.FORMS_EDIT,
		PERMISSIONS.FORMS_SUBMISSIONS,
		PERMISSIONS.SETTINGS_VIEW,
		PERMISSIONS.MEMBERSHIPS_VIEW,
		PERMISSIONS.MEMBERSHIPS_MANAGE,
		PERMISSIONS.TRADING_ROOMS_ACCESS,
		PERMISSIONS.TRADING_ROOMS_MODERATE,
		PERMISSIONS.MEDIA_VIEW,
		PERMISSIONS.MEDIA_UPLOAD
	],
	
	[ROLES.MEMBER]: [
		PERMISSIONS.DASHBOARD_VIEW,
		PERMISSIONS.TRADING_ROOMS_ACCESS,
		PERMISSIONS.MEDIA_VIEW
	],
	
	[ROLES.USER]: [
		PERMISSIONS.DASHBOARD_VIEW
	]
});

// ═══════════════════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Check if an email is a superadmin
 */
export function isSuperadminEmail(email: string | null | undefined): boolean {
	if (!email) return false;
	return SUPERADMIN_EMAILS.includes(email.toLowerCase());
}

/**
 * Check if an email is a developer
 */
export function isDeveloperEmail(email: string | null | undefined): boolean {
	if (!email) return false;
	return DEVELOPER_EMAILS.includes(email.toLowerCase());
}

/**
 * Check if a user has superadmin access
 */
export function isSuperadmin(user: { email?: string; roles?: string[] } | null | undefined): boolean {
	if (!user) return false;
	
	// Check email first (highest priority) - with null check
	if (user.email && isSuperadminEmail(user.email)) return true;
	
	// Check roles
	if (user.roles?.some(role => 
		role.toLowerCase() === ROLES.SUPERADMIN || 
		role.toLowerCase() === 'super_admin' ||
		role.toLowerCase() === 'superadmin'
	)) {
		return true;
	}
	
	return false;
}

/**
 * Check if a user has developer access
 */
export function isDeveloper(user: { email?: string; role?: string; roles?: string[] } | null | undefined): boolean {
	if (!user) return false;
	
	// Check email first (highest priority)
	if (user.email && isDeveloperEmail(user.email)) return true;
	
	// Check role field
	if (user.role?.toLowerCase() === ROLES.DEVELOPER) return true;
	
	// Check roles array
	if (user.roles?.some(role => role.toLowerCase() === ROLES.DEVELOPER)) return true;
	
	return false;
}

/**
 * Check if a user has admin access (admin or superadmin)
 */
export function isAdmin(user: { email?: string; roles?: string[]; is_admin?: boolean } | null | undefined): boolean {
	if (!user) return false;
	
	// Superadmin is always admin
	if (isSuperadmin(user)) return true;
	
	// Check is_admin flag
	if (user.is_admin) return true;
	
	// Check roles
	if (user.roles?.some(role => 
		['admin', 'administrator', 'super-admin', 'super_admin', 'superadmin'].includes(role.toLowerCase())
	)) {
		return true;
	}
	
	return false;
}

/**
 * Check if a user has a specific permission
 * Superadmin always returns true
 */
export function hasPermission(
	user: { email?: string; roles?: string[]; permissions?: string[] } | null | undefined,
	permission: PermissionType | string
): boolean {
	if (!user) return false;
	
	// Superadmin has all permissions
	if (isSuperadmin(user)) return true;
	
	// Check explicit permissions
	if (user.permissions?.includes(permission)) return true;
	
	// Check role-based permissions
	if (user.roles) {
		for (const role of user.roles) {
			const roleKey = role.toLowerCase() as RoleType;
			const rolePerms = ROLE_PERMISSIONS[roleKey];
			if (rolePerms?.includes(permission as PermissionType)) {
				return true;
			}
		}
	}
	
	return false;
}

/**
 * Check if a user has any of the specified permissions
 */
export function hasAnyPermission(
	user: { email?: string; roles?: string[]; permissions?: string[] } | null | undefined,
	permissions: (PermissionType | string)[]
): boolean {
	return permissions.some(p => hasPermission(user, p));
}

/**
 * Check if a user has all of the specified permissions
 */
export function hasAllPermissions(
	user: { email?: string; roles?: string[]; permissions?: string[] } | null | undefined,
	permissions: (PermissionType | string)[]
): boolean {
	return permissions.every(p => hasPermission(user, p));
}

/**
 * Get all permissions for a user
 */
export function getUserPermissions(
	user: { email?: string; roles?: string[]; permissions?: string[] } | null | undefined
): PermissionType[] {
	if (!user) return [];
	
	// Superadmin has all permissions
	if (isSuperadmin(user)) return Object.values(PERMISSIONS);
	
	const permissions = new Set<PermissionType>();
	
	// Add explicit permissions
	if (user.permissions) {
		user.permissions.forEach(p => permissions.add(p as PermissionType));
	}
	
	// Add role-based permissions
	if (user.roles) {
		for (const role of user.roles) {
			const roleKey = role.toLowerCase() as RoleType;
			const rolePerms = ROLE_PERMISSIONS[roleKey];
			if (rolePerms) {
				rolePerms.forEach(p => permissions.add(p));
			}
		}
	}
	
	return Array.from(permissions);
}

/**
 * Get the highest role for a user
 */
export function getHighestRole(user: { email?: string; roles?: string[] } | null | undefined): RoleType {
	if (!user) return ROLES.USER;
	
	// Superadmin email always gets superadmin role - with null check
	if (user.email && isSuperadminEmail(user.email)) return ROLES.SUPERADMIN;
	
	if (!user.roles || user.roles.length === 0) return ROLES.USER;
	
	let highestIndex = 0;
	
	for (const role of user.roles) {
		const normalizedRole = role.toLowerCase().replace('_', '-') as RoleType;
		const index = ROLE_HIERARCHY.indexOf(normalizedRole);
		if (index > highestIndex) {
			highestIndex = index;
		}
	}
	
	return ROLE_HIERARCHY[highestIndex] ?? ROLES.USER;
}
