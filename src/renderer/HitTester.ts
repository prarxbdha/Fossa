import type { DisplayItem } from "../types/display";

export class HitTester {
  constructor(private readonly items: DisplayItem[]) {}

  hitTest(x: number, y: number): DisplayItem | null {
    return (
      this.items.find(
        (item) =>
          x >= item.x &&
          x <= item.x + item.width &&
          y >= item.y &&
          y <= item.y + item.height,
      ) ?? null
    );
  }
}
