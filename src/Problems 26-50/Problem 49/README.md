# Problem 49
### Problem Analysis
- The three numbers 1487, 4817, and 8147 are permutations of each other and are also prime. Each increases by 3330.
- The problem says there are no 1, 2, or 3-digit numbers that satisfy the condition. But there is one other 4-digit number that satisfies the condition.
- My task is to find these 4-digit numbers.
- Then submit the answer as the 12-digit number formed by concatenating the three numbers in ascending order.

### Plan
- Generate all 4-digit prime numbers.
- Then for each prime number, I can find all permutations of the digits.
- For each permutation, I can check if it is a prime number and if it is a 4-digit number.
- And then check if the difference between the numbers is the same within the set of permutations.
- Time to code!

### Conclusion
- Pretty hard to code.
- But I think I have a good understanding of the problem.