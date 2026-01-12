/**
 * Component Registry
 * Apple Principal Engineer ICT 7 Grade - January 2026
 * 
 * Central registry of all available page builder components.
 * Easy to extend - just add new entries to the registry.
 */

import type { ComponentRegistryEntry, ComponentType } from './types';

export const componentRegistry: ComponentRegistryEntry[] = [
	// ═══════════════════════════════════════════════════════════════════════════
	// CONTENT COMPONENTS
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'course-header',
		name: 'Course Header',
		description: 'Title section with course info and optional login button',
		icon: '📋',
		category: 'content',
		defaultConfig: {
			title: 'Course Title',
			subtitle: '',
			description: '',
			instructorName: '',
			instructorTitle: '',
			showLoginButton: true,
			loginButtonText: 'LOGIN TO THE CLASSROOM',
			backgroundColor: '#143E59',
			textColor: '#FFFFFF'
		}
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// MEDIA COMPONENTS
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'video-player',
		name: 'Video Player',
		description: 'Single video with title header',
		icon: '🎬',
		category: 'media',
		defaultConfig: {
			title: 'Video Title',
			subtitle: '',
			videoId: '',
			bunnyVideoGuid: '',
			thumbnailUrl: '',
			autoplay: false
		}
	},
	{
		type: 'video-stack',
		name: 'Video Stack',
		description: 'Multiple videos stacked vertically',
		icon: '📚',
		category: 'media',
		defaultConfig: {
			videos: [],
			showDates: true,
			sortOrder: 'newest'
		}
	},
	{
		type: 'class-downloads',
		name: 'Class Downloads',
		description: 'File download section with Box-like interface',
		icon: '📁',
		category: 'content',
		defaultConfig: {
			title: 'Class Downloads',
			maxHeight: '400px'
		}
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// LAYOUT COMPONENTS
	// ═══════════════════════════════════════════════════════════════════════════
	{
		type: 'spacer',
		name: 'Spacer',
		description: 'Adjustable vertical space',
		icon: '↕️',
		category: 'layout',
		defaultConfig: {
			height: 40
		}
	},
	{
		type: 'divider',
		name: 'Divider',
		description: 'Horizontal line separator',
		icon: '➖',
		category: 'layout',
		defaultConfig: {
			style: 'solid',
			color: '#E0E0E0',
			thickness: 1,
			marginTop: 20,
			marginBottom: 20
		}
	}
];

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

export function getComponentByType(type: ComponentType): ComponentRegistryEntry | undefined {
	return componentRegistry.find(c => c.type === type);
}

export function getComponentsByCategory(category: 'content' | 'media' | 'layout'): ComponentRegistryEntry[] {
	return componentRegistry.filter(c => c.category === category);
}

export function createDefaultConfig(type: ComponentType): Record<string, unknown> {
	const entry = getComponentByType(type);
	return entry ? { ...entry.defaultConfig } : {};
}
