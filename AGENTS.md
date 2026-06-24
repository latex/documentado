# Critical Context

## Build
- MSVC env vars required: `PATH` must include `...\MSVC\14.51.36231\bin\Hostx64\x64`; `LIB` and `INCLUDE` must point to MSVC + Windows SDK 10.0.26100.0 paths.
- `cargo build` (debug) works; `cargo build --release` blocked by Windows App Control policy on this machine.
- Debug binary can be copied as release (`Copy-Item target\debug\documentado.exe target\release\documentado.exe`).

## Repo
- `github.com/latex/documentado`
- `master` is stable/release branch; `develop` is default.
- Tags: `v0.0.1`, `v0.0.2`
- Current Cargo.toml: `0.0.3`
- PR from develop → master, squashed. After merge, develop was recreated from master.

## Release v0.0.2
- Release created at: https://github.com/latex/documentado/releases/tag/v0.0.2
- Binary: `documentado-v0.0.2-windows-x64.zip` (debug build, works same as release)

## TODO: Publish to winget
- Must publish `latex.documentado` to Windows Package Manager (winget).
- Steps:
  1. Fork `https://github.com/microsoft/winget-pkgs`
  2. Create manifest at `manifests/l/latex/documentado/0.0.2/`
  3. Submit PR
- Reference: https://github.com/microsoft/winget-pkgs

## Key Technical Details
- `nucleo 0.5` `fuzzy_match` signature: `fn fuzzy_match(&mut self, haystack: Utf32Str<'_>, pattern: Utf32Str<'_>) -> Option<u16>`
- `ratatui 0.29` uses `Wrap { trim: false }`, not `WrapType`
- Cache at `%LOCALAPPDATA%\com.documentado\documentado\cache\cache.db`
- Config at `%APPDATA%\com.documentado\documentado\config\config.json`
