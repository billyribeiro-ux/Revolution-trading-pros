<script lang="ts">
	import Calculator from '$lib/options-calculator/components/Calculator.svelte';
	import '$lib/options-calculator/styles/calculator-theme.css';
	import {
		generateStructuredData,
		generateFAQStructuredData,
		generateOGMeta
	} from '$lib/options-calculator/utils/seo.js';

	const structuredData = generateStructuredData();
	const faqData = generateFAQStructuredData();
	const ogMeta = generateOGMeta();
</script>

<Calculator />
