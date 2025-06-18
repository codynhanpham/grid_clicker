import type { Grid } from "$lib/components/controller/controller.svelte";


export type Rectangle = {
    x: number;
    y: number;
    width: number;
    height: number;
    style?: RectangleStyle;
}
export type Point = {
    x: number;
    y: number;
}
export type RectangleStyle = {
    fillColor: string | null; // set to null to disable fill
    strokeColor: string | null; // set to null to disable stroke
    lineWidth: number;
}
export const defaultRectangleStyle: RectangleStyle = {
    fillColor: null,
    strokeColor: 'yellow',
    lineWidth: 3
}
export function getDefaultRectangleStyle(): RectangleStyle {
    return defaultRectangleStyle;
}

export type HomeStyle = {
    fillColor: string | null; // set to null to disable fill
    strokeColor: string | null; // set to null to disable stroke
    lineWidth: number;
}
export const defaultHomeStyle: HomeStyle = {
    fillColor: null,
    strokeColor: 'orange',
    lineWidth: 3
}
export function getDefaultHomeStyle(): HomeStyle {
    return defaultHomeStyle;
}




export function strokeRectangle(context: CanvasRenderingContext2D, rectangle: Rectangle, style: RectangleStyle = defaultRectangleStyle, grid: Grid = { rows: 1, columns: 1, alignment: 'center' }) {
    context.lineWidth = style.lineWidth;
    if (style.fillColor) {
        context.fillStyle = style.fillColor;
    }
    if (style.strokeColor) {
        context.strokeStyle = style.strokeColor;
    }

    context.shadowColor = "black";
    context.shadowBlur = 3;
    context.shadowOffsetX = 0;
    context.shadowOffsetY = 0;

    // Add grid lines with points according to the grid + alignment
    const gridPoints: Point[] = addRectangleGridLines(context, rectangle, style, grid);

    // Draw the rectangle on top
    context.beginPath();
    context.rect(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
    if (style.fillColor) {
        context.fill();
    }
    if (style.strokeColor) {
        // solid line dash
        context.setLineDash([]);
        context.stroke();
    }
    context.closePath();

    return {
        rectangle: rectangle,
        gridPoints: gridPoints
    }
}


export function strokeX(context: CanvasRenderingContext2D, x: number, y: number, size: number, color: string, lineWidth: number=2) {
    context.strokeStyle = color;
    context.lineWidth = lineWidth;
    context.beginPath();
    context.moveTo(x - size, y - size);
    context.lineTo(x + size, y + size);
    context.moveTo(x + size, y - size);
    context.lineTo(x - size, y + size);
    context.stroke();
    context.closePath();
}

export function drawText(context: CanvasRenderingContext2D, text: string, x: number, y: number, color: string, fontSize: number=16, lineWidth: number=2) {
    context.font = `${fontSize}px sans-serif`;
    context.fillStyle = color;
    context.lineWidth = lineWidth;
    context.strokeStyle = color;
    context.strokeText(text, x, y);
    context.fillText(text, x, y);
}


export function addRectangleGridLines(context: CanvasRenderingContext2D, rectangle: Rectangle, style: RectangleStyle = defaultRectangleStyle, grid: Grid = { rows: 1, columns: 1, alignment: 'center' }): Point[] {
    const gridLineWidth = style.lineWidth * 0.4;
    const gridLineColor = style.strokeColor;
    const gridLineDash = [gridLineWidth*4, gridLineWidth*6];

    // Calculate the grid lines based on the grid size, except for the bounding rectangle
    const gridLines: [Point, Point][] = [];
    // Horizontal internal lines
    for (let i = 1; i < grid.rows; i++) {
        const rowY = rectangle.y + i * rectangle.height / grid.rows;
        gridLines.push([
            { x: rectangle.x, y: rowY },
            { x: rectangle.x + rectangle.width, y: rowY }
        ]);
    }
    // Vertical internal lines
    for (let j = 1; j < grid.columns; j++) {
        const columnX = rectangle.x + j * rectangle.width / grid.columns;
        gridLines.push([
            { x: columnX, y: rectangle.y },
            { x: columnX, y: rectangle.y + rectangle.height }
        ]);
    }
    
    // Draw the grid lines
    for (const [start, end] of gridLines) {
        context.beginPath();
        context.moveTo(start.x, start.y);
        context.lineTo(end.x, end.y);
        if (gridLineColor) {
            context.setLineDash(gridLineDash);
            context.strokeStyle = gridLineColor;
            context.lineWidth = gridLineWidth;
            context.stroke();
        }
        context.closePath();
    }

    // Determine the points on the grid lines based on the alignment
    const gridPoints: Point[] = getPointsInGrid(rectangle, grid);

    // Draw the grid points
    const pointRadius = style.lineWidth * 1.5; // Adjust as needed
    for (const point of gridPoints) {
        context.beginPath();
        context.arc(point.x, point.y, pointRadius, 0, Math.PI * 2);
        if (style.strokeColor) {
            context.fillStyle = style.strokeColor;
            context.fill();
        }
        context.closePath();
    }

    return gridPoints;
}

function getPointsInGrid(boundingRectangle: Rectangle, grid: Grid): Point[] {
    const gridPoints: Point[] = [];

    const rectX = boundingRectangle.x;
    const rectY = boundingRectangle.y;
    const rectWidth = boundingRectangle.width;
    const rectHeight = boundingRectangle.height;

    const { rows, columns, alignment } = grid;

    // Calculate the cell width and height
    const cellWidth = rectWidth / columns;
    const cellHeight = rectHeight / rows;

    // For each cell in the grid, calculate the point based on alignment
    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < columns; col++) {
            // Calculate cell boundaries
            const cellLeft = rectX + col * cellWidth;
            const cellTop = rectY + row * cellHeight;
            
            // Calculate point position based on alignment
            let pointX: number;
            let pointY: number;

            // Horizontal alignment
            switch (alignment) {
                case 'topleft':
                case 'middleleft':
                case 'bottomleft':
                    pointX = cellLeft;
                    break;
                case 'topcenter':
                case 'center':
                case 'bottomcenter':
                    pointX = cellLeft + cellWidth / 2;
                    break;
                case 'topright':
                case 'middleright':
                case 'bottomright':
                    pointX = cellLeft + cellWidth;
                    break;
                default:
                    pointX = cellLeft + cellWidth / 2; // Default to center
            }

            // Vertical alignment
            switch (alignment) {
                case 'topleft':
                case 'topcenter':
                case 'topright':
                    pointY = cellTop;
                    break;
                case 'middleleft':
                case 'center':
                case 'middleright':
                    pointY = cellTop + cellHeight / 2;
                    break;
                case 'bottomleft':
                case 'bottomcenter':
                case 'bottomright':
                    pointY = cellTop + cellHeight;
                    break;
                default:
                    pointY = cellTop + cellHeight / 2; // Default to center
            }

            gridPoints.push({ x: pointX, y: pointY });
        }
    }

    return gridPoints;
}