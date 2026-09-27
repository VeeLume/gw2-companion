import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

// Tauri sets TAURI_DEV_HOST to the machine's LAN IP when targeting a real
// device, so the dev server must bind that interface (not just localhost).
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	// @veelume/ui is git-installed and ships source: esbuild's dependency
	// pre-bundler cannot parse its `.svelte.ts` rune modules, so the dev server
	// must leave it to the Svelte plugin. Prod builds are not affected.
	optimizeDeps: {
		exclude: ['@veelume/ui'],
		// Excluding a package also stops Vite from pre-bundling ITS dependencies;
		// the kit's own nested copies would then go through the Svelte plugin at
		// request time and hit the virtual-CSS miss. Listing them explicitly
		// restores the normal pre-bundle for the nested copies.
		include: [
			'@veelume/ui > bits-ui',
			'@veelume/ui > @floating-ui/dom',
			'@veelume/ui > @internationalized/date',
			'@veelume/ui > clsx',
			'@veelume/ui > tailwind-merge'
		]
	},
	// Don't let Vite wipe the terminal — it hides Rust compile errors.
	clearScreen: false,
	server: {
		host: host || false,
		port: 1420,
		strictPort: true,
		hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
		watch: {
			// Rust rebuilds are handled by Tauri, not Vite.
			ignored: ['**/src-tauri/**']
		}
	}
});
