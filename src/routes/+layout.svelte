<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { Loading, Shell, setKitContext } from '@veelume/ui';
	import { Settings } from 'lucide-svelte';
	import { appearance } from '$lib/stores/appearance.svelte';
	import { settings } from '$lib/stores/settings.svelte';
	import { updater } from '$lib/stores/updater.svelte';
	import { nav } from '$lib/nav.svelte';
	import UpdateBanner from '$lib/components/UpdateBanner.svelte';

	let { children } = $props();

	// UI text is English; formatting (numbers, dates, 24h clock) follows the OS.
	setKitContext({
		formattingLocale: () => navigator.language
	});

	onMount(async () => {
		// Apply the saved density / colour scheme before anything else renders.
		appearance.init();
		await settings.init();
		// Fire-and-forget: populates the updater store, which drives the banner.
		updater.check();
	});
</script>

{#if settings.loading}
	<div class="flex h-svh items-center justify-center">
		<Loading />
	</div>
{:else}
	<Shell.Root groups={nav.groups}>
		<Shell.Rail>
			{#snippet header({ showLabels })}
				{#if showLabels}
					<span class="min-w-0 flex-1 truncate px-3 text-sm font-semibold text-primary">
						GW2 Companion
					</span>
				{/if}
			{/snippet}
			{#snippet footer()}
				<Shell.SettingsFooter icon={Settings} />
			{/snippet}
		</Shell.Rail>

		<Shell.Content>
			{#snippet banner()}
				<UpdateBanner />
			{/snippet}
			{#snippet bottom()}
				<!-- Four slots, not five: with exactly five destinations nothing would
				     overflow, no More collector would exist, and settings (only in the
				     rail footer) would be unreachable at bar widths. -->
				<Shell.BottomBar slots={4} moreOwns={['/settings']} />
			{/snippet}
			{@render children()}
		</Shell.Content>
	</Shell.Root>
{/if}
