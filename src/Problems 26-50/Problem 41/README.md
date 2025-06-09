# Problem 41
### Problem Analysis
- The number 2143 is prime and also 1-4 pandigital.
- We need to find the largest prime number that is also n-pandigital for n = 1 to 9.

### Plan
- The boundaries of our search area:
  - The max number we have to loop to is 987654321, which is the largest 9-pandigital number.
  - The starting number will be 11.
- For each number in between I will get rid of all even numbers. Then I will check if the number is prime in a separate function.
- If it is prime, I will check if it is n-pandigital.
  - I will begin by checking the amount of digits in the number.
    - For 2 digits, I will just match it with the string "12".
    - For 3 digits, I will match it with "123".
    - For 4 digits, I will match it with "1234".
    - And so on until 9 digits.
    - I will sort the digits of the number and compare it with the string of digits.
    - Then if it does match I will keep the original prime-pandigital number in a vector.
    - At the end of the search, I will sort the vector and return the largest number.
- Hopefully this will work. Seems straightforward enough.

### Conclusion
- I did get the right answer with my brute force to 987654321.
- Since the answer is 7652413, I only needed to loop up to 8 million.
- I changed the code to loop up to 8 million.
- So it's faster now.
  