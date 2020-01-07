module Solutions.Euler510 where

funS n = sum [(raBase + rbBase + rcBase) * maxK * (maxK + 1) `div` 2 |
    beta <- [1..floor $ fromIntegral n ** (1 / 4)],
    alpha <- [1..beta],
    gcd alpha beta == 1,
    let alpha2 = alpha^2,
    let beta2 = beta^2,
    let den = (alpha + beta)^2,
    let maxK = n `div` (beta2 * den),
    let raBase = den * alpha2,
    let rbBase = den * beta2,
    let rcBase = alpha2 * beta2]

euler510 :: IO String
euler510 = return $ show $ funS $ 10^9