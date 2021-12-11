# About

A sudoku solver written in Rust.

The point of making this solver is to attempt to port the APL code for solving a sudoku puzzle seen [here](https://www.youtube.com/watch?v=DmT80OseAGs) to Rust and see what differences in thought arise, and to see how things can be similar.

Overall, the goal of making the solver worked, but a proper port of the APL solution is not possible in **base** Rust. This is due to a fundamental difference between Rust and APL in that Rust is not an array-based language like APL. Theoretically, ndarray might be able to do more to make the rust solution act similarly to the APL solution, but this is still an interesting solution as-is.

# The Solution

The way that the solver creates its solutions is via a breadth-first search.

First, it checks the given puzzle to fill in any spaces where only one value is possible at the beginning. Afterwards, it goes over each space, get all possible values for that space, and creates a vector of puzzles that have those values inserted at that space. It then prunes the vector to remove any puzzles where the insertion prevents a space from containing a value. Repeating this process across all open spaces will create a vector containing all possible answers to the given sudoku puzzle.

# Performance

There are most certainly performance gains to be had. As-is, for the two puzzles in the main function in debug, the solver takes 2 to 3 milliseconds to solve the second, easier puzzle, and about 50 milliseconds to solve the first, harder puzzle (on my machine).

In release (`cargo run --release`), the solver takes 3 to 4 milliseconds for the first, and about 200k-300k nanoseconds for the second puzzle.

# What's With the Massive Hard-Coded Array?

The massive array is a representation of all 81 spaces in an sudoku puzzle and the indices that can contend with each spot. In other words, assuming you flatten a sudoku puzzle into a 1D array, in order to find out which numbers are available for index 0, you check indices 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, and 72. This array was generated once by the function `_generate_contention_indices()`. While I could simply call the function over and over, it's not very efficient to do so since the matrix never changes from run to run. I'd rather generate and print the matrix once, copy it to the code, and then leave the function as a paper trail.