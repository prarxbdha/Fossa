import type { DisplayList } from "../types/display";

const FONT_STACK = '-apple-system, BlinkMacSystemFont, "Segoe UI", Inter, sans-serif';

export class CanvasRenderer {
  constructor(private readonly canvas: HTMLCanvasElement) {}

  render(displayList: DisplayList, cssWidth?: number, cssHeight?: number): void {
    const context = this.canvas.getContext("2d");
    if (!context) return;

    const width = cssWidth ?? this.canvas.clientWidth;
    const height = cssHeight ?? this.canvas.clientHeight;
    const ratio = window.devicePixelRatio || 1;
    const pixelWidth = Math.max(1, Math.round(width * ratio));
    const pixelHeight = Math.max(1, Math.round(height * ratio));

    if (this.canvas.width !== pixelWidth || this.canvas.height !== pixelHeight) {
      this.canvas.width = pixelWidth;
      this.canvas.height = pixelHeight;
    }

    context.setTransform(ratio, 0, 0, ratio, 0, 0);
    context.clearRect(0, 0, width, height);
    context.textBaseline = "top";

    // Clip draw commands to the viewport so an oversized item can never
    // introduce horizontal overflow in the browser chrome.
    context.save();
    context.beginPath();
    context.rect(0, 0, width, height);
    context.clip();

    for (const item of displayList.items) {
      const [red, green, blue, alpha] = item.color;
      context.fillStyle = `rgba(${red}, ${green}, ${blue}, ${alpha / 255})`;

      if (item.kind === "rect") {
        context.fillRect(item.x, item.y, item.width, item.height);
      } else {
        context.font = `${item.font_size}px ${FONT_STACK}`;
        context.fillText(item.text, item.x, item.y);
      }
    }

    context.restore();
  }
}
