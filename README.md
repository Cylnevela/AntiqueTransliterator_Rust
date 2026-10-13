# Linear B Keyboard

A minimal native desktop prototype written in Rust with `egui/eframe`. It composes Unicode Linear B syllabograms, undeciphered signs, ideograms, inscription separators, and Aegean numbers. The UI is available in French and English and works offline after compilation.

## Features

- Initial writing-system selection screen.
- Four categories: main syllables, special syllables, undetermined signs, ideograms.
- Separate popup for Aegean numbers.
- Each key shows a localized name, the sign, and its conventional transliteration.
- `??` means no established phonetic value; `?` marks a conventional or uncertain special value.
- Editable one-line Unicode result, mouse/keyboard cursor movement, selection, copy/paste, backspace, clear, and insertion at the cursor.
- Live concatenated transliteration.
- Direct transliteration input such as `ko`, `a`, or `dwe`.
- Light and dark themes.
- Data separated in `assets/linear_b.json` to ease future writing-system plugins.

## Requirements

1. Install current stable Rust with rustup: <https://rustup.rs/>
2. Windows 10/11 is the primary target. Linux/macOS should also work, subject to graphics-driver and font availability.

## Font setup

On Windows, the application automatically tries `C:\Windows\Fonts\seguihis.ttf` (Segoe UI Historic). For reproducible cross-platform rendering, run:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\download-font.ps1
```

This downloads **Noto Sans Linear B** into `assets/`. The font is not committed in this prototype so its OFL notice and source can remain explicit. Once downloaded, it is loaded by the application and may be packaged beside the executable.

## Build and run

```powershell
cargo run --release
```

The portable executable is generated at:

```text
target\release\linear-b-keyboard.exe
```

Keep the `assets` folder beside the executable if using the downloaded Noto font. On standard Windows 10/11 systems, Segoe UI Historic is used as fallback.

## Project structure

```text
src/main.rs                 Application and UI
assets/linear_b.json        Writing-system data
scripts/download-font.ps1   Optional Noto font downloader
Cargo.toml                  Rust dependencies
```

## Data notes

The Unicode names and code points are the storage basis. Main signs have conventional transliterations. Special values are marked `?`; undeciphered signs and ideograms display `??`. This is intentionally conservative and the dataset should receive specialist review before scholarly publication.

## Adding another writing system later

The first refactoring should introduce a `WritingSystem` descriptor containing metadata, category definitions, number sets, and a list of signs. The current JSON already isolates sign data from UI code, so future scripts can be added without rewriting the keyboard widgets. A fully dynamic plugin loader was deliberately excluded from this minimal prototype.

## Clipboard note

The copied result is Unicode text. The destination application also needs a font that supports Linear B to display it correctly.

## Tests

No automated tests are included in this prototype, by request.
