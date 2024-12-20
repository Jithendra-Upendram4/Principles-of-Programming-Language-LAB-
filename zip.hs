-- listZipWith.hs
listZipWith :: (a -> b -> c) -> [a] -> [b] -> [c]
listZipWith _ [] _ = []
listZipWith _ _ [] = []
listZipWith f (x:xs) (y:ys) = f x y : listZipWith f xs ys

main :: IO ()
main = do
    let list1 = [1, 2, 3]
        list2 = [4, 5, 6]
    print $ listZipWith (+) list1 list2
