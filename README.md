# Circle LED

This is a simple game to be played on a LED string presented as a circle.
It takes 2-6 players and runs like this:

- Start: each player chooses a color
- Collect: for n rounds, players get points by either
  - Gather: being close to each other
  - Disperse: being farthest from each other
- Run: in the endgame the players need to avoid obstacles using the points gathered

Currently only the `Run` is implemented!

The game is played on mobile phones with a simple interface:

- at the beginning each player can choose one of the remaining colors
- `Collect`
  - each player has a circle on the phone where they can point where their avatar should be
  - the avatars are shown on the LED strip
  - after some points, the best get a handicap which dissociates the LED circle from their
   mobile phone circle, so it gets more difficult to play
  - `Gather`: the closer players are, the more often they get points
  - `Disperse`: the farther away from the other players, the more points
- `Run`
  - the avatars start equally spaced on the circle with their colors
  - obstacles as white points run around the circle
  - the mobile device shows 'jump' and 'long jump'
    - 'jump' is short and difficult to time to get right
    - 'long jump' is safer, but costs a point
  - if the player gets caught in an obstacle, they move in the direction of the
   obstacle
  - if two players meet, then one with more points wins.
   If they have equal points, is a random choice
- Winner is last player left

# Run it locally

Devbox should make this easy, but unfortunately this doesn't work yet:

```bash
devbox shell -- dx serve
```

# TODO

- Make timing independant of LED_COUNT

# DONE

- Make moving around twice as fast as the blobs
- Avoid hiding of users
- Add bonus points to get more life
- Don't jump too often
- Jump makes user blink
- Show colors in start
