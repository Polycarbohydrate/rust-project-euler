# Problem 28 Notes
### Problem analysis
The problem wants us to find the sum of all the diagonal numbers in a square that is composed of numbers spiraling out from the center. The spiral starts with 1 at the center and continues outward in a clockwise direction.
```
21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13
```

The start has a 1 in the center, and then the 2 to the right, then continuing clockwise, we have 3, 4, 5, and so on. Our objective is to find the sum of the numbers on the diagonals in a 1001 by 1001 spiral.

### Plan Part 1

- I will need to find a way to generate the spiral numbers.
- Or I can find a pattern that just lets me calculate the diagonal numbers without generating the entire spiral.
- Since my square is 1001 by 1001, the total number of numbers is 1001 * 1001 = 1,002,001.
- I think I will be using a step system that will let the program know where the next number is going to be.
- I will probably loop through 1-1002001 and keep track of the current position in the spiral.
- Hardest part will be knowing when to change direction.
- Looking at the example spiral from the problem, every ring of numbers from the center has a length and width of 2n + 1, where n is the number of rings from the center.
- This equation will help me know when to change direction.
- I will also remember that in the top right corner of the spiral, I will need to move onto the next ring.
- Time to implement the plan.

### Coding notes Part 1
- First problem I encountered was that I needed a way to print or have the spiral numbers in a way that I could see them. 
- Even if I didn't need to print them, it was helpful to visualize the spiral.

After some quick thinking, I realized that I could just push the numbers into a vector. Then that would just invalidate the need to have an algorithm to generate the spiral numbers. I could just use the vector to find the diagonal numbers. I could use the formula I found earlier to find the diagonal numbers. This would be a lot easier than trying to generate the spiral numbers.

### Plan Part 2
- After going into coding I realized that even if I did code the spiral numbers, I would still need to find a way to get the diagonal numbers.
- This would be tedious and time-consuming.
- I instead realized I could just put all the numbers in the vector. Then find the diagonal numbers by first finding the numbers in the corners of the spiral.
- I could do this by taking chunks of numbers from the vector that were in the same ring.
- I need a way to calculate the number of numbers in each ring.
- I can do this by using the formula I found earlier.

$x = 2n + 1$ 

This gives me the length and width of the ring. For the total amount of numbers in the ring, I can do:

$num_L = x^2$ // The number of numbers in square by the nth ring. (Larger & Inclusive)

$num_S = x^2$ // The number of numbers in square by the nth ring. (Smaller & Inclusive)

I would just need to subtract one from the larger n to get the smaller n. Then do:

$num_L - num_S$ = number of numbers in the ring.

This would give me the cutoff point for the ring and I would cut off the vector at that point.

- The next issue would be to find the corners of the ring. 
- I would just divide the ring into 4 parts and take the last number of each part.
- Rest of the issues are just ordering, where to start and end just to match my plans.

### Coding notes Part 2
- I realized that I need to calculate the number of rings in the spiral.
- I found a pattern in the original 5 by 5 spiral. 
- 1 by 1 has 1 ring.
- 3 by 3 has 2 rings.
- 5 by 5 has 3 rings.
- So for every odd number, the number of rings is increased by 1.
- To find the number of rings in a 1001 by 1001 spiral, I just need to divide 1001 by 2 and round up to 501. (Counting the center as a ring)

### Results
I coded some very inefficient code but it did work out in the end... It took 493 seconds for my laptop to run on --release mode though. I am too lazy to optimize it but I am still releasing it as it is. :)