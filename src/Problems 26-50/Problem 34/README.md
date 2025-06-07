# Problem 34
### Problem Analysis
- The problem gives the number $145$ as an example, which is makes $1! + 4! + 5! = 145$.
- The problem asks for all numbers that are equal to the sum of the factorials of their digits.
- Note that 1! and 2! are not sums, they are not included in the answer.

### Plan
- First thing I need to determine is the upper limit (max number of digits) of the search space.
- From some research $9! = 362880$, which is the maximum factorial of a digit.
- The largest number that can be formed with $n$ digits is $10^n - 1$.
- If I plug in $n = 7$, I get $10^7 - 1 = 9999999$.
- If I calculate the sum of the factorials of the digits of $9999999$, I get $7 * 9! = 2540160$.
- If I try $n = 8$, I get $10^8 - 1 = 99999999$.
- If I calculate the sum of the factorials of the digits of $99999999$, I get $8 * 9! = 2903040$.
- This means that the maximum number of digits we need to check is 7, because $10^8 - 1$ is greater than the sum of the factorials of its digits.
- In simple terms even if all digits are 9, the sum of the factorials of the digits will not be greater than the number itself for numbers with more than 7 digits.
- I will loop through all numbers from 3 (Since 1! and 2! don't count to the sum) to 9,999,999 and check if the number is equal to the sum of the factorials of its digits.
- Seems pretty straightforward. Time to code!

### Conclusion
- The code is working as expected and returns the correct output.
- The code is efficient enough to run in a reasonable time for the given input size.
- I was worried it might go over Rust's strict integer limits, but it seems to be working fine.