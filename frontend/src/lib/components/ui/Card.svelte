<script lang="ts">
	export let padding: boolean = true;
	export let hover: boolean = false;
	let className: string = '';
	export { className as class };
</script>

<div
	class="bg-white rounded-lg border border-gray-200 shadow-sm
    {padding ? 'p-6' : ''}
    {hover ? 'hover:shadow-md transition-shadow duration-200' : ''}
    {className}"
>
	<slot />
</div>
