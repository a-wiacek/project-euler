module Solutions.Euler272 where
import Utils.NumberTheory(unPrime, primes)
{-
If n = p_1^{k_1} ... p_l^{k_l} and each equation x^3 = 1 mod p_i^{k_i} has s_i
solutions, then x^3 = 1 mod n has s_1 ... s_l solutions (by chinese remainder
theorem, solutions don't overlap). We want s_1 ... s_l = 243 = 3^5
For each prime number there are either 1 or 3 solutions to x^3 = 1:
https://math.stackexchange.com/questions/15721/solve-x3-equiv-1-pmod-p-for-x
http://oeis.org/A088232
-}

-- To make it easier, sols = log_3(solutions)
-- log_3(solutions), would be 3 else 1
numberOfSolutions p = if p `mod` 6 == 1 then 1 else 0 -- this works for primes >3 only!
e sols = if sols == 5 then 1 else 5 - sols

bound = 10^11
countSolutions (3:ps) product sols
    | sols > 5 = 0
    | product * 3 ^ e sols > bound = if sols == 5 then product else 0
    | otherwise =
        let case1a = countSolutions ps product sols -- we don't take 3
            case1b = countSolutions ps (product * 3) sols -- we take 3 only once
            possiblePower = takeWhile (bound `div` product>=) $ map (3^) [2..]
            cases2 = map (\x -> countSolutions ps (product * x) (sols + 1)) possiblePower
        in case1a + case1b + sum cases2
countSolutions (p:ps) product sols
    | sols > 5 = 0
    | product * p ^ e sols > bound = if sols == 5 then product else 0
    | otherwise = 
        let ss = numberOfSolutions p
            case1 = countSolutions ps product sols -- we don't take p
            possiblePower = takeWhile (bound `div` product>=) $ map (p^) [1..]
            cases2 = map (\x -> countSolutions ps (product * x) (sols + ss)) possiblePower
        in case1 + sum cases2

euler272 :: IO String
euler272 = return $ show $ countSolutions (map unPrime primes) 1 0