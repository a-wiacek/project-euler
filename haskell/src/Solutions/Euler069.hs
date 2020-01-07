module Solutions.Euler069 where
import Utils.NumberTheory(totientArrayUpTo)
import Data.Array.Unboxed((!))

bound = 1000000
totient = totientArrayUpTo bound

max' (a, b, c) (d, e, f) = if c > f then (a, b, c) else (d, e, f)
qTot x = let t = totient ! x in (x, t, fromIntegral x / fromIntegral t)
get (n, _, _) = n

euler069 :: IO String
euler069 = return $ show $ get $ foldr1 max' $ map qTot [2..bound]