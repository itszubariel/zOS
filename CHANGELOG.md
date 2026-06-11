# Changelog

## [0.1.2-alpha] - 2026-06-11
### Added
- Implemented `zfetch` command to display system information.
- Added ASCII art for `zOS` in `zfetch`.
- Added CPU identification via `CPUID` instruction.
- Enhanced memory usage reporting.

## [0.1.1-alpha] - 2026-06-11
### Added
- Full-screen scrollable terminal (80x25 characters).
- Color preservation in terminal history buffer.
- Dynamic theme switching (red/blue) with `theme` command.
- Manual scrolling via Page Up and Page Down keys.
- New source directory organization (`drivers/`, `kernel/`, `ui/`).

### Fixed
- Thinner terminal outline using single-line characters.
- Newline handling in terminal write operations.
- UI contrast improvements (yellow welcome text).
- Broken module imports and project build warnings.

## [0.1.0-alpha] - 2026-06-02
### Added
- Initial kernel structure.
- VGA text mode driver.
- Keyboard interrupt support (IDT/PIC).
- Basic command processor.
- Terminal-like UI with prompt and input buffer.
- Support for Shift/Caps Lock.
- Project documentation (README, LICENSE, CHANGELOG).
