module Solutions.Euler072 where
import Utils.NumberTheory(totientArrayUpTo)
import Data.Array.Unboxed(elems)

euler072 :: IO String
euler072 = return $ show $ pred $ sum $ elems $ totientArrayUpTo 1000000