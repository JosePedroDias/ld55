# merge or die, a ludum dare 55 entry

## basic mechanics

- [x] 2D matrix of M * N sprites
- [x] there's a progression of sprites, let's say they're powers of 2, 2 > 4 > ... > 2048
- [ ] the theme is a bit dark
- [ ] we need audio sfx for
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
- scoring:
    - [ ] time spent to finish the game - number of matches done
