# Simon Game

A Simon memory game written in Rust with [Bevy](https://bevyengine.org/) 0.10.

> **Status: work in progress.** The screens and the tile board are in place,
> but the Simon game loop (play a sequence, check the player's input, game
> over) is not finished yet.

## Running

Requires a recent stable Rust toolchain.

```sh
cargo run
```

On Linux, Bevy also needs the usual system libraries (ALSA and udev
development packages, plus a Vulkan-capable driver). See Bevy's
[Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

## What works today

1. **Splash screen.** Shows the logo and "Press SPACE to start". Press
   <kbd>Space</kbd> to go to the menu.
2. **Main menu.** Has three buttons:
   - **New Game** starts the game screen.
   - **Settings** is a placeholder that does nothing yet.
   - **Quit** closes the window.
3. **Game screen.**
   - Six coloured tiles (red, green, blue, yellow, magenta and cyan) are laid
     out in a circle.
   - Clicking a tile flashes it white for 0.8 s, then it returns to its
     colour.
   - A score label and a gear (settings) icon sit at the top. The gear does
     nothing yet.

## Target gameplay

The full spec is in [docs/GAME_DESIGN.md](docs/GAME_DESIGN.md). In short:

- Classic Simon. Each round adds one random tile to the sequence, and the
  CPU plays the whole sequence back. Then the player repeats it with the
  mouse.
- Turns switch automatically. Player input is ignored while the CPU is
  playing.
- One wrong click ends the game. A game-over screen shows the score and the
  high score, with **Play again** and **Main menu** buttons.
- Score = rounds completed. The high score is saved between sessions.
- Settings:
  - number of tiles, from 3 to 8 (default 4),
  - sound on/off (each tile has its own tone).

  The gear icon pauses the game and opens Settings.

## Not done yet

- Making the CPU play a growing sequence and switching between the CPU's
  turn and the player's turn (partly written, see `src/game/`).
- Checking the player's clicks against the sequence, and a game-over state.
- Keeping the score up to date.
- The Settings screen.
- Custom tile shaders (`assets/shaders/` is a stub).
- Tests (`tests/color_test.rs` is empty).

## Project layout

```
src/
  main.rs          App setup, camera, and the top-level GameState (Splash → Menu → Game)
  splash/          Splash screen
  menu/            Main menu
  game/            Game plugin and the TurnState (Cpu / Player)
    tile/          Tile spawning, click handling, colour reset
    score/         Score resource, score display, CPU sequence step
  utils/           Screen despawning and the icon-button helper
assets/
  fonts/poppins/   Poppins font (OFL)
  icons/           Kenney game icons (see assets/icons/license.txt)
  images/          Splash branding image
  shaders/         WGSL tile shaders (not used yet)
```

## Credits

- Font: [Poppins](https://fonts.google.com/specimen/Poppins), SIL Open Font
  License (`assets/fonts/poppins/OFL.txt`).
- Icons: [Kenney](https://www.kenney.nl) game icon pack, CC0
  (`assets/icons/license.txt`).
