module Solutions.Euler237 where
import Utils.Matrix

p = 10^8 :: Int

aMatrix :: Matrix Int
aMatrix = fromLists [[0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1], [1, -2, 2, 2]]

fastT n =
    let initT = fromLists [[0], [1], [1], [4]]
    in if n < 4
        then initT ! (n + 1, 1)
        else (mulMod (matrixFastPowerMod aMatrix (n - 3) p) initT p) ! (4, 1)

euler237 :: IO String
euler237 = return $ show $ fastT $ 10^12