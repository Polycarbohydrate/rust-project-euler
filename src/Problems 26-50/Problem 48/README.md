# Problem 48
### Problem Analysis
- The series $1^1 + 2^2 + 3^3 + ... + n^n$ can be computed by iterating through the numbers from 1 to n and computing the power of each number to itself.
- I need to find the last ten digits of the sum of a similar series to 1000 to the power of 1000.

### Plan
- I will do this in Python by iterating through the numbers from 1 to 1000, computing the power of each number to itself, and summing them up.
- Rust's type system will be too strict for this problem, as I need to compute very large numbers.
- One loop will be enough.
- I will create a variable to store the sum and initialize it to 0.
- Sounds easy, time to code!

### Conclusion
- Easy problem, I can do it in Python.
- 3 lines of code.