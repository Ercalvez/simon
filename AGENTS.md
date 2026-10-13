# AGENTS.md

Notes for coding agents working on this repository.

## Project

A Simon memory game in Rust using **Bevy 0.10** and `rand` 0.8. It has one
binary crate and no library target. `Cargo.lock` is gitignored.

## Expected behaviour

[docs/GAME_DESIGN.md](docs/GAME_DESIGN.md) is the source of truth for how
the game should behave: rules, turns, score, game over, settings and
persistence. Read it before you change gameplay. If the user decides
something new about the game's behaviour, update that file, including its
"Open points" section. Some decisions there differ from the current code:

- The board size is the `TILE_COUNT` constant (4). It is not configurable
  yet.
- Only the high score is saved. Settings are not.

## Commands

```sh
cargo build      # the first build compiles all of Bevy, which takes a while
cargo run        # opens a window, so it needs a display
cargo test       # tests/color_test.rs exists but is empty
cargo clippy
```

`cargo build` should finish with no errors. Right now it gives one
warning: `utils::Icon` is never used. Don't add new warnings.

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
- `src/game/mod.rs` defines the `GamePlugin` and the `TurnState`:
  `Idle` (default, outside a game), `Cpu` and `Player`.
  - `start_game` (`OnEnter(GameState::Game)`) resets `Score` and `Sequence`
    and switches to `Cpu`.
  - `end_game` (`OnExit(GameState::Game)`) switches back to `Idle`.
  - All game-screen entities are tagged with `OnGameScreen` and despawned on
    exit.
- `src/game/sequence/` holds the `Sequence` resource, an ordered `Vec` of
  tile indices plus the player's position in it, and a `Playback` resource
  (index plus a 1 s repeating timer). It has these systems:
  - `start_cpu_turn` adds a random tile.
  - `play_sequence` lights one tile per tick, then switches to `Player`.
  - `check_player_input` reads `TileClicked` events. On a mistake it
    switches to `GameState::Menu`. When the sequence is complete, it
    increments `Score` and switches to `Cpu`.
- `src/game/tile/` has the `Tile` component (index, colour, `clicked` flag,
  a one-shot 0.8 s timer), `TILE_COUNT`, and the `TileClicked` event.
  - `interact_tile` only runs during `TurnState::Player`, which is how input
    is blocked while the CPU plays.
  - `light_tile` flashes a tile white, and `reset_tile_color` restores its
    colour.
  - Tiles are UI `ButtonBundle`s positioned absolutely in percent on a
    circle, as computed by `resize_tile`.
- `src/game/score/` has the `Score` resource and the score text
  (`ScoreDisplay`) with a gear icon button. `update_score` refreshes the
  text when `Score` changes.
- `src/utils/` has `despawn_screen` and `build_icons(asset_server, name)`.
  The latter returns a 50×50 `ButtonBundle` using
  `assets/icons/PNG/White/1x/<name>.png`.
- Assets load by path relative to `assets/`. Text uses
  `fonts/poppins/Poppins-Light.ttf`.

## Known gaps and bugs

Keep these in mind when you change code nearby. Fix them only when asked.

- A mistake goes straight back to the main menu. The game-over screen from
  the spec doesn't exist yet.
- The high score isn't saved, and there are no tile tones.
- The menu's Settings button and the gear icon do nothing.
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
  "Not done yet") and the "Known gaps" list above. If the intended
  behaviour changes, update `docs/GAME_DESIGN.md` too.
