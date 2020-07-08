module Solutions.Euler069 where
import Utils.List(maxBy)
import Utils.NumberTheory(totientArrayUpTo)
import Data.Array.Unboxed((!))

bound = 1000000
totient = totientArrayUpTo bound

qTot x = let t = totient ! x in (x, t, fromIntegral x / fromIntegral t)
get (n, _, _) = n

euler069 :: IO String
euler069 = return $ show $ get $ maxBy (\(_, _, x) -> x) $ map qTot [2..bound]