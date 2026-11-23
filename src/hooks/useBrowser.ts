import { useCallback, useEffect, useRef, useState } from "react";
import * as browserApi from "../services/browserApi";
import type { BrowserTab } from "../types/browser";

const START_URL = "fossa://start";
const START_TITLE = "Fossa";
const DEFAULT_VIEWPORT_WIDTH = 1024;

const START_CONTENT = `<!doctype html>
<html>
  <head><title>Fossa</title></head>
  <body>
    <h1>Fossa</h1>
    <p>A quiet place to start.</p>
    <p>Enter an address above to begin browsing.</p>
    <p>Try fossa://basic, fossa://typography, fossa://styles or fossa://links.</p>
  </body>
</html>`;

interface BrowserState {
  tabs: BrowserTab[];
  activeTabId: string;
}

interface LoadOptions {
  historyMode?: "append" | "index" | "none";
  historyIndex?: number;
}

function makeId(): string {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }
  return `tab-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

function createTabState(): BrowserTab {
  return {
    id: makeId(),
    url: START_URL,
    title: START_TITLE,
    content: START_CONTENT,
    displayList: null,
    history: { entries: [START_URL], index: 0 },
    loading: false,
    error: null,
  };
}

function deriveTitle(html: string, url: string): string {
  const match = /<title[^>]*>([\s\S]*?)<\/title>/i.exec(html);
  if (match) {
    const title = match[1].replace(/\s+/g, " ").trim();
    if (title) return title;
  }
  return url;
}

export function useBrowser() {
  const [state, setState] = useState<BrowserState>(() => {
    const tab = createTabState();
    return { tabs: [tab], activeTabId: tab.id };
  });

  const stateRef = useRef(state);
  stateRef.current = state;

  const activeTab = state.tabs.find((tab) => tab.id === state.activeTabId) ?? state.tabs[0];
  const renderTabId = activeTab?.id;
  const renderContent = activeTab?.content;
  const renderError = activeTab?.error;

  const [viewportWidth, setViewportWidth] = useState(DEFAULT_VIEWPORT_WIDTH);
  const mountedRef = useRef(true);
  const requestTokens = useRef(new Map<string, number>());
  const renderToken = useRef(0);
  const addressInputRef = useRef<HTMLInputElement>(null);
  const backendSync = useRef<Promise<void>>(Promise.resolve());

  // The Rust side owns one global current_url, not one per tab. Serializing
  // writes keeps that shared mirror in step with the most recent active tab.
  const syncBackendUrl = useCallback((url: string) => {
    backendSync.current = backendSync.current
      .catch(() => undefined)
      .then(() => browserApi.setCurrentUrl(url))
      .catch(() => undefined);
  }, []);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      requestTokens.current.clear();
    };
  }, []);

  const patchTab = useCallback((tabId: string, patch: Partial<BrowserTab>) => {
    setState((prev) => ({
      ...prev,
      tabs: prev.tabs.map((tab) => (tab.id === tabId ? { ...tab, ...patch } : tab)),
    }));
  }, []);

  const isCurrent = useCallback(
    (tabId: string, token: number) =>
      mountedRef.current &&
      requestTokens.current.get(tabId) === token &&
      stateRef.current.tabs.some((tab) => tab.id === tabId),
    [],
  );

  const resolveContent = useCallback(async (target: string): Promise<string> => {
    if (target === START_URL) return START_CONTENT;
    return browserApi.fetchDocument(target);
  }, []);

  const load = useCallback(
    async (tabId: string, target: string, options: LoadOptions = {}) => {
      const token = (requestTokens.current.get(tabId) ?? 0) + 1;
      requestTokens.current.set(tabId, token);
      patchTab(tabId, { loading: true, error: null });

      try {
        const body = await resolveContent(target);
        if (!isCurrent(tabId, token)) return;

        // The backend keeps a single global URL, so mirror it only for the
        // active tab. Inactive tabs explicitly do not own backend state.
        if (stateRef.current.activeTabId === tabId) {
          syncBackendUrl(target);
        }

        setState((prev) => ({
          ...prev,
          tabs: prev.tabs.map((tab) => {
            if (tab.id !== tabId) return tab;

            let history = tab.history;
            if (options.historyMode === "append") {
              const entries = [
                ...tab.history.entries.slice(0, tab.history.index + 1),
                target,
              ];
              history = { entries, index: entries.length - 1 };
            } else if (
              options.historyMode === "index" &&
              typeof options.historyIndex === "number"
            ) {
              history = { ...tab.history, index: options.historyIndex };
            }

            return {
              ...tab,
              url: target,
              title: deriveTitle(body, target),
              content: body,
              // Keep the previous display list until the render effect supplies
              // the new one; a reload has identical content and needs no redraw.
              history,
              loading: false,
              error: null,
            };
          }),
        }));
      } catch (error) {
        if (!isCurrent(tabId, token)) return;
        const message = error instanceof Error ? error.message : String(error);
        patchTab(tabId, {
          loading: false,
          error: message,
          content: message,
          displayList: null,
        });
      }
    },
    [isCurrent, patchTab, resolveContent, syncBackendUrl],
  );

  const navigate = useCallback(
    async (target: string) => {
      const trimmed = target.trim();
      if (!trimmed) return;
      await load(stateRef.current.activeTabId, trimmed, { historyMode: "append" });
    },
    [load],
  );

  const back = useCallback(async () => {
    const tab = stateRef.current.tabs.find(
      (candidate) => candidate.id === stateRef.current.activeTabId,
    );
    if (!tab || tab.history.index <= 0) return;
    const index = tab.history.index - 1;
    await load(tab.id, tab.history.entries[index], {
      historyMode: "index",
      historyIndex: index,
    });
  }, [load]);

  const forward = useCallback(async () => {
    const tab = stateRef.current.tabs.find(
      (candidate) => candidate.id === stateRef.current.activeTabId,
    );
    if (!tab || tab.history.index >= tab.history.entries.length - 1) return;
    const index = tab.history.index + 1;
    await load(tab.id, tab.history.entries[index], {
      historyMode: "index",
      historyIndex: index,
    });
  }, [load]);

  const reload = useCallback(async () => {
    const tab = stateRef.current.tabs.find(
      (candidate) => candidate.id === stateRef.current.activeTabId,
    );
    if (!tab) return;

    await load(tab.id, tab.url, { historyMode: "none" });
  }, [load]);

  const createTab = useCallback(() => {
    const tab = createTabState();
    setState((prev) => ({ tabs: [...prev.tabs, tab], activeTabId: tab.id }));
    syncBackendUrl(tab.url);
    return tab.id;
  }, [syncBackendUrl]);

  const closeTab = useCallback(
    (tabId: string) => {
      const { tabs, activeTabId } = stateRef.current;
      if (tabs.length === 0) return;

      // Invalidates any in-flight request for this tab.
      requestTokens.current.set(tabId, (requestTokens.current.get(tabId) ?? 0) + 1);

      if (tabs.length === 1) {
        const fresh = createTabState();
        setState({ tabs: [fresh], activeTabId: fresh.id });
        syncBackendUrl(fresh.url);
        return;
      }

      const index = tabs.findIndex((tab) => tab.id === tabId);
      if (index === -1) return;

      const remaining = tabs.filter((tab) => tab.id !== tabId);
      let nextActiveId = activeTabId;
      if (activeTabId === tabId) {
        nextActiveId = remaining[Math.min(index, remaining.length - 1)].id;
      }

      setState({ tabs: remaining, activeTabId: nextActiveId });
      if (nextActiveId !== activeTabId) {
        const next = remaining.find((tab) => tab.id === nextActiveId);
        if (next) syncBackendUrl(next.url);
      }
    },
    [syncBackendUrl],
  );

  const activateTab = useCallback((tabId: string) => {
    if (stateRef.current.activeTabId === tabId) return;
    const tab = stateRef.current.tabs.find((candidate) => candidate.id === tabId);
    if (!tab) return;
    setState((prev) => ({ ...prev, activeTabId: tabId }));
    syncBackendUrl(tab.url);
  }, [syncBackendUrl]);

  const selectNext = useCallback(() => {
    const { tabs, activeTabId } = stateRef.current;
    if (tabs.length < 2) return;
    const index = tabs.findIndex((tab) => tab.id === activeTabId);
    activateTab(tabs[(index + 1) % tabs.length].id);
  }, [activateTab]);

  const selectPrevious = useCallback(() => {
    const { tabs, activeTabId } = stateRef.current;
    if (tabs.length < 2) return;
    const index = tabs.findIndex((tab) => tab.id === activeTabId);
    activateTab(tabs[(index - 1 + tabs.length) % tabs.length].id);
  }, [activateTab]);

  const focusAddressBar = useCallback(() => {
    addressInputRef.current?.focus();
    addressInputRef.current?.select();
  }, []);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.ctrlKey && !event.altKey && !event.metaKey) {
        const key = event.key.toLowerCase();
        if (key === "t") {
          event.preventDefault();
          createTab();
        } else if (key === "w") {
          event.preventDefault();
          closeTab(stateRef.current.activeTabId);
        } else if (key === "tab") {
          event.preventDefault();
          if (event.shiftKey) selectPrevious();
          else selectNext();
        } else if (key === "l") {
          event.preventDefault();
          focusAddressBar();
        } else if (key === "r") {
          event.preventDefault();
          void reload();
        }
        return;
      }

      if (event.altKey && !event.ctrlKey && !event.metaKey) {
        if (event.key === "ArrowLeft") {
          event.preventDefault();
          void back();
        } else if (event.key === "ArrowRight") {
          event.preventDefault();
          void forward();
        }
      }
    };

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [back, closeTab, createTab, focusAddressBar, forward, reload, selectNext, selectPrevious]);

  // One render path for the active tab. A monotonically increasing token makes
  // stale results (from an older navigation, tab, or viewport width) a no-op.
  useEffect(() => {
    // An error tab shows styled raw text; it must not be fed back to the engine.
    // Bump the token so a render already in flight cannot repaint the error.
    if (renderError) {
      renderToken.current += 1;
      return;
    }
    if (!renderTabId || renderContent === undefined) return;
    const token = ++renderToken.current;
    const tabId = renderTabId;

    browserApi
      .renderDocument(renderContent, viewportWidth)
      .then((displayList) => {
        if (!mountedRef.current || token !== renderToken.current) return;
        if (!stateRef.current.tabs.some((tab) => tab.id === tabId)) return;
        patchTab(tabId, { displayList });
      })
      .catch(() => {
        if (!mountedRef.current || token !== renderToken.current) return;
        if (!stateRef.current.tabs.some((tab) => tab.id === tabId)) return;
        patchTab(tabId, { displayList: null });
      });
  }, [patchTab, renderContent, renderError, renderTabId, viewportWidth]);

  return {
    tabs: state.tabs,
    activeTabId: state.activeTabId,
    activeTab,
    url: activeTab?.url ?? "",
    content: activeTab?.content ?? "",
    displayList: activeTab?.displayList ?? null,
    loading: activeTab?.loading ?? false,
    error: activeTab?.error ?? null,
    addressInputRef,
    canGoBack: (activeTab?.history.index ?? 0) > 0,
    canGoForward:
      (activeTab?.history.index ?? 0) < (activeTab?.history.entries.length ?? 0) - 1,
    navigate,
    back,
    forward,
    reload,
    createTab,
    closeTab,
    activateTab,
    selectNext,
    selectPrevious,
    focusAddressBar,
    setViewportWidth,
  };
}
