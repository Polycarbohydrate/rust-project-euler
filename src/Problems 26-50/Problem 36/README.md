# Problem 36
### Problem Analysis
- So we are given the number 585 which is a palindrome in base 10 and base 2. Since it reads the same both forward and backward in both bases, it is a double-base palindrome.
- We need to find the sum of all numbers less than 1,000,000 that are double-base palindromes.
- The number in either base may not include leading zeros.

### Plan
- I will loop through all numbers from 1 to 1,000,000.
- I will check if the number is a palindrome in base 10 first by getting its characters then reversing the characters in the vector and comparing the original vector with the reversed vector. If they are the same, then it is a palindrome in base 10.
- Then I will convert the number to binary and check if it is a palindrome in base 2 by getting its characters then reversing the characters in the vector and comparing the original vector with the reversed vector. If they are the same, then it is a palindrome in base 2.
- Seems pretty straightforward, so I will implement it now.

### Conclusion
- It was easy and I had no problems at all!