# Problem 37
#### Problem Analysis
- We are given the number 3797 as an example. This number is prime and if we remove the first digit, we get 797 which is also prime. If we remove the first two digits, we get 97 which is also prime. If we remove the first three digits, we get 7 which is also prime.
- It is prime if you remove digits left to right or right to left.
- We are to find the sum of the only eleven primes that are both truncatable from left to right and right to left.
- 2,3,5,7 are not considered truncatable primes.

### Plan
- My first problem is to find the limits of the search space.
- I will start from the number 11 since any number less than 11 is not a truncatable prime.
- I will get rid of even numbers and numbers that end with 5 since they cannot be primes.
- I will get the individual digits of the number in a vector. 
- Then I will remove the digits from left to right and right to left and check if the number is prime.
- Then, if it passes the test, I will add the prime to a sum.
- Then print the sum.
- Problem is I still don't know the limits of the search space. I will just keep searching until I find 11 truncatable primes.
- Time to code!

### Conclusion
- Had some problem which was checking the last remaining digit when you aren't supposed to.
- Some hiccups with the prime checking function.
- But at the end I solved the problem relatively quickly.