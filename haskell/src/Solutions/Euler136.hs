module Solutions.Euler136 where
import Utils.NumberTheory(unPrime, primesUpTo)

bound = 49999999 :: Int

take4s p = takeWhile (\x -> p * (4^x) <= bound) [if p `mod` 4 == 3 then 0 else 1..2]

euler136 :: IO String
euler136 = return $ show $ (+2) $ sum $ map (length . take4s . unPrime) $ tail (primesUpTo bound)