module Solutions.Euler132 where
import Utils.NumberTheory(unPrime, nextPrime)
import Utils.Numeric(fastPowerMod)

-- 2 and 3 don't divide R(10^9) (last digit is 1 (odd), sum of digits is 10^9 (not divisible by 3)
-- for other primes p | R(10^9) <-> p | 9R(10^9)

ps = map unPrime [nextPrime 5 ..] :: [Integer]

euler132 :: IO String
euler132 = return $ show $ sum $ take 40 $ filter ((==1) . fastPowerMod 10 (10^9)) ps