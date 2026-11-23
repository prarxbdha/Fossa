# Architecture

Fossa has three layers:

- `src/` is the React frontend: toolbar, viewport, browser state hook, Tauri API wrappers, types, and styles.
- `src-tauri/` is the desktop bridge: Tauri commands and shared browser state. It calls into the engine crate.
- `crates/fossa-engine/` is a pure Rust engine with network, HTML, DOM, CSS, layout, and paint stages. Each stage is independent of Tauri.

## Rendering pipeline

For each navigation the frontend calls `browserApi.fetchDocument(url)`, then
`browserApi.renderDocument(html, viewportWidth)`. The Tauri `render_document`
command runs the engine stages in order:

1. `html::tokenizer` → tokens (tags, text, comments, doctypes)
2. `html::parser` → `dom::node::Node` tree
3. `css::parser` → `css::style::StyleSheet` (parsed, but not yet applied)
4. `layout::engine::layout` → a stacked `layout::box_tree::LayoutBox` tree, including greedy word wrapping to the measured viewport width
5. `paint::painter::paint` → `paint::display_list::DisplayList`

The display list is serialized by `src-tauri/src/commands.rs` into the typed
contract in `src/types/display.ts`. `CanvasRenderer` paints it on a DPI-aware
canvas. `BrowserViewport` observes container resizes and reports the width back
to `useBrowser`, which re-renders the document at the new width. A monotonically
increasing render token prevents a stale render (from an older navigation, tab,
or width) from replacing newer content.

`fixtures/` contains small pages used by engine tests, and
`crates/fossa-engine/tests/` holds the integration tests. `fossa://<name>`
URLs are resolved by the `read_fixture` Tauri command.

## Currently supported

- Built-in `fossa://` fixture pages and the Fossa start page.
- Block flow with `h1`–`h3` font sizes and text from `p`, `strong`, `em`, `nav`, and `a`.
- Greedy word wrapping using an approximate glyph width, with a 24px content inset.
- Typed Rust → TypeScript display-list contract; rect and text items; DPI-aware canvas drawing clipped to the viewport.
- Resize-driven re-layout, per-tab loading/error state, and stale-render protection.
- Raw document text fallback when no display list is available.

## Explicitly unsupported

- **Network**: `network::fetch::fetch` is a stub; `http://` and `https://` URLs fail with a visible error. Only bundled fixtures render today.
- **HTML**: the parser discards attributes; there are no images, forms, tables, lists, inline layout, or `<br>` handling. Long unbroken words are clipped rather than split.
- **CSS**: stylesheets are parsed but never matched or applied. Colors, fonts, margins, padding, width, and `class`/`id` selectors have no effect.
- **JavaScript**: no scripting engine, event handlers, or DOM mutation.
- **Interaction**: `HitTester` exists but is not wired to pointer events; links are not clickable and there is no text selection.
- **Scrolling**: the canvas is sized to the viewport, so content taller than the viewport is clipped.
- **Backend tab isolation**: Tauri keeps one global `current_url`; tabs mirror only the active URL and do not own independent backend sessions.

## Next milestone

Apply a minimal stylesheet cascade (type and class selectors for `color` and
`font-size`) from `<style>` blocks into layout/paint, then connect `HitTester`
to link targets emitted as display items.
