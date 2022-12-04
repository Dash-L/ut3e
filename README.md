# $UT^3E$ (Ultimate Tic-Tac-Toe Explorer)

**Currently very WIP!**

## About
Ultimate Tic-Tac-Toe is a variant of Tic-Tac-Toe where there is a larger grid made up of 9 regular Tic-Tac-Toe grids, and the opponent's previous move determines which of the subgrids the player is allowed to play in.
(See some of the other projects referenced below for a better description)
Tic-Tac-Toe is a solved game, that is, each player has a well defined optimal move for each turn, and optimal games end in ties.
Ultimate Tic-Tac-Toe is far more complicated, and this project's goal is to aid in developing theory for it.

## Running
This project is written in rust using the eframe GUI crate, so you must have rust installed to build it.

```sh
# clone the repo
$ git clone https://github.com/Dash-L/ut3e
$ cd ut3e
# run the program
$ cargo run
```

## TODO
- [ ] add tests of the engine
- [ ] make the UI look better (it looks really bad right now)
- [ ] Show winner, allow restarting the game
- [ ] Add some settings?
- [ ] Allow saving and stepping through games (depending on how complex this is, maybe with a tree)
- [ ] Some sort of online multiplayer? Also maybe a centralized place to store previous games?

## Other projects and references
- [wikipedia page](https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe)
- [uttt.ai](https://www.uttt.ai/)
- [Bennett Zhang's online version](https://ultimate-t3.herokuapp.com/)
