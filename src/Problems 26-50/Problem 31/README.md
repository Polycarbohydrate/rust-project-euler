# Problem 31 
### Problem Analysis
- In the United Kingdom, the pound (£) is divided into 100 pence (p).
- There are eight coins in general circulation:
  - 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
- My task is to find the number of different ways to make £2 using these coins.

### Plan
- I think I will use a lot of loops for each coin and just add up the sum of the coins within the loops to see if they reach 200 pence. Then I will just increment a counter if they do.
- Pretty straightforward, but I will have to be careful about the order of the loops so that I don't count the same combination multiple times.
- Time to code!

### Conclusion
- I have implemented the solution using nested loops for each coin denomination.
- The code iterates through each coin and checks if the sum of the coins equals 200 pence.
- The solution is efficient for the given problem and correctly counts the number of combinations to make £2 using the specified coins.

The code ran in ~1 second