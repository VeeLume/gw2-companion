<script lang="ts">
	// Where the bottom bar's More collector lands. `splitBottomNav` runs the
	// same computation as the bar (same `slots`), so the two cannot disagree
	// about what overflowed. Settings rides along: at bar widths the rail
	// footer is gone, so this is its only entry.
	import { Shell, splitBottomNav, type NavItem } from '@veelume/ui';
	import { Settings } from 'lucide-svelte';
	import { nav } from '$lib/nav.svelte';

	const settingsItem: NavItem = { label: 'Settings', icon: Settings, path: '/settings' };

	const items = $derived([...splitBottomNav(nav.items, { slots: 4 }).overflow, settingsItem]);
</script>

<div class="mx-auto max-w-2xl space-y-4 p-4">
	<h1 class="text-lg font-semibold">More</h1>
	<Shell.MoreList {items} />
</div>
