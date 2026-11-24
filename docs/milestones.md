# Milestones

1. **Skeleton** — workspace, Tauri app, React shell, and engine module layout. Complete.
2. **Parsing** — HTML tokenizer/parser producing a DOM tree and CSS parser producing style rules. Complete for the supported subset; attributes are now preserved.
3. **Layout and paint** — box tree construction and display list generation from a styled DOM. Partial: stacked block text layout and text paint work; no box model or scrolling.
4. **Rendering** — canvas painting, hit testing, and text metrics. Partial: canvas rendering and DPR handling work; `HitTester` is not connected.
5. **Browser shell** — tabs, history, navigation, and loading local fixtures. Complete for the frontend; backend tab sessions remain global.
6. **Networking** — HTTP fetching, redirects, and caching. Not implemented; `fetch` is a stub.
7. **Minimal CSS styling** — style extraction, simple selectors, specificity, source order, inheritance, and computed color/font-size through layout and paint. Complete for the documented subset.

Validation for milestone 7:

```bash
cargo test --workspace --locked
npm run tauri dev
```

Open `fossa://styles` and confirm “Styled paragraph” is cyan while the CSS
source is not visible.
