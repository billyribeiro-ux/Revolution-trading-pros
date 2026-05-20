/**
 * R27-D extraction (2026-05-20): shared types for the admin user-edit
 * page. Form-data shape is local to this page; the wire shape lives in
 * the API client.
 */
export interface UserEditFormData {
	name: string;
	first_name: string;
	last_name: string;
	email: string;
	password: string;
	password_confirmation: string;
	roles: string[];
}

export type UserRole = 'admin' | 'super-admin';
