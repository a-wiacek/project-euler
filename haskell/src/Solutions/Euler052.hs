module Solutions.Euler052 where
import Utils.List(allEqual)
import Data.List

loop n = if allEqual $ map (sort . show . (*n)) [1..6] then n else loop (n + 1)

euler052 :: IO String
euler052 = return $ show $ loop 1