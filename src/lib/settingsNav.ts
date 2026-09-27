// The settings categories, in display order: adding a setting is one entry
// here plus one small page under routes/settings/<id>/.
import type { SettingsCategory } from '@veelume/ui';
import { KeyRound, Palette } from 'lucide-svelte';

export const settingsCategories: SettingsCategory[] = [
	{
		id: 'api-key',
		label: 'API key',
		description: 'The Guild Wars 2 account this app reads',
		icon: KeyRound,
		path: '/settings/api-key'
	},
	{
		id: 'appearance',
		label: 'Appearance',
		description: 'Theme and density',
		icon: Palette,
		path: '/settings/appearance'
	}
];
