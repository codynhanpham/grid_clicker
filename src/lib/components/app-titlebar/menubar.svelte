<script lang="ts" module>
	export type MenubarExperimentInfo = {
		name: string;
		path: string;
	};
	export type MenubarData = {
		recentExperiments: MenubarExperimentInfo[] | null;
	} | null;
</script>

<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import * as Menubar from '$lib/components/ui/menubar/index.js';

	import { setMode, mode } from 'mode-watcher';

	import { menuOpenExperiment } from './appmenu';
	import { onMount } from 'svelte';

	let { data = $bindable<MenubarData>(null) }: { data: MenubarData } = $props();

	let recentExperimentsMenu: MenubarExperimentInfo[] | null = $derived.by(() => {
		let recentExps = data?.recentExperiments ?? null;
		if (recentExps) {
			return recentExps.map((exp) => {
				return {
					name: exp.name,
					path: exp.path
				};
			});
		}
		return null;
	});

	onMount(() => {
		// Listen for a single Alt press, prevent default, and focus the menubar
		document.addEventListener('keydown', (event) => {
			if (event.key === 'Alt' && event.repeat === false) {
				event.preventDefault();
				document.getElementById('app-menubar-file')?.focus();
			}
		});
	});
</script>

<Menubar.Root
	data-tauri-drag-region
	class="isolate flex h-max w-full shrink overflow-y-clip rounded-none border-0 py-0.5 bg-transparent"
>
	<Menubar.Menu>
		<Menubar.Trigger
			class="my-0.5 py-0.5 font-normal text-muted-foreground hover:bg-accent hover:text-foreground"
			id="app-menubar-file">File</Menubar.Trigger
		>
		<Menubar.Content sideOffset={4}>
			<Menubar.Item disabled>
				Open...
				<Menubar.Shortcut>Ctrl+O</Menubar.Shortcut>
			</Menubar.Item>
			<Menubar.Sub>
				<Menubar.SubTrigger>Open Recent</Menubar.SubTrigger>
				<Menubar.SubContent class="w-[230px]" side="right" sideOffset={2} align="start">
					{#if !recentExperimentsMenu}
						<Menubar.Item disabled>No recent configs</Menubar.Item>
					{:else}
						{#each recentExperimentsMenu as exp}
							<Menubar.Item onclick={menuOpenExperiment(exp)}>{exp.name}</Menubar.Item>
						{/each}
					{/if}
				</Menubar.SubContent>
			</Menubar.Sub>

			<Menubar.Separator />
			<Menubar.Item disabled>
				Save...
				<Menubar.Shortcut>Ctrl+S</Menubar.Shortcut>
			</Menubar.Item>
			<Menubar.Item disabled>
				Save As...
				<Menubar.Shortcut>Ctrl+Shift+S</Menubar.Shortcut>
			</Menubar.Item>

			<Menubar.Separator />
			<Menubar.Item
				onclick={() => {
					invoke('exit_app');
				}}
			>
				Exit
				<Menubar.Shortcut>Alt+F4</Menubar.Shortcut>
			</Menubar.Item>
		</Menubar.Content>
	</Menubar.Menu>

	<Menubar.Menu>
		<Menubar.Trigger
			class="my-0.5 py-0.5 font-normal text-muted-foreground hover:bg-accent hover:text-foreground"
			>View</Menubar.Trigger
		>
		<Menubar.Content sideOffset={4}>
			<Menubar.Sub>
				<Menubar.SubTrigger>Appearance</Menubar.SubTrigger>
				<Menubar.SubContent class="w-[230px]" side="right" sideOffset={2} align="start">
					<Menubar.CheckboxItem
						checked={mode.current == 'dark'}
						onclick={() => {
							setMode('dark');
						}}
					>
						Dark
					</Menubar.CheckboxItem>
					<Menubar.CheckboxItem
						checked={mode.current == 'light'}
						onclick={() => {
							setMode('light');
						}}
					>
						Light
					</Menubar.CheckboxItem>
				</Menubar.SubContent>
			</Menubar.Sub>
		</Menubar.Content>
	</Menubar.Menu>
</Menubar.Root>
