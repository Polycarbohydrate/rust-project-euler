# Problem 45
### Problem Analysis
- The formula for triangle numbers is $n(n+1)$.
- The formula for pentagonal numbers is $n(3n-1)/2$.
- The formula for hexagonal numbers is $n(2n-1)$.
- It is verified that the 285th triangle number is also a pentagonal and hexagonal number (40,755).
- I have to find the next triangle number that is also pentagonal and hexagonal.

### Plan
- I plan to have a loop that goes from 286 to infinity (until we find it).
- Then I will plug in the value of n into the triangle number formula and check if the result is also a pentagonal and hexagonal number.
- I can check if a number is an either a pentagonal or hexagonal number by using the formulas:
  - $n = \frac{1 + \sqrt{1 + 24x}}{6}$ for pentagonal numbers
  - $n = \frac{1 + \sqrt{1 + 8x}}{4}$ for hexagonal numbers
- If the code meets the condition, it will break and I can print the number.
- Seems very easy, time to code!

### Conclusion
- I think this was the easiest problem so far.
- Had fun!
- 1694500 nanoseconds of runtime
- Very fast, very easy, very fun.