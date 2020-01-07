module Solutions.Euler074 where
import Utils.Numeric(factorial)
import Control.Monad.Reader
import Data.Char
import qualified Data.Set as Set

fn :: Int -> Int
fn = sum . map (factorial . digitToInt) . show

type Env = (Int, Set.Set Int)
updateEnv :: Int -> Env -> Env
updateEnv n (len, env) = (len + 1, Set.insert n env)
cycleLength :: Int -> Reader Env Int
cycleLength n = ask >>= \(len, env) -> if n `Set.member` env || len > 60
    then return len -- no reason to cycle for more than 60 terms
    else local (updateEnv n) $ cycleLength $ fn n

getCycleLength :: Int -> Int
getCycleLength n = runReader (cycleLength n) (0, Set.empty)

euler074 :: IO String
euler074 = return $ show $ length $ filter (==60) $ map getCycleLength [1..999999]