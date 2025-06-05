# Problem 30 Notes
### Problem Analysis
We need to find the sum of all numbers that can be written as the sum of fifth powers of their digits.

### Plan
- First thing I will do is loop through all numbers from 0 to 10.
- I will probably use nested loops to find the sum of the fifth powers of the digits.
- Then I will need to check if the sum is equal to the digits in their order.
- Seems simple enough, I will try to implement it.

### Result:
At first my code only looped to 4 digits, but I realized that the maximum number of digits that can be represented by the sum of fifth powers is 6. So added 2 more letters to loop to 6 digits. Pretty fast computation time.