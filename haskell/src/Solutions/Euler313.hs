module Solutions.Euler313 where
import Utils.NumberTheory(unPrime, primesUpTo)

{-
Initial position of red counter is (1, 1) and m, n > 1.

If grid is a square, we need moves:
* move empty field to the right side of red counter in 2n - 3 moves
* move red counter and empty field by (+1, +1)
  |R B| -> | RB| -> |BRB| -> |BRB| -> |B B| -> |BB | -> |BBB|
  |BBB| -> |BBB| -> | BB| -> |B B| -> |BRB| -> |BRB| -> |BR |
  this happens n - 2 times, each translation costs 6 moves
* move red counter to (n, n) in 4 moves:
  |R | -> | R| -> |BR| -> |BR| -> |B |
  |BB| -> |BB| -> | B| -> |B | -> |BR|
Overall: 2n - 3 + (n - 2) * 6 + 4 = 8n - 11

Let's look at the equation 8n - 11 = p^2. Then p^2 = 5 mod 8,
but the only possibilities are 0, 1 and 4. No solutions.

If grid is not a square, let m = n + d with d > 0 (length > height). We need moves:
* move empty field to the right side of red counter in 2n + d - 3 moves
* move red counter and empty field by (+1, +1)
  this happens n - 1 times, each translation costs 6 moves
* move red counter and empty field by (+1, 0)
  |BBB| -> |BBB| -> | BB| -> |B B| -> |BB | -> |BBB| 
  |R B| -> | RB| -> |BRB| -> |BRB| -> |BRB| -> |BR |
  this happens d - 1 times, each translation costs 5 moves
* 1 move to put red counter in corner
Overall: 2n + d - 3 + (n - 1) * 6 + (d - 1) * 5 + 1 = 8n + 6d - 13

We have equation 8n + 6d - 13 = p^2.
If p = 2, then 8n + 6d = 17 -> no solutions.
If p = 3, then 8n + 6d = 22 -> one solution: n = 2, d = 1.
Let p > 3, then it must be p^2 = 1 mod 6. Let p^2 = 6k + 1.
Then 4n + 3d = 3k + 7.
If (n, d) is a solution, then (n + 3, d - 4) also is a solution.
The solution with the smallest positive n is (1, k + 1), but this is invalid, so
the proper solution with the smallest n is (4, k - 3).
The proper solution with the smallest d is (*, k `mod` 4 + 1).
There are k `div` 4 solutions for given k.
-}

f :: Int -> Int
f 2 = 0
f 3 = 1
f p = (p^2 - 1) `div` 24

countS :: Int -> Int
countS = sum . map (f . unPrime) . primesUpTo

euler313 :: IO String
euler313 = return $ show $ 2 * countS 1000000