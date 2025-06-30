# https://projecteuler.net/problem=57
import time
start_time = time.time()
iteration_count = 2
total_count = 0
while iteration_count <= 1000:
    temp_upper = 1
    temp_lower = 2
    for _ in range(1, iteration_count + 1):
        x = (2 * temp_lower) + temp_upper
        temp_upper = temp_lower
        temp_lower = x
    numerator = temp_upper + temp_lower
    denominator = temp_lower
    if len(str(numerator)) > len(str(denominator)):
        total_count += 1
    iteration_count += 1
print(total_count)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)
