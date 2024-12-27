import Text.Printf (printf)

-- Function to calculate tax under the Old Tax Regime
calculateOldTax :: Double -> Double
calculateOldTax income
    | income <= 250000 = 0
    | income <= 500000 = (income - 250000) * 0.05
    | income <= 1000000 = 12500 + (income - 500000) * 0.2
    | otherwise = 112500 + (income - 1000000) * 0.3

-- Function to calculate tax under the New Tax Regime
calculateNewTax :: Double -> Double
calculateNewTax income
    | income <= 250000 = 0
    | income <= 500000 = (income - 250000) * 0.05
    | income <= 750000 = 12500 + (income - 500000) * 0.1
    | income <= 1000000 = 37500 + (income - 750000) * 0.15
    | income <= 1250000 = 75000 + (income - 1000000) * 0.2
    | income <= 1500000 = 125000 + (income - 1250000) * 0.25
    | otherwise = 187500 + (income - 1500000) * 0.3

-- Main function to take input and calculate tax
main :: IO ()
main = do
    putStrLn "Enter your annual income (in ₹):"
    incomeStr <- getLine
    let income = read incomeStr :: Double

    putStrLn "Choose a tax regime:"
    putStrLn "1. Old Tax Regime"
    putStrLn "2. New Tax Regime"
    choiceStr <- getLine
    let choice = read choiceStr :: Int

    let tax = case choice of
            1 -> calculateOldTax income
            2 -> calculateNewTax income
            _ -> error "Invalid choice. Please choose 1 or 2."

    printf "Your income tax under the chosen regime is: ₹%.2f\n" tax
