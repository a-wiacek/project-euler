module Solutions.Euler067 where
import Utils.Input(getInput)
import Solutions.Euler018(process)

euler067 :: IO String
euler067 = show . process <$> getInput 67