# Problem 39
### Problem Analysis
- Let p be the perimeter of a right triangle with sides a, b, c.
- For p=120 there are 3 solutions 
  - (20,48,52)
  - (24,45,51)
  - (30,40,50)
- For which value of $p \leq 1000$ does the number of solutions exceed all others?

### Plan
- I will create a loop from 1 to 1000.
- I will create 3 variables a, b, c to store the sides of the triangle.
- Side c with be dependent on the values of a and b.
- $a^2 + b^2 = c^2$
- I will check if the sum of a, b, c is equal to p.
- And I will increment a and b until the side lengths match p or get too big.
- I will create a vector to store the number of solutions for each p. .
- I will create 2 variables to store the maximum number of solutions and the corresponding p value.
- I will update them whenever I find a better p value.
- Sounds pretty straightforward. Time to code!

### Conclusion
- I brute forced it by going through more combinations than needed.
- Also, I forgot to check that c can be a decimal and that only c must be an integer. Which I added a check for.