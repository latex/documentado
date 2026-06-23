# Changelog

## v0.0.2 (2026-06-23)

### Features
- **SQLite cache**: Local database caches doc items and content after first fetch. Subsequent opens are instant and work offline.
- **Configurable sources**: JSON config file (`config.json`) lets you add, remove, or modify documentation sources. Created automatically on first run.
- **Source catalog**: 48+ curated documentation sets inspired by Dash/Kapeli. Press `a` to browse and add with one keystroke.
- **Explicit download**: No auto-download. Press `d` on a source to download its item list. Press `d` again to refresh.
- **Generic scraper**: Added `generic` source type that scrapes any documentation website's index page for links.
- **Vim plugin update**: Changed mapping from `<leader>d` to `<leader>do` to avoid conflicts with LazyDocker.

### UI
- Visual indicators: `✓` (cached), `↻` (loading), empty (not downloaded)
- Discover overlay: press `a` to browse curated docsets, `Enter` to add
- Status bar credits: "thanks to Kapeli, Dash & GitHub"
- Updated help overlay with all keybindings
- Updated README with full documentation, config guide, credits

### Technical
- Added `rusqlite` (bundled) for SQLite caching
- Added `src/cache.rs`: Cache struct with items/content storage
- Added `src/config.rs`: Config struct with JSON serialization
- Refactored `docs.rs`: `fetch_module_items` now dispatches by source type
- Refactored `app.rs`: Removed auto-download on source navigation
- Updated `Cargo.toml`: edition 2024, rusqlite dependency

### Credits
- Inspired by [Kapeli/Dash](https://kapeli.com/dash) docset ecosystem
- Community docsets: [Dash-User-Contributions](https://github.com/Kapeli/Dash-User-Contributions), [hashhar/dash-contrib-docset-feeds](https://github.com/hashhar/dash-contrib-docset-feeds)

---

## v0.0.1 (2026-06-22)

### Initial release
- TUI documentation browser (ratatui + crossterm)
- Fuzzy search (nucleo)
- Two sources: Rust Standard Library and Neovim User Manual
- Vim integration: `:Documentado` command, `<leader>do` mappings
- CLI argument support: `documentado.exe <term>`
- Three-panel layout: Sources | Items | Documentation
- Keybindings: `Tab`/`Ctrl+w`/`←`/`→` panel switching, `j`/`k` navigation, `/` search, `o` open in vim, `?` help
- Windows terminal support (separate window via `cmd.exe /c start`)
- Project structure: `src/app.rs`, `src/ui.rs`, `src/docs.rs`, `src/main.rs`
