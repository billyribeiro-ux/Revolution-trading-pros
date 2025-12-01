/**
 * Store Locator & Local SEO Module - Enterprise-Grade Multi-Location Support
 * Following Google Local SEO Best Practices (November 2025)
 *
 * Features:
 * - Multi-location management
 * - LocalBusiness schema generation
 * - Google My Business integration ready
 * - KML file generation for maps
 * - Location clustering for maps
 * - Service area definitions
 * - Special hours and holidays
 * - Multiple phone/email support
 * - Social profiles per location
 * - Reviews aggregation
 * - Geofencing support
 *
 * @version 1.0.0 - November 2025
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export type DayOfWeek =
	| 'Monday'
	| 'Tuesday'
	| 'Wednesday'
	| 'Thursday'
	| 'Friday'
	| 'Saturday'
	| 'Sunday';

export type BusinessType =
	| 'LocalBusiness'
	| 'FinancialService'
	| 'ProfessionalService'
	| 'EducationalOrganization'
	| 'Store'
	| 'Restaurant'
	| 'MedicalBusiness'
	| 'LegalService'
	| 'RealEstateAgent'
	| 'TravelAgency'
	| 'SportsActivityLocation'
	| 'EntertainmentBusiness';

export interface GeoCoordinates {
	latitude: number;
	longitude: number;
}

export interface Address {
	streetAddress: string;
	addressLocality: string; // City
	addressRegion: string; // State/Province
	postalCode: string;
	addressCountry: string;
	coordinates?: GeoCoordinates;
}

export interface BusinessHours {
	dayOfWeek: DayOfWeek;
	opens: string; // HH:MM format
	closes: string; // HH:MM format
	isClosed?: boolean;
}

export interface SpecialHours {
	date: string; // YYYY-MM-DD
	name?: string; // Holiday name
	opens?: string;
	closes?: string;
	isClosed: boolean;
}

export interface ContactInfo {
	type: 'phone' | 'fax' | 'email' | 'tollfree';
	value: string;
	label?: string;
	isPrimary?: boolean;
}

export interface SocialProfile {
	platform:
		| 'facebook'
		| 'twitter'
		| 'instagram'
		| 'linkedin'
		| 'youtube'
		| 'tiktok'
		| 'pinterest';
	url: string;
}

export interface ServiceArea {
	type: 'radius' | 'polygon' | 'administrative';
	// For radius type
	radius?: number;
	radiusUnit?: 'km' | 'mi';
	center?: GeoCoordinates;
	// For polygon type
	polygon?: GeoCoordinates[];
	// For administrative type
	areas?: string[]; // List of cities, counties, etc.
}

export interface Review {
	id: string;
	author: string;
	rating: number; // 1-5
	review: string;
	datePublished: string;
	platform?: string;
}

export interface Location {
	id: string;
	name: string;
	description?: string;
	businessType: BusinessType;
	address: Address;
	contacts: ContactInfo[];
	website?: string;
	hours: BusinessHours[];
	specialHours?: SpecialHours[];
	socialProfiles?: SocialProfile[];
	serviceArea?: ServiceArea;
	priceRange?: string; // $, $$, $$$, $$$$
	paymentAccepted?: string[];
	amenities?: string[];
	images?: {
		type: 'logo' | 'photo' | 'exterior' | 'interior';
		url: string;
		caption?: string;
	}[];
	reviews?: Review[];
	aggregateRating?: {
		ratingValue: number;
		reviewCount: number;
		bestRating?: number;
		worstRating?: number;
	};
	categories?: string[];
	tags?: string[];
	isActive: boolean;
	isPrimary?: boolean;
	googlePlaceId?: string;
	createdAt: string;
	updatedAt?: string;
}

export interface StoreLocatorSettings {
	defaultCenter: GeoCoordinates;
	defaultZoom: number;
	markerIcon?: string;
	clusterEnabled: boolean;
	clusterRadius: number;
	maxLocationsPerPage: number;
	searchRadius: number;
	radiusUnit: 'km' | 'mi';
	showDirections: boolean;
	googleMapsApiKey?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// Default Configuration
// ═══════════════════════════════════════════════════════════════════════════

export const defaultSettings: StoreLocatorSettings = {
	defaultCenter: { latitude: 40.7128, longitude: -74.006 }, // NYC
	defaultZoom: 12,
	clusterEnabled: true,
	clusterRadius: 50,
	maxLocationsPerPage: 20,
	searchRadius: 50,
	radiusUnit: 'mi',
	showDirections: true
};

export const daysOfWeek: DayOfWeek[] = [
	'Monday',
	'Tuesday',
	'Wednesday',
	'Thursday',
	'Friday',
	'Saturday',
	'Sunday'
];

// ═══════════════════════════════════════════════════════════════════════════
// Distance Calculation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Calculate distance between two points using Haversine formula
 */
export function calculateDistance(
	point1: GeoCoordinates,
	point2: GeoCoordinates,
	unit: 'km' | 'mi' = 'mi'
): number {
	const R = unit === 'km' ? 6371 : 3959; // Earth's radius

	const dLat = toRad(point2.latitude - point1.latitude);
	const dLon = toRad(point2.longitude - point1.longitude);

	const a =
		Math.sin(dLat / 2) * Math.sin(dLat / 2) +
		Math.cos(toRad(point1.latitude)) *
			Math.cos(toRad(point2.latitude)) *
			Math.sin(dLon / 2) *
			Math.sin(dLon / 2);

	const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

	return R * c;
}

function toRad(deg: number): number {
	return deg * (Math.PI / 180);
}

/**
 * Find locations within a radius
 */
export function findNearbyLocations(
	locations: Location[],
	center: GeoCoordinates,
	radius: number,
	unit: 'km' | 'mi' = 'mi'
): (Location & { distance: number })[] {
	return locations
		.filter((loc) => loc.address.coordinates && loc.isActive)
		.map((loc) => ({
			...loc,
			distance: calculateDistance(center, loc.address.coordinates!, unit)
		}))
		.filter((loc) => loc.distance <= radius)
		.sort((a, b) => a.distance - b.distance);
}

// ═══════════════════════════════════════════════════════════════════════════
// Schema Generation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generate LocalBusiness JSON-LD schema for a location
 */
export function generateLocationSchema(location: Location): Record<string, any> {
	const schema: Record<string, any> = {
		'@context': 'https://schema.org',
		'@type': location.businessType || 'LocalBusiness',
		name: location.name
	};

	// Description
	if (location.description) {
		schema.description = location.description;
	}

	// Address
	schema.address = {
		'@type': 'PostalAddress',
		streetAddress: location.address.streetAddress,
		addressLocality: location.address.addressLocality,
		addressRegion: location.address.addressRegion,
		postalCode: location.address.postalCode,
		addressCountry: location.address.addressCountry
	};

	// Geo coordinates
	if (location.address.coordinates) {
		schema.geo = {
			'@type': 'GeoCoordinates',
			latitude: location.address.coordinates.latitude,
			longitude: location.address.coordinates.longitude
		};
	}

	// Contact info
	const primaryPhone = location.contacts.find(
		(c) => c.type === 'phone' && c.isPrimary
	);
	if (primaryPhone) {
		schema.telephone = primaryPhone.value;
	}

	const primaryEmail = location.contacts.find(
		(c) => c.type === 'email' && c.isPrimary
	);
	if (primaryEmail) {
		schema.email = primaryEmail.value;
	}

	// Website
	if (location.website) {
		schema.url = location.website;
	}

	// Opening hours
	if (location.hours && location.hours.length > 0) {
		schema.openingHoursSpecification = location.hours
			.filter((h) => !h.isClosed)
			.map((h) => ({
				'@type': 'OpeningHoursSpecification',
				dayOfWeek: h.dayOfWeek,
				opens: h.opens,
				closes: h.closes
			}));
	}

	// Special hours
	if (location.specialHours && location.specialHours.length > 0) {
		schema.specialOpeningHoursSpecification = location.specialHours.map((sh) => {
			const spec: Record<string, any> = {
				'@type': 'OpeningHoursSpecification',
				validFrom: sh.date,
				validThrough: sh.date
			};

			if (sh.isClosed) {
				spec.opens = '00:00';
				spec.closes = '00:00';
			} else {
				if (sh.opens) spec.opens = sh.opens;
				if (sh.closes) spec.closes = sh.closes;
			}

			return spec;
		});
	}

	// Price range
	if (location.priceRange) {
		schema.priceRange = location.priceRange;
	}

	// Payment accepted
	if (location.paymentAccepted && location.paymentAccepted.length > 0) {
		schema.paymentAccepted = location.paymentAccepted.join(', ');
	}

	// Images
	if (location.images && location.images.length > 0) {
		const logo = location.images.find((img) => img.type === 'logo');
		if (logo) {
			schema.logo = logo.url;
		}

		const photos = location.images.filter((img) => img.type !== 'logo');
		if (photos.length > 0) {
			schema.image = photos.map((p) => p.url);
		}
	}

	// Social profiles
	if (location.socialProfiles && location.socialProfiles.length > 0) {
		schema.sameAs = location.socialProfiles.map((p) => p.url);
	}

	// Aggregate rating
	if (location.aggregateRating) {
		schema.aggregateRating = {
			'@type': 'AggregateRating',
			ratingValue: location.aggregateRating.ratingValue,
			reviewCount: location.aggregateRating.reviewCount,
			bestRating: location.aggregateRating.bestRating || 5,
			worstRating: location.aggregateRating.worstRating || 1
		};
	}

	// Service area
	if (location.serviceArea) {
		if (location.serviceArea.type === 'radius' && location.serviceArea.center) {
			schema.areaServed = {
				'@type': 'GeoCircle',
				geoMidpoint: {
					'@type': 'GeoCoordinates',
					latitude: location.serviceArea.center.latitude,
					longitude: location.serviceArea.center.longitude
				},
				geoRadius: `${location.serviceArea.radius} ${location.serviceArea.radiusUnit}`
			};
		} else if (location.serviceArea.areas && location.serviceArea.areas.length > 0) {
			schema.areaServed = location.serviceArea.areas.map((area) => ({
				'@type': 'AdministrativeArea',
				name: area
			}));
		}
	}

	// Google Place ID
	if (location.googlePlaceId) {
		schema.hasMap = `https://www.google.com/maps/place/?q=place_id:${location.googlePlaceId}`;
	}

	return schema;
}

/**
 * Generate schema for multiple locations (Organization with locations)
 */
export function generateMultiLocationSchema(
	organizationName: string,
	locations: Location[]
): Record<string, any> {
	return {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		name: organizationName,
		location: locations.filter((l) => l.isActive).map((loc) => generateLocationSchema(loc))
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// KML Generation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generate KML file content for locations
 */
export function generateKml(
	locations: Location[],
	documentName: string = 'Store Locations'
): string {
	const placemarks = locations
		.filter((loc) => loc.isActive && loc.address.coordinates)
		.map(
			(loc) => `
		<Placemark>
			<name>${escapeXml(loc.name)}</name>
			<description><![CDATA[
				<p>${escapeXml(loc.address.streetAddress)}</p>
				<p>${escapeXml(loc.address.addressLocality)}, ${escapeXml(loc.address.addressRegion)} ${escapeXml(loc.address.postalCode)}</p>
				${loc.contacts.find((c) => c.type === 'phone')?.value ? `<p>Phone: ${loc.contacts.find((c) => c.type === 'phone')!.value}</p>` : ''}
			]]></description>
			<Point>
				<coordinates>${loc.address.coordinates!.longitude},${loc.address.coordinates!.latitude},0</coordinates>
			</Point>
		</Placemark>
	`
		)
		.join('');

	return `<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
	<Document>
		<name>${escapeXml(documentName)}</name>
		<description>Store locations for ${escapeXml(documentName)}</description>
		${placemarks}
	</Document>
</kml>`;
}

function escapeXml(str: string): string {
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&apos;');
}

// ═══════════════════════════════════════════════════════════════════════════
// Hours Utilities
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Check if a location is currently open
 */
export function isLocationOpen(location: Location): boolean {
	const now = new Date();
	const currentDay = daysOfWeek[now.getDay() === 0 ? 6 : now.getDay() - 1];
	const currentTime =
		now.getHours().toString().padStart(2, '0') + ':' + now.getMinutes().toString().padStart(2, '0');

	// Check special hours first
	const todayStr = now.toISOString().split('T')[0];
	const specialHour = location.specialHours?.find((sh) => sh.date === todayStr);

	if (specialHour) {
		if (specialHour.isClosed) return false;
		if (specialHour.opens && specialHour.closes) {
			return currentTime >= specialHour.opens && currentTime <= specialHour.closes;
		}
	}

	// Check regular hours
	const dayHours = location.hours.find((h) => h.dayOfWeek === currentDay);

	if (!dayHours || dayHours.isClosed) return false;

	return currentTime >= dayHours.opens && currentTime <= dayHours.closes;
}

/**
 * Format hours for display
 */
export function formatHours(hours: BusinessHours): string {
	if (hours.isClosed) return 'Closed';
	return `${formatTime(hours.opens)} - ${formatTime(hours.closes)}`;
}

/**
 * Format time from HH:MM to readable format
 */
export function formatTime(time: string): string {
	const [hours, minutes] = time.split(':').map(Number);
	const period = hours >= 12 ? 'PM' : 'AM';
	const displayHours = hours % 12 || 12;
	return `${displayHours}:${minutes.toString().padStart(2, '0')} ${period}`;
}

/**
 * Get next open time
 */
export function getNextOpenTime(location: Location): string | null {
	if (isLocationOpen(location)) return 'Open now';

	const now = new Date();
	const currentDayIndex = now.getDay() === 0 ? 6 : now.getDay() - 1;

	// Check remaining days
	for (let i = 0; i < 7; i++) {
		const dayIndex = (currentDayIndex + i) % 7;
		const dayName = daysOfWeek[dayIndex];
		const dayHours = location.hours.find((h) => h.dayOfWeek === dayName);

		if (dayHours && !dayHours.isClosed) {
			if (i === 0) {
				// Today
				const currentTime =
					now.getHours().toString().padStart(2, '0') +
					':' +
					now.getMinutes().toString().padStart(2, '0');
				if (currentTime < dayHours.opens) {
					return `Opens at ${formatTime(dayHours.opens)}`;
				}
			} else if (i === 1) {
				return `Opens tomorrow at ${formatTime(dayHours.opens)}`;
			} else {
				return `Opens ${dayName} at ${formatTime(dayHours.opens)}`;
			}
		}
	}

	return null;
}

// ═══════════════════════════════════════════════════════════════════════════
// Address Formatting
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Format address for display
 */
export function formatAddress(address: Address, multiline: boolean = false): string {
	const separator = multiline ? '\n' : ', ';

	const parts = [
		address.streetAddress,
		`${address.addressLocality}, ${address.addressRegion} ${address.postalCode}`,
		address.addressCountry
	];

	return parts.filter(Boolean).join(separator);
}

/**
 * Generate Google Maps URL for directions
 */
export function getDirectionsUrl(location: Location, from?: GeoCoordinates): string {
	const destination = location.address.coordinates
		? `${location.address.coordinates.latitude},${location.address.coordinates.longitude}`
		: encodeURIComponent(formatAddress(location.address));

	let url = `https://www.google.com/maps/dir/?api=1&destination=${destination}`;

	if (from) {
		url += `&origin=${from.latitude},${from.longitude}`;
	}

	return url;
}

// ═══════════════════════════════════════════════════════════════════════════
// Svelte Stores
// ═══════════════════════════════════════════════════════════════════════════

const LOCATIONS_KEY = 'store-locations';
const SETTINGS_KEY = 'store-locator-settings';

function createLocationsStore() {
	let initial: Location[] = [];

	if (browser) {
		const stored = localStorage.getItem(LOCATIONS_KEY);
		if (stored) {
			try {
				initial = JSON.parse(stored);
			} catch {
				// Use empty array on parse error
			}
		}
	}

	const { subscribe, set, update } = writable<Location[]>(initial);

	const save = (locations: Location[]) => {
		if (browser) {
			localStorage.setItem(LOCATIONS_KEY, JSON.stringify(locations));
		}
	};

	return {
		subscribe,

		add: (location: Omit<Location, 'id' | 'createdAt'>) => {
			const newLocation: Location = {
				...location,
				id: crypto.randomUUID(),
				createdAt: new Date().toISOString()
			};

			update((locations) => {
				const updated = [...locations, newLocation];
				save(updated);
				return updated;
			});

			return newLocation;
		},

		update: (id: string, data: Partial<Location>) => {
			update((locations) => {
				const updated = locations.map((loc) =>
					loc.id === id ? { ...loc, ...data, updatedAt: new Date().toISOString() } : loc
				);
				save(updated);
				return updated;
			});
		},

		remove: (id: string) => {
			update((locations) => {
				const updated = locations.filter((loc) => loc.id !== id);
				save(updated);
				return updated;
			});
		},

		setPrimary: (id: string) => {
			update((locations) => {
				const updated = locations.map((loc) => ({
					...loc,
					isPrimary: loc.id === id,
					updatedAt: new Date().toISOString()
				}));
				save(updated);
				return updated;
			});
		},

		toggle: (id: string) => {
			update((locations) => {
				const updated = locations.map((loc) =>
					loc.id === id
						? { ...loc, isActive: !loc.isActive, updatedAt: new Date().toISOString() }
						: loc
				);
				save(updated);
				return updated;
			});
		},

		get: (id: string) => {
			const locations = get({ subscribe });
			return locations.find((loc) => loc.id === id);
		},

		findNearest: (coordinates: GeoCoordinates, radius?: number) => {
			const locations = get({ subscribe });
			return findNearbyLocations(
				locations,
				coordinates,
				radius || defaultSettings.searchRadius,
				defaultSettings.radiusUnit
			);
		}
	};
}

function createSettingsStore() {
	let initial = defaultSettings;

	if (browser) {
		const stored = localStorage.getItem(SETTINGS_KEY);
		if (stored) {
			try {
				initial = { ...defaultSettings, ...JSON.parse(stored) };
			} catch {
				// Use defaults on parse error
			}
		}
	}

	const { subscribe, set, update } = writable<StoreLocatorSettings>(initial);

	return {
		subscribe,
		set: (value: StoreLocatorSettings) => {
			if (browser) {
				localStorage.setItem(SETTINGS_KEY, JSON.stringify(value));
			}
			set(value);
		},
		update: (fn: (value: StoreLocatorSettings) => StoreLocatorSettings) => {
			update((current) => {
				const newValue = fn(current);
				if (browser) {
					localStorage.setItem(SETTINGS_KEY, JSON.stringify(newValue));
				}
				return newValue;
			});
		},
		reset: () => {
			if (browser) {
				localStorage.removeItem(SETTINGS_KEY);
			}
			set(defaultSettings);
		}
	};
}

export const locations = createLocationsStore();
export const locatorSettings = createSettingsStore();

// Derived stores
export const activeLocations = derived(locations, ($locations) =>
	$locations.filter((loc) => loc.isActive)
);

export const primaryLocation = derived(locations, ($locations) =>
	$locations.find((loc) => loc.isPrimary)
);

// ═══════════════════════════════════════════════════════════════════════════
// Export Default
// ═══════════════════════════════════════════════════════════════════════════

export default {
	locations,
	settings: locatorSettings,
	activeLocations,
	primaryLocation,
	calculateDistance,
	findNearbyLocations,
	generateLocationSchema,
	generateMultiLocationSchema,
	generateKml,
	isLocationOpen,
	formatHours,
	formatTime,
	getNextOpenTime,
	formatAddress,
	getDirectionsUrl,
	defaultSettings,
	daysOfWeek
};
