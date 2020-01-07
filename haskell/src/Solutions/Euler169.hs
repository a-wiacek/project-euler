module Solutions.Euler169 where
import Control.Monad.Memo
import qualified Data.Map as Map
-- http://oeis.org/A002487

compute :: Integer -> Memo Integer Integer Integer
compute n =
    let n2 = n `div` 2
    in memo compute n2 >>= \f2 -> if even n
        then return f2
        else (f2+) <$> memo compute (n2 + 1)

euler169 :: IO String
euler169 = return $ show $ evalMemo (compute $ 10^25 + 1) $ Map.fromList [(0, 1), (1, 1)]