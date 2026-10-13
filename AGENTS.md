# AGENTS.md

Notes for coding agents working on this repository.

## Project

A Simon memory game in Rust using **Bevy 0.10** and `rand` 0.8. It has one
binary crate and no library target. `Cargo.lock` is gitignored.

## Commands

```sh
cargo build      # the first build compiles all of Bevy, which takes a while
cargo run        # opens a window, so it needs a display
cargo test       # tests/color_test.rs exists but is empty
cargo clippy
```

`cargo build` should finish with no errors. Right now it gives 3 warnings:
an unneeded `mut` in `tile/mod.rs`, `ScorePlugin` is never used, and
`utils::Icon` is never used. Don't add new warnings.

## Bevy version: stick to the 0.10 API

The code uses Bevy **0.10** APIs. Don't use newer Bevy syntax or upgrade
Bevy unless the user asks for it.

- Register systems with `add_system(...)`, `add_systems((...))`,
  `.in_schedule(OnEnter(S))`, `.in_schedule(OnExit(S))`,
  `.in_set(OnUpdate(S))` and `add_startup_system`. Don't use
  `add_systems(Update, ...)`.
- Change state with
  `commands.insert_resource(NextState(Some(state)))`. Read the current
  state with `Res<State<S>>` and its `.0` field.
- Size UI with `Style { size: Size::new(..), position: UiRect {..} }`. Use
  `Interaction::Clicked`, `Color::rgb`, `Camera2dBundle`, `UiImage::new` and
  `Text::from_section`.

## Architecture

- `src/main.rs` holds the `GameState` (`Splash` (default) → `Menu` → `Game`)
  and the camera. It registers `SplashPlugin`, `MenuPlugin` and `GamePlugin`.
- Each screen is a plugin. It spawns its UI `OnEnter`, tags the root entity
  with a marker component (`OnSplashScreen`, `OnMenuScreen`), and despawns it
  `OnExit` with `utils::despawn_screen::<Marker>`. Follow this pattern for new
  screens, such as Settings.
- `src/game/mod.rs` defines the `GamePlugin` and the `TurnState`
  (`Cpu` (default) / `Player`). Systems in `OnEnter(TurnState::Cpu)` are
  gated by `run_if_new_game`, which checks that `GameState == Game`.
- `src/game/tile/` has the `Tile` component (index, colour, `clicked` flag,
  0.8 s timer), the `TileSet` resource, and the tile systems. Tiles are UI
  `ButtonBundle`s positioned absolutely in percent on a circle, as computed
  by `resize_tile`. `Tile` equality is by `index` only.
- `src/game/score/` has the `Score` resource, the score text (`ScoreDisplay`)
  with a gear icon button, `next_turn` (adds a random tile to `TileSet` and
  increments the score) and `move_to_next_turn` (<kbd>Space</kbd> toggles the
  `TurnState`).
- `src/utils/` has `despawn_screen` and `build_icons(asset_server, name)`.
  The latter returns a 50×50 `ButtonBundle` using
  `assets/icons/PNG/White/1x/<name>.png`.
- Assets load by path relative to `assets/`. Text uses
  `fonts/poppins/Poppins-Light.ttf`.

## Known gaps and bugs

Keep these in mind when you change code nearby. Fix them only when asked.

- **The turn loop doesn't run in practice.** `move_to_next_turn` is
  registered in `OnEnter(TurnState::Cpu)`, so it only runs during the one
  frame when that state is entered. `TurnState` starts as `Cpu`, so that
  first entry happens at app start, while `run_if_new_game` is still false.
  As a result, `next_turn`, `update_score` and `toggle_cpu_tiles` never run
  during a game, and the score label stays "Score".
- `TileSet` is a `HashSet`, so the CPU "sequence" has no order and can't
  repeat a colour. A real Simon game needs an ordered `Vec`.
- Player clicks are not checked against the sequence, and there is no game
  over.
- `GamePlugin` despawns `OnGameScreen` on `OnExit(GameState::Splash)`
  (wrong state), and nothing is tagged with `OnGameScreen`. Game UI is never
  cleaned up.
- `ScorePlugin` is defined but never added. `GamePlugin` registers the score
  systems directly.
- The menu's Settings button and the gear icon do nothing.
- `toggle_cpu_tiles` still contains debug `println!` calls.
- `assets/shaders/tile_fragment.wgsl` uses outdated `@stage(fragment)`
  syntax, `tile_vertex.wgsl` is empty, and neither file is loaded.
- `tests/color_test.rs` is empty.

## Conventions

- Keep the plugin-per-module layout (`src/<feature>/mod.rs`).
- Write each Bevy system as a free `pub fn` in its feature module, and
  register it in that feature's plugin.
- Don't add or modify files under `assets/icons/` or `assets/fonts/`. They
  are third-party packs (Kenney CC0, Poppins OFL).
- When gameplay changes, update `README.md` ("What works today" /
  "Not done yet") and the "Known gaps" list above.
