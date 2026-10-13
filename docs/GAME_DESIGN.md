# Game design

This is how the finished Simon game should behave. It describes the target.
For what is actually built so far, see [README.md](../README.md).

## Screen flow

```
Splash ──Space──▶ Main menu ──New Game──▶ Game ──mistake──▶ Game over
                   │  ▲                    │ ▲                │
                   │  │                    │ │                ├─Play again──▶ Game
                   │  └────────────────────┼─┼────────────────┴─Main menu───▶ Main menu
                   │                       │ │
                   └─Settings─▶ Settings ◀─┘ │ (gear icon: pause)
                                    └────────┘ (back: resume)
                   Quit ──▶ exit
```

## Board

- The board shows **N tiles** arranged in a circle. Each tile has its own
  colour.
- N can be set in Settings, from **3 to 8**. The default is **4**.
- There is one colour for each possible tile, so 8 distinct colours are
  needed. The code currently defines 6: red, green, blue, yellow, magenta
  and cyan.
- Each tile has its own **tone**. The tone plays whenever the tile lights
  up, whether the CPU plays it or the player clicks it. Tones are muted when
  sound is off.

## Rules (classic Simon)

1. At the start of a game the sequence is empty and the score is 0.
2. **CPU turn**
   - One random tile is added to the end of the sequence.
   - The sequence is ordered, and the same tile can appear more than once.
   - The CPU plays back the **whole** sequence, lighting up one tile at a
     time (with its tone), with a short gap between tiles.
   - Player input is **ignored** while the CPU is playing.
3. **Player turn** starts automatically when the playback ends.
   - The player clicks tiles with the mouse to repeat the sequence in order.
   - Each click lights the tile up and plays its tone.
   - A click on the wrong tile means **immediate game over**.
   - Once the whole sequence has been repeated correctly, the round is won
     and the score goes up by 1.
   - After a short pause, the next CPU turn starts automatically.
4. There is no time limit on player input.

## Score

- Score = **number of rounds completed**, which means the number of
  sequences reproduced in full. If the game ends on round *k*, the score is
  *k − 1*.
- The current score is shown at the top of the game screen.
- The **high score** is a single best score. It is saved to disk and kept
  between sessions.

## Game over

A game-over screen shows:

- the final score and the high score, with a note when a new high score was
  set,
- a **Play again** button that starts a new game with the same settings,
- a **Main menu** button.

## Settings

The Settings screen opens from the main menu or from the gear icon during a
game. It offers:

| Setting         | Values | Default |
|-----------------|--------|---------|
| Number of tiles | 3–8    | 4       |
| Sound           | on/off | on      |

Settings are **not saved**. They reset to their defaults every time the game
is launched.

### Gear icon during a game

- The gear icon **pauses** the game and opens Settings.
- Going back from Settings **resumes** the paused game.
- If the number of tiles was changed, the current game **restarts** with the
  new board instead of resuming.

## Open points

These details haven't been decided yet. Current defaults are in brackets.

- The two extra tile colours needed for 7–8 tiles.
- The tone frequency for each tile.
- Timings: how long a tile stays lit [0.8 s, the current value], the gap
  between tiles during playback, and the pause between rounds.
- Whether there should be a separate high score for each board size. For
  now, there is a single global best score, whatever the number of tiles.
- Where the high score file is stored. The user's data directory seems the
  obvious choice.
