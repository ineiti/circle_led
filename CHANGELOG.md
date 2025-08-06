Following https://keepachangelog.com/en/1.1.0/ and using
- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

## Current Task

Add more games:
- Drop - send balls falling down on either side of ball
- Scenery - what to show nicely on the LEDs?
- Clock - show time
- Algorithms - allow to write simple algorithms to light up the leds
  - use x/y as arguments, each going from 1 (top, left) to -1 (bottom, right)
  - use alpha as argument, going from 0 (right) to 2*PI (right), counter-clockwise
    - perhaps it should start at the top and go clockwise?
  - use t as argument, as seconds since the start
  - use a parameter on the page to change the references of the parameters
  - allow to give different parameters for R, G, B, going from -1 (0) to 1 (255)
  - allow to give different parameters for H, S, V, going from -1 (0) to 1 (255)
  - make a drag-and-drop interface of formulas
  - sin, cos, square, root
  - modulo of result, or min/max

Questions:
- How to choose the game?

## Unreleased

### Changed

- Put games in their own subdirectory
- UDP sends binary LED colors instead of hex encoded
- 288 LEDs for the new wood-circle
- Added choice in the beginning for different games
- Going back from Snake::Join
- Idling with snake
- Snake: Changing obstacle frequencies in a more linear way with announcements
- Snake: change the player interface
