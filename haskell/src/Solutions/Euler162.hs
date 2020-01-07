module Solutions.Euler162 where
import Data.Char
import Numeric(showHex)

-- count all numbers of length n starting with 1
-- all suffixes - suffixes without a - suffixes without 0 + suffixes without a and 0
case1 n = 16^(n - 1) - 15^(n - 1) - 15^(n - 1) + 14^(n - 1)
-- count all numbers of length n starting with 1
-- all suffixes - suffixes without 1 - suffixes without 0 + suffixes without 1 and 0
caseA = case1
-- count all numbers of length n starting with neither A nor 1
-- all suffixes - suffixes without 1 - suffixes without 0 - suffixes without A
-- + suffixes without 1 and 0 + suffixes without 1 and A + suffixes without A and 0
-- - suffixes without 1, A and 0
caseOther n = 13 * (16^(n - 1) - 3 * 15^(n - 1) + 3 * 14^(n - 1) - 13^(n - 1))
-- count all numbers of length n
ofLength n = case1 n + caseA n + caseOther n

euler162 :: IO String
euler162 = return $ map toUpper $ showHex (sum $ map ofLength [3..16]) ""