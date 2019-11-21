module Solutions.Euler377 where
import Utils.Matrix

p = 10^9 :: Int

initVector :: Matrix Int
initVector = fromLists $ map return
    [ 261994131, 23817625, 2165227, 196833, 17891, 1625, 147, 13, 1 -- initial f values
    , 256, 128, 64, 32, 16, 8, 4, 2, 1 ] -- initial g values

aMatrixFun :: (Int, Int) -> Int
aMatrixFun (r, c)
    -- formula for f_{n + 1}
    | r == 1 && c <= 9 = 10
    | r == 1 = c - 9
    -- formula for g_{n + 1}
    | r == 10 && c >= 10 = 1
    -- shift {f, g}_{n .. n - 7} downwards
    | r /= 10 && r - c == 1 = 1
    -- all remaining entries are 0
    | otherwise = 0

aMatrix = matrix 18 18 aMatrixFun

funF :: Int -> Int
funF n = if n <= 9
    then initVector ! (10 - n, 1)
    else (mulMod (matrixFastPowerMod aMatrix (n - 9) p) initVector p) ! (1, 1)

euler377 :: IO String
euler377 = return $ show $ sum (map funF $ take 17 $ iterate (*13) 13) `mod` p