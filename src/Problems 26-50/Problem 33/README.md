# Problem 33
### Problem Analysis
- We are given the fraction $49/98$. This is a curious fraction as some dumbass has cancelled the 9s to get $4/8$.
- Fractions such as $30/50$ are trivial examples as when canceling the 0s you do get the simplified version.
- There are 4 non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
- We have to find the product of these four fractions, simplify the product, then find the value of the denominator.

### Plan
- Use 2 nestled loops that give us all the numerators and denominators.
- Separate the numerator and denominator of each fraction and see if they have the same number in the top and bottom. Then just get rid of the shared numbers.
- With the new fraction we check to see if it is equal to the original fraction.
- If it is we store the fraction in a vector and then just do some math on paper to simplify and find the denominator.

### Conclusion
- I was too lazy to figure out the math for simplifying fractions in Rust so, I just pulled out my calculator and did it by hand.