module Solutions.Euler035 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed((!))

bound = 999999
primes = primesArrayUpTo bound

allCyclic l =
    let l' = show l
        f pivot = read $ drop pivot l' ++ take pivot l'
    in map f [1..length l']

isCyclic n = all (primes !) $ allCyclic n

euler035 :: IO String
euler035 = return $ show $ length $ filter isCyclic [2..bound]