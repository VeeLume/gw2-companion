<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	// `/` exists only to land somewhere.
	onMount(() => {
		goto('/dashboard', { replaceState: true });
	});
</script>
