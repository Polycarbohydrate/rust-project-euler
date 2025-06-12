# https://projecteuler.net/problem=92
import time
import sys
start_time = time.time()
counter = 0
for number in range(1, 10_000_000):
    if number % 100_000 == 0:
        print(number)
    digits = list(map(int, str(number)))
    summation = 0
    for digit in digits:
        square = digit * digit
        summation += square
    while summation != 1 and summation !=89:
        digits_2 = list(map(int, str(summation)))
        summation = 0
        for digit_2 in digits_2:
            square_2 = digit_2 * digit_2
            summation += square_2
    if summation == 89:
        counter += 1
print(counter)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)
