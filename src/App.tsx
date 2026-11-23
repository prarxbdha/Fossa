import { BrowserToolbar } from "./components/BrowserToolbar";
import { BrowserViewport } from "./components/BrowserViewport";
import { TabBar } from "./components/TabBar";
import { useBrowser } from "./hooks/useBrowser";

function App() {
  const browser = useBrowser();

  return (
    <main className="browser">
      <TabBar
        tabs={browser.tabs}
        activeTabId={browser.activeTabId}
        onSelect={browser.activateTab}
        onClose={browser.closeTab}
        onNewTab={browser.createTab}
      />
      <BrowserToolbar
        canGoBack={browser.canGoBack}
        canGoForward={browser.canGoForward}
        onBack={browser.back}
        onForward={browser.forward}
        onReload={browser.reload}
        url={browser.url}
        onNavigate={browser.navigate}
        addressInputRef={browser.addressInputRef}
      />
      <BrowserViewport
        content={browser.content}
        displayList={browser.displayList}
        error={browser.error}
        loading={browser.loading}
        onViewportResize={browser.setViewportWidth}
      />
    </main>
  );
}

export default App;
