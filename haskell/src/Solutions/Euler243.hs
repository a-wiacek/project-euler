module Solutions.Euler243 where
import Utils.NumberTheory(unPrime, primes)

prs :: [Integer]
prs = map unPrime primes
-- Kind of educated guess: we want to take a lot of distinct primes
-- R(d) = phi(d) / (d - 1)
isOk phi d = phi * 94744 < (d - 1) * 15499
tot (p:ps) product phi = (product, phi, p):tot ps (p * product) (phi * (p - 1))
-- largest prime to take and upper bound
select ((d, phi, p):t) = if phi * 94744 < (d - 1) * 15499 then (d, p) else select t
(bound, maxP) = select $ tot (tail prs) 2 1
primes' = takeWhile (<maxP) prs

phi' k = foldr (\p n -> n * (p - 1) `div` p) k $ filter ((==0) . mod k) primes'

test = [d | factors <- mapM (\x -> map (x^) [0..3]) primes', let d = product factors, isOk (phi' d) d]

euler243 :: IO String
euler243 = return $ show $ minimum test