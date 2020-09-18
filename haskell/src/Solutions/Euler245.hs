module Solutions.Euler245 where
import Data.List(foldl')
import Utils.NumberTheory(Prime, primesUpTo, unPrime)
import Data.Word

-- Execution time: 29314.0568664s

type K = Word
newtype Prod = Prod (K, K) -- product and phi
getProd :: Prod -> K
getProd (Prod (n, _)) = n
leq :: Prod -> K -> Bool
Prod (n, phi) `leq` s = n <= s
isValid :: Prod -> Bool
isValid (Prod (n, phi)) = n - phi > 1 && (n - 1) `mod` (n - phi) == 0
mulProd :: K -> Prod -> Prod
mulProd p (Prod (n, phi)) = Prod (p * n, phi * (p - 1))

newtype Prods = Prods (K, [Prod]) -- sum of already found numbers and potential (to be multiplied) candidates
getSum :: Prods -> K
getSum (Prods (f, a)) = f + sum (map getProd $ filter isValid a)
mulProds :: K -> Prods -> K -> Prods
mulProds bound (Prods (f, a)) p = Prods $ foldr g (f, []) a where
    g prod (f, a)
        | prod' `leq` bound = (f, prod : prod' : a)
        | isValid prod = (f + getProd prod, a)
        | otherwise = (f, a)
        where prod' = mulProd p prod

sqfc :: K -> K
sqfc bound = getSum $ foldl' (mulProds bound) (Prods (0, [Prod (1, 1)])) (tail $ map unPrime $ primesUpTo $ bound `div` 3)

euler245 :: IO String
euler245 = return $ show $ sqfc $ 2 * 10^11