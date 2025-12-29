<script lang="ts">
    interface Props {
        type?: string;
        size?: number;
    }
    
    let { type: _type = 'line', size = 48 }: Props = $props();
</script>

<svg width={size} height={size} viewBox="0 0 48 48" class="text-indigo-400">
    <rect x="8" y="28" width="6" height="12" fill="currentColor" opacity="0.8" />
    <rect x="16" y="20" width="6" height="20" fill="currentColor" opacity="0.8" />
    <rect x="24" y="24" width="6" height="16" fill="currentColor" opacity="0.8" />
    <rect x="32" y="12" width="6" height="28" fill="currentColor" opacity="0.8" />
</svg>
