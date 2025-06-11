# Problem 47
### Problem Analysis
- The first two numbers to have consecutive prime factors are 14 and 15.
  - 2 and 7 are the prime factors of 14.
  - 3 and 5 are the prime factors of 15.
- The first three numbers to have consecutive prime factors are 644, 645 and 646.
  - 2, 7 and 23 are the prime factors of 644.
  - 3, 5 and 43 are the prime factors of 645.
  - 2, 17 and 19 are the prime factors of 646.
- I need to find the first four consecutive integers that have four distinct prime factors each.

### Plan
- I plan to loop from 650 to infinity and factor each number.
- I will factor each number by doing trial division.
- I will have four numbers in different variables to factor.
- Once they are all factored, I will then check if they have four distinct prime factors.
- That is, if each number has 4 different primes.
- I will just push the factors into a vector for each number, sort it and remove duplicates. And if the length of the vector is 4 for each number, I will print the numbers and exit.
- Seems alright, just hardest part is to factor the numbers.
- Time to code.

### Conclusion
- Pretty difficult problem, but I did it.
