import type { DisplayList } from "./display";

export interface BrowserHistory {
  entries: string[];
  index: number;
}

export interface BrowserTab {
  id: string;
  url: string;
  title: string;
  /** Raw document text (or an error message when a load fails). */
  content: string;
  /** Rendered display list supplied by the engine bridge, when available. */
  displayList: DisplayList | null;
  history: BrowserHistory;
  loading: boolean;
  error: string | null;
}
