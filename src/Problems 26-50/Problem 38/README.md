# Problem 38
### Problem Analysis
- The problem is to find the largest 1 to 9 pandigital number that can be formed as the concatenated product of an integer with (1, 2, ..., n) where n > 1.
- For example, for  the number 192 if you multiply it by 1, 2 and 3, you get 192, 384 and 576 which concatenated gives you 192384576 which is a pandigital number.

### Plan
- My first immediate though is to find the limits of the search area.
- The largest 9-digit pandigital number is 987654321.
- So once the concatenated product exceeds this number, we can stop.
- For the main integer that I will be multiplying, I will start from 1 and go up to 200 just to start.
- For my n, I will go from 1 to 9.
- There are certainly plenty of combinations that will exceed the 9-digit pandigital number, so I will check if the concatenated product is a pandigital number and if it is larger than the largest pandigital number found so far.
- I will probably put the largest pandigital number found so far in a variable and update it whenever I find a larger pandigital number.
- As long as the concatenated product is less than or equal to the largest pandigital number, I will continue to check for larger pandigital numbers.
- Time to code!

### Coding
- I ran into a pretty big problem which was how to decide what to multiply the integer by.
- My upper bounds for my integer is 987 which I think is a good start.
- But the problem is that I don't know how many times I should multiply the integer by.
- I will start with 1 then increment my way up checking after each multiplication if the concatenated product has a length of 9. 
- I don't have to worry about it being pandigital in the multiplication function because my main function will check for that.
- I will assign the multiplier to a variable and increment it after each multiplication while also putting the product in a vector.
- I will get the product's characters individually and then push them into a vector. If that vector has a length of nine, I will return the vector.
- If the length becomes greater than 9, I will return an empty vector.
- Sounds good time to test.

### Conclusion
- This one was a bit tough.
- I did have to increase my bounds as I wasn't sure how big the integer could be.