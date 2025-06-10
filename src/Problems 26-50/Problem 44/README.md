# Problem 44
### Problem Analysis
- We are given that the formula for the nth pentagonal number is given by $n(3n-1)/2$.
- The first ten pentagonal numbers are:
  - 1, 5, 12, 22, 35, 51, 70, 92, 117, 145
- It is shown that the fourth and seventh pentagonal numbers (22 and 70) can be used to form a pentagonal pair.
- By adding the two pentagonal numbers, we get 92, which is also a pentagonal number. But when subtracting the two pentagonal numbers, we get 48, which is not a pentagonal number.
- My goal is to find the pentagonal pair such that both the sum and difference of the two pentagonal numbers are also pentagonal numbers. And the difference is the minimum possible.
- Then the answer will be the minimal difference of the two pentagonal numbers.

### Plan
I referenced this article for the pentagonal number formula: https://en.wikipedia.org/wiki/Pentagonal_number

- I will go up to the limit of n = 1 million just to start with.
- Then do nested loops for each pair of pentagonal numbers and perform addition and subtraction checks.
- Then if both the sum and difference are pentagonal numbers, I will check if the difference is minimal by having a counter of the minimum difference and checking if the current difference is less than the minimum difference.
- I can check if the number is pentagonal by using the formula $\sqrt(1 + 24 * n) / 6$ and checking if the result is an integer.
- Again seems easy but looking at the number of solves it seems to be difficult.
- Time to code!

### Conclusion
- Easy, I had to reduce the limit to 10000 to get the answer. I tested the code with 100000 and it was taking too long. So I went to 10000 and got the answer.
- One minor stupid issue was that the autocomplete for my code wrote the inner loop's equation with a +1 instead of -1, so I was getting the wrong answer for a while.
- Overall easy!