// Navigation model. The layout reads `nav.groups`; adding a destination is
// one edit here. Types come from the kit so the shell accepts the items
// directly. Settings is not in `groups` — it lives in the rail footer and,
// at bar widths, on the More page.
import type { NavGroup, NavItem } from '@veelume/ui';
import { Coins, House, ListChecks, Sword, Trophy } from 'lucide-svelte';

const items: NavItem[] = [
	{ label: 'Dashboard', icon: House, path: '/dashboard' },
	{ label: 'Legendaries', icon: Sword, path: '/legendaries' },
	{ label: 'Achievements', icon: Trophy, path: '/achievements' },
	{ label: 'Trading Post', icon: Coins, path: '/trading-post' },
	{ label: 'Checklist', icon: ListChecks, path: '/checklist' }
];

class Nav {
	get groups(): NavGroup[] {
		return [{ items }];
	}

	get items(): NavItem[] {
		return this.groups.flatMap((g) => g.items);
	}
}

export const nav = new Nav();
