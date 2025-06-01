# Problem 26 Notes
I will be basing my notes primarily on [this](https://math.stackexchange.com/questions/377683/length-of-period-of-decimal-expansion-of-a-fraction) article.

### Article analysis
- Two fractions mentioned by the original poster are 1/3 = 0.333... 0.(3) and 119/13 = 9.153846153846... 9.153846(153846)
- The top answer gives the example of fraction 1/7 = 0.142857142857... 0.(142857) which has a period of 6.

When they multiply it by 10^6 it becomes 142857.142857... 142857.(142857) which is the same as the original fraction. They get the equation 10^6 * (1/7) = 142857/7. Then they subtract the original fraction from this equation to get 10^6 * (1/7) - (1/7) = 142857/7 - (1/7). This simplifies to (10^6 - 1)/7 = 142857/7. Then that becomes 1/7 = 142857/(10^6 - 1). This shows that the period of the decimal expansion of a fraction is related to the denominator of the fraction.

```"As you can see, the denominator is one less than a power of 10, and the power is the period of the decimal expansion. This is no accident, and works for any fraction - if you can rewrite it in this form, the denominator reveals the period."```

They then rearrange the equation to get 1/7 = 142857/(10^6 - 1).

```As such, we can use modular arithmetic to look for the period. Since a×d≡0(modd), we have that 10n−1≡0(modd), or 10^n=1 (mod d).```

$10^n \equiv 1\ (mod \ d)$

This means it means that 10^n, when divided by d, leaves a remainder of 1.

```And therefore you can just look for the smallest n>0 satisfying this.```

### Program Analysis

The program will need to find the period of the decimal expansion of fractions with denominators from 1 to 1000. The period is defined as the smallest n such that $10^n \equiv 1\ (mod \ d)$, where d is the denominator.

- n = period
- d = denominator

Solve for n by iterating through values of n starting from 1 and checking if $10^n \equiv 1\ (mod \ d)$.

Seems relatively straightforward.

### Coding the solution

As soon as I put my original code in Rust, I noticed the integer type restrictions causing panics due to overflow. I quickly realized that I needed to switch to Python to avoid these issues.
- I optimized my original code to automatically rule out `d`'s that were divisible by 2 or 5, since they do not contribute to the period of the decimal expansion (Termination).
- Another problem I did not realize until after testing multiple iterations was that there were times when the remainder would repeat, causing an infinite loop. I fixed this by keeping track of the remainders in a set and breaking out of the loop if a remainder was repeated.
- Then I just sorted everything in a dictionary by the value and reversed it to get the largest period.
