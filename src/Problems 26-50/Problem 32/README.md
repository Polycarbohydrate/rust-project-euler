# Problem 32
### Problem Analysis
- A number is 1-9 pandigital if it contains all digits from 1 to 9 at least once. (For the purpose of this problem, 0 is not considered a digit.)
- I need to find the sum of all 1-9 pandigital numbers that can be formed by pandigital multiplicand and multiplier.

### Plan
- I will need to have nested loops to generate all possible multiplicands and multipliers.
- The max length of the product will be 4 digits (in order for it to be pandigital with the length of the multiplicand and multiplier).
- This means that the multiplicand can be at most 3 digits long and the multiplier can be at most 2 digits long.
- So I will have two loops, one for the multiplicand and one for the multiplier.
- Then I will split the multiplicand, multiplier and product into digits and check if they are all unique and if they contain all digits from 1 to 9.
- I will do this by pushing the characters into a vector then applying the .dedup() method to remove duplicates.
- If the length of the vector is 9, then it is pandigital.
- I will then add the product to the variable "sum" and print sum at the end.
- Seems simple! Time to code!

### Conclusion
- I did have some trouble that in the end was because of my ranges for the multiplicand and multiplier.
- I adjusted them a bit and got the right answer.