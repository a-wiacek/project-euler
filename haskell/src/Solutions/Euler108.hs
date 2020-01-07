module Solutions.Euler108 where
import Utils.NumberTheory(unPrime, primes)
import Data.Array.Unboxed
import Data.Word

primes' = map unPrime primes :: [Word64]

-- 1/x + 1/y = 1/n -> (x - n)(y - n) = n^2
-- Let n^2 = p_1^(2a_1) .. p_k^(2a_k). Then equation has ((2a_1 + 1)..(2a_k + 1) + 1)/2 distinct solutions (x <= y).
-- Smallest n has a1 >= a2 >= ..

bound = 15 :: Int
-- Smallest n such that n^2 has more than moreThan divisors
process :: Word64 -> Word64
process moreThan = 
    let limit = 2 * moreThan
        ansBound = product $ take bound primes'
        loop :: UArray Int Word64 -> Int -> Word64
        loop exps d = if d == bound
            then let pr = product $ map (succ . (*2)) $ elems exps
                 in if pr > limit && pr < 2 * limit
                    then product $ zipWith (^) primes' $ elems exps
                    else ansBound
            else let l = if d == 0 then 3 else exps ! (d - 1)
                 in foldr (min . (\i -> loop (exps // [(d, i)]) (d + 1))) ansBound [0..l] 
    in loop (listArray (0, bound - 1) (repeat 0)) 0

euler108 :: IO String
euler108 = return $ show $ process 1000