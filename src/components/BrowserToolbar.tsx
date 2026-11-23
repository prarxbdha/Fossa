import type { RefObject } from "react";
import { AddressBar } from "./AddressBar";
import { NavigationButtons } from "./NavigationButtons";

interface BrowserToolbarProps {
  canGoBack: boolean;
  canGoForward: boolean;
  onBack: () => void;
  onForward: () => void;
  onReload: () => void;
  url: string;
  onNavigate: (url: string) => void;
  addressInputRef?: RefObject<HTMLInputElement | null>;
}

export function BrowserToolbar({
  canGoBack,
  canGoForward,
  onBack,
  onForward,
  onReload,
  url,
  onNavigate,
  addressInputRef,
}: BrowserToolbarProps) {
  return (
    <header className="browser-toolbar">
      <NavigationButtons
        canGoBack={canGoBack}
        canGoForward={canGoForward}
        onBack={onBack}
        onForward={onForward}
        onReload={onReload}
      />
      <AddressBar url={url} onNavigate={onNavigate} inputRef={addressInputRef} />
      <div className="toolbar-controls" aria-hidden="true" />
    </header>
  );
}
