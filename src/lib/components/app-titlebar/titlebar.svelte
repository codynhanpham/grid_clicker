<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';

	import Menubar from './menubar.svelte';
	import type { MenubarData } from './menubar.svelte';

	import IconMinus from '@lucide/svelte/icons/minus';
	import Square from '@lucide/svelte/icons/square';
	import Copy from '@lucide/svelte/icons/copy';
	import IconX from '@lucide/svelte/icons/x';

	let {
		menubarData = $bindable<MenubarData>(null),
		class: className
	}: {
		menubarData: MenubarData;
		class?: string;
	} = $props();

	let isMaximized = $state(false);

	getCurrentWindow().onResized(async () => {
		isMaximized = await getCurrentWindow().isMaximized();
	});

	onMount(async () => {
		isMaximized = await getCurrentWindow().isMaximized();
	});
</script>

<div
	data-tauri-drag-region
	class={cn(
		'bg-background/90 pointer-events-auto fixed left-0 right-0 top-0 z-[99999] h-fit border-b border-primary/25 hover:cursor-grab active:cursor-grabbing',
		className,
		'!max-h-fit'
	)}
>
	<div data-tauri-drag-region class="flex h-fit w-full items-center justify-between gap-8">
		<div data-tauri-drag-region class="h-fit w-full flex-shrink">
			<Menubar bind:data={menubarData} />
		</div>

		<div data-tauri-drag-region class="flex h-fit min-w-max flex-shrink-0 flex-grow items-center gap-0.5">
			<Button
				id="titlebar-minimize"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-accent hover:!text-accent-foreground focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().minimize();
				}}
			>
				<IconMinus aria-label="minimize" class="size-5" strokeWidth="2" />
			</Button>
			<Button
				id="titlebar-restore"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-accent hover:!text-accent-foreground focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().toggleMaximize();
				}}
			>
				{#if isMaximized}
					<Copy aria-label="maximize" class="size-3.5 scale-x-[-1]" />
				{:else}
					<Square aria-label="restore" class="size-3.5" strokeWidth="2" />
				{/if}
			</Button>

			<Button
				id="titlebar-close"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-destructive hover:!text-foreground focus-visible:!bg-destructive focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().close();
				}}
			>
				<IconX aria-label="close" class="size-5" strokeWidth="1.5" />
			</Button>
		</div>
	</div>
</div>
