import { useEffect, useRef } from "react";
import { CanvasRenderer } from "../renderer/CanvasRenderer";
import type { DisplayList } from "../types/display";

interface BrowserViewportProps {
  content: string;
  displayList?: DisplayList | null;
  error?: string | null;
  loading?: boolean;
  onViewportResize?: (width: number) => void;
}

export function BrowserViewport({
  content,
  displayList,
  error,
  loading,
  onViewportResize,
}: BrowserViewportProps) {
  const containerRef = useRef<HTMLElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const lastWidthRef = useRef(0);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    const canvas = canvasRef.current;
    const renderer = canvas ? new CanvasRenderer(canvas) : null;

    const update = () => {
      const bounds = container.getBoundingClientRect();
      if (renderer && displayList) {
        renderer.render(displayList, bounds.width, bounds.height);
      }
      const width = Math.round(bounds.width);
      if (width > 0 && width !== lastWidthRef.current) {
        lastWidthRef.current = width;
        onViewportResize?.(width);
      }
    };

    update();
    const observer = new ResizeObserver(update);
    observer.observe(container);
    return () => observer.disconnect();
  }, [displayList, onViewportResize]);

  return (
    <section
      className="browser-viewport"
      aria-label="Page content"
      ref={containerRef}
    >
      {displayList ? (
        <canvas className="viewport-canvas" ref={canvasRef} aria-hidden="true" />
      ) : (
        <div className="viewport-surface">
          <pre className={`viewport-content${error ? " error" : ""}`}>{content}</pre>
        </div>
      )}
      {loading ? (
        <div className="viewport-loading" role="status">
          Loading…
        </div>
      ) : null}
    </section>
  );
}
