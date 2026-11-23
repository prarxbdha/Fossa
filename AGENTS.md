# AGENTS.md — Fossa Development Instructions

> Repository-level instructions for coding agents working on Fossa.
> **Read this file at the beginning of every task and re-check the applicable rules before each change.** If an agent does not automatically load this file, explicitly ask it to read `AGENTS.md` before working. Instructions in this file do not replace direct user instructions; flag conflicts before proceeding.

## 1. Project identity and goals

Fossa is an experimental desktop web browser and a learning-oriented browser-engine project. Its existing stack is **React + TypeScript + Vite** for the interface, **Tauri + Rust** for desktop integration, and a **custom Rust browser engine**. The project is intended to evolve incrementally, with clear boundaries between browser chrome, application state, and webpage rendering.

Priorities, in order:
1. Preserve working functionality and existing architecture.
2. Make small, understandable, testable changes.
3. Favor correct behavior and explicit limitations over fake functionality.
4. Maintain a minimalist, polished browser interface.
5. Keep the code approachable for someone learning how browsers work.

## 2. Mandatory workflow before EVERY change

1. **Read `AGENTS.md`** at task start; revisit the relevant sections before editing each file or coherent group of files.
2. Inspect the target file, its callers, types, tests, and related interfaces. Do not infer behavior from filenames alone.
3. Check `git status --short` and respect pre-existing user changes. Never overwrite or discard unrelated work.
4. State the intended outcome and the smallest set of files to change. For broad, destructive, or architectural changes, propose a plan and obtain approval first.
5. Implement one coherent change at a time. Preserve existing contracts unless a change is necessary and coordinated.
6. Run relevant validation (see §9), inspect the diff, and fix regressions caused by the change.
7. Report what changed, which checks actually ran, their results, and known limitations. Do not claim unrun tests passed.

If context is missing, inspect the repository or ask the user instead of guessing.

## 3. Existing repository layout — reuse it

```text
Fossa/
├── src/
│   ├── App.tsx
│   ├── components/
│   │   ├── AddressBar.tsx
│   │   ├── BrowserToolbar.tsx
│   │   ├── BrowserViewport.tsx
│   │   ├── NavigationButtons.tsx
│   │   └── TabBar.tsx
│   ├── hooks/useBrowser.ts
│   ├── renderer/CanvasRenderer.ts
│   ├── renderer/HitTester.ts
│   ├── services/browserApi.ts
│   ├── styles/browser.css
│   ├── styles/global.css
│   └── types/
├── src-tauri/
│   ├── src/commands.rs
│   ├── src/state.rs
│   └── tauri.conf.json
├── crates/fossa-engine/src/
│   ├── html/
│   ├── css/
│   ├── dom/
│   ├── layout/
│   ├── paint/
│   └── network/
├── docs/
├── fixtures/
├── Cargo.toml
├── package.json
└── vite.config.ts
```

This is a map of known locations, **not a claim that every module is complete**. Inspect actual code before relying on a feature.

- `src/components/`: visual browser chrome and viewport components.
- `src/hooks/`: React browser state and UI behavior.
- `src/services/`: frontend-to-Tauri API boundary.
- `src/renderer/`: canvas drawing and hit testing.
- `src/types/`: shared frontend contracts.
- `src-tauri/`: native shell, commands, and state.
- `crates/fossa-engine/`: custom browser-engine implementation.
- `fixtures/` and existing tests: preferred sources for regression cases.

**Do not** initialize another frontend, introduce Svelte, create a parallel `frontend/` directory, or move the Rust engine without explicit approval. Add files only when there is a clear responsibility not covered by existing files.

## 4. Architecture boundaries

- Keep browser chrome (tabs, toolbar, address bar, new-tab UI) separate from webpage content and rendering.
- Keep stateful browser behavior in hooks/state modules, not scattered across presentation components.
- Use `browserApi.ts` or an existing service abstraction for Tauri calls; do not sprinkle raw native invocations throughout UI components.
- Maintain typed contracts across TypeScript and Rust. When changing a command or response, update all affected consumers and tests together.
- Preserve the custom rendering pipeline. **Never silently replace it with an iframe, embedded Chromium, or a system WebView.** A rendering-strategy change requires explicit discussion and approval.
- Do not assume the HTML, CSS, DOM, layout, paint, network, canvas, or hit-testing modules support features until verified.
- Clearly distinguish implemented functionality, UI placeholders, and future features. Do not create controls that appear functional but do nothing.

## 5. Frontend design principles

Target: **minimalist, compact, premium desktop-browser UI**.

- Default to a restrained dark theme with neutral graphite/charcoal surfaces.
- Prefer consistent spacing, readable typography, subtle borders, and restrained transitions.
- Avoid decorative gradients, glassmorphism, glows, oversized shadows, and unnecessary UI chrome.
- Use design tokens/CSS variables instead of repeating colors and dimensions.
- Preserve existing React + TypeScript conventions and component boundaries.
- Use recognizable, accessible SVG icons or existing icon assets rather than ambiguous Unicode symbols.
- Make active, hover, disabled, loading, error, and keyboard-focus states distinguishable.
- Provide accessible names for icon-only buttons and preserve keyboard usability.
- Support narrower desktop windows without clipped controls or unwanted horizontal scrolling.
- Do not add fake metrics, fake security indicators, fake bookmarks, or nonfunctional menus.

Suggested palette (adjust only deliberately and consistently):

```css
--fossa-chrome: #17191d;
--fossa-surface: #202329;
--fossa-interactive: #30343b;
--fossa-text: #e7e9ed;
--fossa-muted: #a0a6b0;
--fossa-border: #343840;
```

## 6. Navigation, tabs, and async correctness

- Preserve working back, forward, reload, and address-bar behavior.
- When adding tabs, give each tab independent URL, content/render state, navigation history, loading state, and errors.
- Do not let stale async responses overwrite newer navigation or update closed tabs.
- Avoid race conditions when switching tabs, closing tabs, and navigating concurrently.
- Handle malformed URLs, empty input, network failures, and backend errors gracefully.
- Normalize search queries and URLs through a shared, testable path; do not duplicate inconsistent parsing logic.
- Never report a webpage as loaded until the relevant operation has succeeded.
- Do not assume a global Rust backend state provides per-tab isolation; inspect and address that explicitly.

## 7. Rust engine development rules

- Keep parsing, DOM, styling, layout, painting, and networking concerns separate.
- Prefer incremental vertical slices backed by fixtures and tests.
- Avoid claiming standards compliance or broad webpage support without evidence.
- Keep failures explicit and recoverable; avoid panics for ordinary malformed web content.
- Do not add an entire browser feature or major dependency as a side effect of a UI task.
- Treat fetched web content as untrusted input. Do not execute arbitrary content or weaken Tauri permissions to make a demo work.

## 8. Code and dependency hygiene

- Follow existing formatting, naming, and linting conventions.
- Use strict TypeScript types; avoid `any` and unsafe assertions where practical.
- Prefer small, focused functions and components over clever abstractions.
- Avoid duplicated state, dead code, unnecessary wrappers, and premature optimization.
- Do not install packages, change dependency versions, or rewrite lockfiles without a concrete need; explain additions first.
- Never hardcode tokens, passwords, API keys, or personal credentials. Never log or commit secrets.
- Do not edit generated outputs (`dist/`, `target/`, `node_modules/`, generated Tauri schemas) by hand.
- Do not modify unrelated files for cosmetic cleanup during a focused task.

## 9. Validation checklist

First inspect `package.json`, `Cargo.toml`, and existing scripts; **do not assume commands exist**. Run the applicable checks, such as:

```bash
npm run build
cargo test --workspace
cargo fmt --all -- --check
```

Use project-provided lint/typecheck/test scripts when available. For UI changes, also run or recommend a manual smoke test with:

```bash
npm run tauri dev
```

Check the affected workflows: startup, tab switching (if implemented), address entry, navigation, reload, viewport sizing, and error display. If a check cannot run, say why. Do not modify unrelated code solely to make an unrelated check pass.

## 10. Git and safety

- Inspect `git status` before editing and `git diff` afterward.
- Never run destructive commands such as `git reset --hard`, `git clean -fd`, forced checkout, or history rewrites without explicit approval.
- Do not delete or overwrite user work.
- Do not push, open pull requests, or create commits unless the user requests it.
- If a task is high-risk, recommend a checkpoint commit or branch, but do not create one automatically.
- Keep changes scoped so they can be reviewed and reverted easily.

## 11. Agent communication and completion format

Before implementing a substantial change, briefly explain:
- What currently exists.
- What will change and why.
- Which files are expected to be affected.
- Any risk to existing behavior.

After completing the task, report:
1. **Changes made** — concrete files and behavior.
2. **Validation** — exact commands/tests run and outcomes.
3. **Limitations** — what is still unsupported or unverified.
4. **Next step** — one sensible follow-up, not an unsolicited implementation.

When a request is ambiguous, ask one targeted question. Do not invent implementation details, silently expand scope, or automatically continue into the next milestone.

## 12. Final rule

**Fossa is an existing project, not a blank template. Inspect first, preserve what works, change only what is needed, test what changed, and explain the result.**
