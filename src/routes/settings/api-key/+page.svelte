<script lang="ts">
	import { Button, Settings } from '@veelume/ui';
	import { unwrap } from '$lib/api';
	import { commands } from '$lib/bindings';

	let key = $state('');
	let hasKey = $state<boolean | null>(null);
	let status = $state<
		{ kind: 'idle' } | { kind: 'saving' } | { kind: 'saved' } | { kind: 'error'; message: string }
	>({ kind: 'idle' });

	unwrap(commands.getApiKeyStatus()).then((v) => (hasKey = v));

	async function save() {
		const trimmed = key.trim();
		if (!trimmed) return;
		status = { kind: 'saving' };
		try {
			await unwrap(commands.setApiKey(trimmed));
			key = '';
			hasKey = true;
			status = { kind: 'saved' };
		} catch (e) {
			status = { kind: 'error', message: e instanceof Error ? e.message : String(e) };
		}
	}
</script>

<Settings.Page title="API key">
	<Settings.Section>
		<Settings.Row
			label="API key"
			hint="Create one at account.arena.net/applications with the permissions account, characters, inventories, progression, tradingpost, unlocks and wallet."
		>
			<form
				class="flex w-full min-w-0 gap-2 sm:w-96"
				onsubmit={(e) => {
					e.preventDefault();
					save();
				}}
			>
				<input
					type="password"
					autocomplete="off"
					placeholder={hasKey ? 'Replace the saved key…' : 'Paste your API key…'}
					class="min-w-0 flex-1 rounded-md border border-input bg-background px-3 text-sm"
					style="height: var(--density-target)"
					bind:value={key}
				/>
				<Button type="submit" size="field" disabled={status.kind === 'saving' || !key.trim()}>
					{status.kind === 'saving' ? 'Validating…' : 'Save'}
				</Button>
			</form>
		</Settings.Row>
	</Settings.Section>

	{#if status.kind === 'saved'}
		<p class="text-sm text-success">API key validated and saved.</p>
	{:else if status.kind === 'error'}
		<p class="text-sm text-destructive">{status.message}</p>
	{:else if hasKey}
		<p class="text-sm text-muted-foreground">A key is saved.</p>
	{/if}
</Settings.Page>
