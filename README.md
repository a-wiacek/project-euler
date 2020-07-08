# Project Euler

Collection of solutions to Project Euler problems written in Haskell and/or Rust. One day, other languages might appear here.

## Performance

Not all languages are suitable for squeezing every second of performance.
Consider [problem 512](https://projecteuler.net/problem=512).
Compare performance of Haskell and Rust here.
To solve the problem, I compute totient for every `n` from `1` to `5 x 10^8`.
Both languages use mutable state (ST monad/mutable vector).
Haskell produces answer in around 17 minutes, Rust needs only around 40 seconds.
Haskell also uses 50% more memory (`forM_` allocates additional 1GB?).
Anyways, my goal right now is to rewrite all solutions to Rust and continue solving problems only in Rust.