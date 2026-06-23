# documentado

> A TUI documentation browser inspired by Dash — search, browse, and cache docs from your terminal.

![demo](https://img.shields.io/badge/status-alpha-yellow)
![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange)
![Platform](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macos-lightgrey)

Inspired by [Dash](https://kapeli.com/dash) / [Zeal](https://zealdocs.org/) with a [lazygit](https://github.com/jesseduffield/lazygit)-style TUI. Built with [ratatui](https://github.com/ratatui/ratatui).

## Features

- **Three-panel layout** — Sources | Items | Documentation for fast browsing
- **Fuzzy search** — powered by [nucleo](https://github.com/helix-editor/nucleo)
- **Offline cache** — SQLite-backed local cache; first fetch is saved, subsequent opens are instant
- **Configurable sources** — JSON config file; add any documentation website
- **Source catalog** — 48+ curated docsets (Python, React, Go, Docker, etc.) inspired by Dash
- **Vim integration** — `o` opens the current doc in your `$EDITOR`; Vim plugin included
- **CLI-driven** — pass a search term as argument to jump straight to results

## Installation

### From source

```bash
cargo install --git https://github.com/latex/documentado
```

### Pre-built binaries

Download the latest release from [Releases](https://github.com/latex/documentado/releases).

## Usage

```bash
# Open the browser
documentado

# Download a source: navigate with j/k, press d
# Browse items: press Enter, select with j/k, Enter again to view
# Add more sources: press a to open the catalog

# Search for something right away
documentado Vec
documentado "HashMap"
```

### Keybindings

| Key | Action |
|------|--------|
| `h` / `l` / `Tab` | Move focus between panels |
| `j` / `k` | Navigate up / down |
| `Enter` | Open item / load docs / switch to items |
| `d` | Download / refresh selected source |
| `a` | Add source from curated catalog |
| `/` or `Ctrl+f` | Start search |
| `Esc` | Exit search / go back |
| `o` | Open doc in editor |
| `g` / `G` | Go to top / bottom |
| `?` | Toggle help overlay |
| `q` / `Ctrl+c` | Quit |

### Source indicators

| Indicator | Meaning |
|-----------|---------|
| _(empty)_ | Not downloaded yet |
| `↻` | Downloading... |
| `✓` | Cached locally |

## Cache

Documentation is cached locally in SQLite at:
- **Windows:** `%LOCALAPPDATA%\com.documentado\documentado\cache\cache.db`
- **Linux:** `~/.cache/com.documentado/documentado/cache.db`
- **macOS:** `~/Library/Caches/com.documentado/documentado/cache.db`

Press `d` again on a cached source to refresh.

## Configuration

Sources are defined in a JSON config file at:
- **Windows:** `%APPDATA%\com.documentado\documentado\config\config.json`
- **Linux:** `~/.config/com.documentado/documentado/config.json`
- **macOS:** `~/Library/Application Support/com.documentado/documentado/config.json`

```json
{
  "sources": [
    {
      "name": "Rust Standard Library",
      "url": "https://doc.rust-lang.org/stable/std",
      "type": "rust-std"
    },
    {
      "name": "My Internal Docs",
      "url": "https://docs.mycompany.com",
      "type": "generic"
    }
  ]
}
```

### Source types

| Type | Description |
|------|-------------|
| `rust-std` | Rust Standard Library (custom CSS selector parsing) |
| `neovim` | Neovim User Manual (custom `.help-li` parsing) |
| `generic` | Any documentation site (scrapes all index page links) |

## Vim Integration

> Requires `documentado` to be on your `PATH`.

### Plugin installation

Copy the `plugin/` and `autoload/` directories into your Vim config:

```bash
# Vim
cp -r plugin autoload ~/vimfiles/

# Neovim
cp -r plugin autoload ~/.config/nvim/
```

Or use a plugin manager (after pushing to a repo):

```vim
" vim-plug
Plug 'latex/documentado', { 'rtp': '.' }
```

### Commands

| Command / Mapping | Description |
|-------------------|-------------|
| `:Documentado` | Open browser |
| `:Documentado HashMap` | Search for "HashMap" |
| `<leader>do` (normal) | Search word under cursor |
| `<leader>do` (visual) | Search selected text |

## Build from source

```bash
git clone https://github.com/latex/documentado
cd documentado
cargo build --release
```

**Requirements:** Rust 1.85+, a linker (MSVC on Windows).

On Windows with MSVC:
```powershell
# Use Visual Studio Build Tools or Visual Studio Developer PowerShell
cargo build --release
```

## Credits

- **[Kapeli / Dash](https://kapeli.com/dash)** — Inspiration and original docset ecosystem
- **[Kapeli/feeds](https://github.com/Kapeli/feeds)** — Official Dash docset feeds
- **[Kapeli/Dash-User-Contributions](https://github.com/Kapeli/Dash-User-Contributions)** — Community-contributed docsets
- **[hashhar/dash-contrib-docset-feeds](https://github.com/hashhar/dash-contrib-docset-feeds)** — Zeal-compatible XML feeds
- **[zealdocs/zeal](https://zealdocs.org/)** — Linux/Windows Dash alternative
- **[ratatui](https://github.com/ratatui/ratatui)** — TUI framework
- **[nucleo](https://github.com/helix-editor/nucleo)** — Fuzzy matching
- All documentation authors and open-source maintainers

## License

MIT
