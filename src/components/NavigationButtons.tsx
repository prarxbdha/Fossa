interface NavigationButtonsProps {
  canGoBack: boolean;
  canGoForward: boolean;
  onBack: () => void;
  onForward: () => void;
  onReload: () => void;
}

function BackIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M19 12H5" />
      <path d="m11 18-6-6 6-6" />
    </svg>
  );
}

function ForwardIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M5 12h14" />
      <path d="m13 6 6 6-6 6" />
    </svg>
  );
}

function ReloadIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M20 11a8 8 0 1 0-2.34 5.66" />
      <path d="M20 5v6h-6" />
    </svg>
  );
}

export function NavigationButtons({
  canGoBack,
  canGoForward,
  onBack,
  onForward,
  onReload,
}: NavigationButtonsProps) {
  return (
    <nav className="navigation-buttons" aria-label="Navigation">
      <button
        type="button"
        className="icon-button"
        onClick={onBack}
        disabled={!canGoBack}
        aria-label="Back"
        title="Back (Alt+Left)"
      >
        <BackIcon />
      </button>
      <button
        type="button"
        className="icon-button"
        onClick={onForward}
        disabled={!canGoForward}
        aria-label="Forward"
        title="Forward (Alt+Right)"
      >
        <ForwardIcon />
      </button>
      <button
        type="button"
        className="icon-button"
        onClick={onReload}
        aria-label="Reload"
        title="Reload (Ctrl+R)"
      >
        <ReloadIcon />
      </button>
    </nav>
  );
}
