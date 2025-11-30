/**
 * SEO Module - Enterprise-Grade SEO Toolkit
 * Comprehensive SEO tools following Google November 2025 Standards
 *
 * Modules:
 * - Image SEO: Auto alt/title generation, case transformation
 * - Video Schema: Auto-detection, JSON-LD generation
 * - Email Reports: Scheduled automated SEO reports
 * - Store Locator: Multi-location management, KML, LocalBusiness schema
 *
 * @version 1.0.0 - November 2025
 */

// ═══════════════════════════════════════════════════════════════════════════
// Image SEO Module
// ═══════════════════════════════════════════════════════════════════════════

export {
	// Types
	type CaseTransform,
	type ImageSeoSettings,
	type ImageMetadata,
	type ProcessedImage,

	// Functions
	extractFilename,
	cleanFilename,
	applyCase,
	toTitleCase,
	toSentenceCase,
	generateAltText,
	generateTitle,
	generateCaption,
	generateAvatarAlt,
	generateSeoFilename,
	calculateImageSeoScore,
	analyzeImageSeo,
	processImage,
	processImages,
	generateImageHtml,

	// Store
	imageSeoSettings,
	defaultImageSeoSettings
} from './image-seo';

// ═══════════════════════════════════════════════════════════════════════════
// Video Schema Module
// ═══════════════════════════════════════════════════════════════════════════

export {
	// Types
	type VideoPlatform,
	type VideoClip,
	type VideoInteractionStatistic,
	type VideoInput,
	type VideoSchemaOptions,
	type DetectedVideo,
	type ValidationResult,

	// Functions
	detectVideoPlatform,
	findVideosInContent,
	secondsToIsoDuration,
	isoDurationToSeconds,
	generateVideoSchema,
	generateSchemaFromDetected,
	generateCourseVideoSchema,
	generateHowToVideoSchema,
	generateVideoGallerySchema,
	generateJsonLdScript,
	generateMultipleJsonLdScripts,
	validateVideoSchema
} from './video-schema';

// ═══════════════════════════════════════════════════════════════════════════
// Email Reports Module
// ═══════════════════════════════════════════════════════════════════════════

export {
	// Types
	type ReportFrequency,
	type ReportFormat,
	type ReportSection,
	type ReportRecipient,
	type ReportMetric,
	type ReportChart,
	type ReportSectionConfig,
	type ReportTemplate,
	type ReportBranding,
	type ReportSchedule,
	type GeneratedReport,

	// Functions
	calculateDateRange,
	calculateChange,
	formatMetricValue,
	generateExecutiveSummary,
	generateHtmlReport,
	generateCsvReport,
	calculateNextRun,
	formatSchedule,

	// Stores
	reportTemplates,
	generatedReports,
	defaultBranding,
	defaultSections
} from './email-reports';

// ═══════════════════════════════════════════════════════════════════════════
// Store Locator Module
// ═══════════════════════════════════════════════════════════════════════════

export {
	// Types
	type DayOfWeek,
	type BusinessType,
	type GeoCoordinates,
	type Address,
	type BusinessHours,
	type SpecialHours,
	type ContactInfo,
	type SocialProfile,
	type ServiceArea,
	type Review,
	type Location,
	type StoreLocatorSettings,

	// Functions
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

	// Stores
	locations,
	locatorSettings,
	activeLocations,
	primaryLocation,
	defaultSettings as defaultLocatorSettings,
	daysOfWeek
} from './store-locator';

// ═══════════════════════════════════════════════════════════════════════════
// Default Exports (Module Objects)
// ═══════════════════════════════════════════════════════════════════════════

import imageSeo from './image-seo';
import videoSchema from './video-schema';
import emailReports from './email-reports';
import storeLocator from './store-locator';

export { imageSeo, videoSchema, emailReports, storeLocator };

// Combined default export
export default {
	imageSeo,
	videoSchema,
	emailReports,
	storeLocator
};
