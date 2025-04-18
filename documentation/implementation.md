# Implementation Document

## Architecture

`src/wave_function_collapse` implementation of the main algorithm - which is quite short as it has been abstracted quite a bit

`src/tile_extraction` implementation of "tile extraction", the process of turning an input image into tiles that can be combined according to some `RuleSet`

`src/tile`, `src/grid` data structures for storing the current grid state

`src/rules` data structure for storing and checking generated rules between tiles

`src/utils` helper data structures for positions, directions and "entropy calculations"

`src/wasm` bindings to web assembly (everything here can be called from javascript)

`frontend` Web UI

## Time complexity

Worst case time complexity should be analyzed. Practical testing suggests that the base algorithm (without backtracking) has been implemented with **quadratic** average time complexity.

## Memory complexity

## Further work

## Usage of Large Language Models

## Sources
