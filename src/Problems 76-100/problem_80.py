# https://projecteuler.net/problem=80
import time
from decimal import Decimal, getcontext
start_time = time.time()
getcontext().prec = 102
total_sum = 0
for number in range(1, 101):
    sqrt = Decimal(number).sqrt()
    if len(str(sqrt)) <= 2:
        continue
    modified_string = str(sqrt)
    new = modified_string.replace(".", "")
    digits = list(map(int, new))
    index = len(digits)
    del digits[index - 1]
    del digits[index - 2]
    for digit in digits:
        total_sum += digit
print(total_sum)
print("============================")
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)