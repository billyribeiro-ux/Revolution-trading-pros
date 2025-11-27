/**
 * Apple-Style Animations Library
 *
 * Enterprise-grade animation utilities inspired by Apple.com
 * Features:
 * - Scroll-triggered animations with IntersectionObserver
 * - Parallax effects with smooth interpolation
 * - Spring physics animations
 * - Text reveal animations
 * - 3D perspective transforms
 * - Fluid motion with custom easing
 *
 * @version 1.0.0
 * @level Senior Apple Engineer
 */

// Custom Apple-style easing functions
export const appleEasing = {
	// Apple's signature smooth easing
	smooth: 'cubic-bezier(0.25, 0.1, 0.25, 1)',
	// Quick start, smooth end (for hero animations)
	hero: 'cubic-bezier(0.16, 1, 0.3, 1)',
	// Bouncy spring effect
	spring: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
	// Smooth deceleration
	decel: 'cubic-bezier(0, 0, 0.2, 1)',
	// Smooth acceleration
	accel: 'cubic-bezier(0.4, 0, 1, 1)',
	// Elastic bounce
	elastic: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)',
	// Smooth in-out
	smoothInOut: 'cubic-bezier(0.45, 0, 0.55, 1)',
	// Dramatic slow reveal
	dramatic: 'cubic-bezier(0.19, 1, 0.22, 1)'
};

// Animation duration presets
export const appleDurations = {
	instant: 100,
	fast: 200,
	normal: 400,
	slow: 600,
	dramatic: 1000,
	hero: 1500
};

/**
 * Scroll-triggered animation observer
 * Creates Apple-style reveal animations when elements enter viewport
 */
export function createScrollObserver(options: {
	threshold?: number;
	rootMargin?: string;
	once?: boolean;
}) {
	const { threshold = 0.1, rootMargin = '0px 0px -50px 0px', once = true } = options;

	if (typeof window === 'undefined') return null;

	return new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					entry.target.classList.add('animate-in');
					if (once) {
						observer.unobserve(entry.target);
					}
				} else if (!once) {
					entry.target.classList.remove('animate-in');
				}
			});
		},
		{ threshold, rootMargin }
	);
}

// Global observer instance
let observer: IntersectionObserver | null = null;

export function getScrollObserver() {
	if (!observer && typeof window !== 'undefined') {
		observer = createScrollObserver({ threshold: 0.15, rootMargin: '0px 0px -80px 0px' });
	}
	return observer;
}

/**
 * Svelte action for scroll-triggered animations
 * Usage: <div use:scrollReveal={{ type: 'fade-up', delay: 200 }}>
 */
export function scrollReveal(
	node: HTMLElement,
	params: {
		type?:
			| 'fade'
			| 'fade-up'
			| 'fade-down'
			| 'fade-left'
			| 'fade-right'
			| 'scale'
			| 'scale-up'
			| 'blur'
			| 'flip';
		delay?: number;
		duration?: number;
		easing?: string;
		threshold?: number;
		once?: boolean;
	} = {}
) {
	const {
		type = 'fade-up',
		delay = 0,
		duration = 600,
		easing = appleEasing.hero,
		threshold = 0.15,
		once = true
	} = params;

	// Set initial styles
	const transforms: Record<string, string> = {
		fade: '',
		'fade-up': 'translateY(40px)',
		'fade-down': 'translateY(-40px)',
		'fade-left': 'translateX(40px)',
		'fade-right': 'translateX(-40px)',
		scale: 'scale(0.9)',
		'scale-up': 'scale(0.9) translateY(20px)',
		blur: '',
		flip: 'perspective(1000px) rotateX(10deg)'
	};

	const initialStyles = {
		opacity: '0',
		transform: transforms[type] || '',
		filter: type === 'blur' ? 'blur(10px)' : '',
		transition: `opacity ${duration}ms ${easing} ${delay}ms, transform ${duration}ms ${easing} ${delay}ms, filter ${duration}ms ${easing} ${delay}ms`
	};

	Object.assign(node.style, initialStyles);

	const localObserver = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					node.style.opacity = '1';
					node.style.transform = 'translateY(0) translateX(0) scale(1) rotateX(0)';
					node.style.filter = 'blur(0)';

					if (once) {
						localObserver.unobserve(node);
					}
				} else if (!once) {
					Object.assign(node.style, initialStyles);
				}
			});
		},
		{ threshold, rootMargin: '0px 0px -50px 0px' }
	);

	localObserver.observe(node);

	return {
		destroy() {
			localObserver.disconnect();
		}
	};
}

/**
 * Parallax scroll effect
 * Usage: <div use:parallax={{ speed: 0.5, direction: 'up' }}>
 */
export function parallax(
	node: HTMLElement,
	params: {
		speed?: number;
		direction?: 'up' | 'down' | 'left' | 'right';
		scale?: boolean;
	} = {}
) {
	const { speed = 0.3, direction = 'up', scale = false } = params;

	let ticking = false;
	let lastScrollY = 0;

	function updatePosition() {
		const rect = node.getBoundingClientRect();
		const viewportHeight = window.innerHeight;

		// Calculate how far the element is through the viewport
		const progress = (viewportHeight - rect.top) / (viewportHeight + rect.height);
		const clampedProgress = Math.max(0, Math.min(1, progress));

		// Calculate offset
		const offset = (clampedProgress - 0.5) * 100 * speed;

		let transform = '';
		switch (direction) {
			case 'up':
				transform = `translateY(${-offset}px)`;
				break;
			case 'down':
				transform = `translateY(${offset}px)`;
				break;
			case 'left':
				transform = `translateX(${-offset}px)`;
				break;
			case 'right':
				transform = `translateX(${offset}px)`;
				break;
		}

		if (scale) {
			const scaleValue = 1 + (clampedProgress - 0.5) * 0.1 * speed;
			transform += ` scale(${scaleValue})`;
		}

		node.style.transform = transform;
		node.style.willChange = 'transform';
		ticking = false;
	}

	function onScroll() {
		lastScrollY = window.scrollY;
		if (!ticking) {
			requestAnimationFrame(updatePosition);
			ticking = true;
		}
	}

	window.addEventListener('scroll', onScroll, { passive: true });
	updatePosition();

	return {
		destroy() {
			window.removeEventListener('scroll', onScroll);
		}
	};
}

/**
 * Sticky scroll section with progress tracking
 * Usage: <div use:stickyScroll={{ onProgress: (p) => console.log(p) }}>
 */
export function stickyScroll(
	node: HTMLElement,
	params: {
		onProgress?: (progress: number) => void;
		start?: number;
		end?: number;
	} = {}
) {
	const { onProgress, start = 0, end = 1 } = params;

	let ticking = false;

	function updateProgress() {
		const rect = node.getBoundingClientRect();
		const viewportHeight = window.innerHeight;

		// Calculate scroll progress through the element
		const elementTop = rect.top;
		const elementHeight = rect.height;
		const scrollableDistance = elementHeight - viewportHeight;

		let progress = 0;
		if (elementTop <= 0 && scrollableDistance > 0) {
			progress = Math.min(1, Math.abs(elementTop) / scrollableDistance);
		}

		// Map to start/end range
		const mappedProgress = start + progress * (end - start);

		if (onProgress) {
			onProgress(mappedProgress);
		}

		node.style.setProperty('--scroll-progress', String(mappedProgress));
		ticking = false;
	}

	function onScroll() {
		if (!ticking) {
			requestAnimationFrame(updateProgress);
			ticking = true;
		}
	}

	window.addEventListener('scroll', onScroll, { passive: true });
	updateProgress();

	return {
		destroy() {
			window.removeEventListener('scroll', onScroll);
		}
	};
}

/**
 * Text reveal animation (word by word or character by character)
 * Usage: <p use:textReveal={{ type: 'words', stagger: 50 }}>Hello World</p>
 */
export function textReveal(
	node: HTMLElement,
	params: {
		type?: 'words' | 'chars' | 'lines';
		stagger?: number;
		duration?: number;
		easing?: string;
	} = {}
) {
	const { type = 'words', stagger = 30, duration = 500, easing = appleEasing.hero } = params;

	const originalText = node.textContent || '';
	const parts = type === 'chars' ? originalText.split('') : originalText.split(' ');

	node.innerHTML = '';
	node.style.display = 'inline';

	const wrapper = document.createElement('span');
	wrapper.style.display = 'inline';

	parts.forEach((part, index) => {
		const span = document.createElement('span');
		span.textContent = type === 'words' ? part + ' ' : part;
		span.style.display = 'inline-block';
		span.style.opacity = '0';
		span.style.transform = 'translateY(20px)';
		span.style.transition = `opacity ${duration}ms ${easing} ${index * stagger}ms, transform ${duration}ms ${easing} ${index * stagger}ms`;
		wrapper.appendChild(span);
	});

	node.appendChild(wrapper);

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					const spans = wrapper.querySelectorAll('span');
					spans.forEach((span) => {
						(span as HTMLElement).style.opacity = '1';
						(span as HTMLElement).style.transform = 'translateY(0)';
					});
					observer.unobserve(node);
				}
			});
		},
		{ threshold: 0.1 }
	);

	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
			node.textContent = originalText;
		}
	};
}

/**
 * 3D tilt effect on hover (like iPhone product cards)
 * Usage: <div use:tilt3d={{ intensity: 15, scale: 1.05 }}>
 */
export function tilt3d(
	node: HTMLElement,
	params: {
		intensity?: number;
		scale?: number;
		glare?: boolean;
		glareColor?: string;
	} = {}
) {
	const { intensity = 10, scale = 1.02, glare = true, glareColor = 'rgba(255,255,255,0.3)' } =
		params;

	node.style.transformStyle = 'preserve-3d';
	node.style.transition = 'transform 0.3s ease-out';
	node.style.willChange = 'transform';

	let glareElement: HTMLElement | null = null;
	if (glare) {
		glareElement = document.createElement('div');
		glareElement.style.cssText = `
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      pointer-events: none;
      background: linear-gradient(105deg, transparent 40%, ${glareColor} 45%, transparent 50%);
      opacity: 0;
      transition: opacity 0.3s ease-out;
      border-radius: inherit;
    `;
		node.style.position = 'relative';
		node.style.overflow = 'hidden';
		node.appendChild(glareElement);
	}

	function handleMouseMove(e: MouseEvent) {
		const rect = node.getBoundingClientRect();
		const centerX = rect.left + rect.width / 2;
		const centerY = rect.top + rect.height / 2;

		const percentX = (e.clientX - centerX) / (rect.width / 2);
		const percentY = (e.clientY - centerY) / (rect.height / 2);

		const rotateX = -percentY * intensity;
		const rotateY = percentX * intensity;

		node.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale(${scale})`;

		if (glareElement) {
			glareElement.style.opacity = '1';
			const glareX = 50 + percentX * 30;
			glareElement.style.background = `linear-gradient(${105 + percentX * 10}deg, transparent ${glareX - 10}%, ${glareColor} ${glareX}%, transparent ${glareX + 10}%)`;
		}
	}

	function handleMouseLeave() {
		node.style.transform = 'perspective(1000px) rotateX(0) rotateY(0) scale(1)';
		if (glareElement) {
			glareElement.style.opacity = '0';
		}
	}

	node.addEventListener('mousemove', handleMouseMove);
	node.addEventListener('mouseleave', handleMouseLeave);

	return {
		destroy() {
			node.removeEventListener('mousemove', handleMouseMove);
			node.removeEventListener('mouseleave', handleMouseLeave);
			if (glareElement) {
				glareElement.remove();
			}
		}
	};
}

/**
 * Magnetic hover effect (element follows cursor slightly)
 * Usage: <button use:magnetic={{ strength: 0.3 }}>Click me</button>
 */
export function magnetic(node: HTMLElement, params: { strength?: number } = {}) {
	const { strength = 0.3 } = params;

	node.style.transition = 'transform 0.2s ease-out';

	function handleMouseMove(e: MouseEvent) {
		const rect = node.getBoundingClientRect();
		const centerX = rect.left + rect.width / 2;
		const centerY = rect.top + rect.height / 2;

		const distanceX = (e.clientX - centerX) * strength;
		const distanceY = (e.clientY - centerY) * strength;

		node.style.transform = `translate(${distanceX}px, ${distanceY}px)`;
	}

	function handleMouseLeave() {
		node.style.transform = 'translate(0, 0)';
	}

	node.addEventListener('mousemove', handleMouseMove);
	node.addEventListener('mouseleave', handleMouseLeave);

	return {
		destroy() {
			node.removeEventListener('mousemove', handleMouseMove);
			node.removeEventListener('mouseleave', handleMouseLeave);
		}
	};
}

/**
 * Number counter animation
 * Usage: <span use:countUp={{ target: 1000, duration: 2000 }}>0</span>
 */
export function countUp(
	node: HTMLElement,
	params: {
		target: number;
		duration?: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		easing?: (t: number) => number;
	}
) {
	const {
		target,
		duration = 2000,
		decimals = 0,
		prefix = '',
		suffix = '',
		easing = (t) => 1 - Math.pow(1 - t, 3) // Ease out cubic
	} = params;

	let start = 0;
	let startTime: number | null = null;
	let animationFrame: number;
	let hasAnimated = false;

	function format(value: number): string {
		return prefix + value.toFixed(decimals).replace(/\B(?=(\d{3})+(?!\d))/g, ',') + suffix;
	}

	function animate(timestamp: number) {
		if (!startTime) startTime = timestamp;
		const elapsed = timestamp - startTime;
		const progress = Math.min(elapsed / duration, 1);
		const easedProgress = easing(progress);
		const current = start + (target - start) * easedProgress;

		node.textContent = format(current);

		if (progress < 1) {
			animationFrame = requestAnimationFrame(animate);
		}
	}

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting && !hasAnimated) {
					hasAnimated = true;
					animationFrame = requestAnimationFrame(animate);
					observer.unobserve(node);
				}
			});
		},
		{ threshold: 0.1 }
	);

	observer.observe(node);
	node.textContent = format(start);

	return {
		destroy() {
			observer.disconnect();
			if (animationFrame) {
				cancelAnimationFrame(animationFrame);
			}
		},
		update(newParams: typeof params) {
			if (newParams.target !== target) {
				start = target;
				startTime = null;
				hasAnimated = false;
				observer.observe(node);
			}
		}
	};
}

/**
 * Smooth scroll to element
 */
export function smoothScrollTo(element: HTMLElement | string, offset = 0) {
	const target = typeof element === 'string' ? document.querySelector(element) : element;
	if (!target) return;

	const targetPosition = target.getBoundingClientRect().top + window.scrollY - offset;

	window.scrollTo({
		top: targetPosition,
		behavior: 'smooth'
	});
}

/**
 * CSS class for Apple-style animations (inject into global styles)
 */
export const appleAnimationStyles = `
  /* Base animation states */
  [data-animate] {
    opacity: 0;
    transform: translateY(30px);
    transition: opacity 0.6s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.6s cubic-bezier(0.16, 1, 0.3, 1);
  }

  [data-animate].animate-in {
    opacity: 1;
    transform: translateY(0);
  }

  /* Staggered animations */
  [data-animate-delay="1"] { transition-delay: 100ms; }
  [data-animate-delay="2"] { transition-delay: 200ms; }
  [data-animate-delay="3"] { transition-delay: 300ms; }
  [data-animate-delay="4"] { transition-delay: 400ms; }
  [data-animate-delay="5"] { transition-delay: 500ms; }

  /* Scale animation */
  [data-animate="scale"] {
    transform: scale(0.95);
  }

  [data-animate="scale"].animate-in {
    transform: scale(1);
  }

  /* Blur animation */
  [data-animate="blur"] {
    filter: blur(10px);
  }

  [data-animate="blur"].animate-in {
    filter: blur(0);
  }

  /* Slide animations */
  [data-animate="slide-up"] { transform: translateY(50px); }
  [data-animate="slide-down"] { transform: translateY(-50px); }
  [data-animate="slide-left"] { transform: translateX(50px); }
  [data-animate="slide-right"] { transform: translateX(-50px); }

  [data-animate="slide-up"].animate-in,
  [data-animate="slide-down"].animate-in,
  [data-animate="slide-left"].animate-in,
  [data-animate="slide-right"].animate-in {
    transform: translate(0);
  }

  /* Hero text animation */
  .hero-text-reveal {
    overflow: hidden;
  }

  .hero-text-reveal span {
    display: inline-block;
    transform: translateY(100%);
    transition: transform 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .hero-text-reveal.animate-in span {
    transform: translateY(0);
  }

  /* Smooth hover effects */
  .apple-hover {
    transition: transform 0.3s cubic-bezier(0.25, 0.1, 0.25, 1),
                box-shadow 0.3s cubic-bezier(0.25, 0.1, 0.25, 1);
  }

  .apple-hover:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.12);
  }

  /* Gradient text animation */
  @keyframes gradient-shift {
    0% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
    100% { background-position: 0% 50%; }
  }

  .animated-gradient-text {
    background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
    background-size: 400% 400%;
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    animation: gradient-shift 8s ease infinite;
  }

  /* Pulse glow effect */
  @keyframes pulse-glow {
    0%, 100% { box-shadow: 0 0 20px rgba(79, 70, 229, 0.4); }
    50% { box-shadow: 0 0 40px rgba(79, 70, 229, 0.8); }
  }

  .pulse-glow {
    animation: pulse-glow 2s ease-in-out infinite;
  }
`;
