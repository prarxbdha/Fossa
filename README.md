
# Fossa

A minimalist, open-source desktop browser built around
customization, simplicity, and an efficient browsing experience.

Fossa is an experimental browser built with its own rendering
engine in Rust. The project explores how browsers work from
the ground up, from parsing HTML and resolving CSS to computing
layouts and rendering webpages.

## Philosophy

Modern browsers are incredibly powerful, but often come with
unnecessary complexity. Fossa explores a different approach:

- Minimal interface, maximum usability.
- Customization without unnecessary clutter.
- Performance-conscious engineering.
- Transparency through open-source development.
- Building and understanding browser internals from first principles.

## Architecture

Fossa is built around three core layers:

| Layer | Technology | Responsibility |
| --- | --- | --- |
| Interface | React, TypeScript | Browser UI, tabs and navigation |
| Desktop Bridge | Tauri | Native application and IPC |
| Browser Engine | Rust | Parsing, styling, layout and painting |

Unlike browsers built around existing rendering engines,
Fossa aims to develop its own independent rendering pipeline.

Tauri's WebView hosts the application interface, while webpage
rendering is handled by Fossa's custom Rust engine and
canvas-based renderer.

### Rendering Pipeline

```text
HTML Document
     |
     v
HTML Tokenizer
     |
     v
DOM Construction
     |
     v
CSS Processing
     |
     v
Layout Engine
     |
     v
Paint Instructions
     |
     v
Canvas Renderer
```

The rendering pipeline is under active development.

## Project Structure

```text
Fossa/
├── src/                    # React and TypeScript frontend
├── src-tauri/              # Native desktop bridge
├── crates/
│   └── fossa-engine/       # Custom Rust browser engine
├── fixtures/               # HTML and CSS test pages
├── docs/                   # Architecture and development roadmap
├── AGENTS.md               # Coding agent guidelines
├── Cargo.toml              # Rust workspace configuration
└── package.json            # Frontend configuration
```

## Technology Stack

- Rust
- TypeScript
- React
- Tauri
- Vite
- CSS
- HTML Canvas

## Getting Started

### Prerequisites

- Rust and Cargo
- Node.js and npm
- Git
- Tauri system dependencies

Refer to the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
for platform-specific installation instructions.

### Installation

```bash
git clone https://github.com/prarxbdha/Fossa.git
cd Fossa
npm install
```

### Running Fossa

Launch the desktop application in development mode:

```bash
npm run tauri dev
```

For frontend-only development:

```bash
npm run dev
```

### Building

```bash
npm run tauri build
```

### Testing

Run the Rust workspace tests:

```bash
cargo test --workspace
```

Validate the frontend:

```bash
npm run build
```

## Development Roadmap

Fossa is being developed incrementally, with each milestone
building upon the existing browser engine.

- [x] Desktop application foundation
- [x] Minimalist browser interface
- [x] Frontend tab management
- [x] Navigation history
- [x] Custom new-tab page
- [x] Basic HTML parsing and DOM construction
- [x] Basic layout engine
- [x] Rust-to-canvas rendering pipeline
- [ ] CSS styling and cascade
- [ ] CSS box model
- [ ] Background and border rendering
- [ ] Page scrolling
- [ ] HTTP/HTTPS networking
- [ ] Interactive links and hit testing
- [ ] External resource loading
- [ ] Image rendering
- [ ] JavaScript runtime
- [ ] Browser storage
- [ ] Advanced customization
- [ ] Web standards compatibility

## Documentation

- [Architecture](docs/architecture.md)
- [Development Milestones](docs/milestones.md)
- [Agent Guidelines](AGENTS.md)

## Contributing

Fossa welcomes contributions from developers interested in
browser engines, systems programming, frontend development,
and user experience.

Contributions should preserve the custom rendering architecture,
remain focused, and include appropriate tests.

Before making significant architectural changes, open an issue
to discuss the proposed implementation.

## Project Status

Fossa is currently an experimental browser under active
development. It is not yet intended to replace a production
web browser.

The long-term goal is to build a capable, customizable browser
while understanding every layer that makes the modern web work.

---

Built from first principles.
