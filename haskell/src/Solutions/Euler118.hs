module Solutions.Euler118 where
import Control.Monad
import Data.List
import Utils.List(uniques)
import Utils.NumberTheory(isPrime)

doCommas :: [Int] -> [Bool] -> [Int]
doCommas (h:t) commas = doCommas' h t commas []
doCommas' acc [] [] ans = acc:ans
doCommas' acc (hnums:tnums) (hbools:tbools) ans = if hbools
    then doCommas' hnums tnums tbools (acc:ans)
    else doCommas' (10 * acc + hnums) tnums tbools ans

distinctSets = uniques [sort numbers |
    digits <- permutations [1..9],
    commas <- forM [1..8] $ const [True, False],
    let numbers = doCommas digits commas,
    all isPrime numbers]

euler118 :: IO String
euler118 = return $ show $ length distinctSets 