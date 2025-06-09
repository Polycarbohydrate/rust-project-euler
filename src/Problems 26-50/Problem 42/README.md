# Problem 42
### Problem Analysis
- The formula $t_n = 1/2(n)(n + 1)$ gives the $n$th triangular number.
- By converting a word's letters to their corresponding numbers (A=1, B=2, ..., Z=26), and then adding the numbers, we can find the word's value.
- If the word's value is a triangular number, then the word is a triangular word.
- There is a given text file with words that we need to check.

### Plan
- First of all, I will need to find a way to get each word separately from the text file.
- I placed the file into my project root directory.
- I will have to first get the content of the file into a string.
- Then I will split the string at the commas to get each word.
- Then I will place each word into a vector.
- I will remove the quotes from each word.
- Next, I will iterate through the vector, break each word into characters and find the word value.
- Then I will use the formula to find if the word value is a triangular number.
- I will up $n$ until the triangular number is greater than the word value or equal to it.
- Then I will make a counter to count the number of triangular words go up by one if the word value is a triangular number.
- Seems pretty straightforward. Time to code!

### Conclusion
- Very easy and straightforward problem. Like I planned above.