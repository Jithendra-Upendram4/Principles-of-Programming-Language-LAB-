-- multiplyElements.hs
multiplyElements :: Num a => [a] -> a -> [a]
multiplyElements lst n = [x * n | x <- lst]

main :: IO ()
main = do
    let numbers = [1, 2, 3, 4]
        multiplier = 2
    print $ multiplyElements numbers multiplier
