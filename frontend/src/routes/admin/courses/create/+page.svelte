<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import {
		IconPhoto,
		IconX,
		IconBook,
		IconCheck,
		IconPlus,
		IconGripVertical,
		IconTrash,
		IconSparkles,
		IconBulb,
		IconChartBar,
		IconVideo,
		IconFileText,
		IconClock,
		IconUsers,
		IconCurrencyDollar,
		IconCalendar,
		IconLock,
		IconTarget,
		IconTrendingUp,
		IconCertificate,
		IconDownload,
		IconEye,
		IconSettings,
		IconRefresh,
		IconGift,
		IconCopy,
		IconAlertCircle,
		IconRocket,
		IconStar,
		IconBrandGoogle,
		IconBrandFacebook,
		IconChevronDown
	} from '$lib/icons';
	import { productsApi, AdminApiError } from '$lib/api/admin';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ═══════════════════════════════════════════════════════════════════════════
	// Type Definitions & Interfaces
	// ═══════════════════════════════════════════════════════════════════════════

	interface Module {
		id: string;
		title: string;
		description: string;
		order: number;
		lessons: Lesson[];
		duration_minutes: number;
		is_preview: boolean;
	}

	interface Lesson {
		id: string;
		title: string;
		type: 'video' | 'text' | 'quiz' | 'assignment' | 'live' | 'download';
		content?: string;
		duration_minutes: number;
		is_preview: boolean;
		order: number;
		resources?: string[];
	}

	interface Bonus {
		id: string;
		title: string;
		description: string;
		value: number;
		icon: string;
		type: 'pdf' | 'video' | 'template' | 'access' | 'consultation';
	}

	interface DripSchedule {
		module_id: string;
		days_after_enrollment: number;
	}

	interface ValidationResult {
		field: string;
		status: 'good' | 'warning' | 'error';
		message: string;
		score: number;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete State Management
	// ═══════════════════════════════════════════════════════════════════════════

	let course = $state({
		// Basic Information
		name: '',
		slug: '',
		description: '',
		short_description: '',
		thumbnail: '',
		gallery: [] as string[],
		promo_video: '',

		// Course Structure
		type: 'self-paced' as 'self-paced' | 'cohort' | 'hybrid',
		format: 'video' as 'video' | 'text' | 'mixed' | 'live',
		level: 'beginner' as 'beginner' | 'intermediate' | 'advanced' | 'all-levels',
		duration_hours: 0,
		modules: [] as Module[],

		// Pricing Configuration
		pricing_model: 'one-time' as 'one-time' | 'subscription' | 'payment-plan' | 'free',
		price: 0,
		currency: 'USD',
		subscription_interval: 'monthly' as 'monthly' | 'quarterly' | 'yearly',
		payment_installments: 3,
		early_bird: {
			enabled: false,
			discount: 20,
			expires: ''
		},

		// Access Control
		start_date: '',
		end_date: '',
		enrollment_limit: null as number | null,
		lifetime_access: true,
		certificate_enabled: true,
		drip_schedule: [] as DripSchedule[],
		prerequisites: [] as string[],

		// SEO & Marketing
		meta_title: '',
		meta_description: '',
		keywords: [] as string[],
		og_image: '',
		landing_page_enabled: true,
		affiliate_enabled: false,
		affiliate_commission: 30,

		// Analytics Integration
		ga4_enabled: true,
		fb_pixel_enabled: false,
		conversion_tracking: true,
		utm_source: '',
		utm_medium: '',
		utm_campaign: '',

		// Features & Content
		features: [] as string[],
		outcomes: [] as string[],
		target_audience: [] as string[],
		tools_required: [] as string[],

		// Advanced Settings
		is_active: true,
		is_featured: false,
		allow_comments: true,
		allow_reviews: true,
		show_progress: true,
		completion_threshold: 80,
		max_attempts: 3,
		passing_score: 70,

		// Bonuses & Resources
		bonuses: [] as Bonus[],
		resources: [] as any[]
	});

	// UI State Management
	let activeTab = $state('basic');
	let uploading = $state(false);
	let saving = $state(false);
	let generating = $state(false);
	let analyzing = $state(false);
	let overallScore = $state(0);
	let validationResults = $state<ValidationResult[]>([]);
	let isDragging = $state(false);
	let draggedModule = $state<Module | null>(null);
	let draggedLesson = $state<Lesson | null>(null);
	let hasUnsavedChanges = $state(false);
	let lastSaved = $state<Date | null>(null);
	let autoSaveTimer: ReturnType<typeof setInterval>;
	let expandedModules = $state(new Set<string>());

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle Hooks & Initialization
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		// Initialize with starter module
		if (course.modules.length === 0) {
			addModule();
		}

		// Load any saved drafts
		loadDraft();

		// Setup auto-save every 30 seconds
		autoSaveTimer = setInterval(() => {
			if (hasUnsavedChanges) {
				saveDraft();
			}
		}, 30000);

		// Initialize validation
		validateAll();

		// Cleanup on unmount
		return () => {
			if (autoSaveTimer) clearInterval(autoSaveTimer);
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete Module & Lesson Management
	// ═══════════════════════════════════════════════════════════════════════════

	function addModule() {
		const newModule: Module = {
			id: generateId(),
			title: '',
			description: '',
			order: course.modules.length,
			lessons: [],
			duration_minutes: 0,
			is_preview: false
		};

		course.modules = [...course.modules, newModule];
		expandedModules.add(newModule.id);
		hasUnsavedChanges = true;

		// Auto-add first lesson
		setTimeout(() => addLesson(newModule.id), 100);
	}

	function removeModule(moduleId: string) {
		if (!confirm('Remove this module and all its lessons? This cannot be undone.')) return;

		course.modules = course.modules.filter((m) => m.id !== moduleId);
		expandedModules.delete(moduleId);
		reorderModules();
		updateCourseDuration();
		hasUnsavedChanges = true;
	}

	function duplicateModule(moduleId: string) {
		const module = course.modules.find((m) => m.id === moduleId);
		if (!module) return;

		const duplicate: Module = {
			...module,
			id: generateId(),
			title: `${module.title} (Copy)`,
			order: course.modules.length,
			lessons: module.lessons.map((l) => ({
				...l,
				id: generateId()
			}))
		};

		course.modules = [...course.modules, duplicate];
		expandedModules.add(duplicate.id);
		hasUnsavedChanges = true;
		updateCourseDuration();
	}

	function addLesson(moduleId: string) {
		const module = course.modules.find((m) => m.id === moduleId);
		if (!module) return;

		const newLesson: Lesson = {
			id: generateId(),
			title: '',
			type: 'video',
			content: '',
			duration_minutes: 0,
			is_preview: false,
			order: module.lessons.length,
			resources: []
		};

		module.lessons = [...module.lessons, newLesson];
		course.modules = course.modules;
		hasUnsavedChanges = true;
	}

	function removeLesson(moduleId: string, lessonId: string) {
		const module = course.modules.find((m) => m.id === moduleId);
		if (!module) return;

		module.lessons = module.lessons.filter((l) => l.id !== lessonId);
		updateModuleDuration(module);
		course.modules = course.modules;
		hasUnsavedChanges = true;
	}

	function duplicateLesson(moduleId: string, lessonId: string) {
		const module = course.modules.find((m) => m.id === moduleId);
		if (!module) return;

		const lesson = module.lessons.find((l) => l.id === lessonId);
		if (!lesson) return;

		const duplicate: Lesson = {
			...lesson,
			id: generateId(),
			title: `${lesson.title} (Copy)`,
			order: module.lessons.length
		};

		module.lessons = [...module.lessons, duplicate];
		updateModuleDuration(module);
		course.modules = course.modules;
		hasUnsavedChanges = true;
	}

	function reorderModules() {
		course.modules = course.modules.map((m, i) => ({ ...m, order: i }));
	}

	function reorderLessons(module: Module) {
		module.lessons = module.lessons.map((l, i) => ({ ...l, order: i }));
	}

	function updateModuleDuration(module: Module) {
		module.duration_minutes = module.lessons.reduce((sum, l) => sum + (l.duration_minutes || 0), 0);
		updateCourseDuration();
	}

	function updateCourseDuration() {
		const totalMinutes = course.modules.reduce((sum, m) => sum + m.duration_minutes, 0);
		course.duration_hours = Math.round((totalMinutes / 60) * 10) / 10;
	}

	function toggleModuleExpansion(moduleId: string) {
		if (expandedModules.has(moduleId)) {
			expandedModules.delete(moduleId);
		} else {
			expandedModules.add(moduleId);
		}
		expandedModules = expandedModules;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Advanced Drag and Drop System
	// ═══════════════════════════════════════════════════════════════════════════

	function handleDragStart(event: DragEvent, item: Module | Lesson, type: 'module' | 'lesson') {
		isDragging = true;
		if (type === 'module') {
			draggedModule = item as Module;
		} else {
			draggedLesson = item as Lesson;
		}

		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			event.dataTransfer.setData('text/plain', ''); // Firefox requirement
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDragEnd() {
		isDragging = false;
		draggedModule = null;
		draggedLesson = null;
	}

	function handleModuleDrop(event: DragEvent, targetIndex: number) {
		event.preventDefault();
		event.stopPropagation();

		if (draggedModule) {
			const fromIndex = course.modules.findIndex((m) => m.id === draggedModule!.id);
			if (fromIndex !== targetIndex && fromIndex !== -1) {
				const newModules = [...course.modules];
				newModules.splice(fromIndex, 1);
				newModules.splice(targetIndex, 0, draggedModule);
				course.modules = newModules;
				reorderModules();
				hasUnsavedChanges = true;
			}
		}

		handleDragEnd();
	}

	function handleLessonDrop(event: DragEvent, moduleId: string, targetIndex: number) {
		event.preventDefault();
		event.stopPropagation();

		if (draggedLesson) {
			const module = course.modules.find((m) => m.id === moduleId);
			if (module) {
				const fromIndex = module.lessons.findIndex((l) => l.id === draggedLesson!.id);
				if (fromIndex !== targetIndex && fromIndex !== -1) {
					const newLessons = [...module.lessons];
					newLessons.splice(fromIndex, 1);
					newLessons.splice(targetIndex, 0, draggedLesson);
					module.lessons = newLessons;
					reorderLessons(module);
					course.modules = course.modules;
					hasUnsavedChanges = true;
				}
			}
		}

		handleDragEnd();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete AI Features System
	// ═══════════════════════════════════════════════════════════════════════════

	async function generateWithAI(field: string) {
		generating = true;

		setTimeout(() => {
			switch (field) {
				case 'title':
					course.name = 'Master Technical Analysis: Professional Trading Strategies';
					generateSlug();
					break;

				case 'description':
					course.description = `Master the art of technical analysis with this comprehensive course designed for serious traders. 
					
					This professional-grade training program covers everything from basic chart patterns to advanced trading strategies used by Wall Street professionals. You'll learn how to read market sentiment, identify high-probability setups, and manage risk like a pro.
					
					Through ${course.modules.length || 5} comprehensive modules and over ${course.duration_hours || 10} hours of content, you'll develop the skills needed to trade with confidence in any market condition. Each lesson includes practical examples, real-world case studies, and actionable strategies you can implement immediately.
					
					Whether you're a beginner looking to build a solid foundation or an experienced trader seeking to refine your skills, this course provides the knowledge and tools needed to succeed in today's dynamic markets.`;
					break;

				case 'outcomes':
					course.outcomes = [
						'Master 15+ technical indicators and their practical applications',
						'Identify and trade 20+ chart patterns with confidence',
						'Develop personalized trading strategies based on your risk tolerance',
						'Implement professional risk management techniques to protect capital',
						'Understand market psychology and control emotional trading',
						'Create automated trading systems and alerts',
						'Analyze multiple timeframes for better entry and exit points',
						'Build and backtest your own trading strategies'
					];
					break;

				case 'curriculum':
					course.modules = [
						{
							id: generateId(),
							title: 'Foundation: Understanding Technical Analysis',
							description: 'Build a solid foundation in technical analysis principles',
							order: 0,
							lessons: [
								{
									id: generateId(),
									title: 'Welcome & Course Overview',
									type: 'video',
									duration_minutes: 10,
									is_preview: true,
									order: 0
								},
								{
									id: generateId(),
									title: 'What is Technical Analysis?',
									type: 'video',
									duration_minutes: 15,
									is_preview: true,
									order: 1
								},
								{
									id: generateId(),
									title: 'Chart Types: Candlestick, Bar, Line',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 2
								},
								{
									id: generateId(),
									title: 'Timeframes and Their Importance',
									type: 'video',
									duration_minutes: 15,
									is_preview: false,
									order: 3
								},
								{
									id: generateId(),
									title: 'Quiz: Foundation Concepts',
									type: 'quiz',
									duration_minutes: 10,
									is_preview: false,
									order: 4
								}
							],
							duration_minutes: 70,
							is_preview: false
						},
						{
							id: generateId(),
							title: 'Essential Technical Indicators',
							description: 'Master the most important technical indicators',
							order: 1,
							lessons: [
								{
									id: generateId(),
									title: 'Moving Averages (SMA, EMA, WMA)',
									type: 'video',
									duration_minutes: 30,
									is_preview: false,
									order: 0
								},
								{
									id: generateId(),
									title: 'RSI - Relative Strength Index',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 1
								},
								{
									id: generateId(),
									title: 'MACD - Complete Guide',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 2
								},
								{
									id: generateId(),
									title: 'Bollinger Bands Strategy',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 3
								},
								{
									id: generateId(),
									title: 'Volume Analysis & OBV',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 4
								},
								{
									id: generateId(),
									title: 'Assignment: Indicator Application',
									type: 'assignment',
									duration_minutes: 30,
									is_preview: false,
									order: 5
								}
							],
							duration_minutes: 150,
							is_preview: false
						},
						{
							id: generateId(),
							title: 'Chart Patterns & Price Action',
							description: 'Identify and trade profitable chart patterns',
							order: 2,
							lessons: [
								{
									id: generateId(),
									title: 'Support and Resistance Masterclass',
									type: 'video',
									duration_minutes: 35,
									is_preview: false,
									order: 0
								},
								{
									id: generateId(),
									title: 'Trend Lines and Channels',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 1
								},
								{
									id: generateId(),
									title: 'Head and Shoulders Pattern',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 2
								},
								{
									id: generateId(),
									title: 'Double Tops and Bottoms',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 3
								},
								{
									id: generateId(),
									title: 'Triangle Patterns',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 4
								},
								{
									id: generateId(),
									title: 'Flag and Pennant Patterns',
									type: 'video',
									duration_minutes: 15,
									is_preview: false,
									order: 5
								}
							],
							duration_minutes: 140,
							is_preview: false
						},
						{
							id: generateId(),
							title: 'Risk Management & Trading Psychology',
							description: 'Professional risk management and mindset training',
							order: 3,
							lessons: [
								{
									id: generateId(),
									title: 'Position Sizing Strategies',
									type: 'video',
									duration_minutes: 30,
									is_preview: false,
									order: 0
								},
								{
									id: generateId(),
									title: 'Stop Loss Placement Techniques',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 1
								},
								{
									id: generateId(),
									title: 'Risk-Reward Ratios',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 2
								},
								{
									id: generateId(),
									title: 'Trading Psychology Fundamentals',
									type: 'video',
									duration_minutes: 30,
									is_preview: false,
									order: 3
								},
								{
									id: generateId(),
									title: 'Dealing with Losses',
									type: 'video',
									duration_minutes: 20,
									is_preview: false,
									order: 4
								},
								{
									id: generateId(),
									title: 'Building a Trading Plan',
									type: 'assignment',
									duration_minutes: 45,
									is_preview: false,
									order: 5
								}
							],
							duration_minutes: 170,
							is_preview: false
						},
						{
							id: generateId(),
							title: 'Advanced Strategies & Live Trading',
							description: 'Put it all together with advanced strategies',
							order: 4,
							lessons: [
								{
									id: generateId(),
									title: 'Multi-Timeframe Analysis',
									type: 'video',
									duration_minutes: 35,
									is_preview: false,
									order: 0
								},
								{
									id: generateId(),
									title: 'Divergence Trading Strategy',
									type: 'video',
									duration_minutes: 30,
									is_preview: false,
									order: 1
								},
								{
									id: generateId(),
									title: 'Fibonacci Retracements',
									type: 'video',
									duration_minutes: 25,
									is_preview: false,
									order: 2
								},
								{
									id: generateId(),
									title: 'Live Trading Session #1',
									type: 'live',
									duration_minutes: 60,
									is_preview: false,
									order: 3
								},
								{
									id: generateId(),
									title: 'Live Trading Session #2',
									type: 'live',
									duration_minutes: 60,
									is_preview: false,
									order: 4
								},
								{
									id: generateId(),
									title: 'Final Project: Complete Strategy',
									type: 'assignment',
									duration_minutes: 60,
									is_preview: false,
									order: 5
								}
							],
							duration_minutes: 270,
							is_preview: false
						}
					];
					updateCourseDuration();
					break;

				case 'prerequisites':
					course.prerequisites = [
						'Basic understanding of financial markets',
						'Access to a trading platform or charting software',
						'Commitment to practice and apply learned concepts',
						'Basic math skills (percentages, ratios)',
						'Computer with internet connection'
					];
					break;

				case 'target':
					course.target_audience = [
						'Beginner traders looking to build a solid foundation',
						'Intermediate traders wanting to improve their strategy',
						'Investors seeking to time their entries and exits better',
						'Day traders and swing traders',
						'Anyone interested in technical market analysis'
					];
					break;

				case 'seo':
					course.meta_title = `${course.name || 'Technical Analysis Course'} - Learn Professional Trading`;
					course.meta_description = `Master technical analysis with our comprehensive course. Learn ${course.modules.length} modules covering indicators, patterns, and strategies. Join ${Math.floor(Math.random() * 5000) + 1000}+ students.`;
					course.keywords = [
						'technical analysis',
						'trading course',
						'chart patterns',
						'trading strategies',
						'risk management',
						'stock trading',
						'forex trading',
						'cryptocurrency trading'
					];
					break;
			}

			generating = false;
			hasUnsavedChanges = true;
			validateAll();
		}, 1500);
	}

	async function suggestPricing() {
		analyzing = true;

		setTimeout(() => {
			const analysis = {
				marketAverage: 297,
				yourContent: course.duration_hours,
				competitors: [
					{ name: 'Basic TA Course', price: 97, hours: 5 },
					{ name: 'Intermediate Trading', price: 297, hours: 10 },
					{ name: 'Pro Trader Bootcamp', price: 497, hours: 15 },
					{ name: 'Master Trader Program', price: 997, hours: 25 }
				],
				suggestedPrice: 0
			};

			// Calculate suggested price based on content
			if (course.duration_hours <= 5) {
				analysis.suggestedPrice = 97;
			} else if (course.duration_hours <= 10) {
				analysis.suggestedPrice = 297;
			} else if (course.duration_hours <= 20) {
				analysis.suggestedPrice = 497;
			} else {
				analysis.suggestedPrice = 797;
			}

			// Show pricing analysis in-app instead of alert
			pricingAnalysis = {
				show: true,
				marketAverage: analysis.marketAverage,
				suggestedPrice: analysis.suggestedPrice,
				competitors: analysis.competitors
			};

			analyzing = false;
		}, 2000);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete File Upload System
	// ═══════════════════════════════════════════════════════════════════════════

	let uploadError = $state('');
	let formError = $state('');
	let successMessage = $state('');
	let pricingAnalysis = $state<{
		show: boolean;
		marketAverage: number;
		suggestedPrice: number;
		competitors: { name: string; price: number; hours: number }[];
	} | null>(null);
	let showPublishWarning = $state(false);

	/**
	 * Resize image to max dimensions while maintaining aspect ratio
	 * Converts to optimized JPEG format for faster uploads
	 */
	async function resizeImage(
		file: File,
		maxWidth = 1200,
		maxHeight = 1200,
		quality = 0.85
	): Promise<Blob> {
		return new Promise((resolve, reject) => {
			const img = new Image();
			const canvas = document.createElement('canvas');
			const ctx = canvas.getContext('2d');

			img.onload = () => {
				let { width, height } = img;

				// Calculate new dimensions maintaining aspect ratio
				if (width > maxWidth || height > maxHeight) {
					const ratio = Math.min(maxWidth / width, maxHeight / height);
					width = Math.round(width * ratio);
					height = Math.round(height * ratio);
				}

				canvas.width = width;
				canvas.height = height;

				if (ctx) {
					// Use high-quality image smoothing
					ctx.imageSmoothingEnabled = true;
					ctx.imageSmoothingQuality = 'high';
					ctx.drawImage(img, 0, 0, width, height);

					canvas.toBlob(
						(blob) => {
							if (blob) {
								resolve(blob);
							} else {
								reject(new Error('Failed to create blob from canvas'));
							}
						},
						'image/jpeg',
						quality
					);
				} else {
					reject(new Error('Could not get canvas context'));
				}
			};

			img.onerror = () => reject(new Error('Failed to load image'));
			img.src = URL.createObjectURL(file);
		});
	}

	async function handleImageUpload(event: Event, type: 'thumbnail' | 'gallery' | 'og') {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) return;

		uploadError = '';

		// Validate file type
		if (!file.type.startsWith('image/')) {
			uploadError = 'Please select an image file (JPEG, PNG, WebP, etc.)';
			return;
		}

		// Validate file size (max 50MB before resize)
		if (file.size > 50 * 1024 * 1024) {
			uploadError = 'Image must be less than 50MB';
			return;
		}

		uploading = true;

		try {
			// Resize image before upload (max 1200px, converts to JPEG)
			const resizedBlob = await resizeImage(file, 1200, 1200, 0.85);
			const resizedFile = new File([resizedBlob], file.name.replace(/\.[^.]+$/, '.jpg'), {
				type: 'image/jpeg'
			});

			const formData = new FormData();
			formData.append('file', resizedFile);

			const response = await adminFetch('/api/admin/media/upload', {
				method: 'POST',
				body: formData
			});

			// The response contains an array of uploaded files
			if (response.success && response.data && response.data.length > 0) {
				const url = response.data[0].url;

				switch (type) {
					case 'thumbnail':
						course.thumbnail = url;
						break;
					case 'gallery':
						course.gallery = [...course.gallery, url];
						break;
					case 'og':
						course.og_image = url;
						break;
				}

				hasUnsavedChanges = true;
				uploadError = '';
				validateAll();
			} else {
				throw new Error(response.message || 'Upload failed - no URL returned');
			}
		} catch (error: any) {
			console.error('Failed to upload image:', error);
			uploadError = error.message || 'Failed to upload image. Please try again.';

			// Fallback: Use local preview if server upload fails
			const url = URL.createObjectURL(file);
			switch (type) {
				case 'thumbnail':
					course.thumbnail = url;
					break;
				case 'gallery':
					course.gallery = [...course.gallery, url];
					break;
				case 'og':
					course.og_image = url;
					break;
			}
			hasUnsavedChanges = true;
		} finally {
			uploading = false;
		}
	}

	async function handleVideoUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) return;

		uploadError = '';

		if (!file.type.startsWith('video/')) {
			uploadError = 'Please select a video file (MP4, WebM, etc.)';
			return;
		}

		// Validate file size (max 50MB for direct upload)
		if (file.size > 50 * 1024 * 1024) {
			uploadError = 'Video must be less than 50MB for direct upload';
			return;
		}

		uploading = true;

		try {
			const formData = new FormData();
			formData.append('file', file);

			const response = await adminFetch('/api/admin/media/upload', {
				method: 'POST',
				body: formData
			});

			// The response contains an array of uploaded files
			if (response.success && response.data && response.data.length > 0) {
				course.promo_video = response.data[0].url;
				hasUnsavedChanges = true;
				uploadError = '';
			} else {
				throw new Error(response.message || 'Upload failed - no URL returned');
			}
		} catch (error: any) {
			console.error('Failed to upload video:', error);
			uploadError = error.message || 'Failed to upload video. Please try again.';

			// Fallback: Use local preview if server upload fails
			course.promo_video = URL.createObjectURL(file);
			hasUnsavedChanges = true;
		} finally {
			uploading = false;
		}
	}

	function removeFromGallery(index: number) {
		course.gallery = course.gallery.filter((_, i) => i !== index);
		hasUnsavedChanges = true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete Validation System
	// ═══════════════════════════════════════════════════════════════════════════

	function validateAll() {
		let totalScore = 0;
		let maxScore = 0;
		validationResults = [];

		// Title validation (20 points)
		maxScore += 20;
		if (course.name.length >= 50) {
			totalScore += 20;
			validationResults.push({
				field: 'title',
				status: 'good',
				message: 'Excellent title length!',
				score: 20
			});
		} else if (course.name.length >= 20) {
			totalScore += 15;
			validationResults.push({
				field: 'title',
				status: 'warning',
				message: 'Good title, consider making it more descriptive',
				score: 15
			});
		} else if (course.name.length > 0) {
			totalScore += 5;
			validationResults.push({
				field: 'title',
				status: 'error',
				message: 'Title is too short',
				score: 5
			});
		} else {
			validationResults.push({
				field: 'title',
				status: 'error',
				message: 'Title is required',
				score: 0
			});
		}

		// Description validation (20 points)
		maxScore += 20;
		const wordCount = course.description.split(/\s+/).filter((w) => w.length > 0).length;
		if (wordCount >= 200) {
			totalScore += 20;
			validationResults.push({
				field: 'description',
				status: 'good',
				message: 'Comprehensive description!',
				score: 20
			});
		} else if (wordCount >= 100) {
			totalScore += 15;
			validationResults.push({
				field: 'description',
				status: 'warning',
				message: `${wordCount} words - add more detail`,
				score: 15
			});
		} else if (wordCount > 0) {
			totalScore += 5;
			validationResults.push({
				field: 'description',
				status: 'error',
				message: `Only ${wordCount} words - needs more content`,
				score: 5
			});
		} else {
			validationResults.push({
				field: 'description',
				status: 'error',
				message: 'Description is required',
				score: 0
			});
		}

		// Price validation (15 points)
		maxScore += 15;
		if (course.pricing_model === 'free') {
			totalScore += 15;
			validationResults.push({
				field: 'price',
				status: 'good',
				message: 'Free course configured',
				score: 15
			});
		} else if (course.price >= 97) {
			totalScore += 15;
			validationResults.push({
				field: 'price',
				status: 'good',
				message: 'Good pricing strategy',
				score: 15
			});
		} else if (course.price > 0) {
			totalScore += 10;
			validationResults.push({
				field: 'price',
				status: 'warning',
				message: 'Consider higher pricing for premium content',
				score: 10
			});
		} else {
			validationResults.push({
				field: 'price',
				status: 'error',
				message: 'Set a price for your course',
				score: 0
			});
		}

		// Curriculum validation (25 points)
		maxScore += 25;
		const totalLessons = course.modules.reduce((sum, m) => sum + m.lessons.length, 0);
		if (course.modules.length >= 5 && totalLessons >= 20) {
			totalScore += 25;
			validationResults.push({
				field: 'curriculum',
				status: 'good',
				message: `Excellent! ${course.modules.length} modules, ${totalLessons} lessons`,
				score: 25
			});
		} else if (course.modules.length >= 3 && totalLessons >= 10) {
			totalScore += 20;
			validationResults.push({
				field: 'curriculum',
				status: 'warning',
				message: `Good structure: ${course.modules.length} modules, ${totalLessons} lessons`,
				score: 20
			});
		} else if (course.modules.length > 0) {
			totalScore += 10;
			validationResults.push({
				field: 'curriculum',
				status: 'error',
				message: 'Add more modules and lessons',
				score: 10
			});
		} else {
			validationResults.push({
				field: 'curriculum',
				status: 'error',
				message: 'No curriculum added',
				score: 0
			});
		}

		// Media validation (10 points)
		maxScore += 10;
		if (course.thumbnail && course.promo_video) {
			totalScore += 10;
			validationResults.push({
				field: 'media',
				status: 'good',
				message: 'All media uploaded',
				score: 10
			});
		} else if (course.thumbnail) {
			totalScore += 7;
			validationResults.push({
				field: 'media',
				status: 'warning',
				message: 'Add a promotional video',
				score: 7
			});
		} else {
			validationResults.push({
				field: 'media',
				status: 'error',
				message: 'Upload course thumbnail',
				score: 0
			});
		}

		// SEO validation (10 points)
		maxScore += 10;
		if (course.meta_title && course.meta_description && course.keywords.length >= 5) {
			totalScore += 10;
			validationResults.push({
				field: 'seo',
				status: 'good',
				message: 'SEO optimized',
				score: 10
			});
		} else if (course.meta_title || course.meta_description) {
			totalScore += 5;
			validationResults.push({
				field: 'seo',
				status: 'warning',
				message: 'Complete SEO settings',
				score: 5
			});
		} else {
			validationResults.push({
				field: 'seo',
				status: 'warning',
				message: 'Add SEO metadata',
				score: 0
			});
		}

		overallScore = Math.round((totalScore / maxScore) * 100);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Complete Save & Load System
	// ═══════════════════════════════════════════════════════════════════════════

	function generateSlug() {
		if (!course.slug && course.name) {
			course.slug = course.name
				.toLowerCase()
				.trim()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	}

	function saveDraft() {
		const draft = {
			course,
			timestamp: new Date().toISOString(),
			version: '1.0'
		};

		localStorage.setItem('course-draft', JSON.stringify(draft));
		localStorage.setItem('course-draft-date', new Date().toISOString());
		lastSaved = new Date();
		hasUnsavedChanges = false;

		// Show save notification
		showNotification('Draft saved successfully');
	}

	function loadDraft() {
		const draftStr = localStorage.getItem('course-draft');
		const draftDate = localStorage.getItem('course-draft-date');

		if (draftStr && draftDate) {
			try {
				const draft = JSON.parse(draftStr);
				const date = new Date(draftDate);

				if (confirm(`Load draft from ${date.toLocaleString()}?`)) {
					course = draft.course || draft;
					validateAll();
				}
			} catch (e) {
				console.error('Failed to load draft:', e);
			}
		}
	}

	async function saveCourse() {
		formError = '';
		successMessage = '';

		// Final validation
		if (!course.name) {
			formError = 'Please enter a course title';
			activeTab = 'basic';
			return;
		}

		if (course.pricing_model !== 'free' && !course.price) {
			formError = 'Please set a price for your course';
			activeTab = 'pricing';
			return;
		}

		if (course.modules.length === 0) {
			formError = 'Please add at least one module';
			activeTab = 'curriculum';
			return;
		}

		saving = true;
		validateAll();

		// Simulate API call
		try {
			const payload: any = {
				name: course.name,
				slug: course.slug || undefined,
				type: 'course',
				description: course.short_description || course.description,
				long_description: course.description,
				price: course.pricing_model === 'free' ? 0 : course.price,
				is_active: course.is_active,
				thumbnail: course.thumbnail || undefined,
				meta_title: course.meta_title || undefined,
				meta_description: course.meta_description || undefined,
				indexable: true,
				metadata: {
					short_description: course.short_description,
					gallery: course.gallery,
					promo_video: course.promo_video,
					course_type: course.type,
					format: course.format,
					level: course.level,
					duration_hours: course.duration_hours,
					modules: course.modules,
					pricing_model: course.pricing_model,
					currency: course.currency,
					subscription_interval: course.subscription_interval,
					payment_installments: course.payment_installments,
					early_bird: course.early_bird,
					access: {
						start_date: course.start_date,
						end_date: course.end_date,
						enrollment_limit: course.enrollment_limit,
						lifetime_access: course.lifetime_access,
						certificate_enabled: course.certificate_enabled,
						drip_schedule: course.drip_schedule,
						prerequisites: course.prerequisites
					},
					seo: {
						keywords: course.keywords,
						og_image: course.og_image,
						landing_page_enabled: course.landing_page_enabled
					},
					marketing: {
						affiliate_enabled: course.affiliate_enabled,
						affiliate_commission: course.affiliate_commission
					},
					analytics: {
						ga4_enabled: course.ga4_enabled,
						fb_pixel_enabled: course.fb_pixel_enabled,
						conversion_tracking: course.conversion_tracking,
						utm_source: course.utm_source,
						utm_medium: course.utm_medium,
						utm_campaign: course.utm_campaign
					},
					content: {
						features: course.features,
						outcomes: course.outcomes,
						target_audience: course.target_audience,
						tools_required: course.tools_required
					},
					advanced: {
						is_featured: course.is_featured,
						allow_comments: course.allow_comments,
						allow_reviews: course.allow_reviews,
						show_progress: course.show_progress,
						completion_threshold: course.completion_threshold,
						max_attempts: course.max_attempts,
						passing_score: course.passing_score
					},
					bonuses: course.bonuses,
					resources: course.resources
				}
			};

			await productsApi.create(payload);

			// Clear draft after successful save
			localStorage.removeItem('course-draft');
			localStorage.removeItem('course-draft-date');

			// Show success and redirect
			successMessage = 'Course created successfully! Redirecting...';
			setTimeout(() => goto('/admin/courses'), 1500);
		} catch (error: any) {
			if (error instanceof AdminApiError) {
				if (error.status === 401) {
					goto('/login');
					return;
				}

				if (error.isValidationError && error.errors) {
					const firstField = Object.keys(error.errors)[0];
					if (firstField) {
						const firstMessage = error.errors[firstField]?.[0];
						formError = firstMessage || error.message;
					} else {
						formError = error.message;
					}
				} else {
					formError = error.message;
				}
			} else {
				formError = error.message || 'Failed to save course. Please try again.';
			}
			console.error('Save error:', error);
		} finally {
			saving = false;
		}
	}

	async function publishCourse() {
		if (overallScore < 70) {
			showPublishWarning = true;
			return;
		}

		course.is_active = true;
		await saveCourse();
	}

	async function confirmPublish() {
		showPublishWarning = false;
		course.is_active = true;
		await saveCourse();
	}

	function cancelPublish() {
		showPublishWarning = false;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utility Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function generateId(): string {
		return `${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
	}

	function addBonus() {
		course.bonuses = [
			...course.bonuses,
			{
				id: generateId(),
				title: '',
				description: '',
				value: 0,
				icon: '🎁',
				type: 'pdf'
			}
		];
		hasUnsavedChanges = true;
	}

	function removeBonus(id: string) {
		course.bonuses = course.bonuses.filter((b) => b.id !== id);
		hasUnsavedChanges = true;
	}

	function addOutcome() {
		course.outcomes = [...course.outcomes, ''];
		hasUnsavedChanges = true;
	}

	function removeOutcome(index: number) {
		course.outcomes = course.outcomes.filter((_, i) => i !== index);
		hasUnsavedChanges = true;
	}

	function addPrerequisite() {
		course.prerequisites = [...course.prerequisites, ''];
		hasUnsavedChanges = true;
	}

	function removePrerequisite(index: number) {
		course.prerequisites = course.prerequisites.filter((_, i) => i !== index);
		hasUnsavedChanges = true;
	}

	function addKeyword(event: KeyboardEvent) {
		const input = event.target as HTMLInputElement;
		if (event.key === 'Enter' && input.value.trim()) {
			course.keywords = [...course.keywords, input.value.trim()];
			input.value = '';
			hasUnsavedChanges = true;
		}
	}

	function removeKeyword(index: number) {
		course.keywords = course.keywords.filter((_, i) => i !== index);
		hasUnsavedChanges = true;
	}

	function showNotification(message: string) {
		// Implementation would show toast notification
		console.log('Notification:', message);
	}

	function getCompletionStatus() {
		const checks = {
			'Title & Description': course.name.length > 0 && course.description.length > 0,
			Thumbnail: !!course.thumbnail,
			Pricing: course.price > 0 || course.pricing_model === 'free',
			Curriculum: course.modules.length >= 3,
			Lessons: course.modules.reduce((sum, m) => sum + m.lessons.length, 0) >= 10,
			Outcomes: course.outcomes.length >= 3,
			SEO: course.meta_title && course.meta_description,
			Duration: course.duration_hours > 0
		};

		const completed = Object.values(checks).filter(Boolean).length;
		const total = Object.keys(checks).length;

		return {
			completed,
			total,
			percentage: Math.round((completed / total) * 100),
			checks
		};
	}

	function getLessonIcon(type: string): string {
		const icons: Record<string, string> = {
			video: '📹',
			text: '📄',
			quiz: '📊',
			assignment: '📝',
			live: '🔴',
			download: '📥'
		};
		return icons[type] || '📚';
	}

	function getTotalValue(): number {
		const basePrice = course.price || 0;
		const bonusValue = course.bonuses.reduce((sum, b) => sum + (b.value || 0), 0);
		return basePrice + bonusValue;
	}

	// Reactive statements
	let completionStatus = $derived(getCompletionStatus());
	$effect(() => {
		if (course.modules) updateCourseDuration();
	});
	$effect(() => {
		validateAll();
	});
	let totalValue = $derived(getTotalValue());
</script>

<!-- Complete HTML Template -->
<svelte:head>
	<title>Create Course | Enterprise Admin</title>
</svelte:head>

<!-- Modals and Notifications -->
{#if showPublishWarning}
	<div class="modal-overlay">
		<div class="modal-content">
			<h3>Publish Warning</h3>
			<p>Your course quality score is below 70%. Are you sure you want to publish?</p>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={cancelPublish}>Cancel</button>
				<button class="btn-primary" onclick={confirmPublish}>Publish Anyway</button>
			</div>
		</div>
	</div>
{/if}

{#if pricingAnalysis?.show}
	<div class="modal-overlay">
		<div class="modal-content pricing-modal">
			<h3>AI Pricing Analysis Complete</h3>
			<div class="pricing-details">
				<p><strong>Your Course:</strong> {course.duration_hours} hours of content</p>
				<p><strong>Market Average:</strong> ${pricingAnalysis.marketAverage}</p>

				<h4>Competitor Analysis</h4>
				<ul>
					{#each pricingAnalysis.competitors as comp}
						<li>{comp.name}: ${comp.price} ({comp.hours}h)</li>
					{/each}
				</ul>

				<div class="suggested-price">
					<span>Recommended Price:</span>
					<strong>${pricingAnalysis.suggestedPrice}</strong>
				</div>

				<h4>Pricing Strategies</h4>
				<ul>
					<li>One-time: ${pricingAnalysis.suggestedPrice}</li>
					<li>Payment plan: 3 × ${Math.ceil((pricingAnalysis.suggestedPrice / 3) * 1.1)}</li>
					<li>Subscription: ${Math.ceil(pricingAnalysis.suggestedPrice / 6)}/month</li>
					<li>Early bird: ${Math.ceil(pricingAnalysis.suggestedPrice * 0.7)} (30% off)</li>
				</ul>
			</div>
			<div class="modal-actions">
				<button class="btn-primary" onclick={() => (pricingAnalysis = null)}>Got it</button>
			</div>
		</div>
	</div>
{/if}

<div class="create-page">
	<!-- Form Messages -->
	{#if formError}
		<div class="form-error-banner">
			<IconAlertCircle size={20} />
			<span>{formError}</span>
			<button onclick={() => (formError = '')}><IconX size={16} /></button>
		</div>
	{/if}

	{#if successMessage}
		<div class="form-success-banner">
			<IconCheck size={20} />
			<span>{successMessage}</span>
		</div>
	{/if}
	<!-- Centered Page Header -->
	<div class="page-header">
		<h1>Create New Course</h1>
		<p class="subtitle">Build a comprehensive educational trading course</p>
	</div>

	<!-- Actions Row - Centered -->
	<div class="header-actions">
		{#if hasUnsavedChanges}
			<span class="unsaved-indicator">
				<IconAlertCircle size={16} />
				Unsaved
			</span>
		{/if}

		{#if lastSaved}
			<span class="save-status">
				<IconCheck size={16} />
				Saved {lastSaved.toLocaleTimeString()}
			</span>
		{/if}

		<button class="btn-secondary" onclick={saveDraft} disabled={!hasUnsavedChanges}>
			<IconDownload size={18} />
			Save Draft
		</button>

		<button
			class="btn-secondary"
			onclick={() => window.open(`/preview/course/${course.slug || 'preview'}`, '_blank')}
		>
			<IconEye size={18} />
			Preview
		</button>

		<button
			class="btn-primary"
			onclick={publishCourse}
			disabled={saving || completionStatus.percentage < 30}
			title={completionStatus.percentage < 30
				? 'Complete at least 30% to publish'
				: 'Publish course'}
		>
			{#if saving}
				<IconRefresh size={18} class="spinning" />
				Publishing...
			{:else}
				<IconRocket size={18} />
				Publish Course
			{/if}
		</button>
	</div>

	<!-- Progress Bar -->
	<div class="completion-progress">
		<div class="progress-bar">
			<div class="progress-fill" style="width: {completionStatus.percentage}%"></div>
		</div>
		<div class="progress-details">
			<span class="progress-text">{completionStatus.percentage}% Complete</span>
			<span class="progress-items"
				>{completionStatus.completed}/{completionStatus.total} items • {course.modules.length} modules
				• {course.duration_hours}h</span
			>
		</div>
	</div>

	<!-- Main Content Area -->
	<div class="content-wrapper">
		<!-- Sidebar Navigation -->
		<aside class="sidebar">
			<nav class="sidebar-nav">
				<button
					class="nav-item"
					class:active={activeTab === 'basic'}
					onclick={() => (activeTab = 'basic')}
				>
					<IconBook size={20} />
					<span>Basic Info</span>
					{#if completionStatus.checks['Title & Description']}
						<IconCheck size={16} class="check" />
					{/if}
				</button>

				<button
					class="nav-item"
					class:active={activeTab === 'curriculum'}
					onclick={() => (activeTab = 'curriculum')}
				>
					<IconFileText size={20} />
					<span>Curriculum</span>
					{#if completionStatus.checks['Curriculum'] && completionStatus.checks['Lessons']}
						<IconCheck size={16} class="check" />
					{/if}
				</button>

				<button
					class="nav-item"
					class:active={activeTab === 'pricing'}
					onclick={() => (activeTab = 'pricing')}
				>
					<IconCurrencyDollar size={20} />
					<span>Pricing</span>
					{#if completionStatus.checks['Pricing']}
						<IconCheck size={16} class="check" />
					{/if}
				</button>

				<button
					class="nav-item"
					class:active={activeTab === 'media'}
					onclick={() => (activeTab = 'media')}
				>
					<IconPhoto size={20} />
					<span>Media</span>
					{#if completionStatus.checks['Thumbnail']}
						<IconCheck size={16} class="check" />
					{/if}
				</button>

				<button
					class="nav-item"
					class:active={activeTab === 'seo'}
					onclick={() => (activeTab = 'seo')}
				>
					<IconTarget size={20} />
					<span>SEO & Marketing</span>
					{#if completionStatus.checks['SEO']}
						<IconCheck size={16} class="check" />
					{/if}
				</button>

				<button
					class="nav-item"
					class:active={activeTab === 'advanced'}
					onclick={() => (activeTab = 'advanced')}
				>
					<IconSettings size={20} />
					<span>Advanced</span>
				</button>
			</nav>

			<!-- AI Assistant Panel -->
			<div class="ai-assistant">
				<h3>
					<IconSparkles size={20} />
					AI Assistant
				</h3>
				<div class="ai-actions">
					<button onclick={() => generateWithAI('description')} disabled={generating}>
						<IconBulb size={16} />
						Generate Description
					</button>
					<button onclick={() => generateWithAI('curriculum')} disabled={generating}>
						<IconBook size={16} />
						Build Curriculum
					</button>
					<button onclick={() => generateWithAI('outcomes')} disabled={generating}>
						<IconTarget size={16} />
						Create Outcomes
					</button>
					<button onclick={suggestPricing} disabled={analyzing}>
						<IconChartBar size={16} />
						Analyze Pricing
					</button>
					<button onclick={() => generateWithAI('seo')} disabled={generating}>
						<IconTarget size={16} />
						Optimize SEO
					</button>
				</div>
			</div>

			<!-- Quality Score Card -->
			<div class="validation-card">
				<h3>Quality Score</h3>
				<div class="score-circle">
					<svg viewBox="0 0 100 100">
						<circle cx="50" cy="50" r="45" fill="none" stroke="#1e293b" stroke-width="8" />
						<circle
							cx="50"
							cy="50"
							r="45"
							fill="none"
							stroke={overallScore > 70 ? '#10b981' : overallScore > 40 ? '#f59e0b' : '#ef4444'}
							stroke-width="8"
							stroke-dasharray="{overallScore * 2.83} 283"
							transform="rotate(-90 50 50)"
							style="transition: stroke-dasharray 0.5s ease"
						/>
					</svg>
					<div class="score-text">{overallScore}</div>
				</div>

				{#if validationResults.length > 0}
					<div class="validation-items">
						{#each validationResults as result}
							<div class="validation-item {result.status}">
								<span class="item-label">{result.message}</span>
								<span class="item-score">+{result.score}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</aside>

		<!-- Main Form Content Area -->
		<main class="main-content">
			<!-- Basic Info Tab -->
			{#if activeTab === 'basic'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="form-card">
						<h2>Course Information</h2>

						<div class="form-group">
							<label for="name">
								Course Title *
								<button
									class="ai-btn"
									onclick={() => generateWithAI('title')}
									disabled={generating}
								>
									<IconSparkles size={14} />
								</button>
							</label>
							<input
								id="name" name="name"
								type="text"
								bind:value={course.name}
								onblur={generateSlug}
								placeholder="e.g., Master Technical Analysis for Day Trading"
								class="input-large"
							/>
							<p class="help-text">
								Make it descriptive and benefit-focused (50+ characters recommended)
							</p>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="slug">URL Slug</label>
								<div class="input-group">
									<span class="input-prefix">/courses/</span>
									<input
										id="slug" name="slug"
										type="text"
										bind:value={course.slug}
										placeholder="master-technical-analysis"
									/>
								</div>
							</div>

							<div class="form-group">
								<label for="level">Difficulty Level</label>
								<select id="level" bind:value={course.level}>
									<option value="beginner">Beginner</option>
									<option value="intermediate">Intermediate</option>
									<option value="advanced">Advanced</option>
									<option value="all-levels">All Levels</option>
								</select>
							</div>
						</div>

						<div class="form-group">
							<label for="short-desc"> Short Description (Tagline) </label>
							<input
								id="short-desc" name="short-desc"
								type="text"
								bind:value={course.short_description}
								placeholder="Learn professional trading strategies used by Wall Street traders"
								maxlength="160"
							/>
							<span class="char-count">{course.short_description.length}/160</span>
						</div>

						<div class="form-group">
							<label for="description">
								Full Description
								<button
									class="ai-btn"
									onclick={() => generateWithAI('description')}
									disabled={generating}
								>
									<IconSparkles size={14} />
								</button>
							</label>
							<textarea
								id="description"
								bind:value={course.description}
								placeholder="Provide a detailed overview of what students will learn..."
								rows="10"
							></textarea>
							<p class="help-text">
								Aim for 200+ words. Include outcomes, who it's for, and what makes it unique.
							</p>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="type">Course Type</label>
								<select id="type" bind:value={course.type}>
									<option value="self-paced">Self-Paced</option>
									<option value="cohort">Cohort-Based</option>
									<option value="hybrid">Hybrid</option>
								</select>
							</div>

							<div class="form-group">
								<label for="format">Content Format</label>
								<select id="format" bind:value={course.format}>
									<option value="video">Video-Based</option>
									<option value="text">Text-Based</option>
									<option value="mixed">Mixed Media</option>
									<option value="live">Live Sessions</option>
								</select>
							</div>
						</div>

						{#if course.type === 'cohort' || course.type === 'hybrid'}
							<div class="form-row">
								<div class="form-group">
									<label for="start-date">Start Date</label>
									<input id="start-date" name="start-date" type="date" bind:value={course.start_date} />
								</div>

								<div class="form-group">
									<label for="end-date">End Date</label>
									<input id="end-date" name="end-date" type="date" bind:value={course.end_date} />
								</div>
							</div>
						{/if}
					</div>

					<!-- Learning Outcomes -->
					<div class="form-card">
						<h2>
							Learning Outcomes
							<button
								class="ai-btn"
								onclick={() => generateWithAI('outcomes')}
								disabled={generating}
							>
								<IconSparkles size={14} />
								AI Generate
							</button>
						</h2>

						<div class="list-editor">
							{#each course.outcomes as _, i}
								<div class="list-item">
									<IconCheck size={16} />
									<input
										id="page-course-outcomes-i" name="page-course-outcomes-i" type="text"
										bind:value={course.outcomes[i]}
										placeholder="Students will be able to..."
									/>
									<button onclick={() => removeOutcome(i)}>
										<IconX size={16} />
									</button>
								</div>
							{/each}
							<button class="add-item" onclick={addOutcome}>
								<IconPlus size={16} />
								Add Outcome
							</button>
						</div>
					</div>

					<!-- Prerequisites -->
					<div class="form-card">
						<h2>
							Prerequisites
							<button
								class="ai-btn"
								onclick={() => generateWithAI('prerequisites')}
								disabled={generating}
							>
								<IconSparkles size={14} />
							</button>
						</h2>

						<div class="list-editor">
							{#each course.prerequisites as _, i}
								<div class="list-item">
									<IconAlertCircle size={16} />
									<input
										id="page-course-prerequisites-i" name="page-course-prerequisites-i" type="text"
										bind:value={course.prerequisites[i]}
										placeholder="Required knowledge or skills..."
									/>
									<button onclick={() => removePrerequisite(i)}>
										<IconX size={16} />
									</button>
								</div>
							{/each}
							<button class="add-item" onclick={addPrerequisite}>
								<IconPlus size={16} />
								Add Prerequisite
							</button>
						</div>
					</div>

					<!-- Target Audience -->
					<div class="form-card">
						<h2>
							Target Audience
							<button class="ai-btn" onclick={() => generateWithAI('target')} disabled={generating}>
								<IconSparkles size={14} />
							</button>
						</h2>

						<div class="list-editor">
							{#each course.target_audience as _, i}
								<div class="list-item">
									<IconUsers size={16} />
									<input
										id="page-course-target-audience-i" name="page-course-target-audience-i" type="text"
										bind:value={course.target_audience[i]}
										placeholder="Who is this course for?"
									/>
									<button
										onclick={() =>
											(course.target_audience = course.target_audience.filter(
												(_, idx) => idx !== i
											))}
									>
										<IconX size={16} />
									</button>
								</div>
							{/each}
							<button
								class="add-item"
								onclick={() => (course.target_audience = [...course.target_audience, ''])}
							>
								<IconPlus size={16} />
								Add Audience
							</button>
						</div>
					</div>
				</div>
			{/if}

			<!-- Curriculum Tab -->
			{#if activeTab === 'curriculum'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="curriculum-header">
						<h2>Course Curriculum</h2>
						<div class="curriculum-stats">
							<span>
								<IconBook size={16} />
								{course.modules.length} Modules
							</span>
							<span>
								<IconFileText size={16} />
								{course.modules.reduce((sum, m) => sum + m.lessons.length, 0)} Lessons
							</span>
							<span>
								<IconClock size={16} />
								{course.duration_hours} Hours
							</span>
						</div>
						<button
							class="ai-btn-large"
							onclick={() => generateWithAI('curriculum')}
							disabled={generating}
						>
							<IconSparkles size={16} />
							Generate Sample Curriculum
						</button>
					</div>

					<div class="curriculum-builder">
						{#each course.modules as module, moduleIndex (module.id)}
							<div
								class="module-card"
								class:expanded={expandedModules.has(module.id)}
								animate:flip={{ duration: 300 }}
								draggable="true"
								role="listitem"
								ondragstart={(e: DragEvent) => handleDragStart(e, module, 'module')}
								ondragover={handleDragOver}
								ondrop={(e: DragEvent) => handleModuleDrop(e, moduleIndex)}
								ondragend={handleDragEnd}
								class:dragging={isDragging && draggedModule?.id === module.id}
							>
								<div class="module-header">
									<IconGripVertical size={20} class="drag-handle" />
									<span class="module-number">Module {moduleIndex + 1}</span>
									<input
										id="page-module-title" name="page-module-title" type="text"
										bind:value={module.title}
										placeholder="Enter module title..."
										class="module-title"
									/>
									<span class="module-stats">
										{module.lessons.length} lessons • {module.duration_minutes} min
									</span>
									<div class="module-actions">
										<button class="expand-btn" onclick={() => toggleModuleExpansion(module.id)}>
											<IconChevronDown
												size={18}
												style="transform: rotate({expandedModules.has(module.id)
													? 0
													: -90}deg); transition: transform 0.2s"
											/>
										</button>
										<button onclick={() => duplicateModule(module.id)} title="Duplicate">
											<IconCopy size={16} />
										</button>
										<button onclick={() => removeModule(module.id)} title="Delete">
											<IconTrash size={16} />
										</button>
									</div>
								</div>

								{#if expandedModules.has(module.id)}
									<div transition:slide={{ duration: 200 }}>
										<textarea
											bind:value={module.description}
											placeholder="Brief module description..."
											class="module-description"
											rows="2"
										></textarea>

										<div class="lessons-container">
											{#each module.lessons as lesson, lessonIndex (lesson.id)}
												<div
													class="lesson-item"
													animate:flip={{ duration: 200 }}
													draggable="true"
													role="listitem"
													ondragstart={(e: DragEvent) => handleDragStart(e, lesson, 'lesson')}
													ondragover={handleDragOver}
													ondrop={(e: DragEvent) => handleLessonDrop(e, module.id, lessonIndex)}
													ondragend={handleDragEnd}
												>
													<IconGripVertical size={16} class="lesson-drag" />
													<span class="lesson-number">{lessonIndex + 1}</span>
													<select bind:value={lesson.type} class="lesson-type">
														<option value="video">{getLessonIcon('video')} Video</option>
														<option value="text">{getLessonIcon('text')} Text</option>
														<option value="quiz">{getLessonIcon('quiz')} Quiz</option>
														<option value="assignment"
															>{getLessonIcon('assignment')} Assignment</option
														>
														<option value="live">{getLessonIcon('live')} Live Session</option>
														<option value="download">{getLessonIcon('download')} Resource</option>
													</select>
													<input
														id="page-lesson-title" name="page-lesson-title" type="text"
														bind:value={lesson.title}
														placeholder="Lesson title..."
														class="lesson-title"
													/>
													<input
														id="page-lesson-duration-minutes" name="page-lesson-duration-minutes" type="number"
														bind:value={lesson.duration_minutes}
														onchange={() => updateModuleDuration(module)}
														placeholder="Min"
														class="lesson-duration"
														min="0"
													/>
													<label class="preview-toggle" title="Allow free preview">
														<input id="page-lesson-is-preview" name="page-lesson-is-preview" type="checkbox" bind:checked={lesson.is_preview} />
														<IconEye size={14} />
													</label>
													<button
														onclick={() => duplicateLesson(module.id, lesson.id)}
														title="Duplicate"
													>
														<IconCopy size={14} />
													</button>
													<button onclick={() => removeLesson(module.id, lesson.id)} title="Delete">
														<IconX size={16} />
													</button>
												</div>
											{/each}
										</div>

										<button class="add-lesson" onclick={() => addLesson(module.id)}>
											<IconPlus size={16} />
											Add Lesson
										</button>

										<div class="module-footer">
											<label class="preview-module">
												<input id="page-module-is-preview" name="page-module-is-preview" type="checkbox" bind:checked={module.is_preview} />
												<IconEye size={14} />
												Allow module preview
											</label>
											<span class="module-duration">
												<IconClock size={14} />
												Total: {module.duration_minutes} minutes ({Math.round(
													(module.duration_minutes / 60) * 10
												) / 10} hours)
											</span>
										</div>
									</div>
								{/if}
							</div>
						{/each}

						<button class="add-module" onclick={addModule}>
							<IconPlus size={20} />
							Add Module
						</button>
					</div>

					<!-- Drip Schedule -->
					<div class="form-card">
						<h2>Content Drip Schedule</h2>
						<p class="help-text">Release modules gradually after enrollment</p>

						<div class="drip-schedule">
							{#each course.modules as module, i}
								<div class="drip-item">
									<span class="module-name">{module.title || `Module ${i + 1}`}</span>
									<select>
										<option value="0">Available immediately</option>
										<option value="7">After 1 week</option>
										<option value="14">After 2 weeks</option>
										<option value="30">After 1 month</option>
										<option value="60">After 2 months</option>
									</select>
								</div>
							{/each}
						</div>
					</div>
				</div>
			{/if}

			<!-- Pricing Tab -->
			{#if activeTab === 'pricing'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="form-card">
						<h2>
							Pricing Strategy
							<button class="analyze-btn" onclick={suggestPricing} disabled={analyzing}>
								<IconChartBar size={16} />
								AI Market Analysis
							</button>
						</h2>

						<div class="pricing-models">
							<label class="pricing-model" class:selected={course.pricing_model === 'one-time'}>
								<input id="page-radio" name="page-radio" type="radio" bind:group={course.pricing_model} value="one-time" />
								<div class="model-content">
									<IconCurrencyDollar size={24} />
									<span class="model-title">One-Time Payment</span>
									<span class="model-desc">Single payment for lifetime access</span>
								</div>
							</label>

							<label class="pricing-model" class:selected={course.pricing_model === 'subscription'}>
								<input id="page-radio" name="page-radio" type="radio" bind:group={course.pricing_model} value="subscription" />
								<div class="model-content">
									<IconRefresh size={24} />
									<span class="model-title">Subscription</span>
									<span class="model-desc">Recurring monthly payments</span>
								</div>
							</label>

							<label class="pricing-model" class:selected={course.pricing_model === 'payment-plan'}>
								<input id="page-radio" name="page-radio" type="radio" bind:group={course.pricing_model} value="payment-plan" />
								<div class="model-content">
									<IconCalendar size={24} />
									<span class="model-title">Payment Plan</span>
									<span class="model-desc">Split into installments</span>
								</div>
							</label>

							<label class="pricing-model" class:selected={course.pricing_model === 'free'}>
								<input id="page-radio" name="page-radio" type="radio" bind:group={course.pricing_model} value="free" />
								<div class="model-content">
									<IconGift size={24} />
									<span class="model-title">Free</span>
									<span class="model-desc">No payment required</span>
								</div>
							</label>
						</div>

						{#if course.pricing_model !== 'free'}
							<div class="form-row">
								<div class="form-group">
									<label for="price">Price ({course.currency})</label>
									<div class="price-input">
										<span class="currency">$</span>
										<input
											id="price" name="price"
											type="number"
											bind:value={course.price}
											placeholder="199.00"
											step="0.01"
											min="0"
										/>
									</div>
									<p class="help-text">
										Based on {course.duration_hours}h of content, similar courses price at $97-$497
									</p>
								</div>

								{#if course.pricing_model === 'subscription'}
									<div class="form-group">
										<label for="interval">Billing Interval</label>
										<select id="interval" bind:value={course.subscription_interval}>
											<option value="monthly">Monthly</option>
											<option value="quarterly">Quarterly</option>
											<option value="yearly">Yearly</option>
										</select>
									</div>
								{/if}

								{#if course.pricing_model === 'payment-plan'}
									<div class="form-group">
										<label for="installments">Number of Installments</label>
										<input
											id="installments" name="installments"
											type="number"
											bind:value={course.payment_installments}
											min="2"
											max="12"
										/>
										<p class="help-text">
											{course.payment_installments} payments of ${Math.ceil(
												(course.price * 1.1) / course.payment_installments
											)}
										</p>
									</div>
								{/if}
							</div>
						{/if}

						<!-- Early Bird Discount -->
						<div class="early-bird-section">
							<h3>Early Bird Discount</h3>
							<label class="toggle-label">
								<input id="page-course-early-bird-enabled" name="page-course-early-bird-enabled" type="checkbox" bind:checked={course.early_bird.enabled} />
								<span>Enable limited-time discount</span>
							</label>

							{#if course.early_bird?.enabled}
								<div class="form-row">
									<div class="form-group">
										<label for="early-bird-discount">Discount %</label>
										<input
											id="early-bird-discount" name="early-bird-discount"
											type="number"
											bind:value={course.early_bird.discount}
											min="5"
											max="50"
											step="5"
										/>
										<p class="help-text">
											Early bird price: ${Math.ceil(
												course.price * (1 - course.early_bird.discount / 100)
											)}
										</p>
									</div>
									<div class="form-group">
										<label for="early-bird-expires">Expires On</label>
										<input
											id="early-bird-expires" name="early-bird-expires"
											type="datetime-local"
											bind:value={course.early_bird.expires}
										/>
									</div>
								</div>
							{/if}
						</div>
					</div>

					<!-- Value Stack & Bonuses -->
					<div class="form-card">
						<h2>Value Stack & Bonuses</h2>
						<p class="help-text">Add bonuses to increase perceived value</p>

						<div class="bonus-list">
							{#each course.bonuses as bonus}
								<div class="bonus-item">
									<select bind:value={bonus.icon} class="bonus-icon">
										<option value="📚">📚</option>
										<option value="🎯">🎯</option>
										<option value="💎">💎</option>
										<option value="🚀">🚀</option>
										<option value="📊">📊</option>
										<option value="🎁">🎁</option>
									</select>
									<select bind:value={bonus.type} class="bonus-type">
										<option value="pdf">PDF Guide</option>
										<option value="video">Video</option>
										<option value="template">Template</option>
										<option value="access">Access</option>
										<option value="consultation">Consultation</option>
									</select>
									<input
										id="page-bonus-title" name="page-bonus-title" type="text"
										bind:value={bonus.title}
										placeholder="Bonus title..."
										class="bonus-title"
									/>
									<input
										id="page-bonus-value" name="page-bonus-value" type="number"
										bind:value={bonus.value}
										placeholder="Value"
										min="0"
										class="bonus-value"
									/>
									<button onclick={() => removeBonus(bonus.id)}>
										<IconX size={16} />
									</button>
								</div>
							{/each}

							<button class="add-item" onclick={addBonus}>
								<IconPlus size={16} />
								Add Bonus
							</button>
						</div>

						<div class="total-value">
							<div class="value-breakdown">
								<div class="value-item">
									<span>Course Value:</span>
									<span>${course.price || 0}</span>
								</div>
								<div class="value-item">
									<span>Bonuses Value:</span>
									<span>${course.bonuses.reduce((sum, b) => sum + (b.value || 0), 0)}</span>
								</div>
							</div>
							<div class="value-total">
								<span>Total Value:</span>
								<span class="value-amount">${totalValue}</span>
							</div>
						</div>
					</div>

					<!-- Affiliate Program -->
					<div class="form-card">
						<h2>Affiliate Program</h2>

						<label class="toggle-label">
							<input id="page-course-affiliate-enabled" name="page-course-affiliate-enabled" type="checkbox" bind:checked={course.affiliate_enabled} />
							<span>Enable affiliate program</span>
						</label>

						{#if course.affiliate_enabled}
							<div class="form-group">
								<label for="affiliate-commission">Commission Rate (%)</label>
								<input
									id="affiliate-commission" name="affiliate-commission"
									type="number"
									bind:value={course.affiliate_commission}
									min="10"
									max="50"
									step="5"
								/>
								<p class="help-text">
									Affiliates earn ${(
										((course.price || 0) * course.affiliate_commission) /
										100
									).toFixed(2)} per sale
								</p>
							</div>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Media Tab -->
			{#if activeTab === 'media'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="form-card">
						<h2>Course Thumbnail *</h2>
						<p class="help-text">
							Main image displayed in course listings (1280x720px recommended)
						</p>

						<div class="media-upload">
							{#if course.thumbnail}
								<div class="image-preview large">
									<img src={course.thumbnail} alt="Thumbnail" />
									<button
										class="remove-btn"
										onclick={() => {
											course.thumbnail = '';
											hasUnsavedChanges = true;
										}}
									>
										<IconX size={20} />
										Remove
									</button>
								</div>
							{/if}

							<label class="upload-zone" class:uploading>
								<IconPhoto size={32} />
								<span>{uploading ? 'Uploading...' : 'Click to upload thumbnail'}</span>
								<small>JPG, PNG • Max 5MB • 16:9 aspect ratio</small>
								<input
									id="page-file" name="page-file" type="file"
									accept="image/*"
									onchange={(e: Event) => handleImageUpload(e, 'thumbnail')}
									disabled={uploading}
								/>
							</label>
						</div>
					</div>

					<div class="form-card">
						<h2>Promotional Video</h2>
						<p class="help-text">A video preview helps increase conversions by 80%</p>

						<div class="media-upload">
							{#if course.promo_video}
								<div class="video-preview">
									{#if course.promo_video.startsWith('http')}
										<iframe src={course.promo_video} title="Promo Video"></iframe>
									{:else}
										<video controls src={course.promo_video}>
											<track kind="captions" />
										</video>
									{/if}
									<button
										class="remove-btn"
										onclick={() => {
											course.promo_video = '';
											hasUnsavedChanges = true;
										}}
									>
										<IconX size={20} />
										Remove
									</button>
								</div>
							{:else}
								<div class="video-options">
									<label class="upload-zone" class:uploading>
										<IconVideo size={32} />
										<span>Upload Video File</span>
										<small>MP4, WebM • Max 100MB</small>
										<input
											id="page-file" name="page-file" type="file"
											accept="video/*"
											onchange={handleVideoUpload}
											disabled={uploading}
										/>
									</label>

									<div class="divider">OR</div>

									<div class="video-url">
										<input
											id="page-paste-youtube-vimeo-or-wisti" name="page-paste-youtube-vimeo-or-wisti" type="url"
											placeholder="Paste YouTube, Vimeo, or Wistia URL..."
											onchange={(e: Event) => {
												course.promo_video = (e.target as HTMLInputElement).value;
												hasUnsavedChanges = true;
											}}
										/>
									</div>
								</div>
							{/if}
						</div>
					</div>

					<div class="form-card">
						<h2>Course Gallery</h2>
						<p class="help-text">Additional images to showcase course content</p>

						<div class="gallery-grid">
							{#each course.gallery as image, i}
								<div class="gallery-item">
									<img src={image} alt="Gallery {i + 1}" />
									<button class="remove-btn" onclick={() => removeFromGallery(i)}>
										<IconX size={16} />
									</button>
								</div>
							{/each}

							<label class="gallery-upload">
								<IconPlus size={24} />
								<span>Add Image</span>
								<input
									id="page-file" name="page-file" type="file"
									accept="image/*"
									onchange={(e: Event) => handleImageUpload(e, 'gallery')}
									disabled={uploading || course.gallery.length >= 10}
								/>
							</label>
						</div>
						{#if course.gallery.length >= 10}
							<p class="help-text warning">Maximum 10 gallery images</p>
						{/if}
					</div>
				</div>
			{/if}

			<!-- SEO & Marketing Tab -->
			{#if activeTab === 'seo'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="form-card">
						<h2>
							SEO Optimization
							<button class="ai-btn" onclick={() => generateWithAI('seo')} disabled={generating}>
								<IconSparkles size={14} />
								AI Optimize
							</button>
						</h2>

						<div class="form-group">
							<label for="meta-title">Meta Title</label>
							<input
								id="meta-title" name="meta-title"
								type="text"
								bind:value={course.meta_title}
								placeholder="{course.name || 'Course Title'} | Learn Trading"
								maxlength="60"
							/>
							<span class="char-count">{course.meta_title.length}/60</span>
						</div>

						<div class="form-group">
							<label for="meta-desc">Meta Description</label>
							<textarea
								id="meta-desc"
								bind:value={course.meta_description}
								placeholder="Compelling description for search results..."
								rows="3"
								maxlength="160"
							></textarea>
							<span class="char-count">{course.meta_description.length}/160</span>
						</div>

						<div class="form-group">
							<label for="keywords-input">Keywords</label>
							<div class="tag-input">
								{#each course.keywords as keyword, i}
									<span class="tag">
										{keyword}
										<button onclick={() => removeKeyword(i)}>
											<IconX size={14} />
										</button>
									</span>
								{/each}
								<input
									id="keywords-input" name="keywords-input"
									type="text"
									placeholder="Add keyword and press Enter..."
									onkeydown={addKeyword}
								/>
							</div>
							<p class="help-text">Add 5-10 relevant keywords for better SEO</p>
						</div>

						<div class="form-group">
							<label for="og-image">Social Share Image</label>
							{#if course.og_image}
								<div class="image-preview">
									<img src={course.og_image} alt="Social share preview" />
									<button
										class="remove-btn"
										onclick={() => {
											course.og_image = '';
											hasUnsavedChanges = true;
										}}
									>
										<IconX size={16} />
									</button>
								</div>
							{/if}
							<label class="upload-btn">
								<IconPhoto size={16} />
								Upload OG Image (1200x630px)
								<input
									id="page-file" name="page-file" type="file"
									accept="image/*"
									onchange={(e: Event) => handleImageUpload(e, 'og')}
									disabled={uploading}
								/>
							</label>
						</div>
					</div>

					<div class="form-card">
						<h2>Analytics & Tracking</h2>

						<div class="tracking-toggles">
							<label class="toggle-item">
								<input id="page-course-ga4-enabled" name="page-course-ga4-enabled" type="checkbox" bind:checked={course.ga4_enabled} />
								<IconBrandGoogle size={20} />
								<span>Google Analytics 4</span>
							</label>

							<label class="toggle-item">
								<input id="page-course-fb-pixel-enabled" name="page-course-fb-pixel-enabled" type="checkbox" bind:checked={course.fb_pixel_enabled} />
								<IconBrandFacebook size={20} />
								<span>Facebook Pixel</span>
							</label>

							<label class="toggle-item">
								<input id="page-course-conversion-tracking" name="page-course-conversion-tracking" type="checkbox" bind:checked={course.conversion_tracking} />
								<IconChartBar size={20} />
								<span>Conversion Tracking</span>
							</label>
						</div>

						<div class="utm-params">
							<h3>UTM Parameters</h3>
							<div class="form-row">
								<div class="form-group">
									<label for="utm-source">Source</label>
									<input
										id="utm-source" name="utm-source"
										type="text"
										bind:value={course.utm_source}
										placeholder="facebook"
									/>
								</div>
								<div class="form-group">
									<label for="utm-medium">Medium</label>
									<input
										id="utm-medium" name="utm-medium"
										type="text"
										bind:value={course.utm_medium}
										placeholder="social"
									/>
								</div>
								<div class="form-group">
									<label for="utm-campaign">Campaign</label>
									<input
										id="utm-campaign" name="utm-campaign"
										type="text"
										bind:value={course.utm_campaign}
										placeholder="summer-sale"
									/>
								</div>
							</div>
						</div>
					</div>

					<div class="form-card">
						<h2>Landing Page</h2>

						<label class="toggle-label">
							<input id="page-course-landing-page-enabled" name="page-course-landing-page-enabled" type="checkbox" bind:checked={course.landing_page_enabled} />
							<span>Create dedicated landing page</span>
						</label>

						{#if course.landing_page_enabled}
							<p class="help-text">
								Your course will be available at:
								<code>https://yoursite.com/courses/{course.slug || 'your-course'}</code>
							</p>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Advanced Settings Tab -->
			{#if activeTab === 'advanced'}
				<div class="tab-content" transition:fade={{ duration: 200 }}>
					<div class="form-card">
						<h2>Access Settings</h2>

						<div class="settings-grid">
							<label class="setting-item">
								<input id="page-course-lifetime-access" name="page-course-lifetime-access" type="checkbox" bind:checked={course.lifetime_access} />
								<IconLock size={20} />
								<div>
									<span>Lifetime Access</span>
									<small>Students keep access forever</small>
								</div>
							</label>

							<label class="setting-item">
								<input id="page-course-certificate-enabled" name="page-course-certificate-enabled" type="checkbox" bind:checked={course.certificate_enabled} />
								<IconCertificate size={20} />
								<div>
									<span>Completion Certificate</span>
									<small>Award certificate on completion</small>
								</div>
							</label>

							<label class="setting-item">
								<input id="page-course-allow-comments" name="page-course-allow-comments" type="checkbox" bind:checked={course.allow_comments} />
								<IconUsers size={20} />
								<div>
									<span>Allow Comments</span>
									<small>Students can comment on lessons</small>
								</div>
							</label>

							<label class="setting-item">
								<input id="page-course-allow-reviews" name="page-course-allow-reviews" type="checkbox" bind:checked={course.allow_reviews} />
								<IconStar size={20} />
								<div>
									<span>Allow Reviews</span>
									<small>Students can rate the course</small>
								</div>
							</label>

							<label class="setting-item">
								<input id="page-course-show-progress" name="page-course-show-progress" type="checkbox" bind:checked={course.show_progress} />
								<IconTrendingUp size={20} />
								<div>
									<span>Show Progress Bar</span>
									<small>Display completion progress</small>
								</div>
							</label>

							<label class="setting-item">
								<input id="page-course-is-featured" name="page-course-is-featured" type="checkbox" bind:checked={course.is_featured} />
								<IconRocket size={20} />
								<div>
									<span>Featured Course</span>
									<small>Highlight on homepage</small>
								</div>
							</label>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="enrollment-limit">Enrollment Limit</label>
								<input
									id="enrollment-limit" name="enrollment-limit"
									type="number"
									bind:value={course.enrollment_limit}
									placeholder="Unlimited"
									min="0"
								/>
								<p class="help-text">Leave empty for unlimited enrollment</p>
							</div>

							<div class="form-group">
								<label for="completion-threshold">Completion Threshold (%)</label>
								<input
									id="completion-threshold" name="completion-threshold"
									type="number"
									bind:value={course.completion_threshold}
									min="50"
									max="100"
									step="5"
								/>
								<p class="help-text">Required to earn certificate</p>
							</div>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="max-attempts">Max Quiz Attempts</label>
								<input
									id="max-attempts" name="max-attempts"
									type="number"
									bind:value={course.max_attempts}
									min="1"
									max="10"
								/>
							</div>

							<div class="form-group">
								<label for="passing-score">Passing Score (%)</label>
								<input
									id="passing-score" name="passing-score"
									type="number"
									bind:value={course.passing_score}
									min="50"
									max="100"
									step="5"
								/>
							</div>
						</div>
					</div>

					<div class="form-card">
						<h2>Tools & Resources Required</h2>

						<div class="list-editor">
							{#each course.tools_required as _, i}
								<div class="list-item">
									<IconSettings size={16} />
									<input
										id="page-course-tools-required-i" name="page-course-tools-required-i" type="text"
										bind:value={course.tools_required[i]}
										placeholder="Software, tools, or resources needed"
									/>
									<button
										onclick={() =>
											(course.tools_required = course.tools_required.filter((_, idx) => idx !== i))}
									>
										<IconX size={16} />
									</button>
								</div>
							{/each}
							<button
								class="add-item"
								onclick={() => (course.tools_required = [...course.tools_required, ''])}
							>
								<IconPlus size={16} />
								Add Tool
							</button>
						</div>
					</div>
				</div>
			{/if}
		</main>
	</div>
</div>

<!-- Complete Styles -->
<style>
	/* Reset & Base */
	* {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}

	/* Modal Overlay */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 16px;
		padding: 2rem;
		max-width: 500px;
		width: 90%;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-content h3 {
		color: #f1f5f9;
		font-size: 1.25rem;
		margin-bottom: 1rem;
	}

	.modal-content p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	.pricing-modal {
		max-width: 600px;
	}

	.pricing-details h4 {
		color: var(--primary-500);
		font-size: 1rem;
		margin: 1.5rem 0 0.75rem;
	}

	.pricing-details ul {
		list-style: none;
		color: #94a3b8;
	}

	.pricing-details li {
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.suggested-price {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 8px;
		margin: 1rem 0;
	}

	.suggested-price span {
		color: #94a3b8;
	}

	.suggested-price strong {
		color: #86efac;
		font-size: 1.5rem;
	}

	/* Form Message Banners */
	.form-error-banner,
	.form-success-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 12px;
		margin-bottom: 1.5rem;
		font-size: 0.9375rem;
	}

	.form-error-banner {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #fca5a5;
	}

	.form-error-banner button {
		margin-left: auto;
		background: none;
		border: none;
		color: #fca5a5;
		cursor: pointer;
		padding: 0.25rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.form-error-banner button:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.form-success-banner {
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #86efac;
	}

	.create-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
		min-height: 100vh;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
	}

	/* Centered Header - Email Templates Style */
	.page-header {
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	/* Actions Row - Centered */
	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.unsaved-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #f59e0b;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.save-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #10b981;
		font-size: 0.8125rem;
	}

	/* Progress Bar */
	.completion-progress {
		max-width: 500px;
		margin: 0 auto 1.5rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 4px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, var(--primary-500), var(--primary-600));
		transition: width 0.5s ease;
	}

	.progress-details {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
		color: #64748b;
	}

	/* Layout */
	.content-wrapper {
		display: grid;
		grid-template-columns: 280px 1fr;
		gap: 2rem;
	}

	/* Sidebar */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		position: sticky;
		top: 2rem;
		height: fit-content;
	}

	.sidebar-nav {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 0.5rem;
		backdrop-filter: blur(10px);
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		position: relative;
	}

	.nav-item:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #cbd5e1;
	}

	.nav-item.active {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
	}

	.nav-item :global(.check) {
		position: absolute;
		right: 1rem;
		color: #22c55e;
	}

	/* AI Assistant */
	.ai-assistant {
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.1), rgba(59, 130, 246, 0.1));
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 12px;
		padding: 1.25rem;
		backdrop-filter: blur(10px);
	}

	.ai-assistant h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.05rem;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
		font-weight: 600;
	}
	.ai-actions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.ai-actions button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.875rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: var(--primary-500);
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		font-weight: 500;
	}

	.ai-actions button:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.2);
		transform: translateX(2px);
	}

	.ai-actions button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Validation Card */
	.validation-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
		backdrop-filter: blur(10px);
	}

	.validation-card h3 {
		font-size: 1.05rem;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
		font-weight: 600;
	}

	.score-circle {
		position: relative;
		width: 120px;
		height: 120px;
		margin: 0 auto 1.25rem;
	}

	.score-circle svg {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.score-text {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.validation-items {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.validation-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.875rem;
		background: rgba(148, 163, 184, 0.05);
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.validation-item.good {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
		border: 1px solid rgba(16, 185, 129, 0.2);
	}

	.validation-item.warning {
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
		border: 1px solid rgba(245, 158, 11, 0.2);
	}

	.validation-item.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.item-score {
		font-weight: 600;
		font-size: 0.75rem;
	}

	/* Main Content */
	.main-content {
		min-width: 0;
	}

	.tab-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* Form Cards - Email Templates Style */
	.form-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		padding: 1.5rem;
		backdrop-filter: blur(10px);
	}

	.form-card h2 {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.form-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 1.25rem 0 0.875rem 0;
	}

	/* Form Elements */
	.form-group {
		margin-bottom: 1.25rem;
		position: relative;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #cbd5e1;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
		transition: all 0.2s;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 100px;
	}

	.input-large {
		font-size: 1rem;
		font-weight: 500;
		padding: 0.875rem 1rem;
	}

	.input-group {
		display: flex;
		align-items: center;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		overflow: hidden;
	}

	.input-prefix {
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		color: #64748b;
		border-right: 1px solid rgba(148, 163, 184, 0.2);
		font-size: 0.875rem;
	}

	.input-group input {
		border: none;
		background: transparent;
		margin: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.help-text {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-top: 0.5rem;
	}

	.help-text.warning {
		color: #f59e0b;
	}

	.char-count {
		position: absolute;
		bottom: -1.25rem;
		right: 0;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* AI Buttons */
	.ai-btn,
	.ai-btn-large {
		padding: 0.375rem 0.625rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: var(--primary-500);
		cursor: pointer;
		transition: all 0.2s;
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.ai-btn-large {
		padding: 0.625rem 1rem;
		font-size: 0.95rem;
	}

	.ai-btn:hover:not(:disabled),
	.ai-btn-large:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.2);
		transform: translateY(-1px);
	}

	/* List Editor */
	.list-editor {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.list-item:hover {
		background: rgba(15, 23, 42, 0.8);
		border-color: rgba(148, 163, 184, 0.2);
	}

	.list-item input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		margin: 0;
		font-size: 0.95rem;
	}

	.list-item button {
		padding: 0.375rem;
		background: rgba(239, 68, 68, 0.1);
		border: none;
		border-radius: 4px;
		color: #f87171;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.list-item button:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.add-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #6ee7b7;
		cursor: pointer;
		transition: all 0.2s;
		font-weight: 500;
		font-size: 0.95rem;
	}

	.add-item:hover {
		background: rgba(16, 185, 129, 0.2);
		transform: translateY(-1px);
	}

	/* Curriculum Builder */
	.curriculum-header {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.curriculum-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.curriculum-stats {
		display: flex;
		gap: 1.5rem;
		margin-left: auto;
	}

	.curriculum-stats span {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.95rem;
		font-weight: 500;
	}

	.curriculum-builder {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* Module Cards */
	.module-card {
		background: rgba(30, 41, 59, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.3s;
		cursor: move;
	}

	.module-card.dragging {
		opacity: 0.4;
		transform: scale(0.98);
	}

	.module-card.expanded {
		border-color: rgba(16, 185, 129, 0.2);
		background: rgba(30, 41, 59, 0.8);
	}

	.module-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.drag-handle {
		cursor: move;
		color: #64748b;
		flex-shrink: 0;
	}

	.module-number {
		font-weight: 600;
		color: #10b981;
		font-size: 0.875rem;
		flex-shrink: 0;
	}

	.module-title {
		flex: 1;
		font-size: 1.125rem;
		font-weight: 500;
		background: transparent;
		border: none;
		color: #f1f5f9;
		padding: 0.5rem;
	}

	.module-stats {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-left: auto;
		flex-shrink: 0;
	}

	.module-actions {
		display: flex;
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.module-actions button,
	.expand-btn {
		padding: 0.5rem;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.module-actions button:hover,
	.expand-btn:hover {
		background: rgba(148, 163, 184, 0.2);
		color: #cbd5e1;
	}

	.module-description {
		width: 100%;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		resize: none;
		margin-bottom: 1rem;
	}

	/* Lessons */
	.lessons-container {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1rem;
		padding-left: 2rem;
	}

	.lesson-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.lesson-item:hover {
		background: rgba(15, 23, 42, 0.8);
		border-color: rgba(148, 163, 184, 0.2);
	}

	.lesson-drag {
		cursor: move;
		color: #64748b;
		flex-shrink: 0;
	}

	.lesson-number {
		font-weight: 600;
		color: #64748b;
		font-size: 0.875rem;
		flex-shrink: 0;
	}

	.lesson-type {
		width: 140px;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.lesson-title {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.95rem;
		padding: 0.25rem;
	}

	.lesson-duration {
		width: 80px;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		text-align: center;
		font-size: 0.875rem;
	}

	.preview-toggle {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		cursor: pointer;
		color: #94a3b8;
		padding: 0.375rem;
	}

	.preview-toggle:hover {
		color: #cbd5e1;
	}

	.preview-toggle input {
		margin: 0;
	}

	.add-lesson {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem;
		background: rgba(16, 185, 129, 0.05);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #6ee7b7;
		cursor: pointer;
		font-weight: 500;
		font-size: 0.9rem;
		margin-left: 2rem;
		transition: all 0.2s;
	}

	.add-lesson:hover {
		background: rgba(16, 185, 129, 0.1);
		transform: translateY(-1px);
	}

	.module-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.module-duration {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.preview-module {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.add-module {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05));
		border: 2px dashed rgba(16, 185, 129, 0.3);
		border-radius: 12px;
		color: #10b981;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-module:hover {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.15), rgba(16, 185, 129, 0.1));
		border-color: rgba(16, 185, 129, 0.5);
		transform: translateY(-2px);
	}

	/* Drip Schedule */
	.drip-schedule {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 1rem;
	}

	.drip-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
	}

	.module-name {
		font-weight: 500;
		color: #f1f5f9;
	}

	/* Pricing Section */
	.pricing-models {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.pricing-model {
		position: relative;
		cursor: pointer;
	}

	.pricing-model input[type='radio'] {
		position: absolute;
		opacity: 0;
	}

	.model-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		transition: all 0.2s;
		text-align: center;
	}

	.pricing-model:hover .model-content {
		border-color: rgba(16, 185, 129, 0.3);
		transform: translateY(-2px);
	}

	.pricing-model.selected .model-content {
		background: rgba(16, 185, 129, 0.1);
		border-color: #10b981;
		box-shadow: 0 0 20px rgba(16, 185, 129, 0.2);
	}

	.model-title {
		font-weight: 600;
		color: #f1f5f9;
		font-size: 1rem;
	}

	.model-desc {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.price-input {
		position: relative;
	}

	.price-input .currency {
		position: absolute;
		left: 1.125rem;
		top: 50%;
		transform: translateY(-50%);
		color: #10b981;
		font-weight: 600;
		font-size: 1.125rem;
	}

	.price-input input {
		padding-left: 2.75rem;
		font-size: 1.125rem;
		font-weight: 500;
	}

	.analyze-btn {
		padding: 0.625rem 1.125rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 6px;
		color: #60a5fa;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		font-weight: 500;
	}

	.analyze-btn:hover:not(:disabled) {
		background: rgba(59, 130, 246, 0.2);
		transform: translateY(-1px);
	}

	/* Early Bird Section */
	.early-bird-section {
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.early-bird-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.toggle-label input {
		width: auto;
		margin: 0;
	}

	/* Bonuses */
	.bonus-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.bonus-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
	}

	.bonus-icon {
		width: 50px;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		cursor: pointer;
	}

	.bonus-type {
		width: 130px;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 0.875rem;
	}

	.bonus-title {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		padding: 0.25rem;
	}

	.bonus-value {
		width: 100px;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		text-align: center;
	}

	.total-value {
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05));
		border: 2px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
	}

	.value-breakdown {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(16, 185, 129, 0.2);
	}

	.value-item {
		display: flex;
		justify-content: space-between;
		color: #94a3b8;
		font-size: 0.95rem;
	}

	.value-total {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-weight: 600;
		color: #f1f5f9;
	}

	.value-amount {
		font-size: 1.75rem;
		font-weight: 700;
		color: #10b981;
	}

	/* Media Upload */
	.media-upload {
		margin-top: 1rem;
	}

	.upload-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 3rem;
		background: rgba(15, 23, 42, 0.4);
		border: 2px dashed rgba(148, 163, 184, 0.3);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
	}

	.upload-zone:hover {
		border-color: rgba(16, 185, 129, 0.5);
		background: rgba(16, 185, 129, 0.05);
		transform: translateY(-2px);
	}

	.upload-zone.uploading {
		pointer-events: none;
		opacity: 0.6;
		animation: pulse 1s ease-in-out infinite;
	}

	.upload-zone input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	.upload-zone small {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.image-preview {
		position: relative;
		width: 100%;
		border-radius: 8px;
		overflow: hidden;
		margin-bottom: 1rem;
	}

	.image-preview.large {
		height: 400px;
	}

	.image-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.video-preview {
		position: relative;
		width: 100%;
		aspect-ratio: 16/9;
		border-radius: 8px;
		overflow: hidden;
		margin-bottom: 1rem;
		background: #000;
	}

	.video-preview iframe,
	.video-preview video {
		width: 100%;
		height: 100%;
		border: none;
	}

	.video-options {
		display: grid;
		gap: 1rem;
	}

	.divider {
		text-align: center;
		color: #64748b;
		margin: 1rem 0;
		position: relative;
	}

	.divider::before,
	.divider::after {
		content: '';
		position: absolute;
		top: 50%;
		width: calc(50% - 2rem);
		height: 1px;
		background: rgba(148, 163, 184, 0.2);
	}

	.divider::before {
		left: 0;
	}

	.divider::after {
		right: 0;
	}

	.video-url input {
		width: 100%;
		padding: 0.875rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
	}

	.remove-btn {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.625rem 0.875rem;
		background: rgba(239, 68, 68, 0.9);
		backdrop-filter: blur(10px);
		border: none;
		border-radius: 8px;
		color: white;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.remove-btn:hover {
		background: #dc2626;
		transform: translateY(-1px);
	}

	.gallery-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
		gap: 1rem;
		margin-top: 1rem;
	}

	.gallery-item {
		position: relative;
		aspect-ratio: 16/9;
		border-radius: 8px;
		overflow: hidden;
		border: 2px solid rgba(148, 163, 184, 0.1);
		transition: all 0.2s;
	}

	.gallery-item:hover {
		border-color: rgba(16, 185, 129, 0.3);
		transform: translateY(-2px);
	}

	.gallery-item img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.gallery-upload {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		aspect-ratio: 16/9;
		background: rgba(15, 23, 42, 0.4);
		border: 2px dashed rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		cursor: pointer;
		position: relative;
		transition: all 0.2s;
	}

	.gallery-upload:hover {
		border-color: rgba(16, 185, 129, 0.5);
		background: rgba(16, 185, 129, 0.05);
	}

	.gallery-upload input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	/* SEO Section */
	.tag-input {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		min-height: 100px;
		align-items: flex-start;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 6px;
		color: #10b981;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.tag button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.125rem;
		background: none;
		border: none;
		color: #10b981;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag button:hover {
		color: #ef4444;
	}

	.tag-input input {
		flex: 1;
		min-width: 200px;
		background: transparent;
		border: none;
		color: #f1f5f9;
		outline: none;
		font-size: 0.95rem;
	}

	.upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #cbd5e1;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
		font-weight: 500;
		font-size: 0.95rem;
	}

	.upload-btn:hover {
		background: rgba(148, 163, 184, 0.2);
		transform: translateY(-1px);
	}

	.upload-btn input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	/* Analytics Section */
	.tracking-toggles {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.toggle-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.toggle-item:hover {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(15, 23, 42, 0.8);
	}

	.toggle-item input[type='checkbox'] {
		width: auto;
		margin: 0;
	}

	.toggle-item:has(input[type='checkbox']:checked) span {
		color: #10b981;
	}

	.utm-params {
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.utm-params h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	/* Advanced Settings */
	.settings-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.setting-item {
		display: flex;
		align-items: flex-start;
		gap: 0.875rem;
		padding: 1.125rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.setting-item:hover {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(15, 23, 42, 0.8);
	}

	.setting-item input[type='checkbox'] {
		width: auto;
		margin-top: 0.125rem;
		flex-shrink: 0;
	}

	.setting-item > div {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.setting-item span {
		font-weight: 500;
		color: #f1f5f9;
		font-size: 0.95rem;
	}

	.setting-item small {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.setting-item input[type='checkbox']:checked ~ div span {
		color: #10b981;
	}

	/* Buttons - Email Templates Style */
	.btn-primary,
	.btn-secondary,
	.btn-ghost {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-ghost {
		background: transparent;
		color: #94a3b8;
	}

	.btn-ghost:hover {
		background: rgba(148, 163, 184, 0.1);
	}

	/* Animations */
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

	@keyframes pulse {
		0% {
			opacity: 0.6;
		}
		50% {
			opacity: 0.8;
		}
		100% {
			opacity: 0.6;
		}
	}

	/* Responsive */
	@media (max-width: 1280px) {
		.content-wrapper {
			grid-template-columns: 240px 1fr;
		}
	}

	@media (max-width: 1024px) {
		.create-page {
			padding: 1rem;
		}

		.content-wrapper {
			grid-template-columns: 1fr;
		}

		.sidebar {
			display: none;
		}

		.header-actions {
			flex-direction: column;
			gap: 0.5rem;
		}

		.curriculum-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.curriculum-stats {
			margin-left: 0;
		}
	}

	@media (max-width: 640px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.pricing-models {
			grid-template-columns: 1fr;
		}

		.module-header {
			flex-wrap: wrap;
		}

		.lessons-container {
			padding-left: 0;
		}

		.add-lesson {
			margin-left: 0;
		}
	}
</style>
