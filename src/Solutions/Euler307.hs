module Solutions.Euler307 where
import Data.List
import Data.Ratio
import Text.Printf

-- binom (10^6, k): choose chips with 2 defects
list1 = let l1 n prev = let a = (prev * (10^6 - n + 1)) `div` n in a:l1 (n + 1) a in take 10001 $ 1:l1 1 1
-- binom (10^6 - k, 20000 - 2k): choose chips with one defect
list2 = let l2 n prev = let t = 10^6 - 10^4 + n
                            a = (prev * t * (t - 2 * n + 1)) `div` ((2 * n - 1) * 2 * n)
                        in a:l2 (n + 1) a
        in reverse $ take 10001 $ 1:l2 1 1
-- binom (20000, 2k): choose defects going to 2-chips
list3 = let l3 n prev = let t = 20000 - 2 * n + 2
                            a = prev * t * (t - 1) `div` (2 * n * (2 * n - 1))
                        in a:l3 (n + 1) a
        in reverse $ take 10001 $ 1:l3 1 1
-- (2k)!/(2!)^k: distribute those defects
list4 = let l4 n prev = let a = (prev * (2 * n - 1) * n) in a:l4 (n + 1) a in take 10001 $ 1:l4 1 1
-- (20000 - 2k)!: distribute remaining defects to 1-chips
list5 = let l5 n prev = let a = prev * (2 * n - 1) * 2 * n in a:l5 (n + 1) a in reverse $ take 10001 $ 1:l5 1 1

s = sum (zipWith5 (\a b c d e -> a * b * c * d * e) list1 list2 list3 list4 list5) :: Integer
total = (10^6)^20000 :: Integer
sol = fromRational (s % total) :: Double

euler307 :: IO String
euler307 = return $ printf "%.10f" $ 1 - sol