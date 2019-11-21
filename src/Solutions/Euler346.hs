module Solutions.Euler346 where
import Utils.List(uniques)

-- If we can find a nontrivial representation for k (with >= 3 digits),
-- then it is strong repunit, since in base k - 1 it is written as 11.

bound = 10^12
strongRepunits :: Int -> [Int]
strongRepunits n =
    let l = n * n + n + 1 : map (succ . (*n)) l
    in takeWhile (<bound) l

euler346 :: IO String
euler346 = return $ show $ succ $ sum $ uniques $ concatMap strongRepunits [2..10^6]