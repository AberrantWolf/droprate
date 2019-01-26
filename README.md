# Drop Rate

A Rust crate for choosing outcomes based on a weighted probability list.

You can use this to calculate results with tables on the fly, or to store tables and reuse them to calculate results from the same tables.

(Future) This crate also includes the option to create more "user-friendly" results which will not be even remotely random (though still "randomly generated"), but will conform to a user's expectation that "the longer it's been since SOME_RESULT, the more likely that result becomes. If you store a table, you can use `fair_random`, which stores information about past results in order to adjust the probablility of each outcome such that you will end up with a more uniform distribution of outcomes than would be expected from a proper random result.

## Roadmap

* Finish implementing random results
* Design and implement the `fair` version of the generate function
* Make a macro to more easily create tables in-line
* Generate some samples of `random` and `fair_random`
* Output statistics from a table
  * e.g., theoretical probabilities, relative probabilities, etc...
  * e.g., probability of N in a row, or other patterns