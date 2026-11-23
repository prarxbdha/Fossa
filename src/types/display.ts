export interface DisplayItemBase {
  x: number;
  y: number;
  width: number;
  height: number;
  color: [number, number, number, number];
}

export interface RectItem extends DisplayItemBase {
  kind: "rect";
}

export interface TextItem extends DisplayItemBase {
  kind: "text";
  text: string;
  font_size: number;
}

export type DisplayItem = RectItem | TextItem;

export interface DisplayList {
  items: DisplayItem[];
}
