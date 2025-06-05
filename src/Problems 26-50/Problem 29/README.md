# Problem 29 Notes
### Problem Analysis
We are given the equation $a^b$. We are told to limit $a$ to be $2 \leq a \leq 100$ and limit $b$ to $2 \leq a \leq 100$. Now find all the possible distinct terms of the generated numbers. (No duplicates)

### Plan
- I immediately think to use Python for this as this function will easily go over the max integer limit for u128 in Rust.
- I immediately think to just loop a and b in the function then just put all of the output into a dictionary.
- Then remove duplicates and print the length of the dictionary.
- Pretty easy it seems! Coding time!

### Results
Incredible speed and very easy problem!