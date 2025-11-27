<script lang="ts">
    // Type available for future icon variations
    const type: string = 'line';
    
    interface Props {
        size?: number;
    }
    
    let { size = 48 }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 48 48" class="text-pink-400">
    <path d="M8 40 L16 28 L24 32 L32 16 L40 24" stroke="currentColor" stroke-width="2" fill="none" />
    <circle cx="16" cy="28" r="2" fill="currentColor" />
    <circle cx="24" cy="32" r="2" fill="currentColor" />
    <circle cx="32" cy="16" r="2" fill="currentColor" />
</svg>
