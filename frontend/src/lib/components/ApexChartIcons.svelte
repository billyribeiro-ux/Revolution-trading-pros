<script lang="ts">
    export let type: string = 'line';
    export let size: number = 48;
</script>

<svg width={size} height={size} viewBox="0 0 48 48" class="text-pink-400">
    <path d="M8 40 L16 28 L24 32 L32 16 L40 24" stroke="currentColor" stroke-width="2" fill="none" />
    <circle cx="16" cy="28" r="2" fill="currentColor" />
    <circle cx="24" cy="32" r="2" fill="currentColor" />
    <circle cx="32" cy="16" r="2" fill="currentColor" />
</svg>
