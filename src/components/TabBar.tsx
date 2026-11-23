import type { KeyboardEvent } from "react";
import type { BrowserTab } from "../types/browser";

interface TabBarProps {
  tabs: BrowserTab[];
  activeTabId: string;
  onSelect: (id: string) => void;
  onClose: (id: string) => void;
  onNewTab: () => void;
}

function CloseIcon() {
  return (
    <svg viewBox="0 0 16 16" width="10" height="10" aria-hidden="true">
      <path
        d="M4 4l8 8M12 4l-8 8"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinecap="round"
      />
    </svg>
  );
}

function PlusIcon() {
  return (
    <svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
      <path
        d="M12 5v14M5 12h14"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.7"
        strokeLinecap="round"
      />
    </svg>
  );
}

export function TabBar({
  tabs,
  activeTabId,
  onSelect,
  onClose,
  onNewTab,
}: TabBarProps) {
  const handleTabKeyDown = (event: KeyboardEvent<HTMLDivElement>, id: string) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onSelect(id);
    }
  };

  return (
    <div className="tab-bar" role="tablist" aria-label="Browser tabs">
      {tabs.map((tab) => {
        const active = tab.id === activeTabId;
        const label = tab.title || tab.url;
        return (
          <div
            key={tab.id}
            className={`tab${active ? " active" : ""}`}
            role="tab"
            tabIndex={active ? 0 : -1}
            aria-selected={active}
            title={label}
            onClick={() => onSelect(tab.id)}
            onKeyDown={(event) => handleTabKeyDown(event, tab.id)}
          >
            <span className="tab-title">{label}</span>
            <button
              type="button"
              className="tab-close"
              aria-label={`Close tab: ${label}`}
              title="Close tab (Ctrl+W)"
              onClick={(event) => {
                event.stopPropagation();
                onClose(tab.id);
              }}
            >
              <CloseIcon />
            </button>
          </div>
        );
      })}
      <button
        type="button"
        className="new-tab-button"
        aria-label="New tab"
        title="New tab (Ctrl+T)"
        onClick={onNewTab}
      >
        <PlusIcon />
      </button>
    </div>
  );
}
