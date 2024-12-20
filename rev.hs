-- reverseList.hs
reverseList :: [a] -> [a]
reverseList [] = []
reverseList (x:xs) = reverseList xs ++ [x]

main :: IO ()
main = do
    let list = [1, 2, 3, 4]
    print $ reverseList list
