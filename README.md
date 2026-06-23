# documentado

> A TUI documentation browser for Rust — search, navigate, and open docs right from your terminal.

![demo](https://img.shields.io/badge/status-alpha-yellow)
![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange)
![Platform](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macos-lightgrey)

Inspired by [Dash](https://kapeli.com/dash) / [Zeal](https://zealdocs.org/) with a [lazygit](https://github.com/jesseduffield/lazygit)-style TUI. Built with [ratatui](https://github.com/ratatui/ratatui).

## Features

- **Three-panel layout** — Sources | Items | Documentation for fast browsing
- **Fuzzy search** — powered by [nucleo](https://github.com/helix-editor/nucleo)
- **Async fetching** — loads documentation from `doc.rust-lang.org` without freezing the UI
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

# Search for something right away
documentado Vec
documentado "Option"
```

### Keybindings

| Key | Action |
|------|--------|
| `h` / `l` | Move focus between panels |
| `j` / `k` | Navigate up / down |
| `Enter` | Open item / load docs |
| `/` or `Ctrl+f` | Start search |
| `Esc` | Exit search / clear filter |
| `o` | Open doc in editor |
| `?` | Toggle help overlay |
| `q` | Quit |

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
| `<leader>d` (normal) | Search word under cursor |
| `<leader>d` (visual) | Search selected text |

## Configuration

Set the path to the binary if it's not in your `PATH`:

```vim
let g:documentado_bin = 'C:\tools\documentado.exe'
```

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

## License

MIT
