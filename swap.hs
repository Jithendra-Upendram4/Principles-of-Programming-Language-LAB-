-- swapTuple.hs
swapTuple :: (a, b) -> (b, a)
swapTuple (x, y) = (y, x)

main :: IO ()
main = do
    let tuple = (1, "hello")
    print $ swapTuple tuple
