# Problem 35
### Problem Analysis
- We are given the number $197$ which is called a circular prime because all of its rotations are also prime. For example, $971$ and $719$ are also prime.
- We need to find all circular primes below one million.
- One key thing to understand is that we are not generating the permutations of the digits of the number, but rather rotating the digits. For example, the rotations of $197$ are $197$, $971$, and $719$.

### Plan
- Since we are are not generating permutations, we can simply rotate the digits of the number and check if each rotation is prime.
- I plan to hard code a different rotation function for each number of digits. 
- I will have to deal with one, two, three, four, five, and six-digit numbers.
- For one-digits, I only have to check if the number is prime.
- For two-digits, I will only need to swap the digits.
- For three-digits, I will have to rotate the digits in a way that the first digit eventually goes to the end.
- Same for four, five, and six-digit numbers. Except with a bit more complexity as the number of digits increases.
- I will use a for loop to iterate through all numbers below one million and check if they are circular primes.
- Before checking if a number is circular prime, I will check if it is prime.
- I will continue the loop if the number is evenly divisible by 2 through 10, as these numbers cannot be primes.
- If the number passes the initial check, I will check if it is prime by dividing it by every number from 2 to the square root of the number.
- Then, if it passes the prime check, I will rotate the digits and check if each rotation is prime.
- Sounds good, let's implement it.

### Coding

I need to find out how to swap the digits. 

- If I have the number 100.
- I can first reverse it to get 001.
- Then from 321, I can swap the first and third digits to get 010.
- But if I have the number 583, I can reverse it to get 385.
- I don't think reversing the digits is the right approach.
- My method of swapping the digits is not working.
- I will try a different approach.
- I will rotate the digits mathematically.
- To rotate the digits one to the right, I can get the last digit by dividing the number by 10 and getting the remainder.
- Then I can get the rest of the number by dividing the number by 10 and taking the floor value.
- Then I can multiply the last digit by 10 raised to the power of the number of digits minus one and add it to the rest of the number.

### Conclusion
- I had a bit of trouble with circulating the digits, but I think I have a good understanding of how to do it now.
- I learned that I can rotate the digits mathematically by getting the last digit and the rest of the number. (Thanks google search AI)
