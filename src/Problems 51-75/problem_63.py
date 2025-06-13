# https://projecteuler.net/problem=63
import time
import sys
start_time = time.time()
count = 1
for base in range(2,10):
    for exponent in range(1, 100):
        num = base**exponent
        digits = list(map(int, str(num)))
        if len(digits) == exponent:
            count += 1
print(count)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)
