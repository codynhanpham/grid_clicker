<script lang="ts" module>
	import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
	import { controllerState } from '$lib/components/controller/controller.svelte';
    import { strokeRectangle, getDefaultRectangleStyle, drawText, strokeX } from './canvasDrawing';
    import type { Rectangle, RectangleStyle, Point } from './canvasDrawing';

    let appCanvas: HTMLCanvasElement | undefined = $state(undefined);
    let canvasContext: CanvasRenderingContext2D | null = $state(null);
    export function clearMainCanvas() {
        if (appCanvas && canvasContext) {
            canvasContext.clearRect(0, 0, appCanvas.width, appCanvas.height);
        }

        if (selectedPoints.length) {
            selectedPoints = [];
        }
        if (selectedRectangles.length) {
            selectedRectangles = [];
        }
        if (selectedHomePoint) {
            selectedHomePoint = null;
        }
    }

    
    let selectedRectangles = $state<Rectangle[]>([]);
    let selectedPoints = $state<Point[]>([]);
    let selectedHomePoint = $state<Point | null>(null);

    export function countSelectedPoints(): number {
        return selectedPoints.length;
    }
    export function getSelectedRectangles(): Rectangle[] {
        return selectedRectangles;
    }
    export function getSelectedPoints(): Point[] {
        return selectedPoints;
    }
    export function getSelectedHomePoint(): Point | null {
        return selectedHomePoint;
    }

    export function clearCanvasHome() {
        if (appCanvas && canvasContext) {
            canvasContext.clearRect(0, 0, appCanvas.width, appCanvas.height);
        }
        selectedHomePoint = null;
        // redraw the rectangle
        if (selectedRectangles.length) {
            for (const rectangle of selectedRectangles) {
                strokeRectangle(canvasContext!, rectangle, getDefaultRectangleStyle(), controllerState.grid);
            }
        }
    }
    export function clearCanvasRectangles() {
        if (appCanvas && canvasContext) {
            canvasContext.clearRect(0, 0, appCanvas.width, appCanvas.height);
        }
        selectedRectangles = [];
        selectedPoints = [];
        // redraw home icon
        if (selectedHomePoint && appCanvas) {
            strokeX(canvasContext!, selectedHomePoint.x, selectedHomePoint.y, 8, controllerState.homeStyle.strokeColor!, controllerState.homeStyle.lineWidth);
            let textX = selectedHomePoint.x + appCanvas.width * 0.02;
            let textY = selectedHomePoint.y + appCanvas.height * 0.02;
            if (selectedHomePoint.x > appCanvas.width * 0.8) {
                textX = selectedHomePoint.x - appCanvas.width * 0.06;
            }
            if (selectedHomePoint.y > appCanvas.height * 0.8) {
                textY = selectedHomePoint.y - appCanvas.height * 0.02;
            }
            drawText(canvasContext!, "Home", textX, textY, controllerState.homeStyle.strokeColor!, 16, 0.5);
        }
    }

    function canvasPos2ViewportPos(canvas: HTMLCanvasElement, pos: Point): Point {
        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;
        const x = pos.x / scaleX + rect.left;
        const y = pos.y / scaleY + rect.top;
        return {
            x,
            y
        }
    }

    async function canvasPos2ScreenPos(canvas: HTMLCanvasElement, pos: Point): Promise<Point> {
        const appWindow = getCurrentWindow();
        const windowPos = await appWindow.innerPosition();
        const factor = await appWindow.scaleFactor();
        const canvasPos = canvasPos2ViewportPos(canvas, pos);
        
        const size = new LogicalSize((canvasPos.x), (canvasPos.y));
        const physical = size.toPhysical(factor);

        return {
            x: physical.width + windowPos.x,
            y: physical.height + windowPos.y
        }
    }

    export async function getSelectedPointsScreenPosition(): Promise<Point[] | null> {
        if (!selectedPoints.length) return null;
        if (!appCanvas) return null;
        const output: Point[] = [];

        const appWindow = getCurrentWindow();
        const windowPos = await appWindow.innerPosition();
        const factor = await appWindow.scaleFactor();
        const rect = appCanvas.getBoundingClientRect();
        const scaleX = appCanvas.width / rect.width;
        const scaleY = appCanvas.height / rect.height;
        
        for (const point of selectedPoints) {
            let x = point.x / scaleX + rect.left;
            let y = point.y / scaleY + rect.top;
            const size = new LogicalSize((x), (y));
            const physical = size.toPhysical(factor);

            output.push({
                x: physical.width + windowPos.x,
                y: physical.height + windowPos.y
            });
        }
        return output;
    }
    export async function getSelectedHomePointScreenPosition(): Promise<Point | null> {
        if (!selectedHomePoint) return null;
        if (!appCanvas) return null;
        
        return await canvasPos2ScreenPos(appCanvas, selectedHomePoint);
    }

</script>

<script lang="ts">
    import { cn } from '$lib/utils.js';

    import { onMount } from 'svelte';


    let isDragging = $state(false);
    let dragStart = $state({ x: 0, y: 0 });
    let dragEnd = $state({ x: 0, y: 0 });
    let tempRectangle: Rectangle | null = null;
    let tempGridPoints: Point[] | null = null;
    
    let canvasMousePos: Point | null = $state(null);

    function makeFullSizeCanvas(canvas?: HTMLCanvasElement, window?: Window) {
        if (!canvas || !window) return;
        const rect = canvas.getBoundingClientRect();
        canvas.width = rect.width;
        canvas.height = rect.height;
    }

    function getCanvasMousePos(canvas: HTMLCanvasElement, evt: MouseEvent): Point {
        const rect = canvas.getBoundingClientRect(),
            scaleX = canvas.width / rect.width,
            scaleY = canvas.height / rect.height;

        return {
            x: (evt.clientX - rect.left) * scaleX,
            y: (evt.clientY - rect.top) * scaleY
        }
    }


    onMount(async () => {
        appCanvas = document.getElementById('app-canvas') as HTMLCanvasElement;
        makeFullSizeCanvas(appCanvas, window);
        canvasContext = appCanvas.getContext('2d');

        if (appCanvas) {
            appCanvas.addEventListener('click', async () => {
                getCurrentWindow().setFocus();
            });
        }

        window.addEventListener('resize', () => {
            if (appCanvas) {
                clearMainCanvas();
                makeFullSizeCanvas(appCanvas, window);
            }
        });

        appCanvas.addEventListener('mousedown', (event: MouseEvent) => {
            if (event.button !== 0) return;
            if (!appCanvas) return;
            
            if (controllerState.rectangleSelecting) {
                isDragging = true;
                const mousePos = getCanvasMousePos(appCanvas, event);
                dragStart.x = mousePos.x;
                dragStart.y = mousePos.y;
            }
            else if (controllerState.homeSelecting) {
                clearCanvasHome();
                const mousePos = getCanvasMousePos(appCanvas, event);
                controllerState.homeSelection = mousePos;
                strokeX(canvasContext!, mousePos.x, mousePos.y, 8, controllerState.homeStyle.strokeColor!, controllerState.homeStyle.lineWidth);
                let textX = mousePos.x + appCanvas.width * 0.02;
                let textY = mousePos.y + appCanvas.height * 0.02;
                if (mousePos.x > appCanvas.width * 0.8) {
                    textX = mousePos.x - appCanvas.width * 0.06;
                }
                if (mousePos.y > appCanvas.height * 0.8) {
                    textY = mousePos.y - appCanvas.height * 0.02;
                }
                drawText(canvasContext!, "Home", textX, textY, controllerState.homeStyle.strokeColor!, 16, 0.5);
                selectedHomePoint = mousePos;
            }
        });
        appCanvas.addEventListener('mousemove', (event: MouseEvent) => {
            if (!controllerState.rectangleSelecting || !appCanvas || !canvasContext || !isDragging) return;
            clearCanvasRectangles();

            const mousePos = getCanvasMousePos(appCanvas, event);
            dragEnd.x = mousePos.x;
            dragEnd.y = mousePos.y;

            let { rectangle, gridPoints } = strokeRectangle(canvasContext!, {
                x: Math.min(dragStart.x, dragEnd.x),
                y: Math.min(dragStart.y, dragEnd.y),
                width: Math.abs(dragStart.x - dragEnd.x),
                height: Math.abs(dragStart.y - dragEnd.y)
            }, controllerState.gridStyle, controllerState.grid);
            tempRectangle = rectangle;
            tempGridPoints = gridPoints;
        });
        appCanvas.addEventListener('mouseup', (event: MouseEvent) => {
            if (event.button !== 0) return;
            if (!controllerState.rectangleSelecting || !appCanvas || !canvasContext || !isDragging) return;
            isDragging = false;

            if (dragStart.x === dragEnd.x && dragStart.y === dragEnd.y) {
                // No change in position, do nothing
                return;
            }
            else {
                if (!tempRectangle || !tempGridPoints) return;
                selectedRectangles.push(tempRectangle);
                selectedPoints.push(...tempGridPoints);
                tempRectangle = null;
                tempGridPoints = null;
            }
        });
    });

    $effect(() => {
        if (isDragging && !controllerState.rectangleSelecting) {
            isDragging = false;
        }
    });


    $effect(() => {
        if (controllerState.grid && selectedRectangles.length) {
            if (appCanvas && canvasContext) {
                canvasContext.clearRect(0, 0, appCanvas.width, appCanvas.height);
            }
            // redraw home icon
            if (selectedHomePoint && appCanvas) {
                strokeX(canvasContext!, selectedHomePoint.x, selectedHomePoint.y, 8, controllerState.homeStyle.strokeColor!, controllerState.homeStyle.lineWidth);
                let textX = selectedHomePoint.x + appCanvas.width * 0.02;
                let textY = selectedHomePoint.y + appCanvas.height * 0.02;
                if (selectedHomePoint.x > appCanvas.width * 0.8) {
                    textX = selectedHomePoint.x - appCanvas.width * 0.06;
                }
                if (selectedHomePoint.y > appCanvas.height * 0.8) {
                    textY = selectedHomePoint.y - appCanvas.height * 0.02;
                }
                drawText(canvasContext!, "Home", textX, textY, controllerState.homeStyle.strokeColor!, 16, 0.5);
            }

            // Update the grid with new options
            let newPoints: Point[] = [];
            for (let i = 0; i < selectedRectangles.length; i++) {
                const rect = selectedRectangles[i];
                let { rectangle, gridPoints } = strokeRectangle(canvasContext!, rect, controllerState.gridStyle, controllerState.grid);
                selectedRectangles[i] = rectangle;
                newPoints.push(...gridPoints);
            }
            selectedPoints = newPoints;
        }
    });
</script>

<div class="relative flex-1 w-full h-full overflow-hidden">
    <canvas
        id="app-canvas"
        class={cn(
            "relative flex-1 w-full h-full overflow-hidden",
            ((controllerState.rectangleSelecting || controllerState.homeSelecting) ? "cursor-crosshair" : "")
        )}
        onmousemove={async (event: MouseEvent) => {
            if (!appCanvas) {
                canvasMousePos = null;
                return;
            }
            canvasMousePos = getCanvasMousePos(appCanvas, event);
        }}
        onmouseleave={async () => {
            canvasMousePos = null;
        }}
    >
    </canvas>
    <!-- A dot that follows the mouse cursor -->
    {#if canvasMousePos}
        <div class="absolute w-2 h-2 -translate-1/2 rounded-full bg-emerald-500 pointer-events-none duration-0" style={`left: ${canvasMousePos.x}px; top: ${canvasMousePos.y}px`}></div>
    {/if}
</div>
