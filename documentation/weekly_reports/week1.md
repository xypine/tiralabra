1. What have I done this week?

- Did some preliminary work exploring what topics could be interesting based on the examples given,
  landed on Wave Function Collapse: ~30min
- Attended the starting lecture on Wednesday, confirmed the viability of the topic with the teacher: 1h45min
- Searched for research on the topic: ~2h
- Wrote the initial requirements specification: ~3h
- Wrote this report: ~30min
- Initialized the Rust project: ~5min

Time used: ~8h

2. How has the program progressed?

- Plans have been made

3. What did I learn this week / today?

- Wave function collapse is actually a variant of Paul Merrell's "model synthesis" algorithm (published way back in 2007), with some minor modifications:

  1. During the tile extraction step, Gumin uses a sliding window resulting in overlapping tiles. This makes it possible to generate useful constraints from a single input image.
  2. Gumin uses an entropy calculation when selecting which cell to collapse, while Merrell used a simple scanline approach (collapsing tiles in order). According to Gumin, this removes directional bias present in Model Synthesis.

  - Existing implementations of the WFC algorithm seem to run into conflicts quite rarely, especially on low output sizes. Backtracking may still be better than starting again from scratch, as long as we don't get stuck.

- Complete Backtracking has been proven to be a NP-complete problem

- Because of its high worst-case time complexity, Wave Function Collapse is best suited for generating somewhat small, finite patterns.
  With an infinite output size, the algorithm will at some point end up in an incorrect state, and then have to start all over again. Propagation also causes problems for large output sizes - even if the neighbouring rules are simple, in practice propagation will have to stop at some point. The algorithm is still useful, especially because it allows additional constraints set by the user or some other algorithm.
  - There have been attempts to get WFC to reliably work on infinite / large output sizes:
    https://arxiv.org/pdf/2308.07307 "Extend Wave Function Collapse Algorithm to Large-Scale Content Generation"

3. What was left unclear or caused difficulty?

- Backtracking methods to try

4. What'll I do next?

- write initial unit tests
- write some initial data types and methods for tiles / grid
