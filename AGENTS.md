# AGENTS.md

Notes for coding agents working on this repository.

## Project

A Simon memory game in Rust using **Bevy 0.20** and `rand` 0.8. It has one
binary crate and no library target. `Cargo.lock` is gitignored.

Bevy 0.20 needs **Rust 1.97.1 or newer**.

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

`cargo build` and `cargo clippy` both finish with no warnings. Keep it
that way.

## Bevy 0.20 API

The code targets Bevy **0.20**. Bevy changes a lot between versions, so
don't copy snippets written for older Bevy. When in doubt, check the 0.20
source in `~/.cargo/registry/src/*/bevy_*-0.20.0/`.

- **Systems:** register them with `add_systems(Update, ...)`,
  `add_systems(OnEnter(S), ...)` and `.run_if(in_state(S))`.
- **State changes:** use `ResMut<NextState<S>>` with `.set(...)`. Read the
  current state with `Res<State<S>>` and `.get()`.
- **Components, not bundles:** spawn tuples of components such as
  `Node { width, height, left, top, .. }`, `BackgroundColor`,
  `ImageNode::new`, `Text::new` + `TextFont` + `TextColor`, and `Camera2d`.
  Colours use `Color::srgb`.
- **Clicks:** use picking observers,
  `.observe(|click: On<PointerClick>, ...| ...)` with `click.entity`.
  Don't use `Interaction`, which is deprecated in 0.20. `Button` comes from
  `bevy::ui_widgets::Button`.
- **Messages:** what older Bevy called events are now messages. Use
  `#[derive(Message)]`, `app.add_message::<M>()`, `MessageWriter::write`
  and `MessageReader::read`.
- **Cleanup:** tag screen entities with `DespawnOnExit(GameState::X)`.
  `despawn()` is recursive.

## Architecture

- `src/main.rs` holds the `GameState` (`Splash` (default) → `Menu` → `Game`)
  and the camera. It registers `SplashPlugin`, `MenuPlugin` and `GamePlugin`.
- Each screen is a plugin. It spawns its UI `OnEnter` and tags the root
  entities with `DespawnOnExit(GameState::X)`, so they are removed
  automatically. Follow this pattern for new screens, such as Settings.
- `src/game/mod.rs` defines the `GamePlugin` and the `TurnState`, a
  **sub-state** of `GameState::Game` with `Cpu` (default) and `Player`.
  - `TurnState` only exists during a game, and every new game starts at
    `Cpu`.
  - `start_game` (`OnEnter(GameState::Game)`) resets `Score` and `Sequence`.
    It runs before `OnEnter(TurnState::Cpu)`.
  - `end_game` (`OnExit(GameState::Game)`) logs the final score.
- `src/game/sequence/` holds the `Sequence` resource, an ordered `Vec` of
  tile indices plus the player's position in it, and a `Playback` resource
  (index plus a 1 s repeating timer). It has these systems:
  - `start_cpu_turn` adds a random tile.
  - `play_sequence` lights one tile per tick, then switches to `Player`.
  - `check_player_input` reads `TileClicked` messages. On a mistake it
    switches to `GameState::Menu`. When the sequence is complete, it
    increments `Score` and switches to `Cpu`.
- `src/game/tile/` has the `Tile` component (index, colour, `clicked` flag,
  a one-shot 0.8 s timer), `TILE_COUNT`, and the `TileClicked` message.
  - `interact_tile` is a `PointerClick` observer on each tile. It ignores
    clicks unless `TurnState` is `Player`, which is how input is blocked
    while the CPU plays.
  - `light_tile` flashes a tile white, and `reset_tile_color` restores its
    colour.
  - Tiles are UI `Button` nodes positioned absolutely in percent on a
    circle, as computed by `resize_tile`.
- `src/game/score/` has the `Score` resource and the score text
  (`ScoreDisplay`) with a gear icon button. `update_score` refreshes the
  text when `Score` changes.
- `src/utils/` has two helpers:
  - `text(asset_server, value)` returns the shared white 80px Poppins text
    bundle.
  - `build_icons(asset_server, name)` returns a 50×50 icon button using
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
