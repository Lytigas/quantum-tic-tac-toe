# Quantum Tic Tac Toe

Implements [quantum tic-tac-toe](https://en.wikipedia.org/wiki/Quantum_tic-tac-toe) in Rust.
Currently playable with two characters in a CLI interface. TUI and AI are planned.

## The Game

These rules are modified from [this page](http://www.cel.edu/Quantum/Tic-Tac-Toe/).

Quantum Tic-Tac-Toe is played on the same board as Classical Tic-Tac-Toe. It consists of a 3×3 grid of squares, labeled “1” through “9” (left to right and top to bottom). The game starts with an empty board.

There are two players, X and O. They take turns marking their moves, with X going first. As in Classical Tic-Tac-Toe, player X marks his moves with “X”s and player O marks her moves with “O”s. However, in Quantum Tic-Tac-Toe, each player must place marks in two different squares. The marks are subscripted with the number of the current move, so all of X's moves are subscripted with odd numbers, and all of O's moves are subscripted with even numbers. The moves in Quantum Tic-Tac-Toe are mixed state moves – half in one square, half in another. One only finds out, later in the game, in which square each move actually is.

Mixed state moves can share squares, becoming entangled. There is no limit on the number of mixed state moves that can be in a square. However, at some point, these entanglements will always become circular, and then a new type of move is required – the collapse. Collapses are how the quantum moves are converted to classical moves. Half of each pair of the involved mixed state moves are eliminated. The one that is left is the classical move, there can be only one per square, and further quantum moves into such collapsed squares are forbidden.

It really helps to have an example in an attempt to explain this core feature of Quantum Tic-Tac-Toe. Consider the following four moves in a game: X1 in squares 1&2, O2 in squares 2&5, X3 in squares 5&9, and O4 in squares 5&1. X1 depends on O2 which depends on O4 which depends back onto X1. This self-reference is another way to look at the cyclic entanglement. It is now required to collapse the quantum moves, the mixed state moves, to classical moves, since if more quantum moves were allowed in just these four squares, one would have five moves trying to fit into only four squares. There wouldn't be room for all of them. Since O made the move that caused the cyclic entanglement, X gets to choose how it settles out; he gets to choose the collapse.

There are three moves (X1, O2, O4), and three squares (1, 2, 3), involved in the cycle. The fourth move and square (X3 and square 9) are entangled with the cycle, but not actually a part of it. No matter how the cycle is collapsed, X3 must end up in square 9. It is called a stem. The players have no choices in how stems collapse. They do have lots of choices of how to go about specifying the collapse, but in the end, all collapsing entanglements have only two possibilities. To specify a particular collapse, a player merely selects one subscripted mark from among those mixed state moves involved in the cyclic entanglement to be the classical move in that square. This forces all the other entangled moves to settle out to classical states. Once the collapse has been indicated, X gets to make his next regular mixed state move, X5. Note, that mixed state moves cannot be played in squares that have collapsed to classical moves. Also, mixed state moves cannot be played in the same square (self-collapse), as this would allow Quantum Tic-Tac-Toe to degenerate directly to Classical Tic-Tac-Toe.

Because classical moves only occur from collapses, the game cannot end until at least one collapse occurs. A 3-row only counts if it consists entirely of classical moves. 3-rows of quantum moves, (mixed state moves) don't count. Since multiple squares are involved with each collapse, it is possible for both players to get 3-rows from a single collapse. This is regarded as a tie, something that can't happen in Classical Tic-Tac-Toe, since the only tie in that game is no 3-rows for either players, the cat's game.
