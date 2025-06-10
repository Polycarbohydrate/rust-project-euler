# Problem 46
### Problem Analysis
- Goldbach's other conjecture states that every odd integer greater than 5 can be expressed as the sum of a prime and twice a square.
- So for example, 7 can be expressed as 5 + 2 * 1^2, 9 can be expressed as 7 + 2 * 1^2, and so on.
- I need to find the smallest odd composite number that cannot be expressed as the sum of a prime and twice a square.

### Plan
- My search area will be all odd numbers greater than 33 (as shown in project euler, 33 is able to be expressed as the sum of a prime and twice a square).
- The number will also need to be composite, so I will check if the number is prime.
- The hardest part will be integer factorization.
- Since I will only have composite numbers after the previous checks, I can find the largest prime up to the number.
- I don't really enjoy coding the sieve of Eratosthenes, so I will just check each number up to the square root of the number to find the primes. 
- After some lengthy research, I can't see a really easy way to do this besides brute force.
- So I will need to check every possible combination of prime and square to see if it equals the number.
- This will be tedious... Time to code!

### Conclusion
- I did a full brute force search.
- It worked and was faster than I expected.