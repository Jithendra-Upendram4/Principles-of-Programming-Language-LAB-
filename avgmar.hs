-- averageMarks.hs
average :: [Int] -> Float
average [] = 0
average lst = fromIntegral (sum lst) / fromIntegral (length lst)

averageMarks :: [(String, Int, [Int])] -> [(String, Float)]
averageMarks [] = []
averageMarks ((name, _, marks):xs) =
    (name, average marks) : averageMarks xs

main :: IO ()
main = do
    let students = [("Alice", 1, [80, 90, 85]),
                    ("Bob", 2, [70, 75, 80]),
                    ("Charlie", 3, [85, 95, 90])]
    let averages = averageMarks students
    mapM_ (\(name, avg) -> putStrLn $ name ++ ": " ++ show avg) averages
