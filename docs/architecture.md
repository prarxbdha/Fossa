# Architecture

Fossa has three layers:

- `src/` is the React frontend: toolbar, viewport, browser state hook, Tauri API wrappers, types, and styles.
- `src-tauri/` is the desktop bridge: Tauri commands and shared browser state. It calls into the engine crate.
- `crates/fossa-engine/` is a pure Rust engine with network, HTML, DOM, CSS, layout, and paint stages. Each stage is independent of Tauri.

## Rendering pipeline

For each navigation the frontend calls `browserApi.fetchDocument(url)`, then
`browserApi.renderDocument(html, viewportWidth)`. The Tauri `render_document`
command is thin: it calls `fossa_engine::render::render_document`, converts the
resulting display list into the typed DTO, and returns it over IPC.

The engine entry point runs these stages in order:

1. `html::tokenizer` → tags, attributes, text, and raw `<style>`/`<script>` text.
2. `html::parser` → `dom::node::Node` tree with preserved element attributes.
3. `css::styles_from_dom` → `<style>` contents in document order, parsed by
   `css::parser` into `css::style::StyleSheet`.
4. `css::selector` → simple selector parsing and matching.
5. `css::cascade` → computed `color` and `font-size` with specificity, source
   order, and parent-to-child inheritance in `css::cascade::StyledNode`.
6. `layout::engine::layout` → a stacked `layout::box_tree::LayoutBox` tree using
   resolved font sizes and greedy word wrapping to the measured viewport width.
7. `paint::painter::paint` → `paint::display_list::DisplayList` text items with
   resolved RGBA colors and font sizes.

The display list is serialized by `src-tauri/src/commands.rs` into the typed
contract in `src/types/display.ts`. `CanvasRenderer` paints it on a DPI-aware
canvas. `BrowserViewport` observes container resizes and reports the width back
to `useBrowser`, which re-renders the document at the new width. A monotonically
increasing render token prevents a stale render (from an older navigation, tab,
or width) from replacing newer content.

`fixtures/` contains small pages used by engine tests, and
`crates/fossa-engine/tests/` holds the integration tests. `fossa://<name>`
URLs are resolved by the `read_fixture` Tauri command.

## Supported CSS subset

Selectors:

- `*` (universal)
- `type` (ASCII case-insensitive tag name)
- `.class` (complete whitespace-separated class token, case-sensitive)
- `#id` (exact id value, case-sensitive)

Compound selectors, selector lists, combinators, pseudo-classes,
pseudo-elements, and attribute selectors are rejected rather than matched.

Properties:

- `color`: `#RGB`, `#RRGGBB`, and `black`, `white`, `red`, `green`, `blue`,
  `gray`, `transparent`.
- `font-size`: finite non-negative pixel values such as `16px` or `18.5px`,
  plus `0` and `0px`. Values above `10_000px` are rejected.

Property names are ASCII case-insensitive. CSS block comments and missing
trailing declaration semicolons are accepted. Invalid declarations and
unsupported selectors are skipped without affecting unrelated rules.

Cascade and inheritance:

- Specificity is `(id, class, type/universal)`: ID > class > type > universal.
- For equal specificity, the later declaration in stylesheet source order wins.
- `color` and `font-size` cascade independently, so a class color rule does not
  discard a type font-size rule.
- Both properties inherit from the parent element to its descendants and text
  nodes.
- Built-in heading defaults (`h1` 24px, `h2` 19px, `h3` 16px) apply before
  author declarations and can be overridden by them.

## Supported HTML attribute syntax

Start tags preserve double-quoted, single-quoted, unquoted, and boolean
attributes; whitespace around `=`; hyphenated lowercase names; self-closing
syntax; and basic named/numeric entity decoding in values. Tag and attribute
names are lowercased; values keep their original case. The first occurrence of
a duplicate attribute wins. `<style>` and `<script>` contents are emitted as
raw text so CSS characters such as `<` or `>` do not create spurious tags.

## Currently supported

- Built-in `fossa://` fixture pages and the Fossa start page.
- Block flow with h1–h3 defaults and text from `p`, `strong`, `em`, `nav`, and `a`.
- Greedy word wrapping using an approximate glyph width, with a 24px content inset.
- Styled text from `<style>` blocks reaching layout, paint, the DTO, and canvas.
- Typed Rust → TypeScript display-list contract; rect and text items; DPI-aware canvas drawing clipped to the viewport.
- Resize-driven re-layout, per-tab loading/error state, and stale-render protection.
- Raw document text fallback when no display list is available.

## Validating the styling fixture

Run:

```bash
cargo test --workspace --locked
npm run tauri dev
```

Then open `fossa://styles`. The paragraph “Styled paragraph” is painted with
RGBA `[36, 200, 219, 255]` from `.accent { color: #24c8db; }`. The CSS source
itself is not painted. The deterministic engine assertion is
`render_tests::styles_fixture_produces_cyan_text`.

## Explicitly unsupported

- **Network**: `network::fetch::fetch` is a stub; `http://` and `https://` URLs fail with a visible error.
- **HTML**: no forms, tables, lists, images, `<br>` handling, complete HTML5 error recovery, or document-root normalization. Long unbroken words are clipped rather than split.
- **CSS**: no inline `style` attributes, external stylesheets, full selector grammar, cascade layers, `!important`, CSS variables, `em`/`rem`/percentages, box model, margins, padding, borders, backgrounds, or flex/grid positioning.
- **JavaScript**: no scripting engine, event handlers, or DOM mutation.
- **Interaction**: `HitTester` exists but is not wired to pointer events; links are not clickable and there is no text selection.
- **Scrolling**: the canvas is sized to the viewport, so content taller than the viewport is clipped.
- **Layout debt**: element heights are still advanced twice during child layout, producing oversized vertical gaps. That pre-existing defect did not block styling and remains for a layout milestone.
- **Backend tab isolation**: Tauri keeps one global `current_url`; tabs mirror only the active URL and do not own independent backend sessions.

## Next milestone

Extract link targets from preserved `href` attributes into display items and
connect `HitTester` to pointer events, then address network fetching and
layout height/scrolling as separate incremental slices.
