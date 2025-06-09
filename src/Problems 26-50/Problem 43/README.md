# Problem 43
### Problem Analysis
- I am given the number 1406357289 which is 0 to 9 pandigital.
- It is 10 digits long.
- I need to find the sum of all 0 to 9 pandigital numbers that satisfy the following property:
  - The 2nd to 4th digits form a number that is divisible by 2.
  - The 3rd to 5th digits form a number that is divisible by 3.
  - The 4th to 6th digits form a number that is divisible by 5.
  - The 5th to 7th digits form a number that is divisible by 7.
  - The 6th to 8th digits form a number that is divisible by 11.
  - The 7th to 9th digits form a number that is divisible by 13.
  - The 8th to 10th digits form a number that is divisible by 17.

### Plan
- I hate permutations so, I will just brute force generate all 0 to 9 pandigital numbers.
- First thing is that the numbers will be 10 digits long.
- So I will loop from the first 10-digit number to the last 10-digit number. (1,000,000,000 to 9,999,999,999)
- This will take a while though.
- Then for each number, I will check if it is pandigital.
- If it is pandigital, I will check if it satisfies the properties.
- Then I push all pandigital numbers that satisfy the properties into a vector.
- Then print the sum of the vector.
- Should be easy enough. Just compute time might be long.

### Conclusion
- I was able to properly brute force it though it took a while (~1.3 hours).