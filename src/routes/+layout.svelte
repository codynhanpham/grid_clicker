<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	
	import { register, isRegistered } from '@tauri-apps/plugin-global-shortcut';
	import { getCurrentWindow, type PhysicalSize } from '@tauri-apps/api/window';
	import {
		restoreStateCurrent,
		saveWindowState,
		StateFlags
	} from '@tauri-apps/plugin-window-state';
	import { invoke } from '@tauri-apps/api/core';

	import { TitleBar } from '$lib/components/app-titlebar';
	import type { MenubarData } from '$lib/components/app-titlebar/menubar.svelte';
	import Controller from '$lib/components/controller/controller.svelte';

	import { onMount, onDestroy } from 'svelte';





	let { children } = $props();

	restoreStateCurrent(StateFlags.ALL);


	let windowSize: PhysicalSize | undefined = $state(undefined);
	let menubarData: MenubarData = $state(null);
	let isDesktop = $state(false);
	let isWindowPassthrough = $state(false);

	getCurrentWindow().onResized(async () => {
		windowSize = await getCurrentWindow().innerSize();
	});


	onMount(async () => {
		windowSize = await getCurrentWindow().innerSize();
		// @ts-ignore
		if (typeof window.__TAURI__ !== 'undefined') {
			isDesktop = true;
		}

		// After the app is loaded, show the app window
		getCurrentWindow().show();

		// Save the window state after the app is loaded
		saveWindowState(StateFlags.ALL);
		// Before reloading the page (with Ctrl R, F5, Ctrl F5, etc.), save the window state
		window.addEventListener('beforeunload', () => {
			saveWindowState(StateFlags.ALL);
		});

		if (!(await isRegistered('F8'))) {
			await register('F8', async (event) => {
				if (event.state === 'Released') {
					getCurrentWindow().setFocus();
					getCurrentWindow().unminimize();
					// Example usage in your frontend
					await invoke("toggle_transparency")
					.then((isTransparent) => {
						isWindowPassthrough = isTransparent as boolean;
					})
					.catch((error) => console.error(error));
				}
			});
		}
	});

	onDestroy(() => {
		// Save the window state when the app is closed
		saveWindowState(StateFlags.ALL);
	});
</script>


<ModeWatcher />

<div class="flex flex-col h-screen w-full overflow-hidden">
	<TitleBar bind:menubarData />
	
	<div class="mt-8 bg-background/90 border-b border-primary/25">
		<Controller />
	</div>
	
	{@render children?.()}


	{#if isWindowPassthrough}
		<div id="window-passthrough-notice" class="z-10 opacity-90 select-none pointer-event-none w-max mt-8 bg-background/50 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-4 rounded-md border border-border">
			<span class="select-none pointer-event-none border-0 text-center text-sm md:text-base lg:text-lg text-foreground text-shadow dark:text-muted-foreground">
				Press F8 to turn OFF click-passthrough
			</span>
		</div>
	{/if}
</div>