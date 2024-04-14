# merge or die, a ludum dare 55 entry

## prioritized todo list

- [x] multiple levels (10 min)
    - level 1: dims: 5x5, penalty: 75s, goal: 5
    - level 2: dims: 7x7, penalty: 60s, goal: 6
    - level 3: dims: 8x8, penalty: 45s, goal: 7
    - level 4: dims: 9x9, penalty: 30s, goal: 8
- [x] introduce screen waiting for click to start the level (5 min)
    merge until you get a GOAL
    be careful with The Bar
    click to start
- [x] short instructions outside the canvas? (5 min)
- [ ] keep the high scores on local storage? (20 min)
- [x] RESTART button (location.reload())
- [ ] visuals (30 min)
    - re-export sprites with better resolution (20 min)
    - generate cover image
    - ~background monster making appearance?~
    - ~replace default font maybe?~
- [ ] review text, add cover image (30 min)
- [ ] audio (30-60 min)
    - sfx first selected sprite?
    - ~start level tune~
    - ~end of level tune~
- [ ] ~an unfocused tab/window should pause the game (30 min)~

## basic mechanics

- [x] 2D matrix of M * N sprites
- [x] there's a progression of sprites, let's say they're powers of 2, 1 > 2 > 4 > ... > 2048
- [ ] the theme is a bit dark
- [x] we need audio sfx for
    - applying penalty
    - mistake
    - fill
    - second mark for 5, 4, 3, 2, 1
    - fulfilling sfx 1...7
- [ ] music
    - start game tune
    - game complete
- [ ] dramatic effect to apply for a couple seconds once penalty is applied
- goal:
    - [x] merge from 1 to 7
- penalties:
    - [x] timer running down to 0. when it reach 0, some of the board tiles get added back
    - [x] there may also be a meter adding mistakes (penalty countdown decreases by 10s)
- score:
    - [ ] time spent to finish the game shoud be enough? (we also have number of mistakes and matches)
