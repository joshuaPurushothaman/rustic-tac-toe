# rustic-tac-toe
Exploring Rust further with a Tic-Tac-Toe program!

Note that some things in here are done "weirdly" with the intent of exploring Rust.
For example, the board states are encoded with bitmasks even though `Vec`s would be perfectly fine (and probably just as fast, given compiler optimizations).
