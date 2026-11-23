import { invoke } from "@tauri-apps/api/core";
import type { DisplayList } from "../types/display";

export function getCurrentUrl(): Promise<string> {
  return invoke<string>("get_current_url");
}

export function setCurrentUrl(url: string): Promise<void> {
  return invoke("set_current_url", { url });
}

export function fetchDocument(url: string): Promise<string> {
  const builtin = url.startsWith("fossa://") ? url.slice("fossa://".length) : null;
  if (builtin) {
    return invoke<string>("read_fixture", { name: builtin });
  }
  return invoke<string>("fetch_document", { url });
}

export function renderDocument(html: string, viewportWidth: number): Promise<DisplayList> {
  return invoke<DisplayList>("render_document", {
    html,
    viewportWidth,
  });
}
