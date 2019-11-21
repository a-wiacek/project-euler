module Solutions.Euler531 where
import Data.Array.Unboxed
import Math.NumberTheory.Moduli.Chinese
import Utils.NumberTheory(totientArrayUpTo)

lBound = 1000000
bound = 1005000
tots = totientArrayUpTo bound

funG a n b m = maybe 0 id $ chinese (a, n) (b, m)

funF m n = funG (tots ! n) n (tots ! m) m

ans = sum [funF n m | n <- [lBound..bound - 1], m <- [n + 1..bound - 1]]

euler531 :: IO String
euler531 = return $ show ans