# https://projecteuler.net/problem=97
import time
import sys
start_time = time.time()
sys.set_int_max_str_digits(2357207)
number = 28433 * 2**7830457 + 1
digits = list(map(int, str(number)))
digits.reverse()
part = digits[:10]
part.reverse()
print(part)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)