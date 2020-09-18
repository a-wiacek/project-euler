# Project Euler

Collection of solutions to Project Euler problems written in Haskell and Rust.
One day, other languages might appear here.

## Performance (Haskell vs Rust)

Consider [problem 512](https://projecteuler.net/problem=512).
To solve the problem, I compute totient for every `n` from `1` to `5 x 10^8`.
Both languages use mutable state (ST monad/mutable vector).
Haskell produces answer in around 17 minutes, Rust needs only around 40 seconds.
Haskell also uses 50% more memory (`forM_` allocates additional 1GB?).

On the other hand, Haskell easily allows to write deeply recursive solutions
(compare solutions of [problem 287](https://projecteuler.net/problem=287)).

Anyways, my goal is to continue solving problems in both languages.