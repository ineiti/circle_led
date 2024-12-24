# Circle LED

This is a simple game to be played on a LED string presented as a circle.
It takes 2-6 players and runs like this:

- Start: each player chooses a color
- Collect: for n rounds, players get points by either
  - Gather: being close to each other
  - Disperse: being farthest from each other
- Run: in the endgame the players need to avoid obstacles using the points gathered

Currently only the endgame `Run` is implemented!

The game is played on mobile phones with a simple interface:

- at the beginning each player can choose one of the remaining colors
- `Collect` (not implemented)
  - each player has a circle on the phone where they can point where their avatar should be
  - the avatars are shown on the LED strip
  - after some points, the best get a handicap which dissociates the LED circle from their
   mobile phone circle, so it gets more difficult to play
  - `Gather`: the closer players are, the more often they get points
  - `Disperse`: the farther away from the other players, the more points
- `Run`
  - the device shows a circle of white dots with a big circle of the player's color
    - when a player clicks on their big circle, the point retracts and blinks to indicate it's jumping
      - after a jump, there is a recover phase, where the player cannot jump again
      - during a jump, no obstacles or boni interact with the player
    - a player can move around the circle using the interface, but only with a limited speed
  - the avatars start equally spaced on the circle with their colors
  - each player is shown on the circle, with a width corresponding to their lives left
  - obstacles as white points go around the circle
    - if an obstacle collides with a player who is not jumping
      - the obstacle disappears
      - the player loses one life - if they have 0 lives left, the player is removed
  - boni as green points go around the circle
    - if a boni collides with a player who is not jumping
      - the boni disappears
      - the player gains a life
  - the display goes through different phases of fuzzing the colors
  - Winner is last player left

# Run it locally

Devbox should make this easy, but unfortunately this doesn't work yet:

```bash
devbox shell -- dx serve
```

# TODO

- Make timing independant of LED_COUNT

# DONE

- Countdown also shows in users
- Nicer display of the LEDs with correct calculation (I know trigonometry better than Claude...)
- Make moving around thrice as fast as the blobs
- Avoid hiding of users
- Add bonus points to get more life
- Don't jump too often
- Jump makes user blink
- Show colors in start
