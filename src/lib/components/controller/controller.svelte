<script lang="ts" module>
	import { getDefaultRectangleStyle, getDefaultHomeStyle, type Point, type RectangleStyle, type HomeStyle } from '$routes/canvasDrawing';

    // Note that the values must match the MouseButton enum in the Rust code
    export type CursorClickMode = 'Left' | 'Right' | 'Middle';

    type cursorClickOption = {
        label: string;
        shortLabel?: string;
        value: CursorClickMode;
    }
    type CursorClickOptions = {
        duration: number;
        interval: number;
    }

    const cursorClickOptions: cursorClickOption[] = [
        {
            label: 'Left',
            shortLabel: 'L',
            value: 'Left'
        },
        {
            label: 'Right',
            shortLabel: 'R',
            value: 'Right'
        },
        {
            label: 'Middle',
            shortLabel: 'M',
            value: 'Middle'
        }
    ];

    type HomeClickOptions = {
        duration: number;
        cursorClickMode: CursorClickMode;
        clickAtStart: boolean;
        clickAtEnd: boolean;
        clickInBetween: boolean;
    }

    export type Alignment = 'center' | 'topleft' | 'topcenter' | 'topright' | 'middleleft' | 'middleright' | 'bottomleft' | 'bottomcenter' | 'bottomright';
    export type Grid = {
        rows: number;
        columns: number;
        alignment: Alignment;
    }



    export type ControllerState = {
        rectangleSelecting: boolean;
        cursorClickMode: CursorClickMode;
        cursorClickOptions: CursorClickOptions;
        grid: Grid;
        gridStyle: RectangleStyle;
        homeSelecting: boolean;
        homeSelection?: Point;
        homeClickOptions: HomeClickOptions;
        homeStyle: HomeStyle;

        cursorClickingInProgress: boolean;
    }

    export type ControllerProps = {
        class?: string;
    }

    export let controllerState: ControllerState = $state({
        rectangleSelecting: false,
        cursorClickMode: 'Left',
        cursorClickOptions: {
            duration: 5,
            interval: 10
        } as CursorClickOptions,
        grid: {
            rows: 2,
            columns: 2,
            alignment: 'center'
        } as Grid,
        gridStyle: getDefaultRectangleStyle(),

        homeSelecting: false,
        homeSelection: undefined,
        homeClickOptions: {
            duration: 10,
            cursorClickMode: 'Left',
            clickAtStart: true,
            clickInBetween: true,
            clickAtEnd: false,
        },
        homeStyle: getDefaultHomeStyle(),

        cursorClickingInProgress: false,
    });

</script>

<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

    import { storeGet, storeSet } from "$lib/appstore.js";

    import { Button } from "$lib/components/ui/button/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Switch } from "$lib/components/ui/switch/index.js";
    import { Toggle } from "$lib/components/ui/toggle/index.js";
    
    import { capitalizeFirstLetter, cn } from "$lib/utils.js";

    import { clearMainCanvas, clearCanvasRectangles, clearCanvasHome, getSelectedPointsScreenPosition, getSelectedHomePointScreenPosition, countSelectedPoints, getSelectedHomePoint } from "$routes/+page.svelte";

    import {
		AlignStartVertical,
        Columns3Cog,
        Grid3x2,
        Mouse,
        Play,
        Plus,
        LoaderCircle,
        SquareDashedMousePointer,
        Timer,
        Trash2,
		Gauge,
		CirclePause,
		HousePlus,
		MapPinHouse,
		HousePlug,
		Cable,
		CirclePlay,
		ClockFading,
		CircleStop,

    } from '@lucide/svelte/icons';
	import { onMount, tick } from "svelte";
	import { get } from "svelte/store";
	import Separator from '../ui/separator/separator.svelte';

    let {
        class: className,
    }: ControllerProps = $props();


    let allowStoreSave = $state(false);
    let isSmallScreen = $state(false);

    let gridOptsOpen = $state(false);
    let homeOptsOpen = $state(false);

    let nSelectedPoints = $derived(countSelectedPoints());
    let homeSelected = $derived(getSelectedHomePoint() !== null);

    let clickModeTriggerContent = $derived.by(
        () => {
            const option = cursorClickOptions.find((f) => f.value === controllerState.cursorClickMode);
            if (option) {
                return {
                    label: option.label,
                    shortLabel: option.shortLabel
                }
            }
            return {
                label: "Click mode",
                shortLabel: "Mode"
            };
        }
    );

    let homeClickModeTriggerContent = $derived.by(
        () => {
            const option = cursorClickOptions.find((f) => f.value === controllerState.homeClickOptions.cursorClickMode);
            if (option) {
                return {
                    label: option.label,
                    shortLabel: option.shortLabel
                }
            }
            return {
                label: "Click mode",
                shortLabel: "Mode"
            };
        }
    );


    const gridAlignmentOptions = [
        {
            label: 'Center',
            shortLabel: 'C',
            value: 'center'
        },
        {
            label: 'Top Left',
            shortLabel: 'TL',
            value: 'topleft'
        },
        {
            label: 'Top Center',
            shortLabel: 'TC',
            value: 'topcenter'
        },
        {
            label: 'Top Right',
            shortLabel: 'TR',
            value: 'topright'
        },
        {
            label: 'Middle Left',
            shortLabel: 'ML',
            value: 'middleleft'
        },
        {
            label: 'Middle Right',
            shortLabel: 'MR',
            value: 'middleright'
        },
        {
            label: 'Bottom Left',
            shortLabel: 'BL',
            value: 'bottomleft'
        },
        {
            label: 'Bottom Center',
            shortLabel: 'BC',
            value: 'bottomcenter'
        },
        {
            label: 'Bottom Right',
            shortLabel: 'BR',
            value: 'bottomright'
        }
    ];
    let alignmentTriggerContent = $derived.by(
        () => {
            const option = gridAlignmentOptions.find((f) => f.value === controllerState.grid.alignment);
            if (option) {
                return {
                    label: option.label,
                    shortLabel: option.shortLabel
                }
            }
            return {
                label: "Alignment",
                shortLabel: "Alignment"
            };
        }
    );
    
    onMount(() => {
        isSmallScreen = window.innerWidth < 640;

        setTimeout(() => {
            allowStoreSave = true;
        }, 5000);

        window.addEventListener('resize', () => {
            isSmallScreen = window.innerWidth < 640;
        });
    });

    $effect(() => {
        if (controllerState && allowStoreSave) {
            storeSet('controllerState', controllerState);
        }
    });

</script>




<div class={cn("flex flex-row w-full h-fit items-center justify-between gap-4 select-none p-2", className)}>
    <!-- Left Menu -->
    <div class="flex flex-row w-fit h-fit items-center justify-start gap-2">
        <Toggle
            disabled={controllerState.cursorClickingInProgress}
            title="Grid Selection"
            aria-label="toggle grid selection"
            bind:pressed={controllerState.rectangleSelecting}
            class={cn(
                "cursor-pointer bg-transparent border border-transparent",
                (controllerState.rectangleSelecting ? "!bg-secondary !text-secondary-foreground !border-border" : ""),
            )}
            onclick={() => {
                if (!controllerState.rectangleSelecting) {
                    controllerState.homeSelecting = false;
                }
            }}
        >
            <SquareDashedMousePointer class="size-5" />
        </Toggle>

        <Popover.Root bind:open={gridOptsOpen}>
            <Popover.Trigger
                disabled={controllerState.cursorClickingInProgress}
                class={cn(buttonVariants({ variant: "outline" }), "bg-background/20 hover:bg-accent/80 hover:cursor-pointer", (isSmallScreen ? "!p-2" : ""))} 
                title="Grid Settings"
            >
                <Columns3Cog class="size-5" />
                {#if !isSmallScreen}
                    <span>{capitalizeFirstLetter(alignmentTriggerContent.label)}</span>
                    <Separator orientation="vertical" class="bg-foreground/60" />
                    <span>{controllerState.grid.rows} x {controllerState.grid.columns}</span>
                {/if}
            </Popover.Trigger>
            <Popover.Content class="w-80">
                <div class="grid gap-4">
                    <div class="space-y-1">
                        <h4 class="font-medium leading-none">Grid Settings</h4>
                        <p class="text-muted-foreground text-sm">
                        Dimensions and alignment of the grid
                        </p>
                    </div>
                    <div class="grid gap-2">
                        <div class="grid grid-cols-8 items-center gap-4">
                            <Grid3x2 class="size-5" />
                            <Label for="rows" class="sr-only">Rows</Label>
                            <Input bind:value={controllerState.grid.rows} autocomplete="off" type="number" min={1} step={1} id="rows" placeholder="Rows"  class="col-span-3 h-8 text-center" 
                            onchange={() => {
                                controllerState.grid.rows = Math.round(controllerState.grid.rows);
                                if (controllerState.grid.rows < 1) {
                                    controllerState.grid.rows = 1;
                                }
                            }}
                            />
                            <span class="w-full text-center">x</span>
                            <Label for="columns" class="sr-only">Columns</Label>
                            <Input bind:value={controllerState.grid.columns} autocomplete="off" type="number" min={1} step={1} id="columns" placeholder="Columns"  class="col-span-3 h-8 text-center" 
                                onchange={() => {
                                    controllerState.grid.columns = Math.round(controllerState.grid.columns);
                                    if (controllerState.grid.columns < 1) {
                                        controllerState.grid.columns = 1;
                                    }
                                }}
                            />
                        </div>

                        <div class="grid grid-cols-8 items-center gap-4">
                            <AlignStartVertical class="size-5" />
                            <Select.Root type="single" name="gridalignment" bind:value={controllerState.grid.alignment}>
                                <Select.Trigger class="col-span-7 w-full text-muted-foreground hover:cursor-pointer">
                                    {alignmentTriggerContent.label}
                                </Select.Trigger>
                                <Select.Content collisionPadding={40} sideOffset={1}>
                                    <Select.Group>
                                    <Select.Label>Alignment Mode</Select.Label>
                                    {#each gridAlignmentOptions as gridAlignmentOption (gridAlignmentOption.value)}
                                        <Select.Item
                                            value={gridAlignmentOption.value}
                                            label={gridAlignmentOption.label}
                                        >
                                        {gridAlignmentOption.label}
                                        </Select.Item>
                                    {/each}
                                    </Select.Group>
                                </Select.Content>
                            </Select.Root>
                        </div>
                    </div>

                    <div class="w-full h-fit">
                        <Button
                            disabled={controllerState.cursorClickingInProgress || nSelectedPoints === 0}
                            variant="destructive"
                            title="Delete Grid"
                            class="select-none w-full cursor-pointer"
                            onclick={async () => { clearCanvasRectangles(); gridOptsOpen = false; }}
                        >
                            <Trash2 class="size-5" />
                            Delete Grid
                        </Button>
                    </div>
                </div>
            </Popover.Content>
        </Popover.Root>

        <Separator orientation="vertical" class="mx-0.5 py-3 px-[1px] bg-foreground/50" />

        <Toggle
            disabled={controllerState.cursorClickingInProgress}
            title="Home Selection"
            aria-label="toggle home selection"
            bind:pressed={controllerState.homeSelecting}
            class={cn(
                "cursor-pointer bg-transparent border border-transparent",
                (controllerState.homeSelecting ? "!bg-secondary !text-secondary-foreground !border-border" : ""),
            )}
            onclick={() => {
                if (!controllerState.homeSelecting) {
                    controllerState.rectangleSelecting = false;
                }
            }}
        >
            <MapPinHouse class="size-5" />
        </Toggle>

        <Popover.Root bind:open={homeOptsOpen}>
            <Popover.Trigger
                disabled={controllerState.cursorClickingInProgress}
                class={cn(buttonVariants({ variant: "outline" }), "!p-2 bg-background/20 hover:bg-accent/80 hover:cursor-pointer")}
                title="Home Click Settings"
            >
                <Cable class="size-5" />
            </Popover.Trigger>
            <Popover.Content class="w-86">
                <div class="grid gap-4">
                    <div class="space-y-1">
                        <h4 class="font-medium leading-none">Homing Click Options</h4>
                        <p class="text-muted-foreground text-sm">
                            How homing-clicks behave
                        </p>
                    </div>
                    <div class="grid gap-2">
                        <div class="grid grid-cols-9 items-center gap-4">
                            <Gauge class="size-5" />
                            <Label for="clickDuration" class="text-sm col-span-4" title="Duration of the mouse press, in miliseconds">Click Duration (ms)</Label>
                            <Input bind:value={controllerState.homeClickOptions.duration} autocomplete="off" type="number" min={1} step={1} id="clickDuration" placeholder="Hold Time" class="col-span-4 h-8 text-center" title="Duration of the mouse press, in miliseconds"
                            onchange={() => {
                                controllerState.homeClickOptions.duration = Math.round(controllerState.homeClickOptions.duration);
                                if (controllerState.homeClickOptions.duration < 1) {
                                    controllerState.homeClickOptions.duration = 1;
                                }
                            }}
                            />
                        </div>
                        <Select.Root type="single" name="homecursormode" bind:value={controllerState.homeClickOptions.cursorClickMode}>
                            <Select.Trigger
                                disabled={controllerState.cursorClickingInProgress}
                                class="w-full text-foreground !bg-accent/40 hover:!bg-accent/80 cursor-pointer"
                            >
                                <Mouse class="size-5 text-foreground" />
                                {homeClickModeTriggerContent.label}
                            </Select.Trigger>
                            <Select.Content sideOffset={-1}>
                                <Select.Group>
                                <Select.Label>Click mode</Select.Label>
                                {#each cursorClickOptions as cursorClickOption (cursorClickOption.value)}
                                    <Select.Item
                                        value={cursorClickOption.value}
                                        label={cursorClickOption.label}
                                    >
                                    {cursorClickOption.label}
                                    </Select.Item>
                                {/each}
                                </Select.Group>
                            </Select.Content>
                        </Select.Root>

                        <Separator orientation="horizontal" class="bg-foreground/40 my-2" />

                        <div class={cn("flex flex-row items-center justify-between gap-4", (controllerState.homeClickOptions.clickAtStart ? "" : "text-foreground/70"))}>
                            <div class="w-fit h-7 flex flex-row items-center justify-center gap-4">
                                <CirclePlay class="size-5" />
                                <Label for="homeClickStart" class="text-sm col-span-4 cursor-pointer" title="Click Home before clicking on grid">Home Click Start</Label>
                            </div>
                            <Switch bind:checked={controllerState.homeClickOptions.clickAtStart} class="w-8 h-fit cursor-pointer" id="homeClickStart" title="Click Home before clicking on grid" />
                        </div>

                        <div class={cn("flex flex-row items-center justify-between gap-4", (controllerState.homeClickOptions.clickInBetween ? "" : "text-foreground/70"))}>
                            <div class="w-fit h-7 flex flex-row items-center justify-center gap-4">
                                <ClockFading class="size-5" />
                                <Label for="homeClickInBetween" class="text-sm col-span-4 cursor-pointer" title="Click Home in between points on grid">Home Click In Between</Label>
                            </div>
                            <Switch bind:checked={controllerState.homeClickOptions.clickInBetween} class="w-8 h-fit cursor-pointer" id="homeClickInBetween" title="Click Home in between points on grid" />
                        </div>

                        <div class={cn("flex flex-row items-center justify-between gap-4", (controllerState.homeClickOptions.clickAtEnd ? "" : "text-foreground/70"))}>
                            <div class="w-fit h-7 flex flex-row items-center justify-center gap-4">
                                <CircleStop class="size-5" />
                                <Label for="homeClickEnd" class="text-sm col-span-4 cursor-pointer" title="Click Home after grid clicking is done">Home Click End</Label>
                            </div>
                            <Switch bind:checked={controllerState.homeClickOptions.clickAtEnd} class="w-8 h-fit cursor-pointer" id="homeClickEnd" title="Click Home after grid clicking is done" />
                        </div>
                    </div>
                    <div class="w-full h-fit">
                        <Button
                            disabled={controllerState.cursorClickingInProgress || !homeSelected}
                            variant="destructive"
                            title="Delete Home"
                            class="select-none w-full cursor-pointer"
                            onclick={async () => { clearCanvasHome(); homeOptsOpen = false; }}
                        >
                            <Trash2 class="size-5" />
                            Delete Home
                        </Button>
                    </div>
                </div>
            </Popover.Content>
        </Popover.Root>

        
    </div>



    <!-- Right Menu -->
    <div class="flex flex-row w-fit h-fit items-center justify-end gap-2">
        <Button
            disabled={controllerState.cursorClickingInProgress}
            variant="outline"
            aria-label="Delete selections"
            class={cn(
                "!p-2 bg-transparent cursor-pointer border border-transparent hover:!bg-destructive/60 hover:!text-foreground",
            )}
            title="Delete All Selections"
            onclick={async () => { clearMainCanvas(); }}
        >
            <Trash2 class="size-5" />
        </Button>
        <Select.Root type="single" name="cursormode" bind:value={controllerState.cursorClickMode}>
            <Select.Trigger
                disabled={controllerState.cursorClickingInProgress}
                class="w-fit text-muted-foreground bg-background/20 hover:bg-accent/80 cursor-pointer"
            >
                <Mouse class="size-5" />
                {#if !isSmallScreen}
                    {clickModeTriggerContent.label}
                {:else}
                    {clickModeTriggerContent.shortLabel}
                {/if}
            </Select.Trigger>
            <Select.Content>
                <Select.Group>
                <Select.Label>Click mode</Select.Label>
                {#each cursorClickOptions as cursorClickOption (cursorClickOption.value)}
                    <Select.Item
                        value={cursorClickOption.value}
                        label={cursorClickOption.label}
                    >
                    {cursorClickOption.label}
                    </Select.Item>
                {/each}
                </Select.Group>
            </Select.Content>
        </Select.Root>


        <Popover.Root>
            <Popover.Trigger
                disabled={controllerState.cursorClickingInProgress}
                class={cn(buttonVariants({ variant: "outline" }), "!p-2 bg-background/20 hover:bg-accent/80 hover:cursor-pointer")}
            >
                <Timer class="size-5" />
            </Popover.Trigger>
            <Popover.Content class="w-92">
                <div class="grid gap-4">
                    <div class="space-y-1">
                        <h4 class="font-medium leading-none">Click Options</h4>
                        <p class="text-muted-foreground text-sm">
                            How the auto-clicking behaves
                        </p>
                    </div>
                    <div class="grid gap-2">
                        <div class="grid grid-cols-9 items-center gap-4">
                            <Gauge class="size-5" />
                            <Label for="clickDuration" class="text-sm col-span-4" title="Duration of the mouse press, in miliseconds">Click Duration (ms)</Label>
                            <Input bind:value={controllerState.cursorClickOptions.duration} autocomplete="off" type="number" min={1} step={1} id="clickDuration" placeholder="Hold Time" class="col-span-4 h-8 text-center" title="Duration of the mouse press, in miliseconds" 
                            onchange={() => {
                                controllerState.cursorClickOptions.duration = Math.round(controllerState.cursorClickOptions.duration);
                                if (controllerState.cursorClickOptions.duration < 1) {
                                    controllerState.cursorClickOptions.duration = 1;
                                }
                            }}
                            />
                        </div>

                        <div class="grid grid-cols-9 items-center gap-4">
                            <CirclePause class="size-5" />
                            <Label for="clickInterval" class="text-sm col-span-4" title="Time between the previous click ending and the next click starting, in miliseconds">Click Intervals (ms)</Label>
                            <Input bind:value={controllerState.cursorClickOptions.interval} autocomplete="off" type="number" min={1} step={1} id="clickInterval" placeholder="Between Clicks" class="col-span-4 h-8 text-center" title="Time between the previous click ending and the next click starting, in miliseconds" 
                            onchange={() => {
                                controllerState.cursorClickOptions.interval = Math.round(controllerState.cursorClickOptions.interval);
                                if (controllerState.cursorClickOptions.interval < 1) {
                                    controllerState.cursorClickOptions.interval = 1;
                                }
                            }}
                            />
                        </div>
                        
                    </div>
                </div>
            </Popover.Content>
        </Popover.Root>


        <Button
            disabled={controllerState.cursorClickingInProgress || nSelectedPoints === 0}
            title={nSelectedPoints === 0 ? "Create a selection grid first!" : "Grid Click!"}
            class={cn("h-fit w-fit hover:cursor-pointer", (isSmallScreen ? "!p-2" : ""))}
            onclick={async () => {
                controllerState.cursorClickingInProgress = true;
                controllerState.rectangleSelecting = false;
                controllerState.homeSelecting = false;
                await tick();
                const lastMousePos = await invoke('get_mouse_pos') as Point;
                const points = await getSelectedPointsScreenPosition();
                const home = await getSelectedHomePointScreenPosition();
                if (!points) return;

                if (!controllerState.cursorClickOptions.duration) {
                    controllerState.cursorClickOptions.duration = 30;
                }
                if (!controllerState.cursorClickOptions.interval) {
                    controllerState.cursorClickOptions.interval = 69;
                }
                const mouseOptions = {
                    button: controllerState.cursorClickMode,
                    duration: controllerState.cursorClickOptions.duration,
                    interval: controllerState.cursorClickOptions.interval
                };
                const homeOptions = {
                    button: controllerState.homeClickOptions.cursorClickMode,
                    duration: controllerState.homeClickOptions.duration,
                    click_at_start: controllerState.homeClickOptions.clickAtStart,
                    click_in_between: controllerState.homeClickOptions.clickInBetween,
                    click_at_end: controllerState.homeClickOptions.clickAtEnd,
                }
                await tick();
                await invoke('mouse_click_points', { points: points, home: home, gridOptions: mouseOptions, homeOptions: homeOptions })
                await invoke('mouse_move_to', { point: lastMousePos })
                controllerState.cursorClickingInProgress = false;
            }}
        >
            {#if controllerState.cursorClickingInProgress}
                <div class="flex h-fit w-fit items-center justify-center">
                    <LoaderCircle class="size-5 animate-spin" />
                </div>
                {#if !isSmallScreen}
                    Running...
                {/if}
            {:else}
                <Play class="size-5" />
                {#if !isSmallScreen}
                    Grid Click!
                {/if}
            {/if}
        </Button>
    </div>
</div>