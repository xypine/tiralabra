# Software Requirements Specification

## Practical information for the course

I plan to implement the project in Rust, as I like writing it, it has predictable performance characteristics and it is easy to test and embed in a web based interface.

Languages I know enough to be able to peer review include Typescript / Javascript, Python, Go, Java and maybe C/C++.

All documentation will be written in English.

## The Core Algorithm

This project will mainly aim to implement the "Wave Function Collapse" algorithm based on Maxim Gumin's 2016 implementation [[1](#1)]. The goal of the algorithm is to generate locally similar outputs from a single example input. In the context of Gumin's implementation, this means generating bitmap images from a single example image.

Gumin's implementation step by step (given an input image of size W x H, tile size N, output size I x J):

1. Tile extraction $O(HW)$

- Split the input image to overlapping tiles of size N x N
- Generate rotated and mirrored versions of the tiles
- Deduplicate generated tiles
  $T = \text{number of tiles generated}$

2. Adjacency rule extraction $\text{maybe} O(HW \cdot HW \cdot N^2)$: Check which tiles are next to eachother in the original image
3. Initialize output $O(I \cdot J \cdot T)$: Any cell could be any tile
4. Find the cell with the lowest entropy $O(I \cdot J) \text{or maybe} O(log (I \cdot J) \text{using a priority queue})$

- if all cells have been collapsed, the algorithm is complete
- if the cell has no possible states, restart the algorithm with a different random seed

5. Collapse the cell into one of it's possible states
6. Propagate constraints $O(I \cdot J \cdot T)$: for each neighbour

- remove states that conflict with possible (or collapsed) state of the modified neighbour
- if states were removed, go to step 6 (propagation is recursive)

7. Go to step 4

The time complexity of the whole algorithm depends heavily on how complex the rules of the tiles are. Intricated rules will probably result in fewer propagations, but increase the likelihood that the algorithm fails (especially with large output sizes). The worst case time complexity is probably exponential if the algorithm is allowed to fail or return an invalid solution.

If the algorithm were not allowed to fail, time complexity would be infinite as it's possible to craft inputs that are impossible to solve with some output sizes [[2](#2)]. As such, it would by definition not be a valid algorithm, so this project will focus on the bounded variant which may fail. In Gumin's implementation, each sample dataset contains information on the maximum attempts to try.

## Extensions

The project will include two (optional) modifications to Gumin's implementation:

1. Backtracking
   Gumin's implementation restarts the generation process from scratch if the output contains a tile with zero possible states. I intend to implement some alternative way of error recovery. Full backtracking might not be the best solution, as it is nigh impossible to reason about how early on the contradiction started.
2. Alternative heuristic for random selections
   If the selected cell has multiple possible states to collapse to, Gumin's implementation chooses one of them randomly using the a distribution based on the frequency of the tiles in the original image. One possibility is to use a distribution based on the difference between number of tiles in the original and current image, resulting in a soft "global constraint".

## Testing

- testing will be done using rust's built in testing framework
- unit tests for tile extraction, adjacency rules and propagation logic
- end to end tests for test patterns used in Gumin's implementation

## Secondary Goals

- there should be a browser-based ui for trying out the implementation
- existing implementations are "slow", try to maintain a "realtime" performance for small output sizes
- support one, two and three dimensions

## Sources

### 1

Maxim Gumin, Wave Function Collapse implementation. Retrieved from https://github.com/mxgmn/WaveFunctionCollapse

### 2

I. Karth and A. M. Smith, "WaveFunctionCollapse: Content Generation via Constraint Solving and Machine Learning" in IEEE Transactions on Games, vol. 14, no. 3, pp. 364-376, Sept. 2022, doi: 10.1109/TG.2021.3076368. Available: https://ieeexplore.ieee.org/document/9421370
