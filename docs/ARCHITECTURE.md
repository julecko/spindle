# Architecture

Spindle is a personal CD/MP3 collection cataloguer, in Slovak, built as a
desktop-style app with egui/eframe. This doc records decisions already made
so future work (human or agent) doesn't re-litigate them. If you deviate
from something here, update this file in the same change.

## Crate layout: single crate, not a workspace

One binary (`spindle`), organized into modules, not a Cargo workspace.
Nothing outside this app consumes the data models or sync logic as an
independent library today, so a workspace (separate `-core` lib + bin crate,
path deps, per-crate versioning) would add ceremony with no payoff at this
size.

Revisit only if a second consumer shows up in this repo (e.g. a CLI, or the
sync server itself living alongside the client) — at that point split
`models/` + `state/` + `data/` into a `spindle-core` lib crate and keep
`app.rs`/`screens/`/`widgets/`/`theme.rs` in the bin crate.

## Module map

```
src/
├── main.rs   entry point, eframe::run_native setup
├── app.rs    root App: nav tabs, screen dispatch, floating Add button, modal/detail overlay
├── theme.rs  palette + egui::Visuals, tag-chip colors
├── models/   Cd, Mp3Track, CdType/Mp3Format enums, shared LibraryItem trait, ViewMode/SortKey/DurationBucket
├── state/    AppState (screen, selection, modal), Settings, per-library UI state, search filters
├── screens/  one file per screen: cd_library, mp3_library, search, detail, settings
├── widgets/  card_view, row_view, table_view, add_edit_modal, band_autocomplete, floating add button, disc-icon logo
└── data/     store (local persistence), csv_io (Slovak-header Excel round-trip), sync (server fetch)
```

`screens/*` take `&mut AppState` and render into whatever panel `app.rs`
gives them; they don't own their own state beyond what's in `AppState`
(egui is immediate-mode — there's no separate widget tree to keep in sync,
so all persistent state lives in `state/`, not in the screen/widget
functions themselves).

## Two libraries, one trait

CD and MP3 are separate libraries with separate schemas, but both expose the
same view-mode/sort UI (cards/rows/table; sortable by title/band/duration/
type). Rather than duplicating that UI per library, `Cd` and `Mp3Track` both
implement `models::common::LibraryItem`, and `widgets::{card_view,row_view,
table_view}` are generic over it. Keep new library-agnostic UI generic over
`LibraryItem` rather than writing CD/MP3 variants side by side.

`Mp3Track::format` (MP3/FLAC/WAV) is deliberately not a field in the add/edit
form — brief says it's "kept internally for tag coloring" only.

## Visual design: bright palette, not Nocturne

Palette (cream background, orange accent, teal/yellow secondary tags) is
deliberately built off the Nocturne design system's structure but inverted
to bright/warm, because the brief calls for "bright and fresh" for a casual
home user, not Nocturne's dark UI. Accent color is user-configurable via
Settings (`Settings::accent_color`) and re-applied through `theme::apply()`
whenever it changes — don't hardcode the orange outside of the default.

No brand wordmark anywhere in the UI — logo is a painted disc icon only
(`widgets/disc_icon.rs`, drawn via `egui::Painter`, not an image asset), per
the brief.

## Persistence, CSV, and sync are three separate concerns

- `data::store` — local JSON persistence of the library and settings, so the
  app has state between runs independent of any server.
- `data::csv_io` — Import z Excelu / Export do Excelu. Plain CSV using the
  *exact* Slovak column headers from the brief (`skupina`, `názov albumu`,
  `číslo` / `číslo CD`, `typ CD`, `dĺžka`), since Excel round-trips CSV
  natively — no `.xlsx` dependency needed. `Mp3Format` is not a CSV column
  (see above); it's assigned a default on import.
- `data::sync` — fetches from a server to sync the collection. Runs on a
  background `std::thread` + `mpsc::channel`, polled with `try_recv()` from
  the frame loop, because egui is immediate-mode on the UI thread and a
  blocking network call there would freeze rendering. Currently a stub with
  no real endpoint — wire in the actual API shape when it exists rather than
  guessing at one now.

## Dependencies anticipated (not yet added)

`serde` + `serde_json` (persistence/settings), `csv` (import/export),
`rfd` (native file-picker dialogs for import/export), `egui_extras`
(`TableBuilder`, for the table view mode), `ureq` (sync fetch, blocking +
threaded rather than async — no async runtime elsewhere in the app, so
pulling in tokio/reqwest just for this would be disproportionate).

## egui/eframe version notes (pinned to 0.35.0)

APIs that shifted in recent egui versions and are easy to get wrong if
working from older docs/memory:

- `Rounding` is gone. Corner rounding is `CornerRadius` (`u8` fields,
  `CornerRadius::same(n)` or `From<u8>`); padding/margins are `Margin`
  (`i8` fields, `Margin::same(n)` / `Margin::symmetric(x, y)`). Both take
  integers, not floats.
- `ComboBox::from_id_source` is now `ComboBox::from_id_salt`.
- Popups (e.g. for the band autocomplete dropdown) use the new `Popup`
  builder — `Popup::from_response(&response).open(true).show(|ui| ...)` —
  not the old `egui::popup_below_widget` free function.
- Table view mode uses `egui_extras::TableBuilder` /
  `Column::auto()`/`Column::remainder()` — API confirmed stable at 0.35.0,
  `header()`/`body()`/`row()`/`col()` closures.

Confirm current API against the locally cached crate source under
`~/.cargo/registry/src/*/egui-0.35.0/` (or docs.rs) before relying on
memory — egui's public API moves fast across versions.
