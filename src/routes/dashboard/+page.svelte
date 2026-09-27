<script lang="ts">
	import { Button, Loading } from '@veelume/ui';
	import { unwrap } from '$lib/api';
	import { commands, type AccountView } from '$lib/bindings';

	type View =
		| { kind: 'loading' }
		| { kind: 'no-key' }
		| { kind: 'ready'; account: AccountView }
		| { kind: 'error'; message: string };

	let view = $state<View>({ kind: 'loading' });

	async function load() {
		try {
			if (!(await unwrap(commands.getApiKeyStatus()))) {
				view = { kind: 'no-key' };
				return;
			}
			view = { kind: 'ready', account: await unwrap(commands.getAccountInfo()) };
		} catch (e) {
			view = { kind: 'error', message: e instanceof Error ? e.message : String(e) };
		}
	}

	load();
</script>

<div class="mx-auto w-full max-w-3xl space-y-4 p-[var(--density-padding)]">
	<h1 class="text-2xl font-semibold">Dashboard</h1>

	{#if view.kind === 'loading'}
		<Loading />
	{:else if view.kind === 'no-key'}
		<div class="space-y-3 rounded-lg border bg-card p-[var(--density-padding)]">
			<p>Welcome to GW2 Companion! Add an API key to get started.</p>
			<Button href="/settings/api-key">Set up API key</Button>
		</div>
	{:else if view.kind === 'error'}
		<div class="rounded-lg border bg-card p-[var(--density-padding)] text-sm text-destructive">
			{view.message}
		</div>
	{:else}
		{@const acc = view.account}
		<div class="rounded-lg border bg-card p-[var(--density-padding)]">
			<p class="text-lg font-medium">{acc.name}</p>
			<p class="text-sm text-muted-foreground">
				AP: {(acc.daily_ap ?? 0) + (acc.monthly_ap ?? 0)} · WvW Rank: {acc.wvw_rank ?? 'N/A'} ·
				Fractal Level: {acc.fractal_level ?? 'N/A'}
			</p>
		</div>
	{/if}
</div>
