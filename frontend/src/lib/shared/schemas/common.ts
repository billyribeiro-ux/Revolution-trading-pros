/**
 * Shared Valibot Schemas — Common Types
 *
 * Reusable validation schemas for pagination, slugs, IDs.
 * Used by Remote Functions for input validation at the server boundary.
 *
 * @version 1.0.0
 */

import * as v from 'valibot';

/** Room slug: lowercase alphanumeric with hyphens */
export const RoomSlugSchema = v.pipe(
	v.string(),
	v.nonEmpty('Room slug is required'),
	v.regex(/^[a-z0-9-]+$/, 'Invalid room slug format')
);

/** Positive integer ID */
export const IdSchema = v.pipe(
	v.number(),
	v.integer('ID must be an integer'),
	v.minValue(1, 'ID must be positive')
);

/** Page number (1-indexed) */
export const PageSchema = v.pipe(v.number(), v.integer(), v.minValue(1, 'Page must be at least 1'));

/** Items per page limit */
export const LimitSchema = v.pipe(
	v.number(),
	v.integer(),
	v.minValue(1, 'Limit must be at least 1'),
	v.maxValue(200, 'Limit cannot exceed 200')
);

/** Pagination input */
export const PaginationSchema = v.object({
	page: PageSchema,
	limit: LimitSchema
});

/** Stock ticker symbol */
export const TickerSchema = v.pipe(
	v.string(),
	v.nonEmpty('Ticker is required'),
	v.regex(/^[A-Z]{1,5}$/, 'Ticker must be 1-5 uppercase letters')
);

/** ISO 8601 date string */
export const DateStringSchema = v.pipe(v.string(), v.nonEmpty('Date is required'));

/** Price value (positive number) */
export const PriceSchema = v.pipe(v.number(), v.minValue(0, 'Price must be non-negative'));

/** Optional string that defaults to empty */
export const OptionalStringSchema = v.optional(v.string(), '');
