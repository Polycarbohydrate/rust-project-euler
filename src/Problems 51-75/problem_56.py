# https://projecteuler.net/problem=56
import time
import sys
start_time = time.time()
sum = 0
for a in range(1, 100):
    for b in range(1, 100):
        num = a**b
        digits = list(map(int, str(num)))
        inner_sum = 0
        for digit in digits:
            inner_sum += digit
        if inner_sum > sum:
            sum = inner_sum
print(sum)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)